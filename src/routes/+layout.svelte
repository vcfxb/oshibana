<script lang="ts">
	import * as NavigationMenu from "$lib/components/ui/navigation-menu/index.js";
	import { IsMobile } from "$lib/hooks/is-mobile.svelte.js";
	import { ModeWatcher } from "mode-watcher";
	import './layout.css';

  	const isMobile = new IsMobile();
	let { children } = $props();
	let wordmarkSrc = "/iconography/wordmark-white.svg"; // todo: save themes in db, handle with db

	// $effect(() => {
	// 	wordmarkSrc = mode.current === "dark" ? "/iconography/wordmark-white.svg" : "/iconography/wordmark-black.svg";
	// });
</script>

<svelte:head>
	<link rel="icon" href="iconography/favicon-dark.svg" media="(prefers-color-scheme: light)"/>
	<link rel="icon" href="iconography/favicon-light.svg" media="(prefers-color-scheme: dark)"/>
</svelte:head>

<ModeWatcher track={false} defaultMode={"dark"} />

<NavigationMenu.Root viewport={isMobile.current} class="relative z-10 py-3 px-4 flex w-full justify-center">
	<NavigationMenu.List class="group flex list-none items-center justify-center p-1">
		<NavigationMenu.Item class="mx-5 mr-10">
			<NavigationMenu.Link href="/">
				<img src={wordmarkSrc} class="h-8" alt="Oshibana Wordmark"/>
			</NavigationMenu.Link>
		</NavigationMenu.Item>
	</NavigationMenu.List>
	<NavigationMenu.List class="p-1 group flex list-none items-center justify-center rounded-md flex-wrap px-2 bg-blue-200 dark:bg-blue-950">
		<NavigationMenu.Item>
			<NavigationMenu.Link class="text-lg m-1 hover:bg-blue-300 dark:hover:bg-blue-800" href="/decks">
				Decks
			</NavigationMenu.Link>
		</NavigationMenu.Item>
		<NavigationMenu.Item>
			<NavigationMenu.Link class="text-lg m-1 hover:bg-blue-300 dark:hover:bg-blue-800" href="/users">
				Users
			</NavigationMenu.Link>
		</NavigationMenu.Item>
	</NavigationMenu.List>
</NavigationMenu.Root>

{@render children()}
