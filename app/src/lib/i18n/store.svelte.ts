import en from './en.json';
import it from './it.json';

const messages: Record<string, Record<string, unknown>> = { en, it };

export const UI_LANGUAGES = [
	{ code: 'en', label: 'English' },
	{ code: 'it', label: 'Italiano' }
] as const;

let uiLang = $state(loadPref('franciscus-ui-lang', 'en'));
let corpusLang = $state(loadPref('franciscus-corpus-lang', 'la'));

function loadPref(key: string, fallback: string): string {
	if (typeof localStorage === 'undefined') return fallback;
	return localStorage.getItem(key) ?? fallback;
}

function savePref(key: string, value: string) {
	if (typeof localStorage !== 'undefined') localStorage.setItem(key, value);
}

export function getUiLang(): string {
	return uiLang;
}

export function setUiLang(lang: string) {
	uiLang = lang;
	savePref('franciscus-ui-lang', lang);
}

export function getCorpusLang(): string {
	return corpusLang;
}

export function setCorpusLang(lang: string) {
	corpusLang = lang;
	savePref('franciscus-corpus-lang', lang);
}

export function t(path: string): string {
	const dict = messages[uiLang] ?? messages['en'];
	const keys = path.split('.');
	let node: unknown = dict;
	for (const k of keys) {
		if (node == null || typeof node !== 'object') return path;
		node = (node as Record<string, unknown>)[k];
	}
	return typeof node === 'string' ? node : path;
}
