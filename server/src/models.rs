use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// Deserialized straight from the YAML frontmatter via serde_yaml, so block
// scalars (`description: >`), quoting, and bare `key:` → null all work. `id`
// and any absent optional come from `#[serde(default)]`; `id` is then set from
// the filename by the parser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMeta {
    /// Derived from the filename, not the frontmatter.
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub author: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub reference_edition: Option<String>,
    /// Where the source-language (Latin) text was obtained, on base `<id>.md`
    /// files. Feeds the book page's editorial note for the source rendition,
    /// the counterpart of `translation_source` for translations.
    #[serde(default)]
    pub source: Option<String>,
    // Editorial descriptions are NOT frontmatter — they live in the per-book
    // sidecar (`books/<id>.yaml`, keyed by UI language) since they are an
    // annotation about the work, not part of any rendition. See `BookSidecar`.
    // Translation-only frontmatter; None on source `<id>.md` files. These carry
    // the rendition's provenance, from which the book page's editorial note is
    // generated client-side (per UI language). The hand-authored `notes` field
    // was retired in favour of this — see the provenance note generator in the
    // app. Extra frontmatter keys (e.g. a stale `notes:`) are ignored by serde.
    #[serde(default)]
    pub translator: Option<String>,
    #[serde(default)]
    pub provenance: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub translation_source: Option<String>,
}

// --- Parsed structures (from markdown, before DB insertion) ---

#[derive(Debug, Clone)]
pub enum Block {
    Paragraph {
        id: String,
        label: Option<String>,
        content: String,
        position: u32,
        // Per-paragraph translation provenance; inherits frontmatter defaults.
        // Always None on source `<id>.md` paragraphs.
        provenance: Option<String>,
        by: Option<String>,
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

// --- Per-book YAML sidecar (FORMAT.md §10) ---
// File is `books/<book_id>.yaml`; book_id comes from the filename. It carries
// book-level "cover" properties (editorial descriptions, keyed by UI language)
// at the top, and the paragraph annotations nested under `annotations`.

fn default_provenance() -> String {
    "ai".to_string()
}

/// The whole sidecar. Both sections are optional so a book may have only
/// annotations, only cover descriptions, or both. `description_short` /
/// `description` map a UI language code (`en`, `it`, …) to its text; the long
/// `description` value is authored as Markdown and rendered to HTML at ingest.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct BookSidecar {
    #[serde(default)]
    pub description_short: BTreeMap<String, String>,
    #[serde(default)]
    pub description: BTreeMap<String, String>,
    #[serde(default)]
    pub annotations: Vec<Annotation>,
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
    /// `ai` · `reviewed` · `human`; defaults to `ai`.
    #[serde(default = "default_provenance")]
    pub provenance: String,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Topic-page YAML frontmatter. Both `topic_type` and `topic_value` are NOT
/// carried here — they are derived from the path (`topics/<type>/<value>[.<lang>].md`).
#[derive(Debug, Clone, Deserialize)]
pub struct TopicPageFrontmatter {
    pub description: String,
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
pub const MANIFEST_SCHEMA: u32 = 3;

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
    pub reference_edition: Option<String>,
    /// Base descriptions (English default; see `BookMeta`). Localized variants
    /// come from the DB once it loads; the manifest carries the base for
    /// prerender. The book page's editorial note is generated from rendition
    /// provenance (DB-only), so it is not carried here.
    pub description_short: Option<String>,
    pub description: Option<String>,
    /// Source-language chapter list, in reading order, so `/book/<id>` can
    /// prerender its table of contents without the sql.js DB.
    pub chapters: Vec<ManifestChapter>,
    /// Languages this book has a translation in.
    pub translations: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ManifestChapter {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ManifestTopic {
    #[serde(rename = "type")]
    pub topic_type: String,
    pub value: String,
    pub count: u32,
    /// Source-language label (the base topic-page description).
    pub description: String,
    /// Localized label per UI language. Includes every language with a topic
    /// translation, so the client can switch UI language without the DB.
    pub descriptions: BTreeMap<String, String>,
}
