<script>
	import { FenceIcon, PlusIcon, SproutIcon } from '@lucide/svelte';
</script>

<div class="flex flex-col gap-6 p-8 pb-32 md:pb-8">
	<div class="flex items-center justify-between">
		<div class="flex flex-row items-center gap-4">
			<FenceIcon class="h-8 w-8" />
			<h1 class="text-3xl font-bold">My Garden</h1>
		</div>
		<div class="flex">
			<a href="/app/garden/new" class="btn preset-filled-success-500">
				<PlusIcon size={'1rem'} />
				<span>Add Plant</span>
			</a>
		</div>
	</div>

	<!-- Empty state -->
	<div class="flex flex-col items-center justify-center gap-6 py-16">
		<div class="card preset-tonal p-12 flex flex-col items-center gap-4 max-w-md">
			<SproutIcon size={48} class="text-success-500" />
			<h2 class="text-xl font-semibold text-center">Your garden is empty</h2>
			<p class="text-center opacity-75">
				Start tracking your plants by adding your first one to the garden.
			</p>
			<a href="/app/garden/new" class="btn preset-filled-success-500 mt-2">
				<PlusIcon size={'1rem'} />
				<span>Add Your First Plant</span>
			</a>
		</div>
	</div>

	<!-- Future: Plant grid will be displayed here when plants exist -->
</div>
