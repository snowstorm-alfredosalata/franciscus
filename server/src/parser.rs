use regex::Regex;
use crate::models::*;

pub fn parse_book(input: &str, id: &str) -> Result<ParsedBook, String> {
    let (mut meta, body) = parse_frontmatter(input)?;
    meta.id = id.to_string();
    let mut chapters = parse_body(body)?;
    // Per-<p> provenance/by inherit the translation frontmatter defaults.
    // (Source files have neither, so paragraphs stay None.)
    for ch in &mut chapters {
        for block in &mut ch.blocks {
            if let Block::Paragraph { provenance, by, .. } = block {
                if provenance.is_none() {
                    *provenance = meta.provenance.clone();
                }
                if by.is_none() {
                    *by = meta.translator.clone();
                }
            }
        }
    }
    Ok(ParsedBook { meta, chapters })
}

fn parse_frontmatter(input: &str) -> Result<(BookMeta, &str), String> {
    let trimmed = input.trim_start();
    if !trimmed.starts_with("---") {
        return Err("Missing frontmatter delimiter".into());
    }
    let after_first = &trimmed[3..];
    let end = after_first
        .find("\n---")
        .ok_or("Missing closing frontmatter delimiter")?;
    let yaml_block = &after_first[..end];
    let body = &after_first[end + 4..];

    let meta = parse_yaml_frontmatter(yaml_block)?;
    Ok((meta, body))
}

fn parse_yaml_frontmatter(yaml: &str) -> Result<BookMeta, String> {
    let mut meta: BookMeta =
        serde_yaml::from_str(yaml).map_err(|e| format!("Invalid frontmatter: {e}"))?;
    // serde_yaml keeps the trailing newline a folded `>` scalar produces; trim it.
    let trim = |s: Option<String>| s.map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
    meta.description_short = trim(meta.description_short);
    meta.notes = trim(meta.notes);
    // The long description is authored as Markdown and injected via {@html}, so
    // render it to HTML now (paragraph breaks survive, unlike a single <p>).
    meta.description = trim(meta.description).map(|d| crate::render_markdown(&d));
    Ok(meta)
}

/// Extract `(id, label, provenance, by)` from a `<p>` tag's attribute string.
/// `id` is required; the rest are optional and inherit frontmatter defaults later.
fn p_attrs(
    attrs: &str,
    re_attr: &Regex,
) -> Result<(String, Option<String>, Option<String>, Option<String>), String> {
    let (mut id, mut label, mut provenance, mut by) = (None, None, None, None);
    for c in re_attr.captures_iter(attrs) {
        let val = c[2].to_string();
        match &c[1] {
            "id" => id = Some(val),
            "label" => label = Some(val),
            "provenance" => provenance = Some(val),
            "by" => by = Some(val),
            _ => {}
        }
    }
    Ok((id.ok_or("<p> missing id attribute")?, label, provenance, by))
}

fn replace_verse_markers(content: &str, paragraph_id: &str) -> String {
    let re = Regex::new(r"\[(\d+)\]").unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let n = &caps[1];
        format!(r#"<v id="{paragraph_id}-{n}">{n}</v>"#)
    })
    .into_owned()
}

fn parse_body(body: &str) -> Result<Vec<ParsedChapter>, String> {
    let re_chapter = Regex::new(r#"^##\s+(.+?)\s*<a\s+id="([^"]+)"\s*>\s*</a>\s*$"#).unwrap();
    // ponytail: attribute values may not contain '>' (the per-<p> `by` is a plain
    // name; emails live only in frontmatter `translator`). Order-independent attrs.
    let re_p_open = Regex::new(r#"^<p\s+([^>]*?)\s*>"#).unwrap();
    let re_attr = Regex::new(r#"(\w+)="([^"]*)""#).unwrap();
    let re_p_close = Regex::new(r"^</p>\s*$").unwrap();
    let re_aside_open = Regex::new(r"^<aside>\s*$").unwrap();
    let re_aside_close = Regex::new(r"^</aside>\s*$").unwrap();

    let mut chapters: Vec<ParsedChapter> = Vec::new();
    let mut chapter_pos: u32 = 0;

    enum State {
        Idle,
        InParagraph {
            id: String,
            label: Option<String>,
            provenance: Option<String>,
            by: Option<String>,
            lines: Vec<String>,
        },
        InAside { lines: Vec<String> },
    }

    let mut state = State::Idle;
    let mut block_pos: u32 = 0;
    let mut aside_pos: u32 = 0;

    for line in body.lines() {
        if let Some(caps) = re_chapter.captures(line) {
            if let State::Idle = &state {
            } else {
                return Err(format!("Unclosed block before chapter: {line}"));
            }
            let title = caps[1].to_string();
            let id = caps[2].to_string();
            chapter_pos += 1;
            block_pos = 0;
            aside_pos = 0;
            chapters.push(ParsedChapter {
                id,
                title,
                position: chapter_pos,
                blocks: Vec::new(),
            });
            continue;
        }

        match &mut state {
            State::Idle => {
                if let Some(caps) = re_p_open.captures(line) {
                    let (id, label, provenance, by) = p_attrs(&caps[1], &re_attr)?;
                    let rest = re_p_open.replace(line, "").trim().to_string();
                    let mut lines = Vec::new();
                    if !rest.is_empty() && !re_p_close.is_match(&rest) {
                        lines.push(rest);
                    }
                    if re_p_close.is_match(line) {
                        block_pos += 1;
                        let raw = lines.join("\n").trim().to_string();
                        let content = replace_verse_markers(&raw, &id);
                        if let Some(ch) = chapters.last_mut() {
                            ch.blocks.push(Block::Paragraph {
                                id,
                                label,
                                content,
                                position: block_pos,
                                provenance,
                                by,
                            });
                        }
                    } else {
                        state = State::InParagraph { id, label, provenance, by, lines };
                    }
                } else if re_aside_open.is_match(line) {
                    state = State::InAside { lines: Vec::new() };
                }
            }
            State::InParagraph { id, label, provenance, by, lines } => {
                if re_p_close.is_match(line) {
                    block_pos += 1;
                    let raw = lines.join("\n").trim().to_string();
                    let content = replace_verse_markers(&raw, id);
                    if let Some(ch) = chapters.last_mut() {
                        ch.blocks.push(Block::Paragraph {
                            id: id.clone(),
                            label: label.clone(),
                            content,
                            position: block_pos,
                            provenance: provenance.clone(),
                            by: by.clone(),
                        });
                    }
                    state = State::Idle;
                } else {
                    lines.push(line.to_string());
                }
            }
            State::InAside { lines } => {
                if re_aside_close.is_match(line) {
                    block_pos += 1;
                    aside_pos += 1;
                    let content = lines.join("\n").trim().to_string();
                    if let Some(ch) = chapters.last_mut() {
                        let id = format!("{}-aside-{}", ch.id, aside_pos);
                        ch.blocks.push(Block::Aside {
                            id,
                            content,
                            position: block_pos,
                        });
                    }
                    state = State::Idle;
                } else {
                    lines.push(line.to_string());
                }
            }
        }
    }

    Ok(chapters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_description_short_and_treats_bare_key_as_none() {
        let with = "title: T\nauthor: A\ndescription_short: \"A short blurb\"\n";
        assert_eq!(
            parse_yaml_frontmatter(with).unwrap().description_short.as_deref(),
            Some("A short blurb")
        );

        let without = "title: T\nauthor: A\ndescription_short:\n";
        assert_eq!(parse_yaml_frontmatter(without).unwrap().description_short, None);
    }

    #[test]
    fn renders_description_markdown_to_html() {
        // Folded `>` joins wrapped lines into one paragraph.
        let folded = "title: T\nauthor: A\ndescription: >\n    First line\n    second line.\n";
        assert_eq!(
            parse_yaml_frontmatter(folded).unwrap().description.as_deref(),
            Some("<p>First line second line.</p>")
        );

        // Literal `|` keeps the blank line, so Markdown yields two paragraphs.
        let literal = "title: T\nauthor: A\ndescription: |\n    One.\n\n    Two.\n";
        assert_eq!(
            parse_yaml_frontmatter(literal).unwrap().description.as_deref(),
            Some("<p>One.</p>\n<p>Two.</p>")
        );
    }
}
