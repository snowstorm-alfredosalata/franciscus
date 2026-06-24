use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMeta {
    pub id: String,
    pub title: String,
    pub author: String,
    pub date: Option<String>,
    pub reference_edition: Option<String>,
    pub license: String,
}

// --- Parsed structures (from markdown, before DB insertion) ---

#[derive(Debug, Clone)]
pub enum Block {
    Paragraph {
        id: String,
        label: Option<String>,
        content: String,
        position: u32,
    },
    Aside {
        id: String,
        content: String,
        position: u32,
    },
}

#[derive(Debug, Clone)]
pub struct ParsedChapter {
    pub id: String,
    pub title: String,
    pub position: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct ParsedBook {
    pub meta: BookMeta,
    pub chapters: Vec<ParsedChapter>,
}

// --- JSON annotation sidecar (FORMAT.md §10) ---
// File is `books/<book_id>.json`, a flat array; book_id comes from the filename.

fn default_by_type() -> String {
    "ai".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Annotation {
    pub paragraph: String,
    /// When the annotation spans a paragraph range, the last paragraph.
    #[serde(default)]
    pub paragraph_to: Option<String>,
    /// Comma-separated `type:value` pairs; expanded into one annotation row each.
    #[serde(default)]
    pub attributes: Option<String>,
    /// Comma-separated `reltype:target` pairs; expanded into one relation row each.
    /// `target` is a cross-work paragraph key `<book_id>-<paragraph_id>`.
    #[serde(default)]
    pub relations: Option<String>,
    pub by: String,
    #[serde(default = "default_by_type")]
    pub by_type: String,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AttributePageMeta {
    pub attr_type: String,
    pub attr_value: String,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct AttributePage {
    pub meta: AttributePageMeta,
    pub content: String,
}
