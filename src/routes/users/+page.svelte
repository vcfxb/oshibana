<script lang="ts">
	import type { PageData } from './$types';
	import {
		CircleUserRound,
		CalendarDays,
		Library,
		Layers,
		Users,
		UserPlus,
		UserMinus
	} from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import * as Pagination from '$lib/components/ui/pagination';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';

	let { data }: { data: PageData } = $props();

	function formatDate(date: Date | null) {
		if (!date) return 'Unknown';
		return new Intl.DateTimeFormat('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		}).format(date);
	}

	const sortOptions = [
		{ value: 'newest', label: 'Newest Accounts' },
		{ value: 'followers', label: 'Most Followers' }
	];

	const selectedSortLabel = $derived(
		sortOptions.find((o) => o.value === data.sort)?.label ?? 'Sort by'
	);

	function handleSortChange(value: string | undefined) {
		if (!value) return;
		const url = new URL(window.location.href);
		url.searchParams.set('sort', value);
		url.searchParams.set('page', '1');
		goto(url.toString());
	}

	function handlePageChange(page: number) {
		const url = new URL(window.location.href);
		url.searchParams.set('page', page.toString());
		goto(url.toString());
	}
</script>

<svelte:head>
	<title>Users - Oshibana</title>
</svelte:head>

<div class="mx-auto max-w-7xl px-4 py-12">
	<div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight">Community</h1>
			<p class="mt-2 text-muted-foreground">Browse members of the Oshibana community.</p>
		</div>

		<div class="flex items-center gap-2">
			<span class="text-sm font-medium text-muted-foreground">Sort by:</span>
			<Select.Root type="single" value={data.sort} onValueChange={handleSortChange}>
				<Select.Trigger class="w-[180px]">
					{selectedSortLabel}
				</Select.Trigger>
				<Select.Content>
					{#each sortOptions as option}
						<Select.Item value={option.value}>{option.label}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
	</div>

	{#if data.users.length === 0}
		<div
			class="flex flex-col items-center justify-center rounded-lg border border-dashed p-12 text-center"
		>
			<CircleUserRound class="h-12 w-12 text-muted-foreground" />
			<h3 class="mt-4 text-lg font-semibold">No users found</h3>
			<p class="mt-2 text-sm text-muted-foreground">
				It seems the community is just getting started!
			</p>
		</div>
	{:else}
		<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
			{#each data.users as user}
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
									>{user.followerCount}
									{user.followerCount === 1 ? 'follower' : 'followers'}</span
								>
							</div>
						</div>
					</Card.Content>
					<Card.Footer class="gap-2">
						<Button variant="outline" class="flex-grow" href="/profile/{user.username}">
							View Profile
						</Button>

						{#if data.user && data.user.id !== user.id}
							<form method="POST" action="?/toggleFollow" use:enhance>
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

		{#if data.pagination.totalPages > 1}
			<div class="mt-12 flex justify-center">
				<Pagination.Root
					count={data.pagination.total}
					perPage={data.pagination.limit}
					siblingCount={1}
					page={data.pagination.page}
					onPageChange={handlePageChange}
				>
					{#snippet children({ pages, currentPage })}
						<Pagination.Content>
							<Pagination.Item>
								<Pagination.PrevButton />
							</Pagination.Item>
							{#each pages as page (page.key)}
								{#if page.type === 'ellipsis'}
									<Pagination.Item>
										<Pagination.Ellipsis />
									</Pagination.Item>
								{:else}
									<Pagination.Item>
										<Pagination.Link {page} isActive={currentPage === page.value}>
											{page.value}
										</Pagination.Link>
									</Pagination.Item>
								{/if}
							{/each}
							<Pagination.Item>
								<Pagination.NextButton />
							</Pagination.Item>
						</Pagination.Content>
					{/snippet}
				</Pagination.Root>
			</div>
		{/if}
	{/if}
</div>
