<script lang="ts">
	import './layout.css';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href="iconography/favicon-dark.svg" media="(prefers-color-scheme: light)"/>
	<link rel="icon" href="iconography/favicon-light.svg" media="(prefers-color-scheme: dark)"/>
</svelte:head>



{@render children()}
