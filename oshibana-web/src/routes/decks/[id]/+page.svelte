<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import * as Card from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { PanelsTopLeft, Calendar, User, History, Plus, Minus, Trash2 } from 'lucide-svelte';
	import ManaCost from '$lib/components/ManaCost.svelte';
	import MTGCard from '$lib/components/MTGCard.svelte';

	let { data } = $props();
	let deck = $derived(data.deck);
	let slots = $derived(data.slots);
	let history = $derived(data.history);
	let isOwner = $derived(data.isOwner);

	let activeTab = $state('cards');

	// Group slots by board
	const mainBoard = $derived(slots.filter((s) => s.slot.board === 'main'));
	const sideboard = $derived(slots.filter((s) => s.slot.board === 'side'));
	const maybeBoard = $derived(slots.filter((s) => s.slot.board === 'maybe'));
	const commander = $derived(slots.filter((s) => s.slot.board === 'commander'));

	function formatChangeType(type: string) {
		switch (type) {
			case 'add':
				return 'Added';
			case 'remove':
				return 'Removed';
			case 'update_quantity':
				return 'Changed quantity of';
			default:
				return type;
		}
	}

	function formatDate(date: Date) {
		return new Intl.DateTimeFormat('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		}).format(date);
	}
</script>

<div class="mx-auto max-w-7xl px-4 py-8">
	<div class="mb-8 flex flex-col justify-between gap-6 md:flex-row md:items-end">
		<div>
			<div class="flex items-center gap-2 text-sm text-muted-foreground">
				<a href="/decks" class="hover:underline">Decks</a>
				<span>/</span>
				<span class="text-foreground">{deck.name}</span>
			</div>
			<h1 class="mt-2 text-4xl font-bold">{deck.name}</h1>
			<p class="mt-2 max-w-2xl text-muted-foreground">
				{deck.description || 'No description provided.'}
			</p>

			<div class="mt-4 flex flex-wrap gap-4 text-sm text-muted-foreground">
				<div class="flex items-center gap-1">
					<User class="h-4 w-4" />
					<a href="/profile/{data.author.username}" class="text-foreground hover:underline">
						{data.author.username}
					</a>
				</div>
				<div class="flex items-center gap-1">
					<Calendar class="h-4 w-4" />
					Created {formatDate(deck.createdAt)}
				</div>
			</div>
		</div>

		{#if isOwner}
			<div class="flex gap-2">
				<Button variant="outline">Edit Deck Details</Button>
				<form
					method="POST"
					action="?/deleteDeck"
					onsubmit={(e) => {
						if (!confirm('Are you sure you want to delete this deck?')) e.preventDefault();
					}}
				>
					<input type="hidden" name="deckId" value={deck.id} />
					<Button variant="destructive" type="submit">
						<Trash2 class="mr-2 h-4 w-4" />
						Delete
					</Button>
				</form>
			</div>
		{/if}
	</div>

	<div class="mb-8 flex overflow-x-auto border-b">
		<button
			class="border-b-2 px-4 py-2 font-medium transition-colors {activeTab === 'cards'
				? 'border-primary text-primary'
				: 'border-transparent text-muted-foreground hover:text-foreground'}"
			onclick={() => (activeTab = 'cards')}
		>
			Decklist
		</button>
		<button
			class="border-b-2 px-4 py-2 font-medium transition-colors {activeTab === 'history'
				? 'border-primary text-primary'
				: 'border-transparent text-muted-foreground hover:text-foreground'}"
			onclick={() => (activeTab = 'history')}
		>
			History
		</button>
	</div>

	{#if activeTab === 'cards'}
		<div class="grid gap-8 lg:grid-cols-3">
			<div class="space-y-8 lg:col-span-2">
				{#if commander.length > 0}
					<section>
						<h2 class="mb-4 flex items-center gap-2 text-lg font-bold">
							Commander ({commander.reduce((acc, s) => acc + s.slot.quantity, 0)})
						</h2>
						<div class="grid grid-cols-2 gap-4 sm:grid-cols-3">
							{#each commander as item}
								<div class="space-y-2">
									<MTGCard card={item.card} />
									<div class="flex items-center justify-between px-1 text-sm">
										<span class="font-bold">{item.slot.quantity}x</span>
										{#if isOwner}
											<div class="flex gap-1">
												<form method="POST" action="?/updateSlot">
													<input type="hidden" name="slotId" value={item.slot.id} />
													<input type="hidden" name="quantity" value={item.slot.quantity - 1} />
													<Button variant="ghost" size="icon" class="h-6 w-6" type="submit">
														<Minus class="h-3 w-3" />
													</Button>
												</form>
												<form method="POST" action="?/updateSlot">
													<input type="hidden" name="slotId" value={item.slot.id} />
													<input type="hidden" name="quantity" value={item.slot.quantity + 1} />
													<Button variant="ghost" size="icon" class="h-6 w-6" type="submit">
														<Plus class="h-3 w-3" />
													</Button>
												</form>
											</div>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					</section>
				{/if}

				<section>
					<h2 class="mb-4 flex items-center justify-between text-lg font-bold">
						<span>Mainboard ({mainBoard.reduce((acc, s) => acc + s.slot.quantity, 0)})</span>
					</h2>
					<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4">
						{#each mainBoard as item}
							<div class="space-y-2">
								<MTGCard card={item.card} />
								<div class="flex items-center justify-between px-1 text-sm">
									<span class="font-bold">{item.slot.quantity}x</span>
									{#if isOwner}
										<div class="flex gap-1">
											<form method="POST" action="?/updateSlot">
												<input type="hidden" name="slotId" value={item.slot.id} />
												<input type="hidden" name="quantity" value={item.slot.quantity - 1} />
												<Button variant="ghost" size="icon" class="h-6 w-6" type="submit">
													<Minus class="h-3 w-3" />
												</Button>
											</form>
											<form method="POST" action="?/updateSlot">
												<input type="hidden" name="slotId" value={item.slot.id} />
												<input type="hidden" name="quantity" value={item.slot.quantity + 1} />
												<Button variant="ghost" size="icon" class="h-6 w-6" type="submit">
													<Plus class="h-3 w-3" />
												</Button>
											</form>
										</div>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				</section>

				{#if sideboard.length > 0}
					<section>
						<h2 class="mb-4 text-lg font-bold">
							Sideboard ({sideboard.reduce((acc, s) => acc + s.slot.quantity, 0)})
						</h2>
						<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4">
							{#each sideboard as item}
								<div class="space-y-2">
									<MTGCard card={item.card} />
									<div class="flex items-center justify-between px-1 text-sm">
										<span class="font-bold">{item.slot.quantity}x</span>
										{#if isOwner}
											<div class="flex gap-1">
												<form method="POST" action="?/updateSlot">
													<input type="hidden" name="slotId" value={item.slot.id} />
													<input type="hidden" name="quantity" value={item.slot.quantity - 1} />
													<Button variant="ghost" size="icon" class="h-6 w-6" type="submit">
														<Minus class="h-3 w-3" />
													</Button>
												</form>
												<form method="POST" action="?/updateSlot">
													<input type="hidden" name="slotId" value={item.slot.id} />
													<input type="hidden" name="quantity" value={item.slot.quantity + 1} />
													<Button variant="ghost" size="icon" class="h-6 w-6" type="submit">
														<Plus class="h-3 w-3" />
													</Button>
												</form>
											</div>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					</section>
				{/if}
			</div>

			<div class="space-y-8">
				<Card.Root>
					<Card.Header>
						<Card.Title>Stats</Card.Title>
					</Card.Header>
					<Card.Content class="space-y-4">
						<div class="flex items-center justify-between">
							<span class="text-muted-foreground">Total Cards</span>
							<span class="font-bold">{slots.reduce((acc, s) => acc + s.slot.quantity, 0)}</span>
						</div>
						<div class="flex items-center justify-between">
							<span class="text-muted-foreground">Unique Cards</span>
							<span class="font-bold">{slots.length}</span>
						</div>
					</Card.Content>
				</Card.Root>

				{#if isOwner}
					<Card.Root>
						<Card.Header>
							<Card.Title>Quick Add</Card.Title>
							<Card.Description>Add a card by Scryfall ID (temporary UI)</Card.Description>
						</Card.Header>
						<Card.Content>
							<form method="POST" action="?/addCard" class="space-y-4">
								<input type="hidden" name="deckId" value={deck.id} />
								<div class="space-y-2">
									<Label for="scryfallId">Scryfall ID</Label>
									<Input
										id="scryfallId"
										name="scryfallId"
										placeholder="e.g., f85ab5f9-508e-45de-8fa1-ce1f16552ffc"
										required
									/>
								</div>
								<div class="grid grid-cols-2 gap-4">
									<div class="space-y-2">
										<Label for="quantity">Quantity</Label>
										<Input id="quantity" name="quantity" type="number" value="1" min="1" required />
									</div>
									<div class="space-y-2">
										<Label for="board">Board</Label>
										<select
											name="board"
											id="board"
											class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
										>
											<option value="main">Main</option>
											<option value="side">Side</option>
											<option value="maybe">Maybe</option>
											<option value="commander">Commander</option>
										</select>
									</div>
								</div>
								<Button type="submit" class="w-full">Add Card</Button>
							</form>
						</Card.Content>
					</Card.Root>
				{/if}
			</div>
		</div>
	{:else if activeTab === 'history'}
		<Card.Root>
			<Card.Header>
				<Card.Title>Changelog</Card.Title>
				<Card.Description>Every update made to this deck.</Card.Description>
			</Card.Header>
			<Card.Content>
				{#if history.length > 0}
					<div class="space-y-6">
						{#each history.toReversed() as item}
							<div class="flex gap-4">
								<div class="mt-1">
									<div class="flex h-8 w-8 items-center justify-center rounded-full bg-muted">
										<History class="h-4 w-4" />
									</div>
								</div>
								<div class="flex-grow">
									<div class="flex items-center justify-between">
										<p class="text-sm font-medium">
											{formatChangeType(item.change.changeType)}
											<span class="font-bold">{Math.abs(item.change.quantityChange)}x</span>
											<span class="text-primary underline">
												{item.card?.name || 'Unknown Card'}
											</span>
											to {item.change.board}
										</p>
										<span class="text-xs text-muted-foreground"
											>{formatDate(item.change.createdAt)}</span
										>
									</div>
									<div class="mt-1 flex items-center gap-2">
										{#if item.card}
											<Badge variant="secondary" class="text-[10px] uppercase">
												{item.card.set}
											</Badge>
										{/if}
									</div>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div class="py-12 text-center text-muted-foreground">
						No history yet. Changes will appear here as you edit the deck.
					</div>
				{/if}
			</Card.Content>
		</Card.Root>
	{/if}
</div>
