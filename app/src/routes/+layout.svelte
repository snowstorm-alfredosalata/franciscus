<script lang="ts">
	import '../app.css';
	import { initDb } from '$lib';

	let { children } = $props();

	let ready = $state(false);
	let error = $state<string | null>(null);

	$effect(() => {
		initDb()
			.then(() => { ready = true; })
			.catch((e) => { error = String(e); });
	});
</script>

<svelte:head>
	<title>Franciscus</title>
</svelte:head>

{#if error}
	<main class="min-h-screen flex items-center justify-center">
		<p class="text-red-700">Errore nel caricamento del database: {error}</p>
	</main>
{:else if !ready}
	<main class="min-h-screen flex items-center justify-center">
		<p class="text-stone-500">Caricamento...</p>
	</main>
{:else}
	{@render children()}
{/if}
