<script lang="ts">
	import { enhance } from '$app/forms';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Card from '$lib/components/ui/card';

	let { form } = $props();
</script>

<div
	class="flex min-h-[calc(100vh-65px)] items-center justify-center bg-background px-4 py-12 sm:px-6 lg:px-8"
>
	<div class="w-full max-w-md">
		<Card.Root>
			<Card.Header>
				<Card.Title class="text-center text-3xl font-bold tracking-tight">
					Log in to Oshibana
				</Card.Title>
			</Card.Header>
			<Card.Content>
				<form class="space-y-6" method="POST" use:enhance>
					{#if form?.message}
						<div class="rounded-md bg-destructive/15 p-4 text-sm text-destructive">
							{form.message}
						</div>
					{/if}
					<div class="space-y-4">
						<div class="space-y-2">
							<label for="identifier" class="text-sm leading-none font-medium">
								Email or Username
							</label>
							<Input
								id="identifier"
								name="identifier"
								type="text"
								required
								placeholder="Email or Username"
							/>
						</div>
						<div class="space-y-2">
							<label for="password" class="text-sm leading-none font-medium">Password</label>
							<Input
								id="password"
								name="password"
								type="password"
								autocomplete="current-password"
								required
								placeholder="Password"
							/>
						</div>
					</div>

					<Button type="submit" class="w-full">Log in</Button>
				</form>
			</Card.Content>
			<Card.Footer>
				<div class="w-full text-center text-sm">
					<a href="/register" class="font-medium text-primary hover:text-primary/80">
						Don't have an account? Sign up
					</a>
				</div>
			</Card.Footer>
		</Card.Root>
	</div>
</div>
