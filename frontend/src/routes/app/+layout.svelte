<script lang="ts">
	let searchOpen = $state(false);
	let { children } = $props();
	import { page } from '$app/state';
	// svelte-ignore non_reactive_update
	let inputEl: HTMLInputElement;

	$effect(() => {
		if (!searchOpen) return;
		if (searchOpen && inputEl) {
			inputEl.focus();
		}
		function handleEscape(e: KeyboardEvent) {
			if (e.key === 'Escape') searchOpen = false;
		}

		document.addEventListener('keydown', handleEscape);
		return () => document.removeEventListener('keydown', handleEscape);
	});

	import { SearchIcon, ListTodoIcon, FenceIcon, Settings2, CircleUserRound } from '@lucide/svelte';
	import { Navigation } from '@skeletonlabs/skeleton-svelte';

	const links = [
		{ label: 'Tasks', href: '/app/tasks', icon: ListTodoIcon },
		{ label: 'Garden', href: '/app/garden', icon: FenceIcon },
		{
			label: 'Search',
			href: '#search',
			icon: SearchIcon,
			onclick: () => (searchOpen = !searchOpen),
			active: () => searchOpen
		},
		{
			label: 'Settings',
			href: '/app/settings',
			icon: CircleUserRound
		}
	];
	let anchorBar = 'btn hover:preset-tonal flex-col items-center gap-1';
</script>

<nav>
	<div class="relative grid h-screen w-screen grid-rows-[1fr_auto] border border-surface-200-800">
		<div class="overflow-y-auto">
			{#if searchOpen}
				<div
					class="absolute inset-0 z-5 h-full max-w-none bg-black/30 backdrop-blur-sm backdrop:bg-transparent"
				>
					<div class="m-10 mt-28 card p-2">
						<div class="flex flex-col gap-6">
							<h1 class="text-xl">Search</h1>
							<div class="input-group grid-cols-[auto_1fr_auto]">
								<div class="ig-cell preset-tonal">
									<SearchIcon size={16} />
								</div>
								<input class="ig-input" type="search" placeholder="Search..." bind:this={inputEl} />
							</div>
						</div>
					</div>
				</div>
			{/if}
			{@render children()}
		</div>
		<Navigation layout="bar" class="z-50">
			<Navigation.Menu class="grid grid-cols-4 gap-2">
				{#each links as link (link)}
					{@const Icon = link.icon}
					{#if link.onclick}
						<button onclick={link.onclick} class={anchorBar} class:active={link.active()}>
							<Icon class="size-5" />
							<span class="text-[10px]">{link.label}</span>
						</button>
					{:else}
						<a
							href={link.href}
							onclick={() => (searchOpen = false)}
							class={anchorBar}
							class:active={page.url.pathname === link.href}
						>
							<Icon class="size-5" />
							<span class="text-[10px]">{link.label}</span>
						</a>
					{/if}
				{/each}
			</Navigation.Menu>
		</Navigation>
	</div>
</nav>

<style>
	button.active {
		font-weight: bold;
		color: var(--color-success-500);
		/* or whatever styling you want */
	}
	a.active {
		font-weight: bold;
		color: var(--color-success-500);
		/* or whatever styling you want */
	}
</style>
