<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Drawer from '$lib/components/ui/drawer';
	import { Input } from '$lib/components/ui/input';
	import { Search, LoaderCircle, Plus, Check } from 'lucide-svelte';
	import { searchCards, type ScryfallCard } from '$lib/scryfall';
	import { enhance } from '$app/forms';
	import { formatCurrentPrice } from '$lib/collection';

	let {
		open = $bindable(false),
		locations = [],
		defaultLocationId = 'none'
	}: {
		open: boolean;
		locations: any[];
		defaultLocationId?: string;
	} = $props();

	let searchQuery = $state('');
	let lastSearchedQuery = $state('');
	let searchResults = $state<ScryfallCard[]>([]);
	let isSearching = $state(false);
	let selectedSearchResult = $state<ScryfallCard | null>(null);
	let isAdding = $state(false);
	let addMode: 'close' | 'continue' = $state('continue');
	let searchInput = $state<HTMLInputElement | null>(null);
	let highlightedIndex = $state(0);
	let browserLocale = $state('en-US');

	$effect(() => {
		browserLocale = navigator.language || 'en-US';
	});

	$effect(() => {
		if (open && !selectedSearchResult) {
			// Small timeout to ensure the drawer is rendered
			setTimeout(() => {
				searchInput?.focus();
			}, 100);
		}
	});

	async function handleSearch() {
		if (searchQuery.length < 2 || isSearching) return;
		isSearching = true;
		try {
			const results = await searchCards(searchQuery);
			searchResults = results.data;
			lastSearchedQuery = searchQuery;
			highlightedIndex = 0;

			// If exactly one result, select it automatically
			if (searchResults.length === 1) {
				selectedSearchResult = searchResults[0];
			}
		} catch (e) {
			console.error(e);
		} finally {
			isSearching = false;
		}
	}

	function resetSelection() {
		selectedSearchResult = null;
		isAdding = false;
		highlightedIndex = 0;
	}
	let formElement = $state<HTMLFormElement | null>(null);

	function handleGlobalKeydown(e: KeyboardEvent) {
		if (!open) return;

		if (e.key === 'Escape' && selectedSearchResult) {
			e.preventDefault();
			e.stopImmediatePropagation();
			resetSelection();
			return;
		}

		// Form submission state
		if (selectedSearchResult) {
			if (e.key === 'Enter' && !isAdding) {
				const activeElement = document.activeElement;
				if (activeElement?.tagName === 'INPUT' || activeElement?.tagName === 'SELECT') {
					return;
				}
				e.preventDefault();
				formElement?.requestSubmit();
			}
			return;
		}

		// Search state
		if (
			e.key === 'ArrowDown' ||
			e.key === 'ArrowUp' ||
			e.key === 'ArrowRight' ||
			e.key === 'ArrowLeft'
		) {
			if (searchResults.length > 0) {
				e.preventDefault();
				const cols = window.innerWidth >= 640 ? 2 : 1;
				const maxItems = Math.min(searchResults.length, 20);

				if (e.key === 'ArrowDown') {
					highlightedIndex = Math.min(highlightedIndex + cols, maxItems - 1);
				} else if (e.key === 'ArrowUp') {
					highlightedIndex = Math.max(highlightedIndex - cols, 0);
				} else if (e.key === 'ArrowRight') {
					highlightedIndex = Math.min(highlightedIndex + 1, maxItems - 1);
				} else if (e.key === 'ArrowLeft') {
					highlightedIndex = Math.max(highlightedIndex - 1, 0);
				}
			}
		} else if (e.key === 'Enter' && !isSearching) {
			const activeElement = document.activeElement;
			if (activeElement === searchInput) {
				if (searchResults.length > 0 && searchQuery === lastSearchedQuery) {
					e.preventDefault();
					selectedSearchResult = searchResults[highlightedIndex];
				} else {
					// Let the Input's own onkeydown handle the search trigger
					// or we can handle it here and prevent default
					e.preventDefault();
					handleSearch();
				}
			} else if (searchResults.length > 0) {
				e.preventDefault();
				selectedSearchResult = searchResults[highlightedIndex];
			}
		}
	}
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<Drawer.Root
	bind:open
	onOpenChange={(newOpen) => {
		if (!newOpen && selectedSearchResult) {
			resetSelection();
			open = true;
		}
	}}
>
	<Drawer.Content class="max-h-[90vh]">
		<div class="mx-auto w-full max-w-2xl overflow-y-auto px-4 pt-6 pb-10">
			<Drawer.Header class="px-0 text-left">
				<Drawer.Title class="text-2xl">Add Card to Collection</Drawer.Title>
				<Drawer.Description>Search and add cards to your physical inventory.</Drawer.Description>
			</Drawer.Header>

			<div class="mt-6 space-y-6">
				{#if !selectedSearchResult}
					<div class="flex gap-2">
						<div class="relative flex-1">
							<Search
								class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-muted-foreground"
							/>
							<Input
								bind:ref={searchInput}
								placeholder="Search cards (e.g. 'lightning bolt' or 'set:one rarity:m')..."
								bind:value={searchQuery}
								class="h-12 pl-10 text-lg"
							/>
						</div>
						<Button onclick={handleSearch} disabled={isSearching} class="h-12 px-6">
							{#if isSearching}
								<LoaderCircle class="h-4 w-4 animate-spin" />
							{:else}
								Search
							{/if}
						</Button>
					</div>

					{#if searchResults.length > 0}
						<div class="grid gap-2 sm:grid-cols-2">
							{#each searchResults.slice(0, 20) as card, i}
								<button
									onclick={() => (selectedSearchResult = card)}
									class="flex items-center gap-3 rounded-md border p-2 text-left transition-colors hover:bg-muted {i ===
									highlightedIndex
										? 'bg-muted ring-2 ring-primary'
										: ''}"
								>
									<div class="h-14 w-10 shrink-0 overflow-hidden rounded bg-muted shadow-sm">
										{#if card.image_uris}
											<img
												src={card.image_uris.small}
												alt={card.name}
												class="h-full w-full object-cover"
											/>
										{:else if card.card_faces?.[0]?.image_uris}
											<img
												src={card.card_faces[0].image_uris.small}
												alt={card.name}
												class="h-full w-full object-cover"
											/>
										{/if}
									</div>
									<div class="min-w-0 flex-1">
										<div class="flex items-center justify-between gap-2">
											<div class="truncate font-medium">{card.name}</div>
											<div class="text-[10px] font-bold text-primary">
												{formatCurrentPrice(card, undefined, browserLocale)}
											</div>
										</div>
										<div class="text-xs text-muted-foreground uppercase">
											{card.set} • #{card.collector_number} • {card.rarity}
										</div>
									</div>
								</button>
							{/each}
						</div>
					{/if}
				{:else}
					<div class="rounded-xl border bg-muted/30 p-6">
						<div class="mb-6 flex items-start gap-4">
							<div class="h-24 w-18 shrink-0 overflow-hidden rounded-md shadow-md">
								{#if selectedSearchResult.image_uris}
									<img
										src={selectedSearchResult.image_uris.small}
										alt={selectedSearchResult.name}
										class="h-full w-full object-cover"
									/>
								{:else if selectedSearchResult.card_faces?.[0]?.image_uris}
									<img
										src={selectedSearchResult.card_faces[0].image_uris.small}
										alt={selectedSearchResult.name}
										class="h-full w-full object-cover"
									/>
								{/if}
							</div>
							<div class="flex-1">
								<h4 class="text-xl font-bold">{selectedSearchResult.name}</h4>
								<p class="text-sm text-muted-foreground uppercase">
									{selectedSearchResult.set_name} • #{selectedSearchResult.collector_number}
								</p>
								<Button variant="outline" size="sm" onclick={resetSelection} class="mt-2">
									Change Card
								</Button>
							</div>
						</div>

						<form
							bind:this={formElement}
							method="POST"
							action="?/addCard"
							use:enhance={() => {
								isAdding = true;
								return async ({ result, update }) => {
									if (result.type === 'success') {
										if (addMode === 'close') {
											open = false;
											selectedSearchResult = null;
											searchQuery = '';
											searchResults = [];
										} else {
											// Keep open for more, but clear selection
											selectedSearchResult = null;
										}
										await update();
									}
									isAdding = false;
								};
							}}
							class="space-y-6"
						>
							<input type="hidden" name="scryfallId" value={selectedSearchResult.id} />

							<div class="grid grid-cols-2 gap-4">
								<div class="space-y-2">
									<label
										for="condition"
										class="text-xs font-semibold text-muted-foreground uppercase">Condition</label
									>
									<select
										name="condition"
										id="condition"
										class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-primary"
									>
										<option value="NM">Near Mint</option>
										<option value="LP">Lightly Played</option>
										<option value="MP">Moderately Played</option>
										<option value="HP">Heavily Played</option>
										<option value="DMG">Damaged</option>
									</select>
								</div>
								<div class="space-y-2">
									<label
										for="storageLocationId"
										class="text-xs font-semibold text-muted-foreground uppercase">Location</label
									>
									<select
										name="storageLocationId"
										id="storageLocationId"
										class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-primary"
									>
										<option value="none" selected={defaultLocationId === 'none'}>No Location</option
										>
										{#each locations as location}
											<option value={location.id} selected={defaultLocationId === location.id}>
												{location.name}
											</option>
										{/each}
									</select>
								</div>
							</div>

							<div class="grid grid-cols-2 gap-4 sm:grid-cols-3">
								<div class="space-y-2">
									<label
										for="quantity"
										class="text-xs font-semibold text-muted-foreground uppercase">Quantity</label
									>
									<Input type="number" name="quantity" id="quantity" value="1" min="1" required />
								</div>
								<div class="space-y-2">
									<label
										for="purchasePrice"
										class="text-xs font-semibold text-muted-foreground uppercase">Price Paid</label
									>
									<div class="relative">
										<span
											class="absolute top-1/2 left-3 -translate-y-1/2 text-sm text-muted-foreground"
											>$</span
										>
										<Input
											type="number"
											step="0.01"
											name="purchasePrice"
											id="purchasePrice"
											placeholder="0.00"
											class="pl-7"
										/>
									</div>
								</div>
								<div class="space-y-2">
									<label
										for="language"
										class="text-xs font-semibold text-muted-foreground uppercase">Language</label
									>
									<select
										name="language"
										id="language"
										class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-primary"
									>
										<option value="en">English</option>
										<option value="ja">Japanese</option>
										<option value="zh">Chinese</option>
										<option value="fr">French</option>
										<option value="de">German</option>
										<option value="it">Italian</option>
										<option value="ko">Korean</option>
										<option value="pt">Portuguese</option>
										<option value="ru">Russian</option>
										<option value="es">Spanish</option>
									</select>
								</div>
							</div>

							<div class="flex flex-wrap gap-6 rounded-lg bg-muted/50 p-4">
								<label class="flex cursor-pointer items-center gap-2 text-sm font-medium">
									<input
										type="checkbox"
										name="isFoil"
										value="true"
										class="h-4 w-4 rounded border-input"
									/>
									Foil
								</label>
								<label class="flex cursor-pointer items-center gap-2 text-sm font-medium">
									<input
										type="checkbox"
										name="isAlter"
										value="true"
										class="h-4 w-4 rounded border-input"
									/>
									Alter
								</label>
								<label class="flex cursor-pointer items-center gap-2 text-sm font-medium">
									<input
										type="checkbox"
										name="isProxy"
										value="true"
										class="h-4 w-4 rounded border-input"
									/>
									Proxy
								</label>
							</div>

							<div class="flex flex-col gap-3 sm:flex-row">
								<button
									type="submit"
									onclick={() => {
										addMode = 'continue';
									}}
									disabled={isAdding}
									class="h-12 flex-1 rounded-md bg-primary text-lg font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
								>
									{#if isAdding && addMode === 'continue'}
										<LoaderCircle class="mr-2 inline h-4 w-4 animate-spin" />
									{:else}
										<Plus class="mr-2 inline h-4 w-4" />
									{/if}
									Add & Continue
								</button>
								<button
									type="submit"
									onclick={() => {
										addMode = 'close';
									}}
									disabled={isAdding}
									class="h-12 flex-1 rounded-md bg-secondary text-lg font-medium text-secondary-foreground hover:bg-secondary/80 disabled:opacity-50"
								>
									{#if isAdding && addMode === 'close'}
										<LoaderCircle class="mr-2 inline h-4 w-4 animate-spin" />
									{:else}
										<Check class="mr-2 inline h-4 w-4" />
									{/if}
									Add & Close
								</button>
							</div>
						</form>
					</div>
				{/if}
			</div>
		</div>
	</Drawer.Content>
</Drawer.Root>
