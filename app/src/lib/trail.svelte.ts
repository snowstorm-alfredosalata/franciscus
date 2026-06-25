import { untrack } from 'svelte';
import { browser } from '$app/environment';

/**
 * A single breadcrumb. The trail records where the user has actually been,
 * not the static site hierarchy, so e.g. arriving at a topic page from a
 * chapter keeps the chapter in the trail instead of snapping back to the
 * topics index.
 */
export interface Crumb {
	/** Stable identity for the destination — the pathname, no query or hash. */
	id: string;
	/** Display label, already resolved (translated / titled) at record time. */
	label: string;
	/** Navigable URL. */
	href: string;
	/**
	 * Id of this page's parent, when it has one (e.g. a chapter's book). Used to
	 * collapse sibling navigation — paging next/prev through chapters replaces the
	 * current crumb instead of stacking every chapter onto the trail.
	 */
	parentId?: string;
}

const STORAGE_KEY = 'franciscus-trail';

function load(): Crumb[] {
	if (!browser) return [];
	try {
		const raw = sessionStorage.getItem(STORAGE_KEY);
		return raw ? (JSON.parse(raw) as Crumb[]) : [];
	} catch {
		return [];
	}
}

// Persisted to sessionStorage so a reload or shared-link round-trip keeps the
// path, while a brand-new tab starts fresh.
let trail = $state<Crumb[]>(load());

function persist(value: Crumb[]) {
	if (!browser) return;
	try {
		sessionStorage.setItem(STORAGE_KEY, JSON.stringify(value));
	} catch {
		/* sessionStorage may be unavailable (private mode, quota) */
	}
}

export function getTrail(): Crumb[] {
	return trail;
}

/**
 * Clear the trail. Called when the user lands on a navigation hub (home, the
 * topics index, or a chrome page like About) — these are entry points, not
 * waypoints, so they start a fresh path rather than extending the old one.
 */
export function resetTrail() {
	if (untrack(() => trail.length) === 0) return;
	trail = [];
	persist([]);
}

/**
 * Record arrival at a page for the breadcrumb trail.
 *
 * `chain` is the page's hierarchical ancestry (root → … → current). It is used
 * only to seed a fresh trail on a cold entry (deep link, reload, or a jump from
 * a page that doesn't participate in the trail). When the user genuinely
 * traversed here from a page already in the trail, the trail is preserved and
 * only the current crumb is appended — that is what makes the breadcrumbs
 * reflect the real path rather than the site map.
 *
 * Re-visiting a page already in the trail (browser back, clicking a crumb, or
 * navigating home) truncates the forward history to that point.
 */
export function recordPage(chain: Crumb[]) {
	if (chain.length === 0) return;
	const current = chain[chain.length - 1];

	// recordPage runs inside each page's $effect. EVERY reactive read of `trail`
	// must stay lexically inside this untrack callback — including the no-change
	// comparison. Reading the proxy outside it (e.g. on a value returned from
	// untrack) re-subscribes the effect to `trail`, and writing it back below
	// then re-fires the effect forever (effect_update_depth_exceeded).
	const next = untrack(() => computeNext(trail, chain, current));

	if (next === null) return; // nothing changed
	trail = next;
	persist(next);
}

/** Pure: given the current trail, returns the next trail, or null if unchanged. */
function computeNext(t: Crumb[], chain: Crumb[], current: Crumb): Crumb[] | null {
	const existing = t.findIndex((c) => c.id === current.id);
	let result: Crumb[];
	if (existing !== -1) {
		// Returning to a page already in the trail: drop everything after it and
		// refresh the label (the UI language may have changed since).
		result = t.slice(0, existing + 1);
		result[existing] = current;
	} else if (t.length === 0) {
		// Cold entry: seed the trail from this page's own hierarchy.
		result = [...chain];
	} else {
		const last = t[t.length - 1];
		if (current.parentId && last.parentId === current.parentId) {
			// Sibling navigation (e.g. next/prev chapter): replace, don't stack.
			result = [...t.slice(0, -1), current];
		} else {
			// Continuation of an existing trail: append only the current page.
			result = [...t, current];
		}
	}
	// Skip churn when a re-firing effect produces an identical trail.
	return sameTrail(t, result) ? null : result;
}

function sameTrail(a: Crumb[], b: Crumb[]): boolean {
	if (a.length !== b.length) return false;
	return a.every((c, i) => c.id === b[i].id && c.label === b[i].label);
}
