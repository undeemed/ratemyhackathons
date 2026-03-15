<script lang="ts">
	import { onMount } from 'svelte';
	import createGlobe from 'cobe';
	import gsap from 'gsap';
	import type { GlobeMarker } from '$lib/types';

	let {
		markers = [],
		class: className = '',
		focus = null as { lat: number; lng: number } | null,
	}: {
		markers?: GlobeMarker[];
		class?: string;
		focus?: { lat: number; lng: number } | null;
	} = $props();

	let canvasEl: HTMLCanvasElement;
	let wrapperEl: HTMLDivElement;
	let globe: ReturnType<typeof createGlobe> | undefined;
	let pointerInteracting: number | null = null;
	let pointerInteractionMovement = 0;
	let currentPhi = 0;
	let currentTheta = 0.2;

	// Hover tooltip state
	let hoveredEvent = $state<GlobeMarker | null>(null);
	let tooltipX = $state(0);
	let tooltipY = $state(0);

	const today = Date.now();

	function markerBrightness(dateStr: string | null): number {
		if (!dateStr) return 0.3;
		const daysAway = (new Date(dateStr).getTime() - today) / 86_400_000;
		if (daysAway < 0) return Math.max(0.15, 0.5 + daysAway / 60); // past events fade
		return Math.max(0.2, 1 - daysAway / 180); // future events: closer = brighter
	}

	const cobeMarkers = $derived(
		markers.map((m) => {
			const b = markerBrightness(m.start_date);
			return {
				location: [m.latitude, m.longitude] as [number, number],
				size: 0.1,
				color: [b, b, b] as [number, number, number],
			};
		})
	);

	const doublePi = Math.PI * 2;

	// Convert lat/lng to cobe's phi/theta
	function locationToAngles(lat: number, lng: number): [number, number] {
		return [
			Math.PI - ((lng * Math.PI) / 180 - Math.PI / 2),
			(lat * Math.PI) / 180,
		];
	}

	// Project a lat/lng marker to screen coordinates given current globe rotation
	function markerToScreen(
		lat: number,
		lng: number,
		phi: number,
		theta: number,
		cx: number,
		cy: number,
		radius: number
	): { x: number; y: number; visible: boolean } {
		const latRad = (lat * Math.PI) / 180;
		const lngRad = (lng * Math.PI) / 180;

		const sx = Math.cos(latRad) * Math.sin(lngRad);
		const sy = -Math.sin(latRad);
		const sz = Math.cos(latRad) * Math.cos(lngRad);

		const cosPhi = Math.cos(phi);
		const sinPhi = Math.sin(phi);
		const rx = sx * cosPhi + sz * sinPhi;
		const ry2 = sy;
		const rz = -sx * sinPhi + sz * cosPhi;

		const cosTheta = Math.cos(theta);
		const sinTheta = Math.sin(theta);
		const fy = ry2 * cosTheta - rz * sinTheta;
		const fz = ry2 * sinTheta + rz * cosTheta;

		if (fz < 0.05) return { x: 0, y: 0, visible: false };

		return { x: cx + rx * radius, y: cy + fy * radius, visible: true };
	}

	function findNearestMarker(clientX: number, clientY: number): GlobeMarker | null {
		if (markers.length === 0 || !canvasEl) return null;

		const rect = canvasEl.getBoundingClientRect();
		const cx = rect.left + rect.width / 2;
		const cy = rect.top + rect.height / 2;
		const radius = rect.width / 2;
		const phi = currentPhi + pointerInteractionMovement;
		const threshold = 18;

		let nearest: GlobeMarker | null = null;
		let nearestDist = Infinity;

		for (const marker of markers) {
			const pos = markerToScreen(marker.latitude, marker.longitude, phi, currentTheta, cx, cy, radius);
			if (!pos.visible) continue;

			const dx = clientX - pos.x;
			const dy = clientY - pos.y;
			const dist = Math.sqrt(dx * dx + dy * dy);

			if (dist < threshold && dist < nearestDist) {
				nearestDist = dist;
				nearest = marker;
			}
		}

		return nearest;
	}

	onMount(() => {
		// Delay cobe creation: Svelte mounts children BEFORE parents, so the
		// parent's onMount (which sets GSAP dimensions) hasn't run yet.
		// requestAnimationFrame fires after all onMounts + browser layout.
		const rafId = requestAnimationFrame(() => {
			const size = canvasEl.offsetWidth;
			if (!size) return;

			gsap.fromTo(
				wrapperEl,
				{ opacity: 0 },
				{ opacity: 1, duration: 1.5, ease: 'power2.out', delay: 0.2 }
			);

			globe = createGlobe(canvasEl, {
				devicePixelRatio: 2,
				width: size * 2,
				height: size * 2,
				phi: 0.4,
				theta: 0.2,
				dark: 1,
				diffuse: 1.2,
				mapSamples: 24000,
				mapBrightness: 2,
				mapBaseBrightness: 0.02,
				baseColor: [0.2, 0.2, 0.2],
				markerColor: [1, 1, 1],
				glowColor: [0.1, 0.1, 0.1],
				markers: cobeMarkers,
				onRender: (state) => {
					// When focus is set (showcase mode), rotate to that location
					if (focus && focus.lat !== 0 && focus.lng !== 0) {
						const [targetPhi, targetTheta] = locationToAngles(focus.lat, focus.lng);
						const distPos = (targetPhi - currentPhi + doublePi) % doublePi;
						const distNeg = (currentPhi - targetPhi + doublePi) % doublePi;
						if (distPos < distNeg) {
							currentPhi += distPos * 0.06;
						} else {
							currentPhi -= distNeg * 0.06;
						}
						currentTheta = currentTheta * 0.94 + targetTheta * 0.06;
					} else if (pointerInteracting === null) {
						currentPhi += 0.002;
					}

					state.phi = currentPhi + pointerInteractionMovement;
					state.theta = currentTheta;
					// Use offsetWidth for BOTH — keeps globe perfectly circular
					const s = canvasEl.offsetWidth * 2;
					state.width = s;
					state.height = s;
					state.markers = cobeMarkers;
				},
			});
		});

		return () => {
			cancelAnimationFrame(rafId);
			globe?.destroy();
		};
	});

	function onPointerDown(e: PointerEvent) {
		pointerInteracting = e.clientX - pointerInteractionMovement * 200;
		canvasEl.style.cursor = 'grabbing';
	}
	function onPointerUp() {
		pointerInteracting = null;
		canvasEl.style.cursor = 'grab';
	}
	function onPointerOut() {
		pointerInteracting = null;
		canvasEl.style.cursor = 'grab';
		hoveredEvent = null;
	}
	function onPointerMove(e: PointerEvent) {
		if (pointerInteracting !== null) {
			pointerInteractionMovement = (e.clientX - pointerInteracting) / 200;
		}
		tooltipX = e.clientX;
		tooltipY = e.clientY;

		if (pointerInteracting === null) {
			hoveredEvent = findNearestMarker(e.clientX, e.clientY);
		} else {
			hoveredEvent = null;
		}
	}

	function formatDate(d: string | null) {
		if (!d) return '';
		return new Date(d).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}
</script>

<div bind:this={wrapperEl} class="relative aspect-square w-full {className}" style="opacity: 0;">
	<canvas
		bind:this={canvasEl}
		class="w-full aspect-square cursor-grab"
		onpointerdown={onPointerDown}
		onpointerup={onPointerUp}
		onpointerout={onPointerOut}
		onpointermove={onPointerMove}
	></canvas>
	<!-- Subtle edge fade -->
	<div class="pointer-events-none absolute inset-0 bg-gradient-to-r from-bg/30 via-transparent to-bg/30"></div>
	<div class="pointer-events-none absolute inset-0 bg-gradient-to-b from-bg/20 via-transparent to-bg/40"></div>
</div>

<!-- Tooltip follows cursor near dots -->
{#if hoveredEvent}
	<div
		class="pointer-events-none fixed z-50 border border-border bg-elevated px-4 py-3 shadow-lg"
		style="left: {tooltipX + 16}px; top: {tooltipY - 10}px; max-width: 280px;"
	>
		<p class="text-xs font-bold text-text">{hoveredEvent.name}</p>
		{#if hoveredEvent.start_date}
			<p class="mt-1 text-[10px] uppercase tracking-wider text-muted">{formatDate(hoveredEvent.start_date)}</p>
		{/if}
	</div>
{/if}
