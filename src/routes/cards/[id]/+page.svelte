<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import ManaCost from '$lib/components/ManaCost.svelte';
	import { enhance } from '$app/forms';
	import { Badge } from '$lib/components/ui/badge';
	import { formatPrice, getLanguageLabel } from '$lib/collection';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

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
			<div class="flex items-start justify-center" data-set={data.card.set}>
				{#if data.card.image_uris}
					<div class="w-full max-w-[480px] overflow-hidden rounded-card bg-black shadow-2xl">
						<img
							src={data.card.image_uris.large}
							alt={data.card.name}
							class="block h-auto w-full rounded-card"
						/>
					</div>
				{:else if data.card.card_faces}
					<div class="flex w-full flex-col items-center gap-4">
						{#each data.card.card_faces as face}
							{#if face.image_uris}
								<div class="w-full max-w-[480px] overflow-hidden rounded-card bg-black shadow-2xl">
									<img
										src={face.image_uris.large}
										alt={face.name}
										class="block h-auto w-full rounded-card"
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
						<h1 class="text-3xl font-bold">
							{data.card.printed_name || data.card.name}
							{#if data.card.printed_name && data.card.printed_name !== data.card.name}
								<span class="block text-lg font-normal text-muted-foreground">{data.card.name}</span
								>
							{/if}
						</h1>
						{#if data.card.mana_cost}
							<div class="mt-1 text-xl">
								<ManaCost cost={data.card.mana_cost} />
							</div>
						{/if}
					</div>
					<p class="mt-1 text-lg text-muted-foreground">
						{data.card.printed_type_line || data.card.type_line}
						{#if data.card.printed_type_line && data.card.printed_type_line !== data.card.type_line}
							<span class="ml-1 text-sm font-normal">({data.card.type_line})</span>
						{/if}
					</p>
				</div>

				{#if data.card.oracle_text || data.card.printed_text}
					<div class="flex flex-col gap-4">
						{#if data.card.printed_text}
							<div class="rounded-md bg-muted p-4 whitespace-pre-wrap">
								<div class="mb-2 text-[10px] font-bold text-muted-foreground uppercase">
									Printed Text
								</div>
								{@html formatOracleText(data.card.printed_text)}
							</div>
						{/if}
						{#if data.card.oracle_text}
							<div class="rounded-md bg-muted p-4 whitespace-pre-wrap">
								{#if data.card.printed_text}
									<div class="mb-2 text-[10px] font-bold text-muted-foreground uppercase">
										Oracle Text
									</div>
								{/if}
								{@html formatOracleText(data.card.oracle_text)}
							</div>
						{/if}
					</div>
				{/if}

				{#if !data.card.oracle_text && data.card.card_faces}
					<div class="flex flex-col gap-4">
						{#each data.card.card_faces as face}
							<div class="flex flex-col gap-2 rounded-md bg-muted p-4">
								<div class="flex items-start justify-between">
									<h3 class="font-bold">
										{face.printed_name || face.name}
										{#if face.printed_name && face.printed_name !== face.name}
											<span class="block text-sm font-normal text-muted-foreground"
												>{face.name}</span
											>
										{/if}
									</h3>
									{#if face.mana_cost}
										<ManaCost cost={face.mana_cost} />
									{/if}
								</div>
								<p class="text-sm text-muted-foreground">
									{face.printed_type_line || face.type_line}
									{#if face.printed_type_line && face.printed_type_line !== face.type_line}
										<span class="ml-1 text-[10px] font-normal">({face.type_line})</span>
									{/if}
								</p>

								{#if face.printed_text}
									<div class="mt-2 border-t pt-2 text-sm whitespace-pre-wrap">
										<div class="mb-1 text-[10px] font-bold text-muted-foreground uppercase">
											Printed
										</div>
										{@html formatOracleText(face.printed_text)}
									</div>
								{/if}
								{#if face.oracle_text}
									<div
										class="mt-2 text-sm whitespace-pre-wrap {face.printed_text
											? 'border-t pt-2'
											: ''}"
									>
										{#if face.printed_text}
											<div class="mb-1 text-[10px] font-bold text-muted-foreground uppercase">
												Oracle
											</div>
										{/if}
										{@html formatOracleText(face.oracle_text)}
									</div>
								{/if}
								{#if face.flavor_text}
									<div class="mt-2 text-xs text-muted-foreground italic">
										{face.flavor_text}
									</div>
								{/if}
							</div>
						{/each}
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
						<span class="font-semibold text-muted-foreground">Released:</span>
						{new Date(data.card.released_at).toLocaleDateString(undefined, {
							month: 'long',
							year: 'numeric'
						})}
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
																	class="h-5 bg-amber-600 px-2 text-[10px] text-white"
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

		{#if data.languages && data.languages.data.length > 1}
			<div class="mt-16">
				<h2 class="mb-6 text-2xl font-bold">Other Languages</h2>
				<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
					{#each data.languages.data as langCard}
						{#if langCard.id !== data.card.id}
							<a
								href="/cards/{langCard.id}"
								data-set={langCard.set}
								class="flex items-center gap-3 rounded-lg border bg-card p-3 shadow-sm transition-colors hover:bg-muted/50"
							>
								<div class="aspect-[488/680] h-12 shrink-0 overflow-hidden rounded-card bg-black">
									{#if langCard.image_uris?.small}
										<img
											src={langCard.image_uris.small}
											alt={langCard.lang}
											class="h-full w-full rounded-card object-fill"
										/>
									{:else if langCard.card_faces?.[0]?.image_uris?.small}
										<img
											src={langCard.card_faces[0].image_uris.small}
											alt={langCard.lang}
											class="h-full w-full rounded-card object-fill"
										/>
									{/if}
								</div>
								<div class="flex flex-col overflow-hidden">
									<span class="text-sm font-semibold">{getLanguageLabel(langCard.lang)}</span>
									<span class="truncate text-xs text-muted-foreground">{langCard.name}</span>
								</div>
								<div class="ml-auto flex shrink-0 flex-col items-end">
									<span class="text-xs font-bold"
										>{formatPrice(langCard.prices.usd, 'USD', browserLocale)}</span
									>
									{#if langCard.prices.eur}
										<span class="text-[10px] text-muted-foreground"
											>{formatPrice(langCard.prices.eur, 'EUR', browserLocale)}</span
										>
									{/if}
								</div>
							</a>
						{/if}
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
							data-set={printing.set}
							class="group flex flex-col gap-2 transition-transform hover:scale-105"
							title="{printing.set_name} #{printing.collector_number}"
						>
							<div class="aspect-[488/680] w-full overflow-hidden rounded-card bg-black shadow-sm">
								{#if printing.image_uris?.small}
									<img
										src={printing.image_uris.small}
										alt={printing.set_name}
										class="h-full w-full rounded-card object-fill"
										loading="lazy"
									/>
								{:else if printing.card_faces?.[0]?.image_uris?.small}
									<img
										src={printing.card_faces[0].image_uris.small}
										alt={printing.set_name}
										class="h-full w-full rounded-card object-fill"
										loading="lazy"
									/>
								{:else}
									<div class="flex h-full w-full items-center justify-center bg-muted text-[10px]">
										{printing.set.toUpperCase()}
									</div>
								{/if}
							</div>
							<div class="flex flex-col">
								<div
									class="truncate text-center text-[9px] font-medium tracking-tighter text-foreground/80"
								>
									{printing.set.toUpperCase()} • #{printing.collector_number}
								</div>
								<div class="text-center text-[9px] text-foreground/60">
									{new Date(printing.released_at).toLocaleDateString(undefined, {
										month: 'short',
										year: 'numeric'
									})}
								</div>
							</div>
						</a>
					{/each}
				</div>
			</div>
		{/if}
	{/if}
</div>
