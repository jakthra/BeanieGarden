<script>
	import {
		ArrowLeftIcon,
		CalendarIcon,
		FenceIcon,
		LeafIcon,
		PlusIcon,
		RulerIcon,
		SproutIcon
	} from '@lucide/svelte';
</script>

<div class="flex flex-col gap-4 md:gap-6 p-6 md:p-8 pb-32 md:pb-8">
	<!-- Header with navigation -->
	<div class="flex flex-col gap-4">
		<a href="/app/garden" class="btn preset-tonal w-fit">
			<ArrowLeftIcon size={'1rem'} />
			<span>Back to Garden</span>
		</a>

		<div class="flex items-center gap-4">
			<FenceIcon class="h-8 w-8" />
			<h1 class="text-3xl font-bold">Add Plant to Garden</h1>
		</div>
	</div>

	<!-- Form container -->
	<div class="max-w-3xl">
		<form method="POST" class="flex flex-col gap-4 md:gap-6">
			<!-- Plant Selection Card -->
			<div class="card preset-tonal p-4 md:p-6">
				<div class="flex items-center gap-3 mb-4">
					<LeafIcon size={20} class="text-success-500" />
					<h2 class="text-xl font-semibold">Plant Information</h2>
				</div>
				<div class="flex flex-col gap-4">
					<div class="flex flex-col gap-2">
						<label for="common_plant_id" class="font-medium">Plant Type</label>
						<select id="common_plant_id" name="common_plant_id" required class="input">
							<option value="">Select a plant...</option>
							<!-- Plant options will be loaded from server -->
						</select>
					</div>

					<div class="flex flex-col gap-2">
						<label for="growth_type" class="font-medium">Growth Stage</label>
						<input
							type="text"
							id="growth_type"
							name="growth_type"
							required
							class="input"
							placeholder="e.g., Seedling, Vegetative, Flowering, Mature"
						/>
					</div>
				</div>
			</div>

			<!-- Measurements Card -->
			<div class="card preset-tonal p-4 md:p-6">
				<div class="flex items-center gap-3 mb-4">
					<RulerIcon size={20} class="text-success-500" />
					<h2 class="text-xl font-semibold">Measurements</h2>
				</div>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<div class="flex flex-col gap-2">
						<label for="height" class="font-medium">Height (cm)</label>
						<input
							type="number"
							id="height"
							name="height"
							step="0.1"
							min="0"
							required
							class="input"
							placeholder="0.0"
						/>
					</div>

					<div class="flex flex-col gap-2">
						<label for="width" class="font-medium">Width (cm)</label>
						<input
							type="number"
							id="width"
							name="width"
							step="0.1"
							min="0"
							required
							class="input"
							placeholder="0.0"
						/>
					</div>
				</div>
			</div>

			<!-- Timeline Card -->
			<div class="card preset-tonal p-4 md:p-6">
				<div class="flex items-center gap-3 mb-4">
					<CalendarIcon size={20} class="text-success-500" />
					<h2 class="text-xl font-semibold">Timeline</h2>
				</div>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<div class="flex flex-col gap-2">
						<label for="age_estimate" class="font-medium">Age Estimate (days)</label>
						<input
							type="number"
							id="age_estimate"
							name="age_estimate"
							step="1"
							min="0"
							required
							class="input"
							placeholder="0"
						/>
						<p class="text-sm opacity-75">How old is the plant?</p>
					</div>

					<div class="flex flex-col gap-2">
						<label for="created_at" class="font-medium">Date Added</label>
						<input type="date" id="created_at" name="created_at" required class="input" />
						<p class="text-sm opacity-75">When did you add this plant?</p>
					</div>
				</div>
			</div>

			<!-- Action buttons -->
			<div class="flex gap-3 justify-end pt-4">
				<a href="/app/garden" class="btn preset-tonal">Cancel</a>
				<button type="submit" class="btn preset-filled-success-500">
					<PlusIcon size={'1rem'} />
					<span>Add to Garden</span>
				</button>
			</div>
		</form>
	</div>
</div>
