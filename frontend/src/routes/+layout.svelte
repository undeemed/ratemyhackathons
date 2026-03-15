<script lang="ts">
	import '../app.css';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { ClerkProvider } from 'svelte-clerk';
	import { onMount } from 'svelte';
	import gsap from 'gsap';
	import { browser } from '$app/environment';

	let { children } = $props();
	let showWip = $state(browser && !localStorage.getItem('wip-dismissed'));
	let overlay: HTMLDivElement;
	let modal: HTMLDivElement;

	onMount(() => {
		if (!showWip) return;
		const tl = gsap.timeline({ defaults: { duration: 0.4, ease: 'power3.out' } });
		tl.fromTo(overlay, { opacity: 0 }, { opacity: 1, duration: 0.3 })
		  .fromTo(modal, { opacity: 0, scale: 0.92, y: 30 }, { opacity: 1, scale: 1, y: 0 }, '-=0.15');
	});

	function dismiss() {
		localStorage.setItem('wip-dismissed', '1');
		const tl = gsap.timeline({
			defaults: { duration: 0.25, ease: 'power2.in' },
			onComplete: () => { showWip = false; }
		});
		tl.to(modal, { opacity: 0, scale: 0.95, y: -20 })
		  .to(overlay, { opacity: 0 }, '-=0.15');
	}
</script>

<ClerkProvider appearance={{
	variables: {
		colorPrimary: '#005ea2',
		colorBackground: '#1b1b1b',
		colorText: '#f0f0f0',
		colorTextSecondary: '#a9aeb1',
		colorInputBackground: '#2d2d2d',
		colorInputText: '#f0f0f0',
		colorNeutral: '#f0f0f0',
		colorDanger: '#d83933',
		borderRadius: '0px',
		fontFamily: "'Space Mono', monospace",
		fontFamilyButtons: "'Space Mono', monospace",
	},
	elements: {
		card: { boxShadow: 'none', border: '1px solid #3d4551' },
		headerTitle: { fontFamily: "'Instrument Serif', serif", fontStyle: 'italic', color: '#f0f0f0' },
		headerSubtitle: { color: '#a9aeb1' },
		socialButtonsBlockButton: { border: '1px solid #565c65', borderRadius: '0', color: '#f0f0f0', background: '#2d2d2d' },
		formFieldInput: { border: '1px solid #565c65', borderRadius: '0', background: '#2d2d2d', color: '#f0f0f0' },
		formFieldLabel: { color: '#a9aeb1' },
		formButtonPrimary: {
			borderRadius: '0',
			textTransform: 'uppercase',
			letterSpacing: '0.15em',
			fontSize: '11px',
			background: '#005ea2',
			color: '#f0f0f0',
			'&:hover, &:focus': { background: '#1a4480' },
		},
		footerActionLink: { color: '#73b3e7' },
		dividerLine: { background: '#3d4551' },
		dividerText: { color: '#71767a' },
		footer: { background: '#1b1b1b', borderTop: '1px solid #3d4551' },
		card__footer: { background: '#1b1b1b' },
	}
}}>
	{#if showWip}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div bind:this={overlay} onclick={dismiss}
			class="fixed inset-0 z-[999] flex items-center justify-center bg-black/70 backdrop-blur-sm">
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div bind:this={modal} onclick={(e) => e.stopPropagation()}
				class="relative mx-4 max-w-md border border-border bg-surface px-10 py-8 text-center">
				<p class="font-display text-3xl italic text-text">Work in progress.</p>
				<p class="mt-3 font-mono text-xs uppercase tracking-widest text-muted">
					This site is under active development.
				</p>
				<button onclick={dismiss}
					class="mt-6 border border-border px-6 py-2 font-mono text-xs uppercase tracking-widest text-text transition-colors hover:bg-elevated">
					Enter anyways
				</button>
			</div>
		</div>
	{/if}

	<Nav />
	<main class="min-h-screen pt-14">
		{@render children()}
	</main>
	<Footer />
</ClerkProvider>
