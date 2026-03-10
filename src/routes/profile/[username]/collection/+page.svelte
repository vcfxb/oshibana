<script lang="ts">
	import { Separator } from '$lib/components/ui/separator';
	import CollectionCardTable from '$lib/components/CollectionCardTable.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let browserLocale = $state('en-US');

	$effect(() => {
		browserLocale = navigator.language || 'en-US';
	});
</script>

<div class="mx-auto max-w-[90rem] px-4 py-8">
	<div class="mb-8 flex flex-col justify-between gap-4 sm:flex-row sm:items-end">
		<div>
			<h1 class="mt-2 text-4xl font-bold">{data.profile.username}'s Collection</h1>
			<p class="text-muted-foreground">{data.total} cards total</p>
		</div>
	</div>

	<CollectionCardTable
		collection={data.collection}
		total={data.total}
		page={data.page}
		limit={data.limit}
		sortBy={data.sortBy}
		sortDir={data.sortDir}
		profile={data.profile}
		user={data.user}
		locations={data.locations}
		defaultLocationId="none"
		{browserLocale}
	/>

	<Separator class="my-8" />
</div>
