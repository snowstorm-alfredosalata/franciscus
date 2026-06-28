use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    pub topics: Option<String>,
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

/// Topic-page YAML frontmatter. The `topic_value` is NOT carried here — it is
/// derived from the filename (see `<type>:<value>[.<lang>].md`). The `type:`
/// field is duplicated so we can sanity-check it against the filename prefix.
#[derive(Debug, Clone, Deserialize)]
pub struct TopicPageFrontmatter {
    #[serde(rename = "type")]
    pub topic_type: String,
    pub description: String,
    /// Translation files only: alternative URL slug for that language. Source
    /// files MUST NOT set this; translation files MAY.
    #[serde(default)]
    pub lang_slug: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TopicPage {
    pub frontmatter: TopicPageFrontmatter,
    pub content: String,
}

// --- Build-time manifest (app/static/db-manifest.json) ---
//
// A tiny projection of the DB the hub pages need so they can render (and
// prerender) without the 12 MB sql.js database: corpus meta, the book list,
// and the annotated-topic list. Emitted by the CLI next to `franciscus.db` in
// the same build, so the two cannot drift. The app mirrors these shapes in
// `app/src/lib/types.ts`; keep them in sync manually (no codegen).

/// Bump when the manifest layout changes incompatibly (the app may gate on it).
pub const MANIFEST_SCHEMA: u32 = 1;

#[derive(Debug, Clone, Serialize)]
pub struct Manifest {
    pub schema: u32,
    pub corpus: ManifestCorpus,
    pub books: Vec<ManifestBook>,
    pub topics: Vec<ManifestTopic>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ManifestCorpus {
    pub data_commit: String,
    pub data_commit_date: String,
    pub built_at: String,
    pub book_count: u32,
    /// Corpus translation languages (e.g. `["it"]`); the canonical Latin source
    /// is always present and not listed here.
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ManifestBook {
    pub id: String,
    pub title: String,
    pub author: String,
    pub date: Option<String>,
    /// Languages this book has a translation in.
    pub translations: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ManifestTopic {
    #[serde(rename = "type")]
    pub topic_type: String,
    pub value: String,
    pub count: u32,
    /// Localized URL slug per UI language, when one is registered. Includes
    /// every language so the client can switch UI language without the DB.
    pub slugs: BTreeMap<String, String>,
}
