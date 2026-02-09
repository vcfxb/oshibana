<script lang="ts">
	let { data } = $props();
</script>

<div class="flex min-h-screen flex-col items-center justify-center space-y-4">
	{#if data.user}
		<h1 class="text-2xl font-bold">Welcome back, {data.user.username}!</h1>
		<p class="text-gray-600">You are logged in.</p>
		<form method="POST" action="?/logout">
			<button
				type="submit"
				class="rounded-md bg-red-600 px-4 py-2 text-sm font-semibold text-white hover:bg-red-500"
			>
				Log out
			</button>
		</form>
	{:else}
		<h1 class="text-2xl font-bold">Welcome to Oshibana</h1>
		<p class="text-gray-600">The MTG deck and collection tracker.</p>
		<div class="flex space-x-4">
			<a
				href="/login"
				class="rounded-md bg-indigo-600 px-4 py-2 text-sm font-semibold text-white hover:bg-indigo-500"
			>
				Log in
			</a>
			<a
				href="/register"
				class="rounded-md border border-indigo-600 px-4 py-2 text-sm font-semibold text-indigo-600 hover:bg-indigo-50"
			>
				Sign up
			</a>
		</div>
	{/if}
</div>
