<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import * as Card from '$lib/components/ui/card';
	import { Layout } from 'lucide-svelte';

	let { data } = $props();
	let profile = $derived(data.profile);
</script>

<div class="mx-auto max-w-7xl px-4 py-8">
	<div class="mb-8 flex flex-col justify-between gap-4 sm:flex-row sm:items-end">
		<div>
			<div class="flex items-center gap-2 text-sm text-muted-foreground">
				<a href="/profile/{profile.username}/collection" class="hover:underline">Collection</a>
				<span>/</span>
				<span class="text-foreground">Decks</span>
			</div>
			<h1 class="mt-2 text-4xl font-bold">Physical Decks</h1>
			<p class="text-muted-foreground">
				Decks built using cards from {profile.username}'s collection
			</p>
		</div>
	</div>

	<div class="mb-8 flex gap-4 overflow-x-auto pb-2">
		<Button variant="ghost" href="/profile/{profile.username}/collection" class="shrink-0">
			All Cards
		</Button>
		<Button
			variant="ghost"
			href="/profile/{profile.username}/collection/locations"
			class="shrink-0"
		>
			Locations
		</Button>
		<Button
			variant="secondary"
			href="/profile/{profile.username}/collection/decks"
			class="shrink-0"
		>
			Decks
		</Button>
	</div>

	<Separator class="mb-8" />

	{#if data.decks.length > 0}
		<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
			{#each data.decks as deck}
				<a href="/decks/{deck.id}" class="group block transition-transform hover:scale-[1.02]">
					<Card.Root>
						<Card.Header>
							<div class="flex items-center justify-between">
								<div class="rounded-full bg-primary/10 p-2 text-primary">
									<Layout class="h-5 w-5" />
								</div>
							</div>
							<Card.Title class="mt-4">{deck.name}</Card.Title>
							{#if deck.description}
								<Card.Description class="line-clamp-2">{deck.description}</Card.Description>
							{/if}
						</Card.Header>
						<Card.Content>
							<p class="text-2xl font-bold">{deck.cardCount}</p>
							<p class="text-sm text-muted-foreground">Physical cards assigned</p>
						</Card.Content>
					</Card.Root>
				</a>
			{/each}
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center py-20 text-center">
			<p class="text-xl font-medium">No decks found.</p>
			<p class="mt-2 text-muted-foreground">
				Users can assign physical cards to their digital decklists.
			</p>
			<Button href="/profile/{profile.username}" variant="outline" class="mt-6">
				Back to Profile
			</Button>
		</div>
	{/if}
</div>
