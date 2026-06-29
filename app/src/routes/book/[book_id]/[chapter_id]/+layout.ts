// The chapter reader renders paragraphs/annotations from the sql.js DB, so it
// stays client-only. `prerender = false` keeps the crawler from emitting empty
// shells for chapter URLs linked from the prerendered book index.
export const ssr = false;
export const prerender = false;
