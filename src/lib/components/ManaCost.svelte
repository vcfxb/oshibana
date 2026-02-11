<script lang="ts">
	import ManaSymbol from './ManaSymbol.svelte';

	let { cost }: { cost: string } = $props();

	// Parse "{3}{U}{U}" into ["{3}", "{U}", "{U}"]
	const symbols = $derived(cost.match(/\{[^}]+\}/g) || []);
</script>

<div class="inline-flex items-center gap-0.5">
	{#each symbols as symbol}
		<ManaSymbol {symbol} />
	{/each}
</div>
