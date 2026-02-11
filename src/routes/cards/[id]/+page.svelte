<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import ManaCost from '$lib/components/ManaCost.svelte';
	import { enhance } from '$app/forms';
	import { Badge } from '$lib/components/ui/badge';
	import { getLocationTypeLabel } from '$lib/collection';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	function formatOracleText(text: string) {
		return text.replace(/\{[^}]+\}/g, (match) => {
			const s = match.replace(/\{|\}/g, '').replace(/\//g, '');
			return `<img src="https://svgs.scryfall.io/card-symbols/${s}.svg" alt="${match}" class="inline-block h-[1.1em] w-[1.1em] align-text-bottom mx-0.5" />`;
		});
	}
</script>

<div class="container mx-auto px-4 py-8">
	{#if data.error}
		<div class="rounded-md border border-destructive/20 bg-destructive/10 p-4 text-destructive">
			{data.error}
		</div>
	{:else if data.card}
		<div class="grid gap-8 md:grid-cols-2">
			<div class="flex justify-center">
				{#if data.card.image_uris}
					<div
						class="aspect-[63/88] w-full max-w-[480px] overflow-hidden rounded-[4.8%] bg-black shadow-2xl"
					>
						<img
							src={data.card.image_uris.large}
							alt={data.card.name}
							class="h-full w-full rounded-[4.8%] object-cover"
						/>
					</div>
				{:else if data.card.card_faces}
					<div class="flex flex-col gap-4">
						{#each data.card.card_faces as face}
							{#if face.image_uris}
								<div
									class="aspect-[63/88] w-full max-w-[480px] overflow-hidden rounded-[4.8%] bg-black shadow-2xl"
								>
									<img
										src={face.image_uris.large}
										alt={face.name}
										class="h-full w-full rounded-[4.8%] object-cover"
									/>
								</div>
							{/if}
						{/each}
					</div>
				{/if}
			</div>

			<div class="flex flex-col gap-6">
				<div class="rounded-md bg-muted p-4">
					<div class="flex items-start justify-between gap-4">
						<h1 class="text-3xl font-bold">{data.card.name}</h1>
						{#if data.card.mana_cost}
							<div class="mt-1 text-xl">
								<ManaCost cost={data.card.mana_cost} />
							</div>
						{/if}
					</div>
					<p class="mt-1 text-lg text-muted-foreground">{data.card.type_line}</p>
				</div>

				{#if data.card.oracle_text}
					<div class="rounded-md bg-muted p-4 whitespace-pre-wrap">
						{@html formatOracleText(data.card.oracle_text)}
					</div>
				{/if}

				{#if data.card.flavor_text}
					<div class="whitespace-pre-wrap text-muted-foreground italic">
						{data.card.flavor_text}
					</div>
				{/if}

				<div class="grid grid-cols-2 gap-4 text-sm">
					<div>
						<span class="font-semibold text-muted-foreground">Set:</span>
						{data.card.set_name} ({data.card.set.toUpperCase()})
					</div>
					<div>
						<span class="font-semibold text-muted-foreground">Rarity:</span>
						{data.card.rarity.charAt(0).toUpperCase() + data.card.rarity.slice(1)}
					</div>
					<div>
						<span class="font-semibold text-muted-foreground">Collector #:</span>
						{data.card.collector_number}
					</div>
					<div>
						<span class="font-semibold text-muted-foreground">Artist:</span>
						{data.card.artist}
					</div>
				</div>

				<div class="mt-4 flex flex-col gap-4">
					<Button
						href={data.card.scryfall_uri}
						variant="outline"
						target="_blank"
						rel="noopener noreferrer"
						class="w-full px-6 text-base sm:w-auto"
					>
						View on Scryfall
					</Button>

					{#if data.user}
						<div class="rounded-lg border bg-card p-6 shadow-sm">
							<h3 class="mb-6 text-xl font-semibold">Add to Collection</h3>
							<form method="POST" action="?/addToCollection" use:enhance class="space-y-6">
								<div class="grid grid-cols-2 gap-6">
									<div class="space-y-3">
										<label for="condition" class="text-sm font-medium">Condition</label>
										<select
											name="condition"
											id="condition"
											class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
										>
											<option value="NM">Near Mint</option>
											<option value="LP">Lightly Played</option>
											<option value="MP">Moderately Played</option>
											<option value="HP">Heavily Played</option>
											<option value="DMG">Damaged</option>
										</select>
									</div>
									<div class="space-y-3">
										<label for="storageLocationId" class="text-sm font-medium">Location</label>
										<select
											name="storageLocationId"
											id="storageLocationId"
											class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
										>
											<option value="none">No Location</option>
											{#each data.locations || [] as location}
												<option value={location.id}>
													{location.name} ({getLocationTypeLabel(location.type)})
												</option>
											{/each}
										</select>
									</div>
								</div>

								<div class="flex items-center gap-3">
									<input
										type="checkbox"
										name="isFoil"
										id="isFoil"
										value="true"
										class="h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary"
									/>
									<label for="isFoil" class="text-sm font-medium">Foil printing</label>
								</div>

								<Button type="submit" class="w-full py-6 text-lg">Add to Collection</Button>
							</form>
						</div>
					{/if}
				</div>
			</div>
		</div>

		{#if data.rulings && data.rulings.data.length > 0}
			<div class="mt-16">
				<h2 class="mb-6 text-2xl font-bold">Rulings</h2>
				<div class="flex flex-col gap-4">
					{#each data.rulings.data as ruling}
						<div class="rounded-lg border bg-card p-4 text-card-foreground shadow-sm">
							<div class="mb-2 text-sm font-semibold text-muted-foreground">
								{new Date(ruling.published_at).toLocaleDateString(undefined, {
									year: 'numeric',
									month: 'long',
									day: 'numeric'
								})}
							</div>
							<div class="prose prose-sm dark:prose-invert max-w-none">
								{@html formatOracleText(ruling.comment)}
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		{#if data.prints && data.prints.data.length > 1}
			<div class="mt-16">
				<h2 class="mb-6 text-2xl font-bold">Other Printings</h2>
				<div class="grid grid-cols-3 gap-4 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8">
					{#each data.prints.data as printing}
						<a
							href="/cards/{printing.id}"
							class="group flex flex-col gap-2 transition-transform hover:scale-105"
							title="{printing.set_name} #{printing.collector_number}"
						>
							<div class="aspect-[63/88] w-full overflow-hidden rounded-[4.8%] bg-black shadow-sm">
								{#if printing.image_uris?.small}
									<img
										src={printing.image_uris.small}
										alt={printing.set_name}
										class="h-full w-full rounded-[4.8%] object-cover"
										loading="lazy"
									/>
								{:else if printing.card_faces?.[0]?.image_uris?.small}
									<img
										src={printing.card_faces[0].image_uris.small}
										alt={printing.set_name}
										class="h-full w-full rounded-[4.8%] object-cover"
										loading="lazy"
									/>
								{:else}
									<div class="flex h-full w-full items-center justify-center bg-muted text-[10px]">
										{printing.set.toUpperCase()}
									</div>
								{/if}
							</div>
							<div class="truncate text-center text-xs font-medium text-muted-foreground">
								{printing.set.toUpperCase()} • #{printing.collector_number}
							</div>
						</a>
					{/each}
				</div>
			</div>
		{/if}
	{/if}
</div>
