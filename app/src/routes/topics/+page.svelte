<script lang="ts">
	import { topicColors } from '$lib/topicColors';
	import NoScriptNotice from '$lib/NoScriptNotice.svelte';
	import { getDbState } from '$lib/dbState';
	import { t, getUiLang } from '$lib/i18n';
	import type { PageData } from './$types';

	// Topic list comes from the manifest (root layout load), so this hub renders
	// and prerenders without the sql.js DB.
	let { data }: { data: PageData } = $props();
	const uiLang = $derived(getUiLang());

	// Topic-detail pages read occurrences from the sql.js DB, so their links are
	// inert until the background download finishes (permanently so with no JS).
	const db = getDbState();

	type TopicEntry = { topic_type: string; topic_value: string; count: number; lang_slug: string | null };

	const grouped = $derived.by(() => {
		const map = new Map<string, TopicEntry[]>();
		for (const tpc of data.manifest.topics) {
			const entry: TopicEntry = {
				topic_type: tpc.type,
				topic_value: tpc.value,
				count: tpc.count,
				lang_slug: tpc.slugs[uiLang] ?? null
			};
			const list = map.get(entry.topic_type) ?? [];
			list.push(entry);
			map.set(entry.topic_type, list);
		}
		return map;
	});

	const typeOrder = ['virtue', 'theme', 'event', 'place', 'person'];
</script>

<main id="main-content" tabindex="-1" class="max-w-3xl mx-auto px-4 py-8">
	<h1 class="text-2xl font-display font-bold text-foreground mb-6">{t('topics.heading')}</h1>
	<p class="text-muted-foreground mb-8">
		{t('topics.description')}
	</p>

	<NoScriptNotice />

	{#each typeOrder as topicType}
		{@const items = grouped.get(topicType)}
		{#if items && items.length > 0}
			<section class="mb-8">
				<h2 class="text-lg font-display text-foreground mb-3">
					{t(`topics.typePlurals.${topicType}`)}
				</h2>
				<div class="flex flex-wrap gap-2">
					{#each items as topic}
						<a
							href="/topics/{topic.topic_type}/{topic.topic_value}"
							aria-disabled={!db.ready}
							tabindex={db.ready ? undefined : -1}
							class="inline-flex items-center gap-1.5 text-sm px-3 py-1.5 rounded-full transition-colors {topicColors(topic.topic_type, true)} {db.ready
								? ''
								: 'pointer-events-none opacity-60'}"
						>
							{(topic.lang_slug ?? topic.topic_value).replaceAll('_', ' ')}
							<span class="text-xs opacity-60">({topic.count})</span>
						</a>
					{/each}
				</div>
			</section>
		{/if}
	{/each}
</main>
