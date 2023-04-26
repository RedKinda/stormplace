<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	export let x: number;
	export let y: number;
	export let color: number = 256;

	// color is an 8bit number make it an rgb value by bit shifting
	const coeff = 256 / 4;
	$: rgb = `rgb(${(color & 0x02) * coeff}, ${((color >> 2) & 0x03) * coeff}, ${
		((color >> 5) & 0x03) * coeff
	})`;
	// $: rgb = `rgb(${color >> 16}, ${(color >> 8) & 0xff}, ${color & 0xff})`;
	$: console.log('rgb', rgb);
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	class="pixel"
	style="background-color: {rgb}"
	on:mousedown={() => {
		console.log('painting pixel', x, y);
		invoke('paint_pixel', { x: x, y: y, color: Math.floor(Math.random() * 256) });
	}}
/>

<style>
	.pixel {
		position: relative;
		width: 10px;
		height: 10px;
		/*background-color: blue;*/
		display: inline-block;
		margin: 0px;
	}
	.pixel:hover {
		border: 1px pink solid;
	}
</style>
