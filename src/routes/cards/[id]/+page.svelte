<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import ManaCost from '$lib/components/ManaCost.svelte';
	import { enhance } from '$app/forms';
	import { Badge } from '$lib/components/ui/badge';
	import { getLocationTypeLabel, formatCurrentPrice, formatPrice } from '$lib/collection';
	import { Plus, X } from 'lucide-svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let showAddForm = $state(false);

	let browserLocale = $state('en-US');
	$effect(() => {
		browserLocale = navigator.language || 'en-US';
	});

	function formatOracleText(text: string) {
		return text.replace(/\{[^}]+\}/g, (match) => {
			const s = match.replace(/\{|\}/g, '').replace(/\//g, '');
			return `<img src="https://svgs.scryfall.io/card-symbols/${s}.svg" alt="${match}" class="inline-block h-[1.1em] w-[1.1em] align-text-bottom mx-0.5" />`;
		});
	}
</script>

<div class="container mx-auto px-4 py-8">
	{#if data.card}
		<div class="grid gap-8 md:grid-cols-2">
			<div class="flex justify-center">
				{#if data.card.image_uris}
					<div class="w-full max-w-[480px] overflow-hidden rounded-[4.8%] shadow-2xl">
						<img
							src={data.card.image_uris.large}
							alt={data.card.name}
							class="h-auto w-full rounded-[4.8%]"
						/>
					</div>
				{:else if data.card.card_faces}
					<div class="flex flex-col gap-4">
						{#each data.card.card_faces as face}
							{#if face.image_uris}
								<div class="w-full max-w-[480px] overflow-hidden rounded-[4.8%] shadow-2xl">
									<img
										src={face.image_uris.large}
										alt={face.name}
										class="h-auto w-full rounded-[4.8%]"
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

				<div class="mt-2">
					<h3 class="mb-3 text-lg font-semibold">Market Prices</h3>
					<div class="flex flex-wrap gap-4">
						<div class="flex min-w-[100px] flex-col rounded-lg border bg-card p-3 shadow-sm">
							<span class="text-[10px] font-semibold text-muted-foreground uppercase">USD</span>
							<span class="text-lg font-bold"
								>{formatPrice(data.card.prices.usd, 'USD', browserLocale)}</span
							>
						</div>
						<div class="flex min-w-[100px] flex-col rounded-lg border bg-card p-3 shadow-sm">
							<span class="text-[10px] font-semibold text-muted-foreground uppercase">USD Foil</span
							>
							<span class="text-lg font-bold"
								>{formatPrice(data.card.prices.usd_foil, 'USD', browserLocale)}</span
							>
						</div>
						<div class="flex min-w-[100px] flex-col rounded-lg border bg-card p-3 shadow-sm">
							<span class="text-[10px] font-semibold text-muted-foreground uppercase">EUR</span>
							<span class="text-lg font-bold"
								>{formatPrice(data.card.prices.eur, 'EUR', browserLocale)}</span
							>
						</div>
					</div>
				</div>

				<div class="mt-2">
					<h3 class="mb-3 text-lg font-semibold">Legalities</h3>
					<div class="overflow-hidden rounded-lg border">
						<table class="w-full text-sm">
							<tbody class="divide-y">
								{#each Array(Math.ceil(Object.entries(data.card.legalities).length / 2)) as _, i}
									{@const entries = Object.entries(data.card.legalities)}
									<tr class="flex flex-col divide-y sm:table-row sm:divide-x sm:divide-y-0">
										{#each [entries[i * 2], entries[i * 2 + 1]] as entry}
											{#if entry}
												{@const [format, status] = entry}
												<td class="w-full px-4 py-2 hover:bg-muted/50 sm:w-1/2">
													<div class="flex items-center justify-between gap-2">
														<span class="font-medium capitalize">{format.replace(/_/g, ' ')}</span>
														<div class="flex items-center gap-2">
															<Badge
																variant={status === 'legal'
																	? 'default'
																	: status === 'restricted'
																		? 'secondary'
																		: status === 'banned'
																			? 'destructive'
																			: 'outline'}
																class="h-5 px-2 text-[10px] uppercase"
															>
																{status.replace(/_/g, ' ')}
															</Badge>
															{#if format === 'commander' && data.card.game_changer}
																<Badge
																	variant="secondary"
																	class="h-5 bg-amber-500 px-2 text-[10px] text-white hover:bg-amber-600"
																>
																	GC
																</Badge>
															{/if}
														</div>
													</div>
												</td>
											{:else}
												<td class="hidden w-1/2 px-4 py-2 sm:table-cell"></td>
											{/if}
										{/each}
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>

				<div class="mt-4 flex flex-col gap-4">
					<div class="flex flex-wrap gap-3">
						<Button
							href={data.card.scryfall_uri}
							variant="outline"
							target="_blank"
							rel="noopener noreferrer"
							class="flex-1 px-6 text-base sm:flex-initial"
						>
							View on Scryfall
						</Button>
					</div>
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
