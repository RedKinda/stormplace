<script lang="ts">
	import { spring } from 'svelte/motion';

	export let x: number;
	export let y: number;
	export let color: number = 64;
</script>

<div class="pixel">
	<div class="pixel-button">
		<button on:click={() => (color += 1)}>
			<svg aria-hidden="true" viewBox="0 0 1 1" />
		</button>
	</div>
</div>

<style>
	.pixel {
		position: relative;
		width: 10px;
		height: 10px;
		display: inline-block;
		margin: 0px;
	}

	.pixel-button {
		position: absolute;
		width: 100%;
		height: 100%;
		background-color: white;
		border: none;
		box-sizing: border-box;
	}

	button {
		padding: 2px 10px;
		background-color: #4caf50;
		border: none;
		border-radius: 4px;
		color: white;
		cursor: pointer;
		font-size: 14px;
		text-align: center;
		text-decoration: none;
	}

	button:hover {
		background-color: #3e8e41;
	}

	button:active {
		background-color: #0e8e41;
	}

	button:focus {
		outline: none;
	}
</style>
