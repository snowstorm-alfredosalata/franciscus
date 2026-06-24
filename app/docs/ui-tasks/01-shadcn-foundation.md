# Task 1 — shadcn-svelte foundation

> Part 1 of 3 in the General-UI hardening effort. **Run this first** and let it land before
> Tasks 2 (mobile-responsive) and 3 (accessibility), which build on the components introduced here.
> This file is self-contained: you do not need any prior conversation context.

## Where you are

- Repo: `/home/asalata/dev/asalata/Franciscus/franciscus`
- App: `franciscus/app` — SvelteKit **static SPA**, `ssr=false`, `adapter-static`
  (see `src/routes/+layout.ts`).
- Stack: **Svelte 5** (`^5.56`, runes), **Tailwind v4** (`^4.3`, `@tailwindcss/vite`),
  TypeScript. Dark mode via a `.dark` class on `<html>` (`@custom-variant dark` in
  `src/app.css`; bootstrap script in `src/app.html`).
- Palette today: Tailwind **stone** (neutrals) + **amber** (accent). Keep it.
- Data layer (`$lib`, `sql.js`) is unrelated to this task — do not touch it.

Run commands from `franciscus/app`:

```bash
cd /home/asalata/dev/asalata/Franciscus/franciscus/app
npm run check   # svelte-check (types + a11y)
npm run build   # static build
npm run dev     # dev server
```

## Goal

Install + configure shadcn-svelte and migrate the app's **global chrome** to its primitives,
**preserving the current look and behavior**. This establishes the component vocabulary that
Tasks 2 and 3 consume. Do **not** redesign the visual style or introduce a new palette — that is
a separate roadmap item.

## Why shadcn-svelte

It's a copy-in-source model (components live in your repo), so the only real runtime dep is
`bits-ui` (tree-shakeable headless primitives) plus a tiny `tailwind-variants`/`cn` helper.
`bits-ui` primitives bring correct focus management, keyboard nav, and ARIA for free — which
sets up Task 3 — and are the foundation for planned in-app contributions/auth/moderation UI.

## Steps

### 1. Init

```bash
npx shadcn-svelte@latest init
```

- Choose the **Tailwind v4 + Svelte 5** configuration when prompted.
- Verify it adds: `components.json`, a `cn()` util (typically `src/lib/utils.ts`), CSS variables
  / `@theme` tokens in `src/app.css`, and deps `bits-ui` + `tailwind-variants` (and clsx/
  tailwind-merge) in `package.json`.
- **Map shadcn's color tokens onto the existing stone/amber palette** so nothing changes
  visually. Do not adopt shadcn's default slate/zinc defaults if they shift the look.
- Confirm `npm run dev` still boots and `npm run build` still succeeds before migrating anything.

> **Note on `shadcn-svelte@latest` (1.3.x):** `init` now forces a "design-system preset" (icon
> library + font + theme) and overrode `--base-color stone` to `neutral`, pulling in Inter
> (`@fontsource-variable/inter`) and `@lucide/svelte`. To preserve parity: the Inter `@import` and
> `--font-sans` override were removed (chrome stays on the system stack; `@fontsource-variable/inter`
> uninstalled), and the generated grayscale semantic tokens in `src/app.css` were mapped onto the
> stone palette (neutrals) + amber (`--ring`, accents). `@lucide/svelte` is kept because the
> generated primitives import from it; existing inline SVG icons were left untouched.

### 2. Add only the primitives the app needs

```bash
npx shadcn-svelte@latest add button card badge dropdown-menu sheet tooltip breadcrumb
```

(Use `select` instead of / in addition to `dropdown-menu` if it fits the LanguagePicker better.)
**No speculative components** — add only what the migrations below consume.

### 3. Migrate global chrome (keep look + behavior identical)

| File | Today | Migrate to |
|------|-------|-----------|
| `src/lib/TopNav.svelte` | Desktop inline links + hand-rolled mobile drawer (`fixed` overlay `<div onclick>` + `<aside>`) with `open` state | Desktop: styled links / `Button` link variants. Mobile: **`Sheet`** for the drawer. |
| `src/lib/LanguagePicker.svelte` | Icon button toggling a `fixed`/absolute panel with two native `<select>`s; closes on `svelte:window` click; has suppressed a11y warnings | **`DropdownMenu`** trigger wrapping the controls (native `<select>`s may stay), **or** shadcn `Select` for each. |
| `src/routes/+layout.svelte` | Theme-toggle icon button + lang/theme cluster (`fixed top-3 right-3`); also progress/loading/error states | Theme + picker triggers as icon **`Button`** variants. Leave loading/progress markup as-is. |
| `src/routes/book/[book_id]/+page.svelte`, `book/[book_id]/[chapter_id]/+page.svelte`, `attributes/+page.svelte`, `attributes/[attr_type]/[attr_value]/+page.svelte` | Ad-hoc breadcrumb `<nav>` with literal `/` separators | **`Breadcrumb`** primitive |
| `src/app.css` `ref` rule | `::after` hover tooltip showing `attr(to)` | **Kept as the existing `::after` CSS tooltip** — see note below. Leave the verse `<v>` styling alone. |

Notes:
- **`ref` tooltip not migrated (intentional).** `<ref>` elements are injected via `{@html}` from
  database content, not authored in Svelte templates, so a bits-ui `<Tooltip>` (a Svelte component)
  cannot wrap them without a global JS hover/focus controller. To preserve exact visual parity and
  avoid that extra moving part, the `::after` CSS tooltip in `src/app.css` was kept as-is. The
  `tooltip` primitive was therefore **not** added (avoids a speculative component). Revisit in Task 3
  if accessible (focus-triggered) tooltips are required. `card` was likewise not added — no migration
  consumes it.
- The mobile drawer currently uses `pointer-events`/`z-50` hacks and `svelte-ignore` comments —
  `Sheet`/`DropdownMenu` replace those. You may leave residual `svelte-ignore` comments for
  Task 3 to clean up, but prefer removing any that no longer apply.
- Preserve all existing `aria-label`s, icons (inline SVGs are fine to keep), and the
  light/dark color classes.

### 4. De-duplicate the attribute-badge styling

The `typeColors` map (virtue/topic/event/place/person → bg/text classes, with `dark:` and
`hover:` variants) is **copy-pasted in three files**:

- `src/routes/attributes/+page.svelte`
- `src/routes/attributes/[attr_type]/[attr_value]/+page.svelte`
- `src/routes/book/[book_id]/[chapter_id]/+page.svelte`

Extract it once — a `Badge` variant keyed by attr type, or a shared map in `src/lib` — and reuse
it in all three. Keep the rendered output (colors, rounded-full pills) identical.

## Constraints

- **Visual parity on desktop.** A reviewer comparing before/after on a desktop browser should see
  essentially the same UI. Responsiveness and a11y are Tasks 2 and 3.
- Keep the **stone/amber** palette. Do **not** introduce the manuscript golden/crimson palette
  (separate roadmap line 29).
- Do not expand breadcrumbs into a new feature (line 30) — just migrate the existing markup to the
  primitive.
- Don't touch the data layer, i18n logic, service worker, or DB build.

## Acceptance

- `npm run check` passes (no new type errors).
- `npm run build` succeeds (static build).
- `npm run dev`: desktop UI visually unchanged; nav, language picker, theme toggle, breadcrumbs,
  and `ref` tooltips all still work; no new console errors.
- `components.json`, `cn()` util, and the added `src/lib/components/ui/*` primitives are committed;
  `bits-ui` + `tailwind-variants` present in `package.json`.
