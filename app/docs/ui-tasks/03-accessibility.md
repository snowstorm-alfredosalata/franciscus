# Task 3 — Accessibility (a11y)

> Part 3 of 3 in the General-UI hardening effort. **Run last**, after Task 1 (shadcn-svelte
> foundation) and Task 2 (mobile-responsive). This task assumes the `bits-ui`-based primitives
> from Task 1 (`Sheet`, `DropdownMenu`/`Select`, `Tooltip`, `Button`, `Breadcrumb`) are already
> in place — many keyboard/focus/ARIA concerns are handled by them, so this task mostly fixes
> what they don't cover. This file is self-contained: you do not need prior conversation context.

## Where you are

- Repo: `/home/asalata/dev/asalata/Franciscus/franciscus`, app at `franciscus/app`.
- SvelteKit static SPA (`ssr=false`), **Svelte 5** runes + **Tailwind v4**, stone/amber, `.dark`.
- i18n: UI language store under `src/lib/i18n/` (`t()`, `getUiLang`, language switcher in
  `LanguagePicker.svelte`). UI strings come from `en.json` / `it.json`.

```bash
cd /home/asalata/dev/asalata/Franciscus/franciscus/app
npm run check   # svelte-check surfaces a11y warnings — target zero a11y_* suppressions
npm run dev
```

## Goal

A clean accessibility pass: correct semantics, keyboard operability, focus management, and
screen-reader compatibility — **with the reader fully usable by keyboard and screen reader.**
Fix issues properly rather than silencing warnings.

## Issues to fix

### 1. Heading hierarchy (one `<h1>` per page)
Inner pages currently start at `<h2>` with **no `<h1>`**:
- `src/routes/book/[book_id]/+page.svelte` (book title is `<h2>`)
- `src/routes/book/[book_id]/[chapter_id]/+page.svelte` (chapter title is `<h2>`)
- `src/routes/attributes/+page.svelte` and `attributes/[attr_type]/[attr_value]/+page.svelte`
Promote the page's primary title to `<h1>` and keep sub-sections as `<h2>/<h3>` in order.
`src/routes/+page.svelte` (home) is already correct. Static pages
(`about`/`contribute`/`contact`) use `<h1>` but lack nav/landmarks — align them with the rest.

### 2. Document language reflects UI language
`src/app.html` hardcodes `lang="en"`. Sync `document.documentElement.lang` to the active UI
language client-side (mirror the existing inline theme-bootstrap `<script>` pattern in
`app.html`, and/or update it from the i18n store when the user switches language in
`LanguagePicker.svelte`).

### 3. Landmarks + skip link
- Each route already renders a `<main>` — keep exactly one per page.
- Add `aria-label`s to the distinct `<nav>`s (primary nav, breadcrumbs) so they're
  distinguishable to screen readers.
- Add a **skip-to-content** link in `src/routes/+layout.svelte` (visually hidden until focused)
  targeting the main content, since the fixed chrome comes first in the DOM.

### 4. Reader verse interaction (the key reader-compatibility fix)
`src/routes/book/[book_id]/[chapter_id]/+page.svelte` handles verse clicks via an `onclick` on a
static `<div class="chapter-content">` (around line 158), with
`<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->`.
Verses are already real elements (`<v id="...">`). Make verse selection keyboard-operable:
make each `<v>` (or a wrapping anchor) focusable and activatable via keyboard (Enter/Space),
updating the hash like the current handler does — and **remove the `svelte-ignore` comment**
rather than keep silencing it. Preserve the existing visual size of the superscript verse marks
(`src/app.css` `v { ... }`).

### 5. Remove remaining a11y suppressions
- `src/lib/LanguagePicker.svelte` had `svelte-ignore a11y_*` around its dropdown — now that it's
  a Task-1 `DropdownMenu`/`Select`, confirm Esc-to-close, focus return, and proper roles, and
  delete the suppression comments.
- `src/lib/TopNav.svelte` mobile drawer is now a Task-1 `Sheet` — confirm focus trap + Esc +
  focus return, and delete any leftover suppression comments / `pointer-events` hacks.
- Goal: **no `svelte-ignore a11y_*` comments remain** in the codebase.

### 6. `ref` tooltips keyboard/touch reachable
`<ref>` scripture tooltips were `::after`-on-hover only (`src/app.css`). The Task-1 `Tooltip`
migration should make them focusable and dismissible; verify they open on keyboard focus and via
touch, not hover alone.

### 7. Focus-visible + motion
- Ensure a clear `:focus-visible` outline/ring on all interactive elements (links, buttons,
  dropdown/sheet triggers, verse anchors). The amber ring pattern
  (`focus:ring-2 focus:ring-amber-400`) already exists on some controls — apply consistently.
- Respect `prefers-reduced-motion` for the existing `transition-*` and `animate-pulse`
  (loading bar in `+layout.svelte`) usages.

### 8. Color contrast
Spot-check text/background pairs for WCAG AA, especially light stone-on-white
(`text-stone-400/500` on white) and the attribute badge pills in both light and dark modes.
Bump shades where contrast is insufficient.

## Constraints

- Keep the stone/amber palette and visual design; a11y fixes should be visually minimal
  (skip link hidden until focus, focus rings, etc.).
- Reuse Task-1 primitives; don't reintroduce bespoke interactive markup.
- Don't touch the data layer, i18n message content, or DB build.

## Acceptance

- `npm run check` passes with **zero `svelte-ignore a11y_*`** comments remaining in the codebase.
- Keyboard-only walkthrough works end to end: skip link → primary nav → language picker (open,
  choose, Esc) → mobile `Sheet` (open, trap, Esc, focus return) → reader (focus and activate a
  verse, open a `ref` tooltip).
- `document.documentElement.lang` matches the selected UI language.
- Each page has exactly one `<h1>` and an in-order heading outline.
- An axe DevTools scan of home, a chapter (reader), the attributes index, and an attribute detail
  page reports **no critical violations**.
- `npm run build` succeeds.
