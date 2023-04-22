<script lang="ts">
	import Pixel from './Pixel.svelte';

	export let x_size: number = 10;
	export let y_size: number = 10;
</script>

<svelte:head>
	<title>Splace</title>
	<meta name="description" content="Storm place!" />
</svelte:head>

<div class="text-column">
	<h1>Stormplace</h1>

	<div
		class="grid-container"
		style="grid-template-columns: repeat({x_size}, 1fr); grid-template-rows: repeat({y_size}, 1fr)"
	>
		{#each Array.from({ length: x_size }, (_, i) => i) as x}
			{#each Array.from({ length: y_size }, (_, i) => i) as y}
				<div class="grid-item" style="grid-column: {x + 1}; grid-row: {y + 1}">
					<Pixel {x} {y} />
				</div>
			{/each}
		{/each}
	</div>
</div>

<style>
	.grid-container {
		display: grid;
		height: 100%;
		grid-template-columns: repeat(auto-fill, minmax(10px, 1fr));
		grid-auto-rows: 10px; /* Set the same value as the minimum width of the columns */
	}
</style>
