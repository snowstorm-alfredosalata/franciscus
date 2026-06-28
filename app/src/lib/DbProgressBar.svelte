<script lang="ts">
	import { t } from '$lib/i18n';
	import type { DbProgress } from '$lib/db';

	// Shared indicator for the background corpus-DB download. The progress is
	// real (db.ts streams the body and reports byte counts); we show a determinate
	// bar whenever the server sent a Content-Length, and a sweeping indeterminate
	// bar otherwise. The label starts as "Loading…" (app shell / pre-download) and
	// becomes "Downloading…" once bytes are actually arriving over the network.
	let { progress }: { progress: DbProgress | null } = $props();

	const pct = $derived(
		progress && progress.total ? Math.round((progress.loaded / progress.total) * 100) : null
	);
	const label = $derived(progress && !progress.cached ? t('app.downloading') : t('app.loading'));
</script>

<div role="status" aria-label={label}>
	<p class="text-sm text-muted-foreground mb-2">
		{label}{#if pct !== null}<span class="tabular-nums"> {pct}%</span>{/if}
	</p>
	<div class="h-1.5 w-full overflow-hidden rounded-full bg-muted">
		{#if pct !== null}
			<div
				class="h-full rounded-full bg-primary transition-[width] duration-150"
				style="width: {pct}%"
			></div>
		{:else}
			<div class="db-progress-indeterminate h-full w-1/4 rounded-full bg-primary"></div>
		{/if}
	</div>
</div>
