<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import Label from '@/components/ui/label/label.svelte';

	let { children } = $props();

	let searchValue = $state('');
	let searchResults = $state(null);

	async function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			try {
				const response = await fetch(`/api/search?q=${encodeURIComponent(searchValue)}`);
				const data = await response.json();
				searchResults = data;
			} catch (error) {
				console.error('Search failed:', error);
				searchResults = null;
			}
		}
	}
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children?.()}

<div class="flex w-full max-w-sm flex-col gap-1.5">
	<Label>Search</Label>
	<input
		bind:value={searchValue}
		onkeydown={handleKeydown}
		class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
		placeholder="Type and press Enter to search..."
	/>

	{#if searchResults}
		<div class="mt-2 rounded border p-2">
			<h3>Results:</h3>
			<pre>{JSON.stringify(searchResults, null, 2)}</pre>
		</div>
	{/if}
</div>
