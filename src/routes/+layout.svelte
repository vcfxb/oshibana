<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Sheet from "$lib/components/ui/sheet";
	import { Menu } from "lucide-svelte";
	import { ModeWatcher } from "mode-watcher";
	import './layout.css';

	let { children } = $props();
	const wordmarkSrc = "/iconography/wordmark-white.svg"; // todo: save themes in db, handle with db

	// $effect(() => {
	// 	wordmarkSrc = mode.current === "dark" ? "/iconography/wordmark-white.svg" : "/iconography/wordmark-black.svg";
	// });
</script>

<svelte:head>
	<link rel="icon" href="iconography/favicon-dark.svg" media="(prefers-color-scheme: light)"/>
	<link rel="icon" href="iconography/favicon-light.svg" media="(prefers-color-scheme: dark)"/>
</svelte:head>

<ModeWatcher track={false} defaultMode={"dark"} />

<nav class="border-b px-4 py-3 bg-background">
  <div class="flex items-center justify-between mx-auto max-w-7xl">
    
    <div class="flex items-center gap-8">
      <a href="/" class="hover:bg-accent px-3 py-4 rounded-md">
        <img src={wordmarkSrc} class="h-7" alt="Oshibana Wordmark"/>
      </a>

      <div class="hidden md:flex items-center gap-6 font-medium text-muted-foreground text-lg">
        <a href="/cards" class="hover:text-foreground transition-colors">Cards</a>
        <a href="/decks" class="hover:text-foreground transition-colors">Decks</a>
		<a href="/users" class="hover:text-foreground transition-colors">Users</a>
        
      </div>
    </div>

    <div class="flex items-center gap-4">
      <div class="hidden sm:flex items-center gap-4">
        <Button variant="outline" href="/signup">Sign up</Button>
        <Button href="/login">Login</Button>
      </div>

      <div class="md:hidden">
        <Sheet.Root>
          <Sheet.Trigger>
            <Button variant="outline" size="icon">
              <Menu class="h-5 w-5"/>
            </Button>
          </Sheet.Trigger>
          
          <Sheet.Content side="right">
            <div class="flex flex-col gap-4 m-8">
              <a href="/cards" class="text-lg font-medium">Cards</a>
              <a href="/decks" class="text-lg font-medium">Decks</a>
              <a href="/users" class="text-lg font-medium">Users</a>
              
              <div class="border-t my-2"></div>

              <div class="flex flex-col gap-4">
                <Button variant="outline" href="/signup">Signup</Button>
                <Button href="/login">Login</Button>
              </div>
            </div>
          </Sheet.Content>
        </Sheet.Root>
      </div>
    </div>

  </div>
</nav>

{@render children()}
