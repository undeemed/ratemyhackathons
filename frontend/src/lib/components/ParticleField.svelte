<script lang="ts">
	import { onMount } from 'svelte';

	let {
		density = 80,
		speed = 0.3,
		color = '255, 255, 255',
		maxOpacity = 0.6,
		connectDistance = 120,
		class: className = '',
	}: {
		density?: number;
		speed?: number;
		color?: string;
		maxOpacity?: number;
		connectDistance?: number;
		class?: string;
	} = $props();

	let canvasEl: HTMLCanvasElement;

	onMount(() => {
		const ctx = canvasEl.getContext('2d');
		if (!ctx) return;

		let animId: number;
		let width = 0;
		let height = 0;
		let particles: { x: number; y: number; vx: number; vy: number; size: number; opacity: number }[] = [];
		let mouse = { x: -1000, y: -1000 };

		// Pre-compute squared distance threshold (avoids Math.sqrt every frame)
		const connectDistSq = connectDistance * connectDistance;

		function resize() {
			const rect = canvasEl.parentElement?.getBoundingClientRect();
			width = rect?.width ?? window.innerWidth;
			height = rect?.height ?? window.innerHeight;
			canvasEl.width = width * 2;
			canvasEl.height = height * 2;
			ctx!.scale(2, 2);
			initParticles();
		}

		function initParticles() {
			// Reduced cap: 100 max (was 200) — halves O(n²) connection checks
			const count = Math.floor((width * height) / (10000 / density * 100));
			particles = Array.from({ length: Math.min(count, 100) }, () => ({
				x: Math.random() * width,
				y: Math.random() * height,
				vx: (Math.random() - 0.5) * speed,
				vy: (Math.random() - 0.5) * speed,
				size: Math.random() * 1.5 + 0.5,
				opacity: Math.random() * maxOpacity,
			}));
		}

		let lastFrameTime = 0;
		const targetInterval = 1000 / 30; // Throttle to 30fps (was uncapped 60fps)

		function draw(timestamp: number) {
			animId = requestAnimationFrame(draw);

			// Throttle: skip frames to maintain ~30fps
			const delta = timestamp - lastFrameTime;
			if (delta < targetInterval) return;
			lastFrameTime = timestamp - (delta % targetInterval);

			ctx!.clearRect(0, 0, width, height);

			const len = particles.length;

			// Draw connections — using squared distance to avoid sqrt
			for (let i = 0; i < len; i++) {
				const pi = particles[i];
				for (let j = i + 1; j < len; j++) {
					const pj = particles[j];
					const dx = pi.x - pj.x;
					const dy = pi.y - pj.y;
					const distSq = dx * dx + dy * dy;

					if (distSq < connectDistSq) {
						const dist = Math.sqrt(distSq); // sqrt only for matched pairs
						const opacity = (1 - dist / connectDistance) * 0.15;
						ctx!.beginPath();
						ctx!.strokeStyle = `rgba(${color}, ${opacity})`;
						ctx!.lineWidth = 0.5;
						ctx!.moveTo(pi.x, pi.y);
						ctx!.lineTo(pj.x, pj.y);
						ctx!.stroke();
					}
				}
			}

			// Draw particles
			for (const p of particles) {
				// Mouse repulsion
				const mdx = p.x - mouse.x;
				const mdy = p.y - mouse.y;
				const mDistSq = mdx * mdx + mdy * mdy;
				if (mDistSq < 22500) { // 150 * 150
					const mDist = Math.sqrt(mDistSq);
					const force = (150 - mDist) / 150;
					p.x += (mdx / mDist) * force * 2;
					p.y += (mdy / mDist) * force * 2;
				}

				p.x += p.vx;
				p.y += p.vy;

				// Wrap around edges
				if (p.x < 0) p.x = width;
				if (p.x > width) p.x = 0;
				if (p.y < 0) p.y = height;
				if (p.y > height) p.y = 0;

				ctx!.beginPath();
				ctx!.arc(p.x, p.y, p.size, 0, Math.PI * 2);
				ctx!.fillStyle = `rgba(${color}, ${p.opacity})`;
				ctx!.fill();
			}
		}

		function onMouseMove(e: MouseEvent) {
			const rect = canvasEl.getBoundingClientRect();
			mouse.x = e.clientX - rect.left;
			mouse.y = e.clientY - rect.top;
		}

		function onMouseLeave() {
			mouse.x = -1000;
			mouse.y = -1000;
		}

		resize();
		animId = requestAnimationFrame(draw);

		window.addEventListener('resize', resize);
		canvasEl.addEventListener('mousemove', onMouseMove);
		canvasEl.addEventListener('mouseleave', onMouseLeave);

		return () => {
			cancelAnimationFrame(animId);
			window.removeEventListener('resize', resize);
			canvasEl.removeEventListener('mousemove', onMouseMove);
			canvasEl.removeEventListener('mouseleave', onMouseLeave);
		};
	});
</script>

<div class="pointer-events-auto absolute inset-0 overflow-hidden {className}">
	<canvas
		bind:this={canvasEl}
		class="h-full w-full"
	></canvas>
</div>
