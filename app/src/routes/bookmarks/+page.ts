// Bookmarks live in localStorage (client-only), but we prerender the page so the
// no-JS NoScriptNotice ships in the static HTML instead of a blank shell. The
// list itself is rendered client-side after mount.
export const prerender = true;
export const ssr = true;
