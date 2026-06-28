<script lang="ts">
	import DbGate from '$lib/DbGate.svelte';

	// The topic-detail route (/topics/[topic_type]/[topic_value]) reads
	// occurrences from the sql.js DB, so it waits behind the DB-ready gate. The
	// /topics hub one level up renders from the manifest and is NOT gated.
	let { children } = $props();
</script>

<DbGate>
	{@render children()}
</DbGate>
