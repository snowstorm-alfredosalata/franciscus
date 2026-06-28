import type { LayoutLoad } from './$types';
import type { Manifest } from '$lib/types';

// NB: we intentionally do NOT set `ssr = false` here. A layout that opts out of
// SSR also opts its `load` out of running on the server, which would leave the
// manifest undefined when the prerendered hub pages render. Instead the SPA-only
// routes (book / topic-detail / bookmarks) set `ssr = false` themselves, and the
// hub routes opt into `prerender` via their own `+page.ts`.

// Load the hub manifest in both Node (prerender) and the browser (client nav),
// so hub pages have real corpus data without touching the 12 MB sql.js DB.
export const load: LayoutLoad = async ({ fetch }) => {
	const res = await fetch('/db-manifest.json');
	if (!res.ok) throw new Error(`Failed to load db-manifest.json: ${res.status}`);
	const manifest: Manifest = await res.json();
	return { manifest };
};
