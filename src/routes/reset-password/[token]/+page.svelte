<script lang="ts">
	import { enhance } from '$app/forms';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Card from '$lib/components/ui/card';

	let { data, form } = $props();
</script>

<div
	class="flex min-h-[calc(100vh-65px)] items-center justify-center bg-background px-4 py-12 sm:px-6 lg:px-8"
>
	<div class="w-full max-w-md">
		<Card.Root>
			<Card.Header>
				<Card.Title class="text-center text-3xl font-bold tracking-tight">
					Reset Password
				</Card.Title>
				<Card.Description class="text-center">
					{#if data.valid}
						Enter your new password below.
					{:else}
						This link is invalid or has expired.
					{/if}
				</Card.Description>
			</Card.Header>
			<Card.Content>
				{#if data.valid}
					<form class="space-y-6" method="POST" use:enhance>
						{#if form?.message}
							<div class="rounded-md bg-destructive/15 p-4 text-sm text-destructive">
								{form.message}
							</div>
						{/if}

						<div class="space-y-4">
							<div class="space-y-2">
								<label for="password" class="text-sm leading-none font-medium">
									New Password
								</label>
								<Input
									id="password"
									name="password"
									type="password"
									required
									minlength={8}
									placeholder="New Password"
								/>
							</div>
							<div class="space-y-2">
								<label for="confirm" class="text-sm leading-none font-medium">
									Confirm Password
								</label>
								<Input
									id="confirm"
									name="confirm"
									type="password"
									required
									minlength={8}
									placeholder="Confirm Password"
								/>
							</div>
						</div>
						<Button type="submit" class="w-full">Update Password</Button>
					</form>
				{:else}
					<div class="space-y-4 text-center">
						<p class="text-sm text-muted-foreground">
							Password reset links are only valid for 2 hours. Please request a new link if needed.
						</p>
						<Button href="/forgot-password" variant="outline" class="w-full">
							Request New Link
						</Button>
					</div>
				{/if}
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
