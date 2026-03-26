<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import * as Card from '$lib/components/ui/card';
	import { Layout, Plus, ArrowLeft } from 'lucide-svelte';

	let { data } = $props();
	let profile = $derived(data.profile);
</script>

<div class="mx-auto max-w-7xl px-4 py-8">
	<div class="mb-8 flex flex-col justify-between gap-4 sm:flex-row sm:items-end">
		<div>
			<a
				href="/profile/{profile.username}"
				class="mb-2 flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground"
			>
				<ArrowLeft class="h-4 w-4" />
				Back to Profile
			</a>
			<h1 class="text-4xl font-bold">{profile.username}'s Decks</h1>
			<p class="text-muted-foreground">
				Digital decklists created by {profile.username}.
			</p>
		</div>
		{#if data.isOwnProfile}
			<Button href="/decks/new" class="w-full sm:w-auto">
				<Plus class="mr-2 h-4 w-4" />
				Create Deck
			</Button>
		{/if}
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
							<div class="flex justify-between border-t pt-4">
								<div class="text-center">
									<p class="text-xl font-bold">{deck.virtualCount}</p>
									<p class="text-xs tracking-wider text-muted-foreground uppercase">Cards</p>
								</div>
								<div class="text-center">
									<p class="text-xl font-bold">{deck.physicalCount}</p>
									<p class="text-xs tracking-wider text-muted-foreground uppercase">Physical</p>
								</div>
							</div>
						</Card.Content>
					</Card.Root>
				</a>
			{/each}
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center py-20 text-center">
			<p class="text-xl font-medium">No decks found.</p>
			<p class="mt-2 text-muted-foreground">
				{data.isOwnProfile
					? "You haven't created any decks yet."
					: `${profile.username} hasn't created any decks yet.`}
			</p>
			{#if data.isOwnProfile}
				<Button href="/decks/new" variant="outline" class="mt-6">Create Your First Deck</Button>
			{/if}
		</div>
	{/if}
</div>
