// Book + chapter routes render entirely client-side from sql.js; keep them out
// of SSR/prerender (the root layout defaults to SSR for the prerendered hubs).
export const ssr = false;
