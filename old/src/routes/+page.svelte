<script lang="ts">
	import MTGCard from '$lib/components/MTGCard.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Search, Dices } from 'lucide-svelte';
	import { getRandomCard } from '$lib/scryfall';
	import { fade } from 'svelte/transition';

	let { data } = $props();
	let featuredCards = $derived(data.featuredCards || []);
	let isShaking = $state(false);
	let isDealing = $state(false);

	async function reroll() {
		if (isShaking || isDealing) return;
		isShaking = true;
		isDealing = true; // Drop the "curtain" immediately

		try {
			// Fetch new cards while curtain is down
			const newCards = await Promise.all([
				getRandomCard(),
				getRandomCard(),
				getRandomCard(),
				getRandomCard(),
				getRandomCard()
			]);

			// Small delay so the fetch feels substantial and the curtain is visible
			await new Promise((resolve) => setTimeout(resolve, 300));

			featuredCards = newCards;
		} catch (e) {
			console.error('Failed to reroll cards:', e);
		} finally {
			isDealing = false; // Lift the "curtain"
			setTimeout(() => {
				isShaking = false;
			}, 200);
		}
	}
</script>

<svelte:head>
	<title>Oshibana</title>
</svelte:head>

<div class="container mx-auto flex min-h-[calc(100-3.5rem)] flex-col items-center px-4 py-16">
	<div class="mb-16 text-center">
		<h1 class="mb-4 text-5xl font-extrabold tracking-tight lg:text-6xl">
			Welcome to <span class="text-primary">Oshibana</span>
		</h1>
		<p class="mx-auto max-w-2xl text-xl text-muted-foreground italic">
			An advanced Magic: The Gathering collection and deck tracker.
		</p>
	</div>

	<div class="mb-20 w-full max-w-2xl">
		<form
			action="/cards"
			method="GET"
			class="flex flex-col gap-3 sm:flex-row sm:items-stretch sm:gap-2"
		>
			<div class="relative flex-1">
				<Search class="absolute top-1/2 left-3 h-5 w-5 -translate-y-1/2 text-muted-foreground" />
				<input
					type="text"
					name="q"
					placeholder="Search for any card..."
					class="w-full rounded-full border bg-background py-4 pr-6 pl-12 text-lg shadow-lg focus:ring-2 focus:ring-primary focus:outline-none"
				/>
			</div>
			<Button
				type="submit"
				class="w-full rounded-full px-10 text-lg shadow-lg sm:h-auto sm:w-auto sm:py-4"
			>
				Search
			</Button>
		</form>
	</div>

	<div class="min-h-[400px] w-full">
		<div class="flex items-center justify-center gap-4">
			<h2 class="text-2xl font-bold">
				<span class="sm:hidden">Featured Card</span>
				<span class="hidden sm:inline">Featured Cards</span>
			</h2>
			<button
				type="button"
				onclick={reroll}
				class="flex h-10 w-10 items-center justify-center rounded-full text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
				title="Randomize cards"
			>
				<div class="flex h-6 w-6 items-center justify-center {isShaking ? 'shake' : ''}">
					<Dices class="h-6 w-6" />
				</div>
			</button>
		</div>

		<div class="text-muted-foreground w-full flex items-center justify-center mb-6 italic">
			<span class="sm:hidden"> A random Magic card </span>
			<span class="hidden sm:inline"> Five random cards from all of Magic </span>
		</div>

		<div class="relative grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5">
			{#each featuredCards as card, i (card.id)}
				<div class={i > 0 ? 'hidden sm:block' : 'mx-auto w-full max-w-sm sm:max-w-none'}>
					<MTGCard {card} />
				</div>
			{/each}

			{#if isDealing || featuredCards.length === 0}
				<div
					class="absolute inset-0 z-10 flex items-center justify-center rounded-xl bg-background/50 backdrop-blur-sm"
					in:fade={{ duration: 200 }}
					out:fade={{ duration: 200 }}
				>
					<div class="flex flex-col items-center gap-2">
						<Dices class="h-8 w-8 animate-bounce text-primary" />
						<span class="text-sm font-medium text-muted-foreground">Refreshing...</span>
					</div>
				</div>
			{/if}
		</div>
	</div>

	<div class="mt-24 grid gap-12 text-center md:grid-cols-3">
		<div>
			<h3 class="mb-2 text-xl font-bold">Collection Tracking</h3>
			<p class="text-muted-foreground">
				Keep track of every physical card you own and their locations, in or out of decks.
			</p>
		</div>
		<div>
			<h3 class="mb-2 text-xl font-bold">Deck Building</h3>
			<p class="text-muted-foreground">
				Build and refine your decks with a powerful search and real-time validation.
			</p>
		</div>
		<div>
			<h3 class="mb-2 text-xl font-bold">Inventory Management</h3>
			<p class="text-muted-foreground">
				Know exactly where your cards are, whether they are in a binder, box, or a deck.
			</p>
		</div>
	</div>
</div>

<style>
	@keyframes shake {
		0% {
			transform: rotate(0deg);
		}
		25% {
			transform: rotate(15deg);
		}
		50% {
			transform: rotate(0deg);
		}
		75% {
			transform: rotate(-15deg);
		}
		100% {
			transform: rotate(0deg);
		}
	}

	.shake {
		animation: shake 0.2s ease-in-out infinite;
	}
</style>
