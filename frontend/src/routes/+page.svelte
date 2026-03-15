<script lang="ts">
	import Globe from '$lib/components/Globe.svelte';
	import EventCard from '$lib/components/EventCard.svelte';
	import { fadeIn, slideUp, staggerChildren, countUp } from '$lib/animations/gsap';
	import { ArrowRight } from 'lucide-svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	let heroSearchQuery = $state('');

	function handleHeroSearch(e: SubmitEvent) {
		e.preventDefault();
		if (heroSearchQuery.trim()) {
			window.location.href = `/search?q=${encodeURIComponent(heroSearchQuery.trim())}`;
		}
	}
</script>

<svelte:head>
	<title>RateMyHackathons</title>
	<meta name="description" content="The internet's honest record of hackathon experiences." />
</svelte:head>

<!-- ═══════ HERO ═══════ -->
<section class="relative flex min-h-screen items-center overflow-hidden">
	<!-- Globe positioned behind, right-aligned, oversized -->
	<div class="pointer-events-none absolute -right-[10%] top-1/2 w-[80vw] max-w-[900px] -translate-y-1/2 opacity-40 sm:pointer-events-auto sm:opacity-60 lg:w-[55vw]" use:fadeIn={{ y: 0, duration: 1.5 }}>
		<Globe markers={data.markers} />
	</div>

	<div class="relative z-10 mx-auto w-full max-w-[1400px] px-6 py-32">
		<div class="max-w-3xl" use:fadeIn={{ y: 50, duration: 1 }}>
			<h1 class="font-display text-[clamp(3rem,10vw,9rem)] italic leading-[0.9] tracking-tight">
				Every<br />hackathon,<br />
				<span class="text-muted">rated.</span>
			</h1>

			<div class="mt-8 h-px w-24 bg-border"></div>

			<p class="mt-8 max-w-md text-xs leading-relaxed text-muted">
				No sponsored placements. No corporate filters. Just thousands of
				honest reviews from people who were actually there.
			</p>

			<!-- Search: just a line, not a box -->
			<form onsubmit={handleHeroSearch} class="mt-12 max-w-sm">
				<div class="group flex items-center border-b border-border transition-colors focus-within:border-text">
					<input
						bind:value={heroSearchQuery}
						type="text"
						placeholder="Search hackathons..."
						class="w-full bg-transparent py-3 text-sm text-text placeholder:text-dim focus:outline-none"
					/>
					<button type="submit" class="text-dim transition-colors group-focus-within:text-text">
						<ArrowRight class="h-4 w-4" />
					</button>
				</div>
			</form>
		</div>
	</div>
</section>

<!-- ═══════ MARQUEE TICKER ═══════ -->
<div class="overflow-hidden border-y border-border py-4">
	<div class="animate-marquee flex whitespace-nowrap">
		{#each [...Array(2)] as _}
			{#each ['HackMIT', 'TreeHacks', 'PennApps', 'CalHacks', 'HackGT', 'MHacks', 'HackNY', 'SFHacks', 'ETHGlobal', 'Junction', 'HackZurich', 'AngelHack', 'DeveloperWeek', 'Launch Hack', 'Hack the North', 'YC Hacks'] as name}
				<span class="mx-6 text-xs uppercase tracking-[0.3em] text-dim">{name}</span>
				<span class="mx-2 text-dim">&bull;</span>
			{/each}
		{/each}
	</div>
</div>

<!-- ═══════ STATS ═══════ -->
<section class="py-32">
	<div class="mx-auto max-w-[1400px] px-6">
		<div class="grid grid-cols-2 gap-px border border-border md:grid-cols-4" use:staggerChildren={{ stagger: 0.15, y: 20 }}>
			<div class="p-8 text-center">
				<div class="font-display text-6xl italic sm:text-7xl" use:countUp={{ target: data.totalEvents || 150 }}>0</div>
				<p class="mt-2 text-[10px] uppercase tracking-[0.3em] text-dim">Events tracked</p>
			</div>
			<div class="border-l border-border p-8 text-center">
				<div class="font-display text-6xl italic sm:text-7xl" use:countUp={{ target: 45 }}>0</div>
				<p class="mt-2 text-[10px] uppercase tracking-[0.3em] text-dim">Cities</p>
			</div>
			<div class="border-l border-border p-8 text-center max-md:border-l-0 max-md:border-t">
				<div class="font-display text-6xl italic sm:text-7xl" use:countUp={{ target: 30 }}>0</div>
				<p class="mt-2 text-[10px] uppercase tracking-[0.3em] text-dim">Companies</p>
			</div>
			<div class="border-l border-border p-8 text-center max-md:border-t">
				<div class="font-display text-6xl italic sm:text-7xl" use:countUp={{ target: 4 }}>0</div>
				<p class="mt-2 text-[10px] uppercase tracking-[0.3em] text-dim">Sources</p>
			</div>
		</div>
	</div>
</section>

<!-- ═══════ FEATURED EVENTS ═══════ -->
<section class="border-t border-border py-32">
	<div class="mx-auto max-w-[1400px] px-6">
		<div class="mb-16 flex items-end justify-between" use:fadeIn>
			<div>
				<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Selection</span>
				<h2 class="mt-2 font-display text-5xl italic sm:text-6xl">Recent events</h2>
			</div>
			<a href="/events" class="hover-line hidden text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text sm:block">
				View all &rarr;
			</a>
		</div>

		{#if data.events.length > 0}
			<!-- Magazine layout: one big + smaller grid -->
			<div class="grid gap-px md:grid-cols-2" use:staggerChildren={{ stagger: 0.1 }}>
				{#each data.events.slice(0, 1) as event (event.id)}
					<EventCard {event} featured={true} />
				{/each}
				<div class="grid gap-px">
					{#each data.events.slice(1, 3) as event (event.id)}
						<EventCard {event} />
					{/each}
				</div>
			</div>

			{#if data.events.length > 3}
				<div class="mt-px grid gap-px md:grid-cols-3" use:staggerChildren={{ stagger: 0.08 }}>
					{#each data.events.slice(3, 6) as event (event.id)}
						<EventCard {event} />
					{/each}
				</div>
			{/if}
		{:else}
			<div class="border border-border py-24 text-center" use:fadeIn>
				<p class="font-display text-2xl italic text-muted">Nothing here yet.</p>
				<p class="mt-2 text-xs text-dim">Start the backend and crawler to populate events.</p>
			</div>
		{/if}

		<div class="mt-8 text-center sm:hidden">
			<a href="/events" class="text-xs uppercase tracking-[0.2em] text-muted">View all events &rarr;</a>
		</div>
	</div>
</section>

<!-- ═══════ HOW IT WORKS ═══════ -->
<section class="border-t border-border py-32">
	<div class="mx-auto max-w-[1400px] px-6">
		<div class="mb-20" use:fadeIn>
			<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Process</span>
			<h2 class="mt-2 font-display text-5xl italic sm:text-6xl">How it works</h2>
		</div>

		<div class="grid gap-px border border-border md:grid-cols-3" use:staggerChildren={{ stagger: 0.2 }}>
			{#each [
				{ num: '01', title: 'Discover', text: 'Browse hackathons worldwide. Filter by city, date, company, or rating. See them on a globe.' },
				{ num: '02', title: 'Attend', text: 'Read honest reviews before you commit. Know the vibe, the prizes, the food, the WiFi.' },
				{ num: '03', title: 'Rate', text: 'Been there? Share what it was actually like. Help the next person make a better call.' },
			] as step}
				<div class="p-8 transition-colors hover:bg-elevated md:p-12">
					<span class="font-display text-7xl italic text-dim sm:text-8xl">{step.num}</span>
					<h3 class="mt-6 text-sm font-bold uppercase tracking-[0.15em]">{step.title}</h3>
					<p class="mt-3 text-xs leading-relaxed text-muted">{step.text}</p>
				</div>
			{/each}
		</div>
	</div>
</section>

<!-- ═══════ PULL QUOTE ═══════ -->
<section class="border-t border-border py-32">
	<div class="mx-auto max-w-[900px] px-6 text-center" use:slideUp>
		<span class="font-display text-8xl italic text-dim sm:text-9xl">&ldquo;</span>
		<p class="font-display text-3xl italic leading-snug sm:text-4xl lg:text-5xl">
			The only hackathon directory that cares about what actually happened, not what the sponsors say happened.
		</p>
		<div class="mx-auto mt-8 h-px w-16 bg-border"></div>
	</div>
</section>

<!-- ═══════ CTA ═══════ -->
<section class="border-t border-border py-32">
	<div class="mx-auto max-w-[1400px] px-6 text-center" use:fadeIn>
		<h2 class="font-display text-6xl italic sm:text-7xl lg:text-8xl">Start exploring</h2>
		<p class="mx-auto mt-6 max-w-md text-xs leading-relaxed text-muted">
			Thousands of events. Real reviews. Zero agenda.
		</p>
		<div class="mt-12 flex flex-col items-center gap-4 sm:flex-row sm:justify-center">
			<a
				href="/events"
				class="border border-text px-10 py-4 text-xs uppercase tracking-[0.2em] transition-colors hover:bg-text hover:text-bg"
			>
				Browse events
			</a>
			<a
				href="/search"
				class="border border-border px-10 py-4 text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:border-text hover:text-text"
			>
				Search
			</a>
		</div>
	</div>
</section>
