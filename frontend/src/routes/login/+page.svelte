<script lang="ts">
	import { ArrowRightIcon, AtSignIcon, EyeOffIcon, TriangleAlert } from '@lucide/svelte';
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

<div class="flex h-screen items-center justify-start">
	<div class="flex flex-col items-center gap-12 pl-12 md:flex-row md:items-start md:gap-6 md:pl-28">
		<div class="pt-6 lg:pt-0">
			<img class="w-54 object-cover" src="/icon-pxiel-cropped.png" alt="icon" />
		</div>
		<form method="POST" class="flex flex-col items-center gap-4">
			<h1 class="text-3xl font-extrabold">BeanieGenie</h1>
			<div class="input-group grid-cols-[auto_1fr_auto]">
				<div class="ig-cell preset-tonal">
					<AtSignIcon size={16} />
				</div>
				<input class="ig-input" type="email" name="email" placeholder="Email" required />
			</div>
			<div class="input-group grid-cols-[auto_1fr_auto]">
				<div class="ig-cell preset-tonal">
					<EyeOffIcon size={16} />
				</div>
				<input class="ig-input" type="password" name="password" placeholder="Password" required />
			</div>

			<button type="submit" class="btn w-full preset-filled">
				<span class="text-mono">Login</span>
				<ArrowRightIcon size={18} />
			</button>
			{#if form?.error}
				<div
					class="alert alert-error flex w-full flex-row items-center gap-2 card preset-outlined-error-500 p-4"
				>
					<TriangleAlert size={'1rem'} class="text-error-500" />
					{form.error}
				</div>
			{/if}

			{#if form?.success}
				<div class="alert alert-success">Login successful!</div>
			{/if}
		</form>
	</div>
</div>
