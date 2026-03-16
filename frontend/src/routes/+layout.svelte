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
	let overlay = $state<HTMLDivElement>(undefined!);
	let modal = $state<HTMLDivElement>(undefined!);

	function handleOverlayKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') dismiss();
	}

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
		colorPrimary: '#ffffff',
		colorBackground: '#000000',
		colorText: '#ffffff',
		colorTextSecondary: '#999999',
		colorInputBackground: '#111111',
		colorInputText: '#ffffff',
		colorNeutral: '#ffffff',
		colorTextOnPrimaryBackground: '#000000',
		colorDanger: '#ef5350',
		borderRadius: '0px',
		fontFamily: "'Space Mono', monospace",
		fontFamilyButtons: "'Space Mono', monospace",
	},
	elements: {
		card: { boxShadow: 'none', background: '#000', border: '1px solid #333', width: '560px', maxWidth: '100%', padding: '32px' },
		headerTitle: { fontFamily: "'Instrument Serif', serif", fontStyle: 'italic', color: '#fff' },
		headerSubtitle: { color: '#999' },
		socialButtonsBlockButton: { border: '1px solid #333', borderRadius: '0', color: '#fff', background: '#111' },
		socialButtonsBlockButtonText: { color: '#fff' },
		formFieldInput: { border: '1px solid #333', borderRadius: '0', background: '#111', color: '#fff' },
		formFieldLabel: { color: '#ccc' },
		formButtonPrimary: {
			borderRadius: '0',
			textTransform: 'uppercase',
			letterSpacing: '0.15em',
			fontSize: '11px',
			background: '#fff',
			color: '#000',
			'&:hover, &:focus': { background: '#ccc' },
		},
		footerAction: { background: '#000' },
		footerActionText: { color: '#999' },
		footerActionLink: { color: '#fff' },
		dividerLine: { background: '#333' },
		dividerText: { color: '#666' },
		footer: { background: '#000' },
		footerPages: { background: '#000' },
		footerPagesLink: { color: '#999' },
	}
}}>
	{#if showWip}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div bind:this={overlay} onclick={dismiss} onkeydown={handleOverlayKeydown}
			class="fixed inset-0 z-[999] flex items-center justify-center bg-black/70 backdrop-blur-sm">
			<!-- svelte-ignore a11y_no_static_element_interactions, a11y_click_events_have_key_events -->
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
