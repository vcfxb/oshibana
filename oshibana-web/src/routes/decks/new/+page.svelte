<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
		CardFooter
	} from '$lib/components/ui/card';
	import { ArrowLeft, PanelsTopLeft, Shield, Swords, Users, Info } from 'lucide-svelte';
	import { enhance } from '$app/forms';
	import * as Select from '$lib/components/ui/select';

	let { data } = $props();
	let loading = $state(false);

	const formats = [
		{ value: 'commander', label: 'Commander / EDH' },
		{ value: 'standard', label: 'Standard' },
		{ value: 'pioneer', label: 'Pioneer' },
		{ value: 'modern', label: 'Modern' },
		{ value: 'legacy', label: 'Legacy' },
		{ value: 'vintage', label: 'Vintage' },
		{ value: 'pauper', label: 'Pauper' },
		{ value: 'oathbreaker', label: 'Oathbreaker' },
		{ value: 'brawl', label: 'Brawl' },
		{ value: 'limited', label: 'Limited' },
		{ value: 'other', label: 'Other' }
	];

	let selectedFormat = $state('commander');
	let isCommander = $derived(['commander', 'brawl', 'oathbreaker'].includes(selectedFormat));
</script>

<svelte:head>
	<title>Create New Deck - Oshibana</title>
</svelte:head>

<div class="mx-auto max-w-2xl px-4 py-12">
	<a
		href="/decks"
		class="mb-6 flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground"
	>
		<ArrowLeft class="h-4 w-4" />
		Back to Decks
	</a>

	<Card.Root>
		<Card.Header>
			<div class="flex items-center gap-2">
				<div class="rounded-full bg-primary/10 p-2 text-primary">
					<PanelsTopLeft class="h-5 w-5" />
				</div>
				<Card.Title class="text-2xl">Create New Deck</Card.Title>
			</div>
			<Card.Description>
				Enter the initial details for your new deck. You can start adding cards immediately after
				creation.
			</Card.Description>
		</Card.Header>
		<form
			method="POST"
			use:enhance={() => {
				loading = true;
				return async ({ update }) => {
					await update();
					loading = false;
				};
			}}
		>
			<Card.Content class="space-y-6">
				<div class="space-y-2">
					<Label for="name">Deck Name</Label>
					<Input id="name" name="name" placeholder="e.g., Mono-Green Stompy" required autofocus />
				</div>

				<div class="space-y-2">
					<Label for="format">Format</Label>
					<Select.Root type="single" bind:value={selectedFormat} name="format">
						<Select.Trigger class="w-full">
							{formats.find((f) => f.value === selectedFormat)?.label ?? 'Select a format'}
						</Select.Trigger>
						<Select.Content>
							{#each formats as format}
								<Select.Item value={format.value}>{format.label}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>

				{#if isCommander}
					<div class="space-y-4 rounded-lg border bg-muted/30 p-4">
						<div class="flex items-center gap-2 text-sm font-medium">
							<Shield class="h-4 w-4 text-primary" />
							Commander Settings
						</div>
						<div class="space-y-2">
							<Label for="commander">Commander(s)</Label>
							<Input id="commander" name="commander" placeholder="Enter card name(s)..." />
							<p class="text-[10px] text-muted-foreground">
								You can also add these later from the deckbuilder.
							</p>
						</div>
					</div>
				{/if}

				<div class="space-y-2">
					<Label for="description">Description</Label>
					<Input
						id="description"
						name="description"
						placeholder="A brief overview of the deck's strategy..."
					/>
				</div>

				<div class="space-y-4 border-t pt-4">
					<div class="flex items-center gap-2 text-sm font-medium">
						<Swords class="h-4 w-4 text-primary" />
						Physical Tracking
					</div>
					<div class="space-y-2">
						<Label for="physicalLocation">Link to Physical Deck</Label>
						<Select.Root type="single" name="physicalLocation">
							<Select.Trigger class="w-full">Select a physical location (optional)</Select.Trigger>
							<Select.Content>
								<Select.Item value="none">Not linked to a physical deck</Select.Item>
								{#each data.storageLocations.filter((l) => l.type === 'physical_deck') as location}
									<Select.Item value={location.id}>{location.name}</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>
						<p class="text-xs text-muted-foreground">
							Link this digital deck to a physical location to track card availability and proxies.
						</p>
					</div>
				</div>
			</Card.Content>
			<Card.Footer class="flex flex-col gap-4 border-t pt-6">
				<Button type="submit" class="w-full" disabled={loading}>
					{loading ? 'Creating...' : 'Create Deck'}
				</Button>
				<div
					class="flex items-start gap-2 rounded-md bg-accent/50 p-3 text-[11px] text-muted-foreground"
				>
					<Info class="mt-0.5 h-3 w-3 shrink-0" />
					<p>
						Deck changes are automatically tracked in a historical changelog. You can view previous
						versions of your deck at any time.
					</p>
				</div>
			</Card.Footer>
		</form>
	</Card.Root>
</div>
