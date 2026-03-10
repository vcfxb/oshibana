<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Pagination from '$lib/components/ui/pagination';
	import { Badge } from '$lib/components/ui/badge';
	import { enhance } from '$app/forms';
	import {
		Trash2,
		Plus,
		ArrowUpAZ,
		ArrowDownZA,
		ArrowUpDown,
		Pencil,
		Search,
		X
	} from 'lucide-svelte';
	import {
		formatCurrentPrice,
		formatPrice,
		getCurrentPriceValue,
		type CollectionSortBy,
		type SortDir
	} from '$lib/collection';
	import CollectionAddCardDrawer from './CollectionAddCardDrawer.svelte';
	import CollectionEditCardDrawer from './CollectionEditCardDrawer.svelte';
	import { goto } from '$app/navigation';
	import type { DbStorageLocation, DbUser } from '$lib/server/db/types';
	import type { CollectionData } from '$lib/server/collection';

	let {
		collection,
		total,
		page,
		limit,
		profile,
		user,
		sortBy = 'date-updated',
		sortDir = 'desc',
		q = '',
		locations = [],
		defaultLocationId = 'none',
		browserLocale = 'en-US',
		emptyMessage = 'No cards found in this collection.'
	}: {
		collection: CollectionData['items'];
		total: number;
		page: number;
		limit: number;
		profile: DbUser;
		user?: DbUser | null;
		sortBy?: CollectionSortBy;
		sortDir?: SortDir;
		q?: string;
		locations?: DbStorageLocation[];
		defaultLocationId?: string;
		browserLocale?: string;
		emptyMessage?: string;
		backButtonHref?: string;
		backButtonText?: string;
	} = $props();

	let isDrawerOpen = $state(false);
	let isEditDrawerOpen = $state(false);
	let editingItem = $state<any>(null);
	let searchQuery = $state(q);

	$effect(() => {
		searchQuery = q;
	});

	function handleSearch(e: Event) {
		e.preventDefault();
		updateUrl({ q: searchQuery, page: '1' });
	}

	function clearSearch() {
		searchQuery = '';
		updateUrl({ q: '', page: '1' });
	}

	function openEditDrawer(item: any) {
		editingItem = item;
		isEditDrawerOpen = true;
	}

	function updateUrl(params: Record<string, string>) {
		const url = new URL(window.location.href);
		Object.entries(params).forEach(([key, value]) => {
			url.searchParams.set(key, value);
		});
		goto(url.toString(), { keepFocus: true, noScroll: true });
	}

	function getPageUrl(pageNumber: number) {
		const url = new URL(window.location.href);
		url.searchParams.set('page', pageNumber.toString());
		return url.pathname + url.search;
	}

	function handleSortChange(newSortBy: CollectionSortBy) {
		if (newSortBy === sortBy) {
			// Toggle direction if same column
			updateUrl({ sortDir: sortDir === 'asc' ? 'desc' : 'asc', page: '1' });
		} else {
			updateUrl({ sortBy: newSortBy, sortDir: 'asc', page: '1' });
		}
	}

	function toggleSortDir() {
		updateUrl({ sortDir: sortDir === 'asc' ? 'desc' : 'asc', page: '1' });
	}

	const totalPages = $derived(Math.ceil(total / limit));

	const sortOptions: { label: string; value: CollectionSortBy }[] = [
		{ label: 'Date Updated', value: 'date-updated' },
		{ label: 'Name', value: 'name' },
		{ label: 'Value', value: 'value' },
		{ label: 'Purchase Price', value: 'purchase-price' },
		{ label: 'Set', value: 'set' },
		{ label: 'Quantity', value: 'quantity' },
		{ label: 'Total Value', value: 'total-value' }
	];

	const languageFlags: Record<string, string> = {
		en: '🇺🇸',
		ja: '🇯🇵',
		fr: '🇫🇷',
		de: '🇩🇪',
		it: '🇮🇹',
		ko: '🇰🇷',
		pt: '🇧🇷',
		ru: '🇷🇺',
		es: '🇪🇸',
		zh: '🇨🇳'
	};
</script>

<div class="mb-8 flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
	<div class="flex items-center gap-4">
		{#if user && user.id === profile.id}
			<Button onclick={() => (isDrawerOpen = true)}>
				<Plus class="mr-2 h-4 w-4" />
				Add Card
			</Button>
		{/if}

		<form onsubmit={handleSearch} class="relative flex-1 sm:max-w-xs">
			<Search class="absolute top-1/2 left-2.5 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
			<Input type="search" placeholder="Search cards..." class="pl-9" bind:value={searchQuery} />
			{#if searchQuery}
				<button
					type="button"
					onclick={clearSearch}
					class="absolute top-1/2 right-2.5 -translate-y-1/2 text-muted-foreground hover:text-foreground"
				>
					<X class="h-4 w-4" />
				</button>
			{/if}
		</form>
	</div>

	<div class="flex flex-wrap items-center gap-3">
		<div class="flex items-center gap-2">
			<span class="text-xs font-medium text-muted-foreground uppercase">Sort by:</span>
			<select
				value={sortBy}
				onchange={(e) => handleSortChange(e.currentTarget.value as CollectionSortBy)}
				class="rounded-md border bg-background px-2 py-1 text-sm focus:ring-2 focus:ring-primary focus:outline-none"
			>
				{#each sortOptions as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
			<Button variant="ghost" size="icon" class="h-8 w-8" onclick={toggleSortDir}>
				{#if sortDir === 'asc'}
					<ArrowUpAZ class="h-4 w-4" />
				{:else}
					<ArrowDownZA class="h-4 w-4" />
				{/if}
			</Button>
		</div>
		{#if total > 0}
			<div class="mx-1 h-4 w-[1px] bg-border"></div>
			<p class="text-sm text-muted-foreground">{total} cards</p>
		{/if}
	</div>
</div>

<CollectionAddCardDrawer bind:open={isDrawerOpen} {locations} {defaultLocationId} />
<CollectionEditCardDrawer bind:open={isEditDrawerOpen} item={editingItem} {locations} />

{#if collection.length > 0}
	<div class="overflow-x-auto rounded-lg border bg-card">
		<table class="w-full text-left text-sm">
			<thead class="border-b bg-muted/50 text-xs font-medium tracking-wider">
				<tr>
					<th class="px-4 py-3">
						<button
							onclick={() => handleSortChange('name')}
							class="flex items-center gap-1 hover:text-foreground"
						>
							Card
							{#if sortBy === 'name'}
								<ArrowUpDown class="h-3 w-3" />
							{/if}
						</button>
					</th>
					<th class="px-4 py-3">
						<button
							onclick={() => handleSortChange('set')}
							class="flex items-center gap-1 hover:text-foreground"
						>
							Set • #CN
							{#if sortBy === 'set'}
								<ArrowUpDown class="h-3 w-3" />
							{/if}
						</button>
					</th>
					<th class="px-4 py-3 text-center">
						<button
							onclick={() => handleSortChange('quantity')}
							class="mx-auto flex items-center gap-1 hover:text-foreground"
						>
							Qty
							{#if sortBy === 'quantity'}
								<ArrowUpDown class="h-3 w-3" />
							{/if}
						</button>
					</th>
					<th class="px-4 py-3">Location</th>
					<th class="px-4 py-3">Condition</th>
					<th class="px-4 py-3">Language</th>
					<th class="px-4 py-3">
						<button
							onclick={() => handleSortChange('value')}
							class="flex items-center gap-1 hover:text-foreground"
						>
							Value
							{#if sortBy === 'value'}
								<ArrowUpDown class="h-3 w-3" />
							{/if}
						</button>
					</th>
					<th class="px-4 py-3">
						<button
							onclick={() => handleSortChange('total-value')}
							class="flex items-center gap-1 hover:text-foreground"
						>
							Total Value
							{#if sortBy === 'total-value'}
								<ArrowUpDown class="h-3 w-3" />
							{/if}
						</button>
					</th>
					<th class="px-4 py-3">
						<button
							onclick={() => handleSortChange('purchase-price')}
							class="flex items-center gap-1 hover:text-foreground"
						>
							Paid
							{#if sortBy === 'purchase-price'}
								<ArrowUpDown class="h-3 w-3" />
							{/if}
						</button>
					</th>
					<th class="px-4 py-3">Flags</th>
					<th class="px-4 py-3">
						<button
							onclick={() => handleSortChange('date-updated')}
							class="flex items-center gap-1 hover:text-foreground"
						>
							Date Updated
							{#if sortBy === 'date-updated'}
								<ArrowUpDown class="h-3 w-3" />
							{/if}
						</button>
					</th>
					{#if user && user.id === profile.id}
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
									<div class="flex flex-col">
										<a
											href="/cards/{item.physicalCard.scryfallId}"
											class="font-medium hover:underline"
										>
											{item.cardData.name}
										</a>
										{#if item.physicalCard.notes}
											<span
												class="mt-0.5 line-clamp-2 max-w-[250px] text-[10px] leading-tight text-muted-foreground"
												title={item.physicalCard.notes}
											>
												{item.physicalCard.notes}
											</span>
										{/if}
									</div>
								</div>
							{:else}
								<span class="animate-pulse text-muted-foreground">Loading...</span>
							{/if}
						</td>
						<td class="px-4 py-3">
							{#if item.cardData}
								<span class="text-muted-foreground">
									{item.cardData.set.toUpperCase()} • #{item.cardData.collectorNumber}
								</span>
							{/if}
						</td>
						<td class="px-4 py-3 text-center font-medium">
							{item.physicalCard.quantity}
						</td>
						<td class="px-4 py-3 text-xs">
							{#if item.physicalCard.storageLocationId}
								{@const location = locations.find(
									(l) => l.id === item.physicalCard.storageLocationId
								)}
								{#if location}
									<a
										href="/profile/{profile.username}/collection/locations/{location.id}"
										class="hover:underline"
									>
										{location.name}
									</a>
								{:else}
									<span class="text-muted-foreground italic">Unknown</span>
								{/if}
							{:else}
								<span class="text-muted-foreground italic">None</span>
							{/if}
						</td>
						<td class="px-4 py-3 text-xs">
							<Badge variant="outline" class="uppercase">{item.physicalCard.condition}</Badge>
						</td>
						<td class="px-4 py-3 text-xs">
							<div class="flex items-center gap-1.5">
								<span class="text-lg leading-none"
									>{languageFlags[item.physicalCard.language || 'en'] || ''}</span
								>
								<span class="uppercase">{item.physicalCard.language || 'en'}</span>
							</div>
						</td>
						<td class="px-4 py-3">
							{formatCurrentPrice(item.cardData, item.physicalCard, browserLocale)}
						</td>
						<td class="px-4 py-3 text-muted-foreground">
							{#if item.cardData}
								{@const price = getCurrentPriceValue(item.cardData, item.physicalCard)}
								{#if price}
									{formatPrice(price * item.physicalCard.quantity, 'USD', browserLocale)}
								{:else}
									—
								{/if}
							{:else}
								—
							{/if}
						</td>
						<td class="px-4 py-3 text-muted-foreground">
							{#if item.physicalCard.purchasePrice}
								{formatPrice(item.physicalCard.purchasePrice, 'USD', browserLocale)}
							{:else}
								—
							{/if}
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
						<td class="px-4 py-3 text-center font-medium text-muted-foreground">
							{item.physicalCard.updatedAt.toLocaleString()}
						</td>
						{#if user && user.id === profile.id}
							<td class="px-4 py-3 text-right">
								<div class="flex justify-end gap-2">
									<Button
										size="icon"
										variant="ghost"
										class="h-8 w-8 text-muted-foreground"
										onclick={() => openEditDrawer(item)}
									>
										<Pencil class="h-4 w-4" />
									</Button>
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
								</div>
							</td>
						{/if}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	{#if totalPages > 1}
		<div class="mt-12">
			<Pagination.Root count={total} perPage={limit} {page}>
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
		<p class="mb-2 text-xl font-medium">{emptyMessage}</p>
	</div>
{/if}
