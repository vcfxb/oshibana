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
	import { ArrowLeft, Layout } from 'lucide-svelte';
	import { enhance } from '$app/forms';

	let loading = $state(false);
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

	<Card>
		<CardHeader>
			<div class="flex items-center gap-2">
				<div class="rounded-full bg-primary/10 p-2 text-primary">
					<Layout class="h-5 w-5" />
				</div>
				<CardTitle class="text-2xl">Create New Deck</CardTitle>
			</div>
			<CardDescription>
				Enter the initial details for your new deck. You can start adding cards immediately after
				creation.
			</CardDescription>
		</CardHeader>
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
			<CardContent class="space-y-4">
				<div class="space-y-2">
					<Label for="name">Name</Label>
					<Input id="name" name="name" placeholder="e.g., Mono-Green Stompy" required autofocus />
				</div>
				<div class="space-y-2">
					<Label for="description">Description</Label>
					<Input
						id="description"
						name="description"
						placeholder="A brief overview of the deck's strategy..."
					/>
				</div>
			</CardContent>
			<CardFooter class="flex flex-col gap-4 border-t pt-6">
				<Button type="submit" class="w-full" disabled={loading}>
					{loading ? 'Creating...' : 'Create Deck'}
				</Button>
				<p class="text-center text-xs text-muted-foreground">
					Deck changes are automatically tracked in a historical changelog.
				</p>
			</CardFooter>
		</form>
	</Card>
</div>
