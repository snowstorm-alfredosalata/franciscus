<script lang="ts">
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import { getTrail, type Crumb } from '$lib/trail.svelte.js';

	let { class: className = 'mb-6' }: { class?: string } = $props();

	// Collapse long trails: keep the root and the last two entries, hiding the
	// middle behind an ellipsis so the bar stays readable on every screen.
	type Entry = { ellipsis: true } | { ellipsis?: false; crumb: Crumb; current: boolean };

	const entries = $derived.by<Entry[]>(() => {
		const trail = getTrail();
		const last = trail.length - 1;
		const toEntry = (crumb: Crumb, i: number): Entry => ({ crumb, current: i === last });

		if (trail.length <= 4) {
			return trail.map(toEntry);
		}
		return [
			toEntry(trail[0], 0),
			{ ellipsis: true },
			toEntry(trail[last - 1], last - 1),
			toEntry(trail[last], last)
		];
	});
</script>

{#if getTrail().length > 1}
	<Breadcrumb.Root class={className}>
		<Breadcrumb.List class="text-sm text-muted-foreground">
			{#each entries as entry, i}
				{#if i > 0}
					<Breadcrumb.Separator>/</Breadcrumb.Separator>
				{/if}
				{#if entry.ellipsis}
					<Breadcrumb.Item>
						<Breadcrumb.Ellipsis />
					</Breadcrumb.Item>
				{:else if entry.current}
					<Breadcrumb.Item>
						<Breadcrumb.Page class="text-foreground">{entry.crumb.label}</Breadcrumb.Page>
					</Breadcrumb.Item>
				{:else}
					<Breadcrumb.Item>
						<Breadcrumb.Link href={entry.crumb.href} class="hover:text-foreground"
							>{entry.crumb.label}</Breadcrumb.Link
						>
					</Breadcrumb.Item>
				{/if}
			{/each}
		</Breadcrumb.List>
	</Breadcrumb.Root>
{/if}
