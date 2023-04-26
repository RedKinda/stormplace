<script lang="ts">
	import Pixel from './Pixel.svelte';

	export let x_size: number = 100;
	export let y_size: number = 100;

	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let colors = {};

	onMount(async () => {
		// listen on 'pixel-update' events
		await listen('pixel-update', (event) => {
			// update the colors object
			console.log(event);
			let p = event.payload;
			console.log('updating pixel', p);
			colors[p[0] + ',' + p[1]] = p[2];
		});

		invoke('start_stream');
	});
</script>

<svelte:head>
	<title>Splace</title>
	<meta name="description" content="Storm place!" />
</svelte:head>

<div class="wawa">
	<h1>Stormplace</h1>

	<div
		class="grid-container"
		style="grid-template-columns: repeat({x_size}, 10px); grid-template-rows: repeat({y_size}, 10px);"
	>
		{#each Array.from({ length: x_size }, (_, i) => i) as x}
			{#each Array.from({ length: y_size }, (_, i) => i) as y}
				<div class="grid-item" style="grid-column: {x + 1}; grid-row: {y + 1}">
					<Pixel {x} {y} color={colors[x + ',' + y] ?? 256} />
				</div>
			{/each}
		{/each}
	</div>
</div>

<style>
	.grid-container {
		display: grid;
		margin: auto;
		height: auto;
		min-width: 100px;
		width: auto;
		/*grid-template-columns: repeat(auto-fill, minmax(10px, 1fr));*/
		grid-auto-rows: 10px; /* Set the same value as the minimum width of the columns */
		grid-auto-columns: 10px; /* Set the same value as the minimum width of the columns */
	}

	.wawa {
		margin: auto;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}
</style>
