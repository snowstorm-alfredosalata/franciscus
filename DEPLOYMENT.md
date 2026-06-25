# Deployment

Franciscus is a fully static site: a SvelteKit SPA plus a prebuilt SQLite
database that the browser queries client-side via sql.js (WASM). There is **no
application server on the read path**, so "deploying" means building two
artifacts and copying them to a static host:

1. **`franciscus.db`** — the SQLite + FTS5 corpus, built by the Rust CLI in
   [`server/`](server/) from the Markdown sources in the **separate**
   [`franciscus-data`](https://github.com/snowstorm-alfredosalata/franciscus-data)
   repo.
2. **The static SPA** — built by SvelteKit (`adapter-static`) into `app/build/`,
   with `franciscus.db` and `sql-wasm.wasm` shipped as static assets.

This document describes the GitHub Actions → GitHub Pages pipeline that produces
and publishes both. The workflow is [`.github/workflows/deploy.yml`](.github/workflows/deploy.yml).

> Status: **drafted, not yet run.** Treat this as a starting point — see
> [Before the first run](#before-the-first-run) for the manual setup steps that
> can't be expressed in the workflow file.

---

## How the build maps to the Makefile

The workflow deliberately runs the same target a developer runs locally rather
than reimplementing the build:

```
make app   ==   make install   (npm install + copy fts5-sql-bundle wasm to app/static/)
             +  make db        (cargo run -- build  →  app/static/franciscus.db)
             +  npm run build   (SvelteKit → app/build/)
```

The only thing CI has to arrange is the **two-repo layout**, because
`make db` defaults to `DATA_DIR ?= ../franciscus-data`. The workflow checks the
app repo into `franciscus/` and the data repo into `franciscus-data/` so they
sit side by side, exactly as on a developer's machine — no `DATA_DIR` override
needed.

```
runner workspace/
├── franciscus/        ← this repo (app + server)
└── franciscus-data/   ← corpus repo, checked out by the workflow
```

## Pipeline shape

Two jobs, the standard GitHub Pages pattern:

- **build** — checkout both repos → set up Rust + Node → `make app` → write
  `CNAME` → upload `app/build/` as a Pages artifact.
- **deploy** — `actions/deploy-pages@v4` publishes the artifact.

Publishing uses the **official GitHub Pages "Actions" source** (artifact +
`deploy-pages`), *not* a `gh-pages` branch. Rationale:

- No build output committed to git — keeps history clean and the repo small
  (the `.db` alone is ~4.5 MB and changes every build).
- First-class deploy environment, URL, and concurrency handling.
- It is the approach GitHub now documents and recommends.

A `gh-pages` branch (e.g. `peaceiris/actions-gh-pages`) remains a valid
alternative if you ever want a browsable built branch; it is not used here.

## Triggers

| Trigger | When | Why |
|---|---|---|
| `push` to `master` | app code/content changes | normal deploys |
| `workflow_dispatch` | manual, from the Actions tab | redeploy without a commit |
| `repository_dispatch` (`corpus-updated`) | the data repo changes | rebuild when the corpus changes — see below |

---

## Hosting model: custom domain at the root

The app is configured to be served from a **domain root**, not a subpath. This
is not optional with the current code:

- [`src/lib/db.ts`](app/src/lib/db.ts) fetches the database from the absolute
  path **`/franciscus.db`** and the WASM from **`/sql-wasm.wasm`**.
- There is no SvelteKit `base` path configured.

So the site must live at `https://<domain>/`, e.g. a custom domain like
`franciscus.example.org`. The workflow writes a `CNAME` file into the artifact
so GitHub Pages serves it under that domain.

### Setting the domain

Pick **one** of:

1. **Repo variable (what the workflow expects).** Set an Actions *variable*
   `PAGES_CUSTOM_DOMAIN` (Settings → Secrets and variables → Actions →
   Variables) to your domain. The workflow writes it to `app/build/CNAME` on
   every run. Nothing domain-specific is committed to the repo.
2. **Committed file.** Create `app/static/CNAME` containing the domain.
   `adapter-static` copies `static/` verbatim into `build/`, so it ends up in
   the artifact automatically. If you choose this, delete the "Write CNAME" step
   from the workflow.

### DNS

Point the domain at GitHub Pages:

- **Subdomain** (e.g. `franciscus.example.org`): a `CNAME` record →
  `snowstorm-alfredosalata.github.io`.
- **Apex** (e.g. `example.org`): `A`/`AAAA` records to GitHub's Pages IPs (or
  an `ALIAS`/`ANAME` if your DNS provider supports it).

Then, in the repo, Settings → Pages → set the custom domain and enable
"Enforce HTTPS" once the certificate is issued.

### If you ever drop the custom domain

To serve from the project-page URL
`https://snowstorm-alfredosalata.github.io/franciscus/` instead, the absolute
paths break and you must:

1. Configure a SvelteKit `base` path of `/franciscus` (via `kit.paths.base`).
2. Change `DB_URL` and the `locateFile` WASM path in
   [`app/src/lib/db.ts`](app/src/lib/db.ts) (and any other root-absolute asset
   references, e.g. in the service worker) to be prefixed with `base`.
3. Remove the CNAME handling.

This is the only scenario that requires code changes; the chosen custom-domain
setup needs none.

---

## Before the first run

These steps can't live in the workflow file and must be done once in the GitHub
UI / settings:

1. **Make both repos public** (or, for `franciscus-data`, provide a token — see
   below). The workflow checks out `franciscus-data` from a second repository.
2. **Enable Pages with the Actions source.** Repo → Settings → Pages → "Build
   and deployment" → Source = **GitHub Actions**.
3. **Set the custom domain** — either the `PAGES_CUSTOM_DOMAIN` Actions variable
   or `app/static/CNAME` (see above) — and configure DNS.
4. **Confirm the default branch** is `master` (both repos currently use
   `master`; the workflow triggers are set accordingly).

### Cross-repo checkout token

`actions/checkout` for the second repo uses
`${{ secrets.DATA_REPO_TOKEN || github.token }}`:

- **Both repos public:** nothing to do — the default `GITHUB_TOKEN` can read a
  public repo.
- **`franciscus-data` still private** (e.g. before launch): create a secret
  `DATA_REPO_TOKEN` — a PAT or fine-grained token with **read** access to
  `franciscus-data` — and the checkout uses it automatically.

---

## Rebuilding when the corpus changes

Because the corpus lives in a different repo, a push to `franciscus-data` does
**not** by itself trigger a deploy here. Two ways to handle this:

- **Manual / lazy (default):** the site rebuilds on the next push to this repo,
  or whenever you hit "Run workflow" in the Actions tab. Fine for editorial-pace
  content.
- **Automatic:** the optional companion workflow
  [`franciscus-data/.github/workflows/trigger-rebuild.yml`](https://github.com/snowstorm-alfredosalata/franciscus-data)
  sends a `repository_dispatch` (`corpus-updated`) to this repo on every corpus
  push. This requires a secret `APP_REPO_DISPATCH_TOKEN` **in the data repo** —
  a token with `contents: write` on `franciscus`, because the default
  `GITHUB_TOKEN` cannot dispatch across repositories.

The `deploy.yml` workflow already listens for the `corpus-updated` event, so
once the data-repo workflow and its token are in place, corpus edits deploy
automatically.

---

## Toolchain notes

- **Rust:** stable (`dtolnay/rust-toolchain@stable`); `server/` is edition 2021
  and builds `rusqlite` with the `bundled` feature, so no system SQLite is
  needed. Cargo build + registry are cached with `Swatinem/rust-cache`.
- **Node:** pinned to 22 (Vite 8 / Svelte 5 need a current LTS). npm cache keyed
  on `app/package-lock.json`.
- **WASM source:** `make install` copies `sql-wasm.wasm` from
  **`fts5-sql-bundle`**, not stock `sql.js` — FTS5 search depends on this. Do
  not change it.

## Local dry run

You can exercise the exact build the workflow performs without GitHub:

```bash
# from a parent dir containing both checkouts side by side
cd franciscus
make app
# → app/build/ is the deployable artifact; serve it from a root path:
npx serve app/build      # or any static server; visit http://localhost:3000
```

If `franciscus.db` fails to load when served from a subpath, that's the
root-path constraint described above — serve from `/`.
