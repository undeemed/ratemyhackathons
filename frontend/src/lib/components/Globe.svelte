<script lang="ts">
	import { onMount } from 'svelte';
	import createGlobe from 'cobe';
	import type { GlobeMarker } from '$lib/types';

	let { markers = [], class: className = '' }: { markers?: GlobeMarker[]; class?: string } = $props();

	let canvasEl: HTMLCanvasElement;
	let globe: ReturnType<typeof createGlobe> | undefined;
	let pointerInteracting: number | null = null;
	let pointerInteractionMovement = 0;
	let currentPhi = 0;

	const cobeMarkers = $derived(
		markers.map((m) => ({
			location: [m.latitude, m.longitude] as [number, number],
			size: 0.04,
		}))
	);

	onMount(() => {
		let width = canvasEl.offsetWidth;

		globe = createGlobe(canvasEl, {
			devicePixelRatio: 2,
			width: width * 2,
			height: width * 2,
			phi: 0.4,
			theta: 0.2,
			dark: 1,
			diffuse: 0.6,
			mapSamples: 24000,
			mapBrightness: 1.8,
			mapBaseBrightness: 0.01,
			baseColor: [0.15, 0.15, 0.15],
			markerColor: [1, 1, 1],
			glowColor: [0.05, 0.05, 0.05],
			markers: cobeMarkers,
			onRender: (state) => {
				if (pointerInteracting === null) {
					currentPhi += 0.002;
				}
				state.phi = currentPhi + pointerInteractionMovement;
				state.width = canvasEl.offsetWidth * 2;
				state.height = canvasEl.offsetWidth * 2;
				state.markers = cobeMarkers;
			},
		});

		const onResize = () => { width = canvasEl.offsetWidth; };
		window.addEventListener('resize', onResize);

		return () => {
			globe?.destroy();
			window.removeEventListener('resize', onResize);
		};
	});

	function onPointerDown(e: PointerEvent) {
		pointerInteracting = e.clientX - pointerInteractionMovement * 200;
		canvasEl.style.cursor = 'grabbing';
	}
	function onPointerUp() { pointerInteracting = null; canvasEl.style.cursor = 'grab'; }
	function onPointerOut() { pointerInteracting = null; canvasEl.style.cursor = 'grab'; }
	function onPointerMove(e: PointerEvent) {
		if (pointerInteracting !== null) {
			pointerInteractionMovement = (e.clientX - pointerInteracting) / 200;
		}
	}
</script>

<div class="relative aspect-square {className}">
	<canvas
		bind:this={canvasEl}
		class="h-full w-full cursor-grab"
		onpointerdown={onPointerDown}
		onpointerup={onPointerUp}
		onpointerout={onPointerOut}
		onpointermove={onPointerMove}
	></canvas>
	<!-- Fade edges into black -->
	<div class="pointer-events-none absolute inset-0 bg-gradient-to-r from-bg via-transparent to-bg"></div>
	<div class="pointer-events-none absolute inset-0 bg-gradient-to-b from-bg/60 via-transparent to-bg"></div>
</div>
