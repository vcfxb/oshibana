<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Users, Library, Layers } from 'lucide-svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Oshibana</title>
</svelte:head>

<div class="container mx-auto flex min-h-[calc(100vh-3.5rem)] flex-col items-center justify-center px-4 py-16">
	<div class="mb-16 text-center">
		<h1 class="mb-4 text-5xl font-extrabold tracking-tight lg:text-6xl">
			Welcome to <span class="text-primary">Oshibana</span>
		</h1>
		<p class="mx-auto max-w-2xl text-xl text-muted-foreground italic">
			An advanced Magic: The Gathering collection and deck tracker.
		</p>
	</div>

	<div class="grid w-full max-w-4xl grid-cols-1 gap-8 md:grid-cols-3">
		<Card.Root class="flex flex-col items-center justify-center p-6 text-center">
			<Card.Header class="flex flex-col items-center gap-2 pb-2">
				<div class="rounded-full bg-primary/10 p-3 text-primary">
					<Users class="h-8 w-8" />
				</div>
				<Card.Title class="text-4xl font-bold">{data.stats.users}</Card.Title>
			</Card.Header>
			<Card.Content>
				<p class="text-lg font-medium text-muted-foreground">Users</p>
			</Card.Content>
		</Card.Root>

		<Card.Root class="flex flex-col items-center justify-center p-6 text-center">
			<Card.Header class="flex flex-col items-center gap-2 pb-2">
				<div class="rounded-full bg-primary/10 p-3 text-primary">
					<Library class="h-8 w-8" />
				</div>
				<Card.Title class="text-4xl font-bold">{data.stats.decks}</Card.Title>
			</Card.Header>
			<Card.Content>
				<p class="text-lg font-medium text-muted-foreground">Decks</p>
			</Card.Content>
		</Card.Root>

		<Card.Root class="flex flex-col items-center justify-center p-6 text-center">
			<Card.Header class="flex flex-col items-center gap-2 pb-2">
				<div class="rounded-full bg-primary/10 p-3 text-primary">
					<Layers class="h-8 w-8" />
				</div>
				<Card.Title class="text-4xl font-bold">{data.stats.cards}</Card.Title>
			</Card.Header>
			<Card.Content>
				<p class="text-lg font-medium text-muted-foreground">Cards Collected</p>
			</Card.Content>
		</Card.Root>
	</div>
</div>
