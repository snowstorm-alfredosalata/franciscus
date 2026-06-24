# Task 2 — Mobile-responsive layout

> Part 2 of 3 in the General-UI hardening effort. **Run after Task 1** (shadcn-svelte
> foundation) has landed — this task assumes the `Sheet`, `Button`, `Card`, `Badge`,
> `Breadcrumb`, and `Tooltip` primitives from `src/lib/components/ui/` already exist and that
> global chrome (TopNav, LanguagePicker, theme toggle, breadcrumbs) is built on them.
> This file is self-contained: you do not need any prior conversation context.

## Where you are

- Repo: `/home/asalata/dev/asalata/Franciscus/franciscus`, app at `franciscus/app`.
- SvelteKit static SPA, **Svelte 5** runes + **Tailwind v4**, stone/amber palette, `.dark` mode.
- Viewport meta is already correct (`src/app.html`: `width=device-width, initial-scale=1`).
- Tailwind breakpoint in use today is `md:` (768px). The app already has *some* responsive logic
  (e.g. TopNav desktop vs mobile, Footer repositioning).

```bash
cd /home/asalata/dev/asalata/Franciscus/franciscus/app
npm run build
npm run dev   # then use DevTools device toolbar
```

## Goal

Make **every page usable and clean from ~320px wide up** to desktop. No horizontal scroll, no
fixed chrome covering content, comfortable tap targets, sensible reflow.

## Known problem areas (verify + fix)

### Fixed chrome overlapping content
`src/routes/+layout.svelte` positions three fixed clusters over the page:
- `TopNav` at `fixed top-4 left-4`
- language picker + theme toggle at `fixed top-3 right-3`
- `Footer` (Verbum Caro logo, `h-12`) `fixed` bottom-center on mobile (`src/lib/Footer.svelte`)

The layout compensates only with `pt-20 md:pt-24 pb-16 md:pb-20` padding on the content wrapper.
Confirm at small widths that the top clusters don't collide with each other or with page `<h1>`/
breadcrumbs, and that the bottom logo never overlaps the last lines of content or the
chapter-nav prev/next row. Adjust padding/positioning (or reflow) as needed.

### Reader (`src/routes/book/[book_id]/[chapter_id]/+page.svelte`)
- Each paragraph renders: a fixed-width label span (`inline-block min-w-8 ... mr-2`) + the
  `.para-text` body + an annotation-badge row indented with `ml-10`. On narrow screens this
  indent + the wrapping pills can get cramped — make the label/text/badges reflow cleanly.
- Annotation badge row (`flex flex-wrap gap-1`) — confirm pills wrap, don't overflow.
- Chapter prev/next nav (`flex justify-between`, each link `w-1/2`) — confirm long chapter titles
  wrap rather than overflow or collide in the middle.

### Other pages
- Home / search (`src/routes/+page.svelte`): search input, result cards (`block p-4 ... border`),
  source list — check padding and wrapping. `max-w-3xl mx-auto px-4` containers are the norm.
- Book index (`book/[book_id]/+page.svelte`): chapter list cards.
- Attributes index (`attributes/+page.svelte`): badge grids (`flex flex-wrap gap-2`).
- Attribute detail (`attributes/[attr_type]/[attr_value]/+page.svelte`): occurrence cards +
  `prose` intro block.
- Static pages (`about`, `contribute`, `contact`): `max-w-4xl px-4` — light check only.
- Breadcrumbs (now the Task-1 `Breadcrumb` primitive): confirm they wrap/truncate gracefully on
  narrow widths.

### Tap targets
Ensure interactive elements are **≥ 44×44px** on touch: nav links, the menu/`Sheet` trigger,
theme + language triggers, dropdown items, and the reader's verse anchors (`<v>` elements are
small superscripts — give them adequate hit area without changing their visual size, or rely on
Task 3's anchor treatment).

## Constraints

- Visual parity on desktop — this task only changes behavior at smaller widths (and any minor
  desktop spacing needed to support it).
- Keep the stone/amber palette and existing dark-mode classes.
- Prefer Tailwind responsive utilities (`sm:`/`md:`/`lg:`) over custom CSS; reuse the Task-1
  primitives rather than re-introducing bespoke markup.
- Don't touch the data layer, i18n, or DB build.

## Acceptance

- `npm run build` succeeds.
- Manual check in DevTools device toolbar at **320 / 375 / 768 / 1024 px**:
  - No horizontal scrollbar at any width.
  - Fixed chrome never covers headings, breadcrumbs, body text, or the chapter prev/next row.
  - Reader paragraphs, badges, and chapter nav reflow cleanly.
  - All interactive targets are comfortably tappable.
- `npm run check` still passes.
