<script>
	import { FenceIcon, PlusIcon } from '@lucide/svelte';
</script>

<div class="p-8">
	<div class="flex items-center justify-between">
		<div class="flex flex-row items-center gap-4">
			<FenceIcon class="h-full" />
			<h1 class="text-2xl">Garden</h1>
		</div>
		<div class="flex">
			<button type="button" class="btn preset-filled-primary-500"
				><PlusIcon size={'1rem'} /> Add</button
			>
		</div>
	</div>
</div>
