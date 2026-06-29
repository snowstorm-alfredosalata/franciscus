<script lang="ts">
	import DbGate from '$lib/DbGate.svelte';

	// The chapter reader needs the full sql.js DB; wait behind the ready gate.
	// (The book index one level up renders from the manifest, ungated.)
	let { children } = $props();
</script>

<DbGate>
	{@render children()}
</DbGate>
