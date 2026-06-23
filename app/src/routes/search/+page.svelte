<script lang="ts">
	import { searchParagraphs } from '$lib';
	import { t, getCorpusLang } from '$lib/i18n';

	let inputValue = $state('');
	let query = $state('');

	$effect(() => {
		const val = inputValue;
		const timer = setTimeout(() => {
			query = val;
		}, 200);
		return () => clearTimeout(timer);
	});

	const corpusLang = $derived(getCorpusLang());

	const results = $derived.by(() => {
		if (!query.trim()) return [];
		try {
			return searchParagraphs(query, corpusLang);
		} catch (e) {
			console.error('search failed:', e);
			return [];
		}
	});

	function resultUrl(r: { book_id: string; chapter_id: string; paragraph_id: string }): string {
		return `/book/${r.book_id}/${r.chapter_id}?q=${encodeURIComponent(query)}#${r.paragraph_id}`;
	}
</script>

<main class="max-w-3xl mx-auto px-4 py-8">
	<nav class="text-sm text-stone-400 dark:text-stone-500 mb-6">
		<a href="/" class="hover:text-stone-600 dark:hover:text-stone-300">{t('nav.sources')}</a>
		<span> / </span>
		<span class="text-stone-600 dark:text-stone-300">{t('search.heading')}</span>
	</nav>

	<h1 class="text-2xl font-serif font-bold text-stone-800 dark:text-stone-100 mb-6">{t('search.heading')}</h1>

	<input
		type="search"
		bind:value={inputValue}
		placeholder={t('search.placeholder')}
		class="w-full px-4 py-3 rounded-lg border border-stone-300 dark:border-stone-600
		       bg-white dark:bg-stone-800 text-stone-800 dark:text-stone-100
		       placeholder:text-stone-400 dark:placeholder:text-stone-500
		       focus:outline-none focus:ring-2 focus:ring-amber-400 dark:focus:ring-amber-600
		       font-serif text-lg"
	/>

	{#if query.trim()}
		<p class="text-sm text-stone-400 dark:text-stone-500 mt-4 mb-2">
			{results.length} {results.length === 1 ? t('search.resultCountOne') : t('search.resultCount')}
		</p>
	{/if}

	{#if results.length > 0}
		<ul class="space-y-4 mt-4">
			{#each results as r}
				<li>
					<a
						href={resultUrl(r)}
						class="block p-4 rounded-lg border border-stone-200 dark:border-stone-700
						       hover:border-stone-400 dark:hover:border-stone-500 transition-colors group"
					>
						<div class="text-sm text-stone-500 dark:text-stone-400 mb-1">
							<span class="font-medium text-stone-700 dark:text-stone-300">{r.book_title}</span>
							<span> &mdash; </span>
							<span>{r.chapter_title}</span>
							{#if r.paragraph_label}
								<span class="text-stone-400 dark:text-stone-500"> &sect;{r.paragraph_label}</span>
							{/if}
						</div>
						<p class="font-serif text-stone-700 dark:text-stone-300 leading-relaxed">
							{@html r.snippet}
						</p>
					</a>
				</li>
			{/each}
		</ul>
	{:else if query.trim()}
		<p class="text-stone-500 dark:text-stone-400 mt-6 font-serif">{t('search.noResults')}</p>
	{/if}
</main>
