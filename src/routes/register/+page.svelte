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
					Create your account
				</Card.Title>
			</Card.Header>
			<Card.Content>
				{#if form?.success}
					<div class="space-y-4 text-center">
						<div class="rounded-md bg-green-500/15 p-4 text-sm text-green-600">
							{form.message}
						</div>
						<Button href="/login" variant="outline" class="w-full">Go to login</Button>
					</div>
				{:else}
					<form class="space-y-6" method="POST" use:enhance>
						{#if form?.message}
							<div class="rounded-md bg-destructive/15 p-4 text-sm text-destructive">
								{form.message}
							</div>
						{/if}
						<div class="space-y-4">
							<div class="space-y-2">
								<label for="username" class="text-sm leading-none font-medium">Username</label>
								<Input id="username" name="username" type="text" required placeholder="Username" />
							</div>
							<div class="space-y-2">
								<label for="email" class="text-sm leading-none font-medium">Email address</label>
								<Input
									id="email"
									name="email"
									type="email"
									autocomplete="email"
									required
									placeholder="Email address"
								/>
							</div>
							<div class="space-y-2">
								<label for="password" class="text-sm leading-none font-medium">Password</label>
								<Input
									id="password"
									name="password"
									type="password"
									required
									placeholder="Password"
								/>
							</div>
						</div>

						<Button type="submit" class="w-full">Sign up</Button>
					</form>
				{/if}
			</Card.Content>
			<Card.Footer>
				<div class="w-full text-center text-sm">
					<a href="/login" class="font-medium text-primary hover:text-primary/80">
						Already have an account? Log in
					</a>
				</div>
			</Card.Footer>
		</Card.Root>
	</div>
</div>
