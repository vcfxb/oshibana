<script lang="ts">
	let { symbol }: { symbol: string } = $props();

	// Map common symbols to their Scryfall SVG filenames
	// The format is usually {U} -> U.svg, {10} -> 10.svg, {W/P} -> WP.svg
	const symbolPath = $derived.by(() => {
		const s = symbol.replace(/\{|\}/g, '').replace(/\//g, '');
		return `https://svgs.scryfall.io/card-symbols/${s}.svg`;
	});
</script>

<img src={symbolPath} alt={symbol} class="inline-block h-[1.1em] w-[1.1em] align-text-bottom" />
