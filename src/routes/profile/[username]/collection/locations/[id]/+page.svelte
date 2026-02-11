<script lang="ts">
	import MTGCard from '$lib/components/MTGCard.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Pagination from '$lib/components/ui/pagination';
	import { Separator } from '$lib/components/ui/separator';
	import { Badge } from '$lib/components/ui/badge';
	import { enhance } from '$app/forms';
	import { Trash2 } from 'lucide-svelte';
	import { getLocationTypeLabel } from '$lib/collection';

	let { data } = $props();
	let profile = $derived(data.profile);
	let location = $derived(data.location);
	let collection = $derived(data.collection);
	let totalPages = $derived(Math.ceil(data.total / data.limit));

	function getPageUrl(page: number) {
		const url = new URL(window.location.href);
		url.searchParams.set('page', page.toString());
		return url.pathname + url.search;
	}
</script>

<div class="mx-auto max-w-7xl px-4 py-8">
	<div class="mb-8">
		<div class="flex items-center gap-2 text-sm text-muted-foreground">
			<a href="/profile/{profile.username}/collection" class="hover:underline">Collection</a>
			<span>/</span>
			<a href="/profile/{profile.username}/collection/locations" class="hover:underline"
				>Locations</a
			>
			<span>/</span>
			<a
				href="/profile/{profile.username}/collection/locations/{data.location.id}"
				class="hover:underline">{data.location.name}</a
			>
		</div>
		<div class="mt-2 flex items-end justify-between">
			<div>
				<h1 class="text-4xl font-bold">
					<span class="font-medium text-muted-foreground"
						>{getLocationTypeLabel(location.type)}:</span
					>
					{location.name}
				</h1>
				<p class="my-2 text-muted-foreground">{data.total} cards in this location</p>
			</div>
			{#if data.user && data.user.id === profile.id}
				<Button href="/cards">Add More Cards</Button>
			{/if}
		</div>
		{#if location.description}
			<p class="mt-4 max-w-2xl text-sm">{location.description}</p>
		{/if}
	</div>

	<Separator class="mb-8" />

	{#if collection.length > 0}
		<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6">
			{#each collection as item}
				<div class="group relative flex flex-col gap-2">
					{#if item.cardData}
						<MTGCard card={item.cardData} />
						<div class="mt-1 flex flex-wrap gap-1">
							{#if item.physicalCard.isFoil}
								<Badge variant="secondary" class="text-[10px] uppercase">Foil</Badge>
							{/if}
							<Badge variant="outline" class="text-[10px] uppercase"
								>{item.physicalCard.condition}</Badge
							>
						</div>

						{#if data.user && data.user.id === profile.id}
							<form
								method="POST"
								action="?/removeCard"
								use:enhance
								class="absolute top-2 right-2 opacity-0 transition-opacity group-hover:opacity-100"
							>
								<input type="hidden" name="physicalCardId" value={item.physicalCard.id} />
								<Button size="icon" variant="destructive" type="submit" class="h-8 w-8 shadow-md">
									<Trash2 class="h-4 w-4" />
								</Button>
							</form>
						{/if}
					{:else}
						<div class="aspect-[63/88] w-full animate-pulse rounded-xl bg-muted"></div>
						<p class="text-xs text-muted-foreground">Loading card data...</p>
					{/if}
				</div>
			{/each}
		</div>

		{#if totalPages > 1}
			<div class="mt-12">
				<Pagination.Root count={data.total} perPage={data.limit} page={data.page}>
					{#snippet children({ pages, currentPage })}
						<Pagination.Content>
							<Pagination.Item>
								<Pagination.PrevButton />
							</Pagination.Item>
							{#each pages as page (page.key)}
								{#if page.type === 'ellipsis'}
									<Pagination.Item>
										<Pagination.Ellipsis />
									</Pagination.Item>
								{:else}
									<Pagination.Item>
										<Pagination.Link
											{page}
											isActive={currentPage === page.value}
											href={getPageUrl(page.value)}
										>
											{page.value}
										</Pagination.Link>
									</Pagination.Item>
								{/if}
							{/each}
							<Pagination.Item>
								<Pagination.NextButton />
							</Pagination.Item>
						</Pagination.Content>
					{/snippet}
				</Pagination.Root>
			</div>
		{/if}
	{:else}
		<div class="flex flex-col items-center justify-center py-20 text-center">
			<p class="text-xl font-medium">No cards found in this location.</p>
			<Button href="/profile/{profile.username}/collection" variant="outline" class="mt-6">
				Back to All Cards
			</Button>
		</div>
	{/if}
</div>
