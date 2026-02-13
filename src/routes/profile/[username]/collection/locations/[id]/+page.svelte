<script lang="ts">
	import MTGCard from '$lib/components/MTGCard.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Pagination from '$lib/components/ui/pagination';
	import { Separator } from '$lib/components/ui/separator';
	import { Badge } from '$lib/components/ui/badge';
	import { enhance } from '$app/forms';
	import { Trash2, Plus } from 'lucide-svelte';
	import { getLocationTypeLabel, formatCurrentPrice } from '$lib/collection';
	import CollectionAddCardDrawer from '$lib/components/CollectionAddCardDrawer.svelte';

	let { data } = $props();
	let profile = $derived(data.profile);
	let location = $derived(data.location);
	let collection = $derived(data.collection);
	let totalPages = $derived(Math.ceil(data.total / data.limit));
	let browserLocale = $state('en-US');

	$effect(() => {
		browserLocale = navigator.language || 'en-US';
	});

	let isDrawerOpen = $state(false);

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
				<div class="flex gap-2">
					<Button onclick={() => (isDrawerOpen = true)}>
						<Plus class="mr-2 h-4 w-4" />
						Add Card
					</Button>
					<Button href="/profile/{profile.username}/collection" variant="outline"
						>Back to Collection</Button
					>
				</div>
			{/if}
		</div>
		{#if location.description}
			<p class="mt-4 max-w-2xl text-sm">{location.description}</p>
		{/if}
	</div>

	<CollectionAddCardDrawer
		bind:open={isDrawerOpen}
		locations={data.locations}
		defaultLocationId={location.id}
	/>

	<Separator class="mb-8" />

	{#if collection.length > 0}
		<div class="overflow-x-auto rounded-lg border bg-card">
			<table class="w-full text-left text-sm">
				<thead class="border-b bg-muted/50 text-xs font-medium tracking-wider uppercase">
					<tr>
						<th class="px-4 py-3">Card</th>
						<th class="px-4 py-3">Set</th>
						<th class="px-4 py-3">Condition</th>
						<th class="px-4 py-3">Language</th>
						<th class="px-4 py-3">Value</th>
						<th class="px-4 py-3">Flags</th>
						{#if data.user && data.user.id === profile.id}
							<th class="px-4 py-3 text-right">Actions</th>
						{/if}
					</tr>
				</thead>
				<tbody class="divide-y">
					{#each collection as item}
						<tr class="group hover:bg-muted/30">
							<td class="px-4 py-3">
								{#if item.cardData}
									<div class="flex items-center gap-3">
										<div class="relative h-12 w-9 shrink-0 overflow-hidden rounded shadow-sm">
											{#if item.cardData.imageUri}
												<img
													src={item.cardData.imageUri}
													alt={item.cardData.name}
													class="h-full w-full object-cover"
												/>
											{:else}
												<div class="flex h-full w-full items-center justify-center bg-muted"></div>
											{/if}
										</div>
										<a
											href="/cards/{item.physicalCard.scryfallId}"
											class="font-medium hover:underline"
										>
											{item.cardData.name}
										</a>
									</div>
								{:else}
									<span class="animate-pulse text-muted-foreground">Loading...</span>
								{/if}
							</td>
							<td class="px-4 py-3">
								{#if item.cardData}
									<span class="text-muted-foreground"
										>{item.cardData.set.toUpperCase()} • #{item.cardData.collectorNumber}</span
									>
								{/if}
							</td>
							<td class="px-4 py-3 text-xs">
								<Badge variant="outline" class="uppercase">{item.physicalCard.condition}</Badge>
							</td>
							<td class="px-4 py-3 text-xs">
								<span class="uppercase">{item.physicalCard.language}</span>
							</td>
							<td class="px-4 py-3">
								{formatCurrentPrice(item.cardData, item.physicalCard, browserLocale)}
							</td>
							<td class="px-4 py-3">
								<div class="flex flex-wrap gap-1">
									{#if item.physicalCard.isFoil}
										<Badge variant="secondary" class="text-[10px] uppercase">Foil</Badge>
									{/if}
									{#if item.physicalCard.isAlter}
										<Badge
											variant="secondary"
											class="bg-purple-500 text-[10px] text-white uppercase hover:bg-purple-600"
											>Alter</Badge
										>
									{/if}
									{#if item.physicalCard.isProxy}
										<Badge
											variant="secondary"
											class="bg-orange-500 text-[10px] text-white uppercase hover:bg-orange-600"
											>Proxy</Badge
										>
									{/if}
								</div>
							</td>
							{#if data.user && data.user.id === profile.id}
								<td class="px-4 py-3 text-right">
									<form method="POST" action="?/removeCard" use:enhance>
										<input type="hidden" name="physicalCardId" value={item.physicalCard.id} />
										<Button
											size="icon"
											variant="ghost"
											type="submit"
											class="h-8 w-8 text-destructive"
										>
											<Trash2 class="h-4 w-4" />
										</Button>
									</form>
								</td>
							{/if}
						</tr>
					{/each}
				</tbody>
			</table>
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
