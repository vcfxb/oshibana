<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Sheet from '$lib/components/ui/sheet';
	import { Menu } from 'lucide-svelte';
	import { ModeWatcher } from 'mode-watcher';
	import './layout.css';

	let { children, data } = $props();
	let mobileMenuOpen = $state(false);
	const wordmarkSrc = '/iconography/wordmark-white.svg'; // todo: save themes in db, handle with db

	// $effect(() => {
	// 	wordmarkSrc = mode.current === "dark" ? "/iconography/wordmark-white.svg" : "/iconography/wordmark-black.svg";
	// });
</script>

<svelte:head>
	<link rel="icon" href="iconography/favicon-dark.svg" media="(prefers-color-scheme: light)" />
	<link rel="icon" href="iconography/favicon-light.svg" media="(prefers-color-scheme: dark)" />
</svelte:head>

<ModeWatcher track={false} defaultMode={'dark'} />

<nav class="border-b bg-background px-4 py-3">
	<div class="mx-auto flex max-w-7xl items-center justify-between">
		<div class="flex items-center gap-8">
			<a href="/" class="rounded-md px-3 py-4 hover:bg-accent">
				<img src={wordmarkSrc} class="h-7" alt="Oshibana Wordmark" />
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
					<span class="mr-2 text-sm font-medium text-muted-foreground">
						{data.user.username}
					</span>
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
							<a href="/cards" class="text-lg font-medium" onclick={() => (mobileMenuOpen = false)}>
								Cards
							</a>
							<a href="/decks" class="text-lg font-medium" onclick={() => (mobileMenuOpen = false)}>
								Decks
							</a>
							<a href="/users" class="text-lg font-medium" onclick={() => (mobileMenuOpen = false)}>
								Users
							</a>

							<div class="my-2 border-t"></div>

							<div class="flex flex-col gap-4">
								{#if data.user}
									<div class="py-2 text-center font-medium">
										Logged in as {data.user.username}
									</div>
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

{@render children()}
