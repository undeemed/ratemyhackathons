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
			const count = Math.floor((width * height) / (10000 / density * 100));
			particles = Array.from({ length: Math.min(count, 200) }, () => ({
				x: Math.random() * width,
				y: Math.random() * height,
				vx: (Math.random() - 0.5) * speed,
				vy: (Math.random() - 0.5) * speed,
				size: Math.random() * 1.5 + 0.5,
				opacity: Math.random() * maxOpacity,
			}));
		}

		function draw() {
			ctx!.clearRect(0, 0, width, height);

			// Draw connections
			for (let i = 0; i < particles.length; i++) {
				for (let j = i + 1; j < particles.length; j++) {
					const dx = particles[i].x - particles[j].x;
					const dy = particles[i].y - particles[j].y;
					const dist = Math.sqrt(dx * dx + dy * dy);

					if (dist < connectDistance) {
						const opacity = (1 - dist / connectDistance) * 0.15;
						ctx!.beginPath();
						ctx!.strokeStyle = `rgba(${color}, ${opacity})`;
						ctx!.lineWidth = 0.5;
						ctx!.moveTo(particles[i].x, particles[i].y);
						ctx!.lineTo(particles[j].x, particles[j].y);
						ctx!.stroke();
					}
				}
			}

			// Draw particles
			for (const p of particles) {
				// Mouse repulsion
				const mdx = p.x - mouse.x;
				const mdy = p.y - mouse.y;
				const mDist = Math.sqrt(mdx * mdx + mdy * mdy);
				if (mDist < 150) {
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

			animId = requestAnimationFrame(draw);
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
		draw();

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
