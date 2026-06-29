# Handoff: make home → book links work without JS

Small, deferred cleanup. Now that `/book/<id>` is prerendered static HTML
(metadata + chapter TOC, no sql.js), the home page's book links are disabled
without JS for no reason — a leftover from when the book route needed the DB.

**Not needed for SEO/crawlers:** bots follow the `href` regardless of
`aria-disabled` / `pointer-events-none` / `tabindex=-1` (that's how the prerender
crawl found `/book/<id>` and filled the sitemap). This is purely a no-JS-human +
correctness fix. Leaving it as-is costs nothing for crawlability.

## Two changes

1. **Home book links — make unconditionally clickable.**
   `app/src/routes/+page.svelte`, the `{#each books as book}` list. Drop the
   `db.ready` gate on the `<a href="/book/{book.id}">` (the `aria-disabled`,
   `tabindex`, and `pointer-events-none opacity-60` bits). The search box keeps
   its gate — search is the one home feature that needs the DB, and the
   `NoScriptNotice` in the search slot stays accurate.

2. **Book page — add the script-needed notice for chapters.**
   `app/src/routes/book/[book_id]/+page.svelte`. Drop `<NoScriptNotice />`
   (`$lib/NoScriptNotice.svelte`) near the chapter list. The chapter reader
   (`/book/<id>/<chapter>`) genuinely needs JS/DB, so its links stay disabled
   without JS — correctly this time — and the notice explains why. The existing
   `app.noScript` i18n string already fits; no new strings.

Net: no-JS visitor can browse home → any book → full metadata + chapter list, and
only hits the "needs scripts" wall at the chapter text.

## Verify

`npm run build` (in `app/`), then open `build/book/LMj.html` with JS off — the
chapter links should render disabled with the notice above them, and the home
book links should be plain clickable anchors.
