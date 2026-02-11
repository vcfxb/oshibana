<script lang="ts">
	import MTGCard from '$lib/components/MTGCard.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Search } from 'lucide-svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<div class="container mx-auto px-4 py-8">
	<h1 class="mb-8 text-3xl font-bold">Search Cards</h1>

	<form
		method="GET"
		class="mb-8 flex max-w-2xl flex-col gap-3 sm:flex-row sm:items-stretch sm:gap-2"
	>
		<div class="relative flex-1">
			<Search class="absolute top-1/2 left-3 h-5 w-5 -translate-y-1/2 text-muted-foreground" />
			<input
				type="text"
				name="q"
				value={data.q}
				placeholder="Search by name, type, text, etc..."
				class="w-full rounded-full border bg-background py-3 pr-4 pl-10 text-lg shadow-sm focus:ring-2 focus:ring-primary focus:outline-none"
			/>
		</div>
		<Button
			type="submit"
			class="w-full rounded-full px-8 text-lg shadow-sm sm:h-auto sm:w-auto sm:py-3"
		>
			Search
		</Button>
	</form>

	{#if data.error}
		<div
			class="mb-8 rounded-md border border-destructive/20 bg-destructive/10 p-4 text-destructive"
		>
			{data.error}
		</div>
	{/if}

	{#if data.results}
		<div class="grid grid-cols-2 gap-6 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
			{#each data.results.data as card}
				<MTGCard {card} />
			{/each}
		</div>

		{#if data.results.total_cards === 0}
			<p class="text-center text-muted-foreground">No cards found.</p>
		{/if}
	{/if}
</div>
