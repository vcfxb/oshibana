<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import * as Card from '$lib/components/ui/card';
	import * as Sheet from '$lib/components/ui/sheet';
	import {
		Package,
		Inbox,
		Bookmark,
		Plus,
		Trash2,
		WalletCards,
		EllipsisIcon,
		Pencil
	} from 'lucide-svelte';
	import { enhance } from '$app/forms';
	import { getLocationTypeLabel } from '$lib/collection';

	let { data } = $props();
	let profile = $derived(data.profile);
	let isOwner = $derived(data.user?.id === profile.id);

	let isSheetOpen = $state(false);
	let isEditSheetOpen = $state(false);
	let editingLocation = $state<any>(null);

	function openEditSheet(location: any) {
		editingLocation = location;
		isEditSheetOpen = true;
	}
</script>

<div class="mx-auto max-w-7xl px-4 py-8">
	<div class="mb-8 flex flex-col justify-between gap-4 sm:flex-row sm:items-end">
		<div>
			<div class="flex items-center gap-2 text-sm text-muted-foreground">
				<a href="/profile/{profile.username}/collection" class="hover:underline">Collection</a>
				<span>/</span>
				<span class="text-foreground">Locations</span>
			</div>
			<h1 class="mt-2 text-4xl font-bold">Storage Locations</h1>
			<p class="text-muted-foreground">
				Manage where {profile.username} keeps their physical cards
			</p>
		</div>

		{#if isOwner}
			<Sheet.Root bind:open={isSheetOpen}>
				<Sheet.Trigger>
					{#snippet children()}
						<Button class="flex items-center gap-2">
							<Plus class="h-4 w-4" />
							New Location
						</Button>
					{/snippet}
				</Sheet.Trigger>
				<Sheet.Content side="right" class="p-6 sm:max-w-md">
					<Sheet.Header class="mb-6">
						<Sheet.Title class="text-2xl">Create New Location</Sheet.Title>
						<Sheet.Description class="text-base">
							Add a new binder, box, or shelf to organize your collection.
						</Sheet.Description>
					</Sheet.Header>
					<form
						method="POST"
						action="?/createLocation"
						use:enhance={() => {
							return async ({ result }) => {
								if (result.type === 'success') {
									isSheetOpen = false;
								}
							};
						}}
						class="space-y-6"
					>
						<div class="space-y-2">
							<label for="locationName" class="text-sm font-medium">Location Name</label>
							<input
								type="text"
								id="locationName"
								name="locationName"
								placeholder="e.g. Modern Binder #1"
								required
								class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
							/>
						</div>

						<div class="space-y-2">
							<label for="type" class="text-sm font-medium">Type</label>
							<select
								id="type"
								name="type"
								required
								class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
							>
								<option value="binder">Binder</option>
								<option value="box">Box</option>
								<option value="shelf">Shelf</option>
								<option value="physical_deck">Physical Deck</option>
								<option value="other">Other</option>
							</select>
						</div>
						<div class="space-y-2">
							<label for="description" class="text-sm font-medium">Description (Optional)</label>
							<textarea
								id="description"
								name="description"
								placeholder="What's inside?"
								rows="3"
								class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
							></textarea>
						</div>
						<Sheet.Footer>
							<Button type="submit" class="w-full">Create Location</Button>
						</Sheet.Footer>
					</form>
				</Sheet.Content>
			</Sheet.Root>
		{/if}
	</div>

	<Separator class="mb-8" />

	{#if data.locations.length > 0}
		<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
			{#each data.locations as location}
				<div class="group relative">
					<a
						href="/profile/{profile.username}/collection/locations/{location.id}"
						class="block transition-transform hover:scale-[1.02]"
					>
						<Card.Root>
							<Card.Header>
								<div class="flex items-center justify-between">
									<div class="rounded-full bg-primary/10 p-2 text-primary">
										{#if location.type === 'binder'}
											<Bookmark class="h-5 w-5" />
										{:else if location.type === 'box'}
											<Package class="h-5 w-5" />
										{:else if location.type === 'shelf'}
											<Inbox class="h-5 w-5" />
										{:else if location.type === 'physical_deck'}
											<WalletCards class="h-5 w-5" />
										{:else}
											<EllipsisIcon class="h-5 w-5" />
										{/if}
									</div>
									<span class="text-sm font-medium tracking-wider text-muted-foreground">
										{getLocationTypeLabel(location.type)}
									</span>
								</div>
								<Card.Title class="mt-4">{location.name}</Card.Title>
								{#if location.description}
									<Card.Description class="line-clamp-2">{location.description}</Card.Description>
								{/if}
							</Card.Header>
							<Card.Content>
								<p class="text-2xl font-bold">{location.cardCount}</p>
								<p class="text-sm text-muted-foreground">Cards stored here</p>
							</Card.Content>
						</Card.Root>
					</a>

					{#if isOwner}
						<div
							class="absolute top-2 right-2 flex gap-1 opacity-0 transition-opacity group-hover:opacity-100"
						>
							<Button
								size="icon"
								variant="secondary"
								class="h-8 w-8 shadow-md"
								onclick={() => openEditSheet(location)}
							>
								<Pencil class="h-4 w-4" />
							</Button>
							<form
								method="POST"
								action="?/deleteLocation"
								use:enhance
								onsubmit={(e) => {
									if (
										!confirm(
											'Are you sure you want to delete this location? Cards inside will NOT be deleted.'
										)
									) {
										e.preventDefault();
									}
								}}
							>
								<input type="hidden" name="id" value={location.id} />
								<Button size="icon" variant="destructive" type="submit" class="h-8 w-8 shadow-md">
									<Trash2 class="h-4 w-4" />
								</Button>
							</form>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center py-20 text-center">
			<p class="text-xl font-medium">No storage locations found.</p>
			<p class="mt-2 text-muted-foreground">
				Users can organize their collection into binders, boxes, and more.
			</p>
		</div>
	{/if}

	{#if isOwner && editingLocation}
		<Sheet.Root bind:open={isEditSheetOpen}>
			<Sheet.Content side="right" class="p-6 sm:max-w-md">
				<Sheet.Header class="mb-6">
					<Sheet.Title class="text-2xl">Edit Location</Sheet.Title>
					<Sheet.Description class="text-base">
						Update the details for this storage location.
					</Sheet.Description>
				</Sheet.Header>
				<form
					method="POST"
					action="?/updateLocation"
					use:enhance={() => {
						return async ({ result }) => {
							if (result.type === 'success') {
								isEditSheetOpen = false;
							}
						};
					}}
					class="space-y-6"
				>
					<input type="hidden" name="id" value={editingLocation.id} />
					<div class="space-y-2">
						<label for="edit-locationName" class="text-sm font-medium">Location Name</label>
						<input
							type="text"
							id="edit-locationName"
							name="locationName"
							value={editingLocation.name}
							required
							class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
						/>
					</div>

					<div class="space-y-2">
						<label for="edit-type" class="text-sm font-medium">Type</label>
						<select
							id="edit-type"
							name="type"
							value={editingLocation.type}
							required
							class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
						>
							<option value="binder">Binder</option>
							<option value="box">Box</option>
							<option value="shelf">Shelf</option>
							<option value="physical_deck">Physical Deck</option>
							<option value="other">Other</option>
						</select>
					</div>
					<div class="space-y-2">
						<label for="edit-description" class="text-sm font-medium">Description (Optional)</label>
						<textarea
							id="edit-description"
							name="description"
							value={editingLocation.description || ''}
							rows="3"
							class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
						></textarea>
					</div>
					<Sheet.Footer>
						<Button type="submit" class="w-full">Save Changes</Button>
					</Sheet.Footer>
				</form>
			</Sheet.Content>
		</Sheet.Root>
	{/if}
</div>
