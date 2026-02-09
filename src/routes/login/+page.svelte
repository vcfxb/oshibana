<script lang="ts">
	import { enhance } from '$app/forms';

	let { form } = $props();
</script>

<div
	class="flex min-h-[calc(100vh-65px)] items-center justify-center bg-background px-4 py-12 sm:px-6 lg:px-8"
>
	<div class="w-full max-w-md space-y-8">
		<div>
			<h2 class="mt-6 text-center text-3xl font-bold tracking-tight text-foreground">
				Log in to your account
			</h2>
		</div>
		<form class="mt-8 space-y-6" method="POST" use:enhance>
			{#if form?.message}
				<div class="rounded-md bg-destructive/15 p-4 text-sm text-destructive">
					{form.message}
				</div>
			{/if}
			<div class="-space-y-px rounded-md shadow-sm">
				<div>
					<label for="identifier" class="sr-only">Email or Username</label>
					<input
						id="identifier"
						name="identifier"
						type="text"
						required
						class="relative block w-full rounded-t-md border-input bg-background px-3 py-1.5 text-foreground ring-1 ring-border ring-inset placeholder:text-muted-foreground focus:z-10 focus:ring-2 focus:ring-primary focus:ring-inset sm:text-sm sm:leading-6"
						placeholder="Email or Username"
					/>
				</div>
				<div>
					<label for="password" class="sr-only">Password</label>
					<input
						id="password"
						name="password"
						type="password"
						autocomplete="current-password"
						required
						class="relative block w-full rounded-b-md border-input bg-background px-3 py-1.5 text-foreground ring-1 ring-border ring-inset placeholder:text-muted-foreground focus:z-10 focus:ring-2 focus:ring-primary focus:ring-inset sm:text-sm sm:leading-6"
						placeholder="Password"
					/>
				</div>
			</div>

			<div>
				<button
					type="submit"
					class="group relative flex w-full justify-center rounded-md bg-primary px-3 py-2 text-sm font-semibold text-primary-foreground hover:bg-primary/90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary"
				>
					Log in
				</button>
			</div>
		</form>
		<div class="text-center text-sm">
			<a href="/register" class="font-medium text-primary hover:text-primary/80">
				Don't have an account? Sign up
			</a>
		</div>
	</div>
</div>
