<script lang="ts">
	import type { ScryfallCard } from '$lib/scryfall';

	let { card }: { card: ScryfallCard } = $props();

	let imageUrl = $derived(
		card.image_uris?.normal || card.card_faces?.[0]?.image_uris?.normal || ''
	);
</script>

<a
	href="/cards/{card.id}"
	class="group relative aspect-[63/88] w-full overflow-hidden rounded-[4.8%] bg-black shadow-md transition-all hover:scale-105 hover:shadow-xl"
>
	{#if imageUrl}
		<img
			src={imageUrl}
			alt={card.name}
			class="h-full w-full rounded-[4.8%] object-cover"
			loading="lazy"
		/>
	{:else}
		<div
			class="flex h-full w-full items-center justify-center bg-black p-4 text-center text-xs text-muted-foreground"
		>
			{card.name}<br />(No Image)
		</div>
	{/if}
</a>
