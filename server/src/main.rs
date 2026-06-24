mod models;
mod parser;
mod db;

use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "franciscus-server")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Build {
        #[arg(long)]
        data_dir: PathBuf,
        #[arg(long, default_value = "franciscus.db")]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Build { data_dir, output } => run_build(&data_dir, &output),
    }
}

fn parse_attribute_page(text: &str) -> Result<models::AttributePage, String> {
    let text = text.trim();
    if !text.starts_with("---") {
        return Err("Missing YAML frontmatter".to_string());
    }
    let after_first = &text[3..];
    let end = after_first.find("---").ok_or("Missing closing --- in frontmatter")?;
    let yaml_str = &after_first[..end];
    let body = after_first[end + 3..].trim().to_string();
    let meta: models::AttributePageMeta =
        serde_yaml::from_str(yaml_str).map_err(|e| format!("YAML parse error: {e}"))?;
    Ok(models::AttributePage { meta, content: body })
}

fn run_build(data_dir: &PathBuf, output: &PathBuf) {
    let output_str = output.to_str().expect("Invalid output path");

    if output.exists() {
        std::fs::remove_file(output).expect("Failed to remove existing database");
    }

    let conn = db::open_or_create(output_str);
    db::create_tables(&conn);

    let mut book_count = 0u32;
    let mut translation_count = 0u32;
    let mut annotation_count = 0u32;
    let mut attr_page_count = 0u32;

    let books_dir = data_dir.join("books");
    let mut translation_files: Vec<PathBuf> = Vec::new();
    let mut annotation_files: Vec<PathBuf> = Vec::new();

    if books_dir.is_dir() {
        for entry in std::fs::read_dir(&books_dir).expect("Cannot read books directory") {
            let entry = entry.expect("Cannot read entry");
            let path = entry.path();
            match path.extension().and_then(|e| e.to_str()) {
                Some("md") => {
                    let stem = path.file_stem().unwrap().to_string_lossy();
                    if stem.contains('.') {
                        translation_files.push(path);
                    } else {
                        let text = std::fs::read_to_string(&path).expect("Cannot read file");
                        match parser::parse_book(&text) {
                            Ok(book) => {
                                println!("  book: {} ({})", book.meta.title, book.meta.id);
                                db::insert_book(&conn, &book);
                                book_count += 1;
                            }
                            Err(e) => {
                                eprintln!("  error parsing {}: {e}", path.display());
                            }
                        }
                    }
                }
                // Annotation sidecar; deferred so its paragraphs exist first (FK).
                Some("json") => annotation_files.push(path),
                _ => {}
            }
        }
    }

    for path in &translation_files {
        let stem = path.file_stem().unwrap().to_string_lossy();
        let dot_pos = stem.find('.').unwrap();
        let lang = &stem[dot_pos + 1..];
        let text = std::fs::read_to_string(path).expect("Cannot read translation file");
        match parser::parse_book(&text) {
            Ok(book) => {
                println!("  translation: {} [{}]", book.meta.id, lang);
                db::insert_translations(&conn, &book, lang);
                translation_count += 1;
            }
            Err(e) => {
                eprintln!("  error parsing translation {}: {e}", path.display());
            }
        }
    }

    for path in &annotation_files {
        let book_id = path.file_stem().unwrap().to_string_lossy();
        let text = std::fs::read_to_string(path).expect("Cannot read annotation file");
        match serde_json::from_str::<Vec<models::Annotation>>(&text) {
            Ok(annotations) => {
                let (attr_rows, rel_rows) = db::insert_annotations(&conn, &book_id, &annotations);
                println!("  annotations: {book_id} ({} entries, {attr_rows} attribute + {rel_rows} relation rows)", annotations.len());
                annotation_count += (attr_rows + rel_rows) as u32;
            }
            Err(e) => {
                eprintln!("  error parsing {}: {e}", path.display());
            }
        }
    }

    let attributes_dir = data_dir.join("attributes");
    if attributes_dir.is_dir() {
        for entry in std::fs::read_dir(&attributes_dir).expect("Cannot read attributes dir") {
            let entry = entry.expect("Cannot read entry");
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "md") {
                let text = std::fs::read_to_string(&path).expect("Cannot read attribute file");
                match parse_attribute_page(&text) {
                    Ok(page) => {
                        println!("  attribute: {} / {}", page.meta.attr_type, page.meta.attr_value);
                        db::insert_attribute_page(&conn, &page);
                        attr_page_count += 1;
                    }
                    Err(e) => {
                        eprintln!("  error parsing {}: {e}", path.display());
                    }
                }
            }
        }
    }

    db::create_fts_index(&conn);
    println!("  fts5 search index built");

    println!(
        "Build complete: {} book(s), {} translation(s), {} annotation(s), {} attribute page(s) -> {}",
        book_count,
        translation_count,
        annotation_count,
        attr_page_count,
        output.display()
    );
}
