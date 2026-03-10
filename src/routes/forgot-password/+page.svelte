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
					Forgot Password
				</Card.Title>
				<Card.Description class="text-center">
					Enter your email address and we'll send you a link to reset your password.
				</Card.Description>
			</Card.Header>
			<Card.Content>
				<form class="space-y-6" method="POST" use:enhance>
					{#if form?.message}
						<div
							class="rounded-md p-4 text-sm {form.success
								? 'bg-green-500/15 text-green-500'
								: 'bg-destructive/15 text-destructive'}"
						>
							{form.message}
						</div>
					{/if}

					{#if !form?.success}
						<div class="space-y-2">
							<label for="email" class="text-sm leading-none font-medium"> Email Address </label>
							<Input id="email" name="email" type="email" required placeholder="you@example.com" />
						</div>
						<Button type="submit" class="w-full">Send Reset Link</Button>
					{/if}
				</form>
			</Card.Content>
			<Card.Footer>
				<div class="w-full text-center text-sm">
					<a href="/login" class="font-medium text-primary hover:text-primary/80">
						Back to Login
					</a>
				</div>
			</Card.Footer>
		</Card.Root>
	</div>
</div>
