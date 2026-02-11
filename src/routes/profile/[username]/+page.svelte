<script lang="ts">
	let { data } = $props();
    let profile = $derived(data.profile);
    let decks = $derived(data.decks);
</script>

<svelte:head>
	<title>{profile.username}'s Profile - Oshibana</title>
</svelte:head>

<div class="mx-auto max-w-4xl px-4 py-8">
	<div class="mb-8 flex flex-col gap-6 border-b pb-6 md:flex-row md:items-end md:justify-between">
		<div>
			<h1 class="text-4xl font-bold">{profile.username}</h1>
			<p class="mt-2 text-muted-foreground">
				Member since {new Date(profile.createdAt).toLocaleDateString()}
			</p>
		</div>

		<div class="flex gap-8">
			<div class="text-center">
				<p class="text-2xl font-bold">{data.stats.deckCount}</p>
				<p class="text-xs tracking-wider text-muted-foreground uppercase">Decks</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-bold">{data.stats.cardCount}</p>
				<p class="text-xs tracking-wider text-muted-foreground uppercase">Cards</p>
			</div>
		</div>
	</div>

	<section>
		<h2 class="mb-4 text-2xl font-semibold">Decks</h2>
		{#if decks.length > 0}
			<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
				{#each decks as deck}
					<a
						href="/decks/{deck.id}"
						class="block rounded-lg border bg-card p-4 transition-colors hover:bg-accent"
					>
						<h3 class="font-medium">{deck.name}</h3>
						{#if deck.description}
							<p class="mt-1 line-clamp-2 text-sm text-muted-foreground">
								{deck.description}
							</p>
						{/if}
						<p class="mt-4 text-xs text-muted-foreground">
							Updated {new Date(deck.updatedAt).toLocaleDateString()}
						</p>
					</a>
				{/each}
			</div>
		{:else}
			<p class="text-muted-foreground">This user hasn't created any decks yet.</p>
		{/if}
	</section>
</div>
