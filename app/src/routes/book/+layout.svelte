<script lang="ts">
	import DbGate from '$lib/DbGate.svelte';

	// Book + chapter routes read the corpus from the sql.js DB, so they wait
	// behind the DB-ready gate (moved here from the root layout).
	let { children } = $props();
</script>

<DbGate>
	{@render children()}
</DbGate>
