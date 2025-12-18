<script lang="ts">
	import { enhance } from '$app/forms';
	import { CircleUserRound } from '@lucide/svelte';
	import { Avatar } from '@skeletonlabs/skeleton-svelte';
	export let data;
</script>

<div class="p-8">
	<div class="flex flex-col">
		<div class="flex flex-row items-center gap-4">
			<CircleUserRound class="h-full" />
			<h1 class="text-2xl">Settings</h1>
		</div>
		<div>
			<div>
				<div class="space-y-6 card p-6">
					<!-- User Info Section -->
					<div class="flex items-center gap-4">
						<Avatar />
						<div class="flex-1">
							<h3 class="h4">{data.user?.name}</h3>
							<p class="text-sm opacity-70">{data.user?.email}</p>
							{#if data.user && data.user.role}
								<span class="variant-soft-surface mt-2 badge">{data.user?.role}</span>
							{/if}
						</div>
					</div>

					<!-- Divider -->
					<hr class="border-t-2 opacity-50" />

					<!-- Actions -->
					<div class="flex justify-end">
						<form method="POST" action="?/deleteCookie" use:enhance>
							<button type="submit">Logout</button>
						</form>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
