/**
 * Single source of truth for attribute-type pill colors (virtue / topic / event /
 * place / person). Previously copy-pasted across the attributes and reader pages.
 * Returns Tailwind class strings so each call site keeps its own size/shape.
 */

// Categorical, NOT brand chrome: these five must stay mutually distinguishable.
// topic moved blue→cyan and event amber→orange so they no longer read as the
// royal-blue / gold brand colors. All pairs (700-on-100 light, 300-on-900 dark)
// clear WCAG AA. The neutral fallback uses parchment to match the new palette.
const base: Record<string, string> = {
	virtue: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900 dark:text-emerald-300',
	topic: 'bg-cyan-100 text-cyan-700 dark:bg-cyan-900 dark:text-cyan-300',
	event: 'bg-orange-100 text-orange-700 dark:bg-orange-900 dark:text-orange-300',
	place: 'bg-purple-100 text-purple-700 dark:bg-purple-900 dark:text-purple-300',
	person: 'bg-rose-100 text-rose-700 dark:bg-rose-900 dark:text-rose-300'
};

const hover: Record<string, string> = {
	virtue: 'hover:bg-emerald-200 dark:hover:bg-emerald-800',
	topic: 'hover:bg-cyan-200 dark:hover:bg-cyan-800',
	event: 'hover:bg-orange-200 dark:hover:bg-orange-800',
	place: 'hover:bg-purple-200 dark:hover:bg-purple-800',
	person: 'hover:bg-rose-200 dark:hover:bg-rose-800'
};

const fallback = 'bg-parchment-100 text-parchment-700 dark:bg-royal-900 dark:text-royal-200';

/** Color classes for an attribute type. Pass `true` to include hover variants (for links). */
export function attrColors(type: string, withHover = false): string {
	const b = base[type] ?? fallback;
	return withHover && hover[type] ? `${b} ${hover[type]}` : b;
}
