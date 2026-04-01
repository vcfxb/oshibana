<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Drawer from '$lib/components/ui/drawer';
	import { Input } from '$lib/components/ui/input';
	import { LoaderCircle, Check, Save } from 'lucide-svelte';
	import { enhance } from '$app/forms';
	import type { DbStorageLocation } from '$lib/server/db/types';

	let {
		open = $bindable(false),
		item,
		locations = []
	}: {
		open: boolean;
		item: any;
		locations: DbStorageLocation[];
	} = $props();

	let isSaving = $state(false);
	let formElement = $state<HTMLFormElement | null>(null);

	function handleGlobalKeydown(e: KeyboardEvent) {
		if (!open) return;

		if (e.key === 'Enter' && !isSaving) {
			const activeElement = document.activeElement;
			if (activeElement?.tagName === 'BUTTON' || activeElement?.tagName === 'TEXTAREA') {
				return;
			}
			e.preventDefault();
			formElement?.requestSubmit();
		}
	}
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<Drawer.Root bind:open>
	<Drawer.Content class="max-h-[90vh]">
		<div class="mx-auto w-full max-w-5xl overflow-y-auto px-4 pt-6 pb-10">
			<Drawer.Header class="px-0 text-left">
				<Drawer.Title class="text-2xl">Edit Card</Drawer.Title>
				<Drawer.Description>Update the details for this card in your collection.</Drawer.Description
				>
			</Drawer.Header>

			<div class="mt-6">
				{#if item}
					<div class="rounded-xl border bg-muted/30 p-6">
						<div class="flex flex-col gap-6 lg:flex-row lg:items-start">
							<div
								class="relative aspect-[488/680] w-full shrink-0 overflow-hidden rounded-card bg-black shadow-2xl lg:w-80"
								data-set={item.cardData?.set}
							>
								{#if item.cardData?.imageUri}
									<img
										src={item.cardData.imageUri}
										alt={item.cardData.name}
										class="h-full w-full rounded-card object-fill"
									/>
								{:else}
									<div class="flex h-full w-full items-center justify-center bg-muted"></div>
								{/if}
							</div>

							<div class="min-w-0 flex-1">
								<div class="mb-6">
									<h4 class="truncate text-2xl font-bold">{item.cardData?.name}</h4>
									<p class="mt-1 text-sm text-muted-foreground uppercase">
										{item.cardData?.setName} • #{item.cardData?.collectorNumber}
									</p>
								</div>

								<form
									bind:this={formElement}
									method="POST"
									action="?/updateCard"
									use:enhance={() => {
										isSaving = true;
										return async ({ result, update }) => {
											if (result.type === 'success') {
												open = false;
												await update();
											}
											isSaving = false;
										};
									}}
									class="space-y-6"
								>
									<input type="hidden" name="physicalCardId" value={item.physicalCard.id} />

									<div class="grid grid-cols-2 gap-4">
										<div class="space-y-2">
											<label
												for="edit-condition"
												class="text-xs font-semibold text-muted-foreground uppercase"
												>Condition</label
											>
											<select
												name="condition"
												id="edit-condition"
												class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-primary"
												value={item.physicalCard.condition}
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
												for="edit-storageLocationId"
												class="text-xs font-semibold text-muted-foreground uppercase"
												>Location</label
											>
											<select
												name="storageLocationId"
												id="edit-storageLocationId"
												class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-primary"
												value={item.physicalCard.storageLocationId || 'none'}
											>
												<option value="none">No Location</option>
												{#each locations as location}
													<option value={location.id}>
														{location.name}
													</option>
												{/each}
											</select>
										</div>
									</div>

									<div class="grid grid-cols-2 gap-4 sm:grid-cols-3">
										<div class="space-y-2">
											<label
												for="edit-quantity"
												class="text-xs font-semibold text-muted-foreground uppercase"
												>Quantity</label
											>
											<Input
												type="number"
												name="quantity"
												id="edit-quantity"
												value={item.physicalCard.quantity}
												min="1"
												required
											/>
										</div>
										<div class="space-y-2">
											<label
												for="edit-purchasePrice"
												class="text-xs font-semibold text-muted-foreground uppercase"
												>Price Paid</label
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
													id="edit-purchasePrice"
													placeholder="0.00"
													class="pl-7"
													value={item.physicalCard.purchasePrice
														? (item.physicalCard.purchasePrice / 100).toFixed(2)
														: ''}
												/>
											</div>
										</div>
										<div class="space-y-2">
											<label
												for="edit-language"
												class="text-xs font-semibold text-muted-foreground uppercase"
												>Language</label
											>
											<select
												name="language"
												id="edit-language"
												class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-primary"
												value={item.physicalCard.language || 'en'}
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
												checked={item.physicalCard.isFoil}
												class="h-4 w-4 rounded border-input"
											/>
											Foil
										</label>
										<label class="flex cursor-pointer items-center gap-2 text-sm font-medium">
											<input
												type="checkbox"
												name="isAlter"
												value="true"
												checked={item.physicalCard.isAlter}
												class="h-4 w-4 rounded border-input"
											/>
											Alter
										</label>
										<label class="flex cursor-pointer items-center gap-2 text-sm font-medium">
											<input
												type="checkbox"
												name="isProxy"
												value="true"
												checked={item.physicalCard.isProxy}
												class="h-4 w-4 rounded border-input"
											/>
											Proxy
										</label>
									</div>

									<div class="space-y-2">
										<div class="flex items-center justify-between">
											<label
												for="edit-notes"
												class="text-xs font-semibold text-muted-foreground uppercase">Notes</label
											>
											<span class="text-[10px] text-muted-foreground">Optional • Max 250 chars</span
											>
										</div>
										<textarea
											name="notes"
											id="edit-notes"
											rows="2"
											maxlength="250"
											placeholder="Add any specific details about this card..."
											class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-primary"
											value={item.physicalCard.notes || ''}
										></textarea>
									</div>

									<div class="flex justify-end">
										<Button type="submit" disabled={isSaving} class="h-12 px-8 text-lg">
											{#if isSaving}
												<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
												Saving...
											{:else}
												<Save class="mr-2 h-4 w-4" />
												Save Changes
											{/if}
										</Button>
									</div>
								</form>
							</div>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</Drawer.Content>
</Drawer.Root>
