<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import '@fontsource/noto-sans-mono';

	onMount(() => {
		if ('serviceWorker' in navigator) {
			navigator.serviceWorker.register('/service-worker.js');
		}
	});

	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

{@render children()}

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>
