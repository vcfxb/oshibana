<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Sheet from '$lib/components/ui/sheet';
	import { Menu, CircleUserRound } from 'lucide-svelte';
	import { ModeWatcher } from 'mode-watcher';
	import './layout.css';
	import whiteWordmarkRaw from '$lib/assets/wordmark-white.svg?raw';

	let { children, data } = $props();
	let mobileMenuOpen = $state(false);
	const wordmarkSrc = '/iconography/wordmark-white.svg'; // todo: save themes in db, handle with db

	// $effect(() => {
	// 	wordmarkSrc = mode.current === "dark" ? "/iconography/wordmark-white.svg" : "/iconography/wordmark-black.svg";
	// });

	const currentYear = new Date().getFullYear();
</script>

<svelte:head>
	<link rel="icon" href="/iconography/favicon-dark.svg" media="(prefers-color-scheme: light)" />
	<link rel="icon" href="/iconography/favicon-light.svg" media="(prefers-color-scheme: dark)" />
</svelte:head>

<ModeWatcher track={false} defaultMode={'dark'} />

<div class="flex min-h-screen flex-col">
	<nav class="border-b bg-background px-4 py-3">
		<div class="mx-auto flex max-w-7xl items-center justify-between">
			<div class="flex min-w-0 items-center gap-2 md:gap-8">
				<a href="/" class="flex-shrink-0 rounded-md px-2 py-2 hover:bg-accent md:px-3 md:py-4">
					<div class="md:h7 h-6 w-auto scale-110">
						{@html whiteWordmarkRaw}
					</div>
				</a>

				<div class="hidden items-center gap-6 text-lg font-medium text-muted-foreground md:flex">
					<a href="/cards" class="transition-colors hover:text-foreground">Cards</a>
					<a href="/decks" class="transition-colors hover:text-foreground">Decks</a>
					<a href="/users" class="transition-colors hover:text-foreground">Users</a>
				</div>
			</div>

			<div class="flex items-center gap-4">
				<div class="hidden items-center gap-4 sm:flex">
					{#if data.user}
						<a
							href="/profile/{data.user.username}"
							class="mr-2 flex items-center gap-2 text-sm font-medium text-muted-foreground transition-colors hover:text-foreground"
						>
							<CircleUserRound class="h-5 w-5" />
							{data.user.username}
						</a>
						<form method="POST" action="/?/logout">
							<Button variant="outline" type="submit">Log out</Button>
						</form>
					{:else}
						<Button variant="outline" href="/register">Sign up</Button>
						<Button href="/login">Login</Button>
					{/if}
				</div>

				<div class="md:hidden">
					<Sheet.Root bind:open={mobileMenuOpen}>
						<Sheet.Trigger class={buttonVariants({ variant: 'outline', size: 'icon' })}>
							<Menu class="h-5 w-5" />
						</Sheet.Trigger>

						<Sheet.Content side="right">
							<div class="m-8 flex flex-col gap-4">
								<a
									href="/cards"
									class="text-lg font-medium"
									onclick={() => (mobileMenuOpen = false)}
								>
									Cards
								</a>
								<a
									href="/decks"
									class="text-lg font-medium"
									onclick={() => (mobileMenuOpen = false)}
								>
									Decks
								</a>
								<a
									href="/users"
									class="text-lg font-medium"
									onclick={() => (mobileMenuOpen = false)}
								>
									Users
								</a>

								<div class="my-2 border-t"></div>

								<div class="flex flex-col gap-4">
									{#if data.user}
										<a
											href="/profile/{data.user.username}"
											class="flex items-center justify-center gap-2 py-2 text-center font-medium transition-colors hover:text-primary"
											onclick={() => (mobileMenuOpen = false)}
										>
											<CircleUserRound class="h-5 w-5" />
											Logged in as {data.user.username}
										</a>
										<form
											method="POST"
											action="/?/logout"
											class="flex w-full flex-col"
											onsubmit={() => (mobileMenuOpen = false)}
										>
											<Button variant="outline" type="submit">Log out</Button>
										</form>
									{:else}
										<Button
											variant="outline"
											href="/register"
											onclick={() => (mobileMenuOpen = false)}
										>
											Signup
										</Button>
										<Button href="/login" onclick={() => (mobileMenuOpen = false)}>Login</Button>
									{/if}
								</div>
							</div>
						</Sheet.Content>
					</Sheet.Root>
				</div>
			</div>
		</div>
	</nav>

	<main class="flex-grow">
		{@render children()}
	</main>

	<footer class="border-t bg-background px-4 py-12">
		<div class="mx-auto flex max-w-7xl flex-col items-center justify-between gap-8 md:flex-row">
			<div class="flex flex-col items-center gap-6 md:flex-row">
				<div class="aspect-[63/88] h-28 overflow-hidden rounded-[4.8%] bg-black shadow-lg">
					<img
						src="https://cards.scryfall.io/small/front/f/8/f85ab5f9-508e-45de-8fa1-ce1f16552ffc.jpg?1701537448"
						alt="Lotus Petal Magic Card"
						class="h-full w-full rounded-[4.8%] object-cover"
					/>
				</div>
				<div class="max-w-4xl text-center md:text-left">
					<p class="mt-2 text-sm leading-relaxed text-muted-foreground">
						Oshibana is the Japanese art of using pressed flower petals and other botanical
						materials for the purposes of illustration or sculpture.
					</p>
					<div class="mt-4 text-xs/5 text-muted-foreground/60">
						<p class="mb-2">
							Card data provided by <a
								href="https://scryfall.com"
								class="hover:text-foreground hover:underline">Scryfall</a
							>. Scryfall data is &copy; Scryfall, LLC.
						</p>
						<hr class="border-border" />
						<p class="mt-2 mb-2">
							Portions of Oshibana are unofficial Fan Content permitted under the Wizards of the
							Coast Fan Content Policy. The literal and graphical information presented on this site
							about Magic: The Gathering, including card images and mana symbols, is copyright
							Wizards of the Coast, LLC. Oshibana is not produced by or endorsed by Wizards of the
							Coast.
						</p>
						<hr class="border-border" />
						<p class="mt-2">
							Oshibana is not affiliated with Scryfall, LLC or Wizards of the Coast, LLC.
						</p>
					</div>
				</div>
			</div>
			<div class="text-sm font-medium text-muted-foreground/80">
				&copy; Venus Xeon-Blonde 2026{currentYear > 2026 ? ` - ${currentYear}` : ''}
			</div>
		</div>
	</footer>
</div>
