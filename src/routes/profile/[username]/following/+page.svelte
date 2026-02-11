<script lang="ts">
	import type { PageData } from './$types';
	import {
		CircleUserRound,
		CalendarDays,
		Library,
		Layers,
		Users,
		UserPlus,
		UserMinus,
		ArrowLeft
	} from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { enhance } from '$app/forms';

	let { data }: { data: PageData } = $props();

	function formatDate(date: Date | null) {
		if (!date) return 'Unknown';
		return new Intl.DateTimeFormat('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		}).format(date);
	}
</script>

<svelte:head>
	<title>Users followed by {data.profileUser.username} - Oshibana</title>
</svelte:head>

<div class="mx-auto max-w-7xl px-4 py-12">
	<div class="mb-8 flex items-center gap-4">
		<Button variant="ghost" size="icon" href="/profile/{data.profileUser.username}">
			<ArrowLeft class="h-5 w-5" />
		</Button>
		<div>
			<h1 class="text-3xl font-bold tracking-tight">Following</h1>
			<p class="mt-2 text-muted-foreground">
				Users followed by <span class="font-medium text-foreground"
					>{data.profileUser.username}</span
				>
			</p>
		</div>
	</div>

	{#if data.following.length === 0}
		<div
			class="flex flex-col items-center justify-center rounded-lg border border-dashed p-12 text-center"
		>
			<Users class="h-12 w-12 text-muted-foreground" />
			<h3 class="mt-4 text-lg font-semibold">Not following anyone</h3>
			<p class="mt-2 text-sm text-muted-foreground">
				{data.user?.id === data.profileUser.id
					? "You aren't following anyone yet."
					: `${data.profileUser.username} isn't following anyone yet.`}
			</p>
		</div>
	{:else}
		<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
			{#each data.following as user}
				<Card.Root class="flex flex-col">
					<Card.Header class="flex flex-row items-center gap-4 space-y-0">
						<div class="flex h-12 w-12 items-center justify-center rounded-full bg-accent">
							<CircleUserRound class="h-8 w-8 text-accent-foreground" />
						</div>
						<div class="flex flex-col overflow-hidden">
							<Card.Title class="truncate text-lg">
								<a href="/profile/{user.username}" class="hover:underline">
									{user.username}
								</a>
							</Card.Title>
							<Card.Description class="flex items-center gap-1 text-xs">
								<CalendarDays class="h-3 w-3" />
								Joined {formatDate(user.createdAt)}
							</Card.Description>
						</div>
					</Card.Header>
					<Card.Content class="flex-grow space-y-4">
						<div class="flex flex-wrap gap-x-4 gap-y-2 text-sm text-muted-foreground">
							<div class="flex items-center gap-1">
								<Layers class="h-4 w-4" />
								<span>{user.deckCount} {user.deckCount === 1 ? 'deck' : 'decks'}</span>
							</div>
							<div class="flex items-center gap-1">
								<Library class="h-4 w-4" />
								<span>{user.cardCount} {user.cardCount === 1 ? 'card' : 'cards'}</span>
							</div>
							<div class="flex items-center gap-1">
								<Users class="h-4 w-4" />
								<span
									>{user.followerCount} {user.followerCount === 1 ? 'follower' : 'followers'}</span
								>
							</div>
						</div>
					</Card.Content>
					<Card.Footer class="gap-2">
						<Button variant="outline" class="flex-grow" href="/profile/{user.username}">
							View Profile
						</Button>

						{#if data.user && data.user.id !== user.id}
							<form method="POST" action="/users?/toggleFollow" use:enhance>
								<input type="hidden" name="userId" value={user.id} />
								<Button
									variant={user.isFollowing ? 'secondary' : 'default'}
									size="icon"
									type="submit"
									title={user.isFollowing ? 'Unfollow' : 'Follow'}
								>
									{#if user.isFollowing}
										<UserMinus class="h-4 w-4" />
									{:else}
										<UserPlus class="h-4 w-4" />
									{/if}
								</Button>
							</form>
						{/if}
					</Card.Footer>
				</Card.Root>
			{/each}
		</div>
	{/if}
</div>
