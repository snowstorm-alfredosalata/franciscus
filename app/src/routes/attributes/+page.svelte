<script lang="ts">
	import { getDistinctAttributes, type AttributeSummary } from '$lib';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import { attrColors } from '$lib/attrColors';
	import { t } from '$lib/i18n';

	const attributes = getDistinctAttributes();

	const grouped = $derived.by(() => {
		const map = new Map<string, AttributeSummary[]>();
		for (const a of attributes) {
			const list = map.get(a.attr_type) ?? [];
			list.push(a);
			map.set(a.attr_type, list);
		}
		return map;
	});

	const typeOrder = ['virtue', 'topic', 'event', 'place', 'person'];
</script>

<main id="main-content" tabindex="-1" class="max-w-3xl mx-auto px-4 py-8">
	<Breadcrumb.Root class="mb-6">
		<Breadcrumb.List class="text-sm text-muted-foreground">
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/" class="hover:text-foreground">{t('nav.sources')}</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator>/</Breadcrumb.Separator>
			<Breadcrumb.Item>
				<Breadcrumb.Page class="text-foreground">{t('nav.attributes')}</Breadcrumb.Page>
			</Breadcrumb.Item>
		</Breadcrumb.List>
	</Breadcrumb.Root>

	<h1 class="text-2xl font-display font-bold text-foreground mb-6">{t('attributes.heading')}</h1>
	<p class="text-muted-foreground mb-8">
		{t('attributes.description')}
	</p>

	{#each typeOrder as attrType}
		{@const items = grouped.get(attrType)}
		{#if items && items.length > 0}
			<section class="mb-8">
				<h2 class="text-lg font-display text-foreground mb-3">
					{t(`attributes.typePlurals.${attrType}`)}
				</h2>
				<div class="flex flex-wrap gap-2">
					{#each items as attr}
						<a
							href="/attributes/{attr.attr_type}/{attr.attr_value}"
							class="inline-flex items-center gap-1.5 text-sm px-3 py-1.5 rounded-full transition-colors {attrColors(attr.attr_type, true)}"
						>
							{attr.attr_value.replaceAll('_', ' ')}
							<span class="text-xs opacity-60">({attr.count})</span>
						</a>
					{/each}
				</div>
			</section>
		{/if}
	{/each}
</main>
