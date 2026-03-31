<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';

	let { data } = $props();
</script>

<div
	class="flex min-h-[calc(100vh-65px)] items-center justify-center bg-background px-4 py-12 sm:px-6 lg:px-8"
>
	<div class="w-full max-w-md text-center">
		<Card.Root>
			<Card.Header>
				<Card.Title class="text-3xl font-bold tracking-tight">Email verification</Card.Title>
			</Card.Header>
			<Card.Content>
				{#if data.success}
					<div class="space-y-4">
						<div class="rounded-md bg-green-500/15 p-4 text-sm text-green-600">
							{data.message}
						</div>
						<p class="text-sm text-muted-foreground">You can now log in to your account.</p>
						<Button href="/login" class="w-full">Go to login</Button>
					</div>
				{:else}
					<div class="space-y-4">
						<div class="rounded-md bg-destructive/15 p-4 text-sm text-destructive">
							{data.message}
						</div>
						<p class="text-sm text-muted-foreground">
							If you're having issues, please try requesting a new verification email from the login
							page or contact support.
						</p>
						<Button href="/login" variant="outline" class="w-full">Return to login</Button>
					</div>
				{/if}
			</Card.Content>
		</Card.Root>
	</div>
</div>
