# franciscus-app

Web portal for exploring Franciscan sources: texts by Celano, Bonaventure and others, with thematic annotations and cross-work parallels.

## Requirements

- Rust 1.75+ (edition 2021)
- The [franciscus-data](../franciscus-data) repo for source texts

## Quick start

```bash
# Clone the data repo (if you haven't already)
git clone <url-franciscus-data> ../franciscus-data

# Import texts into the SQLite database
cargo run -- import ../franciscus-data/books

# Start the server
cargo run -- serve
```

The portal will be available at `http://127.0.0.1:3000`.

## Commands

```
franciscus-web import <data_dir>      Import books and annotations into franciscus.db
franciscus-web serve [--port N]       Start the web server (default: port 3000)
                     [--db PATH]      SQLite database path (default: franciscus.db)
```

## Stack

| Component   | Technology                         |
|-------------|------------------------------------|
| Framework   | Leptos 0.7 (SSR)                   |
| Server      | Axum 0.7 + Tokio                   |
| Database    | SQLite (rusqlite, bundled)         |
| Parser      | Regex on custom Markdown format    |
| Styling     | Vanilla CSS (`style/main.css`)     |

## Structure

```
src/
  main.rs      CLI (import / serve), Axum + Leptos setup
  parser.rs    Parser for the source Markdown format
  db.rs        SQLite schema, import and queries
  models.rs    Data structures (Book, Chapter, Paragraph, Annotation, ...)
  app.rs       Leptos components (Home, Book, Chapter, navigation)
style/
  main.css     Portal stylesheet
```

## Routes

| Path                              | Page                      |
|-----------------------------------|---------------------------|
| `/`                               | List of works             |
| `/book/:book_id`                  | Chapters of a work        |
| `/book/:book_id/chapter/:ch_id`   | Chapter reading view      |

## License

[AGPL-3.0-or-later](LICENSE) — see the LICENSE file for the full text.
