<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { enhance } from '$app/forms';
	import { UserPlus, UserMinus } from 'lucide-svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let profile = $derived(data.profile);
	let decks = $derived(data.decks);
</script>

<svelte:head>
	<title>{profile.username}'s Profile - Oshibana</title>
</svelte:head>

<div class="mx-auto max-w-4xl px-4 py-8">
	<div class="mb-8 flex flex-col gap-6 border-b pb-6 md:flex-row md:items-end md:justify-between">
		<div class="flex flex-col gap-4">
			<div>
				<h1 class="text-4xl font-bold">{profile.username}</h1>
				<p class="mt-2 text-muted-foreground">
					Member since {new Date(profile.createdAt).toLocaleDateString()}
				</p>
			</div>

			{#if data.user && data.user.id !== profile.id}
				<form method="POST" action="?/toggleFollow" use:enhance>
					<Button
						variant={data.isFollowing ? 'secondary' : 'default'}
						class="flex items-center gap-2"
						type="submit"
					>
						{#if data.isFollowing}
							<UserMinus class="h-4 w-4" />
							Unfollow
						{:else}
							<UserPlus class="h-4 w-4" />
							Follow
						{/if}
					</Button>
				</form>
			{/if}
		</div>

		<div class="flex gap-8">
			<a
				href="/profile/{profile.username}/followers"
				class="text-center transition-opacity hover:opacity-70"
			>
				<p class="text-2xl font-bold">{data.stats.followerCount}</p>
				<p class="text-xs tracking-wider text-muted-foreground uppercase">Followers</p>
			</a>
			<a
				href="/profile/{profile.username}/following"
				class="text-center transition-opacity hover:opacity-70"
			>
				<p class="text-2xl font-bold">{data.stats.followingCount}</p>
				<p class="text-xs tracking-wider text-muted-foreground uppercase">Following</p>
			</a>
			<a
				href="/profile/{profile.username}/decks"
				class="text-center transition-opacity hover:opacity-70"
			>
				<p class="text-2xl font-bold">{data.stats.deckCount}</p>
				<p class="text-xs tracking-wider text-muted-foreground uppercase">Decks</p>
			</a>
			<a
				href="/profile/{profile.username}/collection"
				class="text-center transition-opacity hover:opacity-70"
			>
				<p class="text-2xl font-bold">{data.stats.cardCount}</p>
				<p class="text-xs tracking-wider text-muted-foreground uppercase">Cards</p>
			</a>
		</div>
	</div>

	<div class="mb-8 flex flex-wrap gap-4">
		<Button
			href="/profile/{profile.username}/collection"
			variant="outline"
			class="min-w-[150px] flex-1"
		>
			View Full Collection
		</Button>
		<Button
			href="/profile/{profile.username}/collection/locations"
			variant="outline"
			class="min-w-[150px] flex-1"
		>
			{#if data.user && data.user.id === profile.id}
				Manage Locations
			{:else}
				View Locations
			{/if}
		</Button>
		<Button href="/profile/{profile.username}/decks" variant="outline" class="min-w-[150px] flex-1">
			View Decks
		</Button>
	</div>

	<section>
		<div class="mb-4 flex items-center justify-between">
			<h2 class="text-2xl font-semibold">Decks</h2>
			<a
				href="/profile/{profile.username}/decks"
				class="text-sm font-medium text-primary hover:underline"
			>
				View All
			</a>
		</div>
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
