<script lang="ts">
	import { onMount } from 'svelte';
	import { getBookmarks, toggleBookmark } from '$lib/bookmarks.svelte.js';
	import { getDbState } from '$lib/dbState';
	import { t } from '$lib/i18n';
	import NoScriptNotice from '$lib/NoScriptNotice.svelte';
	import BookmarkIcon from '@lucide/svelte/icons/bookmark';

	const bookmarks = $derived(getBookmarks());

	// Bookmarks deep-link into chapters, which need the sql.js DB; keep the links
	// inert until the background download finishes.
	const db = getDbState();

	// Bookmarks come from localStorage, so render the list only after mount; the
	// prerendered HTML carries the NoScriptNotice instead of a misleading state.
	let mounted = $state(false);
	onMount(() => {
		mounted = true;
	});
</script>

<main id="main-content" tabindex="-1" class="max-w-3xl mx-auto px-4 py-8">
	<h1 class="text-2xl font-display font-bold text-foreground mb-6">{t('bookmarks.heading')}</h1>

	<NoScriptNotice />

	{#if !mounted}
		<!-- placeholder until client mount; no-JS sees the notice above -->
	{:else if bookmarks.length === 0}
		<p class="text-muted-foreground">{t('bookmarks.empty')}</p>
	{:else}
		<ul class="space-y-2">
			{#each bookmarks as b (b.href)}
				<li class="flex items-center gap-2 p-3 rounded-lg border border-border">
					<a
						href={b.href}
						aria-disabled={!db.ready}
						tabindex={db.ready ? undefined : -1}
						class="flex-1 text-foreground transition-colors {db.ready
							? 'hover:text-primary'
							: 'pointer-events-none opacity-60'}"
					>
						{b.label}
					</a>
					<button
						type="button"
						onclick={() => toggleBookmark(b.href, b.label)}
						aria-label={t('a11y.removeBookmark')}
						class="p-1 pointer-coarse:p-2 rounded text-primary hover:text-destructive focus:outline-none focus:ring-2 focus:ring-ring transition-colors"
					>
						<BookmarkIcon class="w-4 h-4" fill="currentColor" />
					</button>
				</li>
			{/each}
		</ul>
	{/if}
</main>
