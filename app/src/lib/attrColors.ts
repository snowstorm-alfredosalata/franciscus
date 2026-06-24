/**
 * Single source of truth for attribute-type pill colors (virtue / topic / event /
 * place / person). Previously copy-pasted across the attributes and reader pages.
 * Returns Tailwind class strings so each call site keeps its own size/shape.
 */

const base: Record<string, string> = {
	virtue: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900 dark:text-emerald-300',
	topic: 'bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300',
	event: 'bg-amber-100 text-amber-700 dark:bg-amber-900 dark:text-amber-300',
	place: 'bg-purple-100 text-purple-700 dark:bg-purple-900 dark:text-purple-300',
	person: 'bg-rose-100 text-rose-700 dark:bg-rose-900 dark:text-rose-300'
};

const hover: Record<string, string> = {
	virtue: 'hover:bg-emerald-200 dark:hover:bg-emerald-800',
	topic: 'hover:bg-blue-200 dark:hover:bg-blue-800',
	event: 'hover:bg-amber-200 dark:hover:bg-amber-800',
	place: 'hover:bg-purple-200 dark:hover:bg-purple-800',
	person: 'hover:bg-rose-200 dark:hover:bg-rose-800'
};

const fallback = 'bg-stone-100 text-stone-700 dark:bg-stone-800 dark:text-stone-300';

/** Color classes for an attribute type. Pass `true` to include hover variants (for links). */
export function attrColors(type: string, withHover = false): string {
	const b = base[type] ?? fallback;
	return withHover && hover[type] ? `${b} ${hover[type]}` : b;
}
