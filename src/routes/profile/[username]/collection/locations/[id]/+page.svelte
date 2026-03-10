<script lang="ts">
	import { Separator } from '$lib/components/ui/separator';
	import { getLocationTypeLabel } from '$lib/collection';
	import CollectionCardTable from '$lib/components/CollectionCardTable.svelte';

	let { data } = $props();
	let profile = $derived(data.profile);
	let location = $derived(data.location);
	let browserLocale = $state('en-US');

	$effect(() => {
		browserLocale = navigator.language || 'en-US';
	});
</script>

<div class="mx-auto max-w-[90rem] px-4 py-8">
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
		<div class="mt-2">
			<h1 class="text-4xl font-bold">
				<span class="font-medium text-muted-foreground">{getLocationTypeLabel(location.type)}:</span
				>
				{location.name}
			</h1>
			<p class="my-2 text-muted-foreground">{data.total} cards in this location</p>
		</div>
		{#if location.description}
			<p class="mt-4 max-w-2xl text-sm">{location.description}</p>
		{/if}
	</div>

	<CollectionCardTable
		collection={data.collection}
		total={data.total}
		page={data.page}
		limit={data.limit}
		sortBy={data.sortBy}
		sortDir={data.sortDir}
		q={data.q}
		{profile}
		user={data.user}
		locations={data.locations}
		defaultLocationId={location.id}
		{browserLocale}
		emptyMessage="No cards found in this location."
		backButtonHref="/profile/{profile.username}/collection"
		backButtonText="Back to Collection"
	/>

	<Separator class="my-8" />
</div>
