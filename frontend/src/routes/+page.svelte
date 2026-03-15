<script lang="ts">
	import Globe from '$lib/components/Globe.svelte';
	import EventCard from '$lib/components/EventCard.svelte';
	import { fadeIn, slideUp, staggerChildren, countUp } from '$lib/animations/gsap';
	import { ArrowRight } from 'lucide-svelte';
	import type { PageData } from './$types';
	import type { EventSummary } from '$lib/types';

	let { data }: { data: PageData } = $props();

	let heroSearchQuery = $state('');

	function handleHeroSearch(e: SubmitEvent) {
		e.preventDefault();
		if (heroSearchQuery.trim()) {
			window.location.href = `/search?q=${encodeURIComponent(heroSearchQuery.trim())}`;
		}
	}

	// Fallback demo events when backend isn't running
	const demoEvents: EventSummary[] = [
		{ id: '1', name: 'TreeHacks', description: 'Stanford\'s flagship hackathon — 1,600 hackers, 36 hours, $250K+ in prizes', location: 'Stanford, CA', url: null, start_date: '2026-02-14', end_date: '2026-02-16', image_url: null, latitude: 37.43, longitude: -122.17, companies: [{ id: 'c1', name: 'Google', role: 'sponsor' }], avg_rating: 4.7, review_count: 89, created_at: '' },
		{ id: '2', name: 'HackMIT', description: 'MIT\'s annual hackathon bringing together 1,000+ students', location: 'Cambridge, MA', url: null, start_date: '2026-10-01', end_date: '2026-10-02', image_url: null, latitude: 42.36, longitude: -71.09, companies: [{ id: 'c2', name: 'Microsoft', role: 'sponsor' }], avg_rating: 4.5, review_count: 124, created_at: '' },
		{ id: '3', name: 'ETHGlobal London', description: 'Ethereum\'s premier web3 hackathon in the heart of London', location: 'London, UK', url: null, start_date: '2026-03-28', end_date: '2026-03-30', image_url: null, latitude: 51.51, longitude: -0.13, companies: [{ id: 'c3', name: 'Ethereum Foundation', role: 'organizer' }], avg_rating: 4.3, review_count: 67, created_at: '' },
		{ id: '4', name: 'CalHacks', description: 'UC Berkeley\'s largest collegiate hackathon', location: 'Berkeley, CA', url: null, start_date: '2026-06-20', end_date: '2026-06-22', image_url: null, latitude: 37.87, longitude: -122.26, companies: [{ id: 'c4', name: 'Meta', role: 'sponsor' }], avg_rating: 4.1, review_count: 56, created_at: '' },
		{ id: '5', name: 'Hack the North', description: 'Canada\'s biggest hackathon — 3,000+ hackers at the University of Waterloo', location: 'Waterloo, ON', url: null, start_date: '2026-09-13', end_date: '2026-09-15', image_url: null, latitude: 43.47, longitude: -80.54, companies: [{ id: 'c5', name: 'Shopify', role: 'sponsor' }], avg_rating: 4.8, review_count: 201, created_at: '' },
		{ id: '6', name: 'Junction', description: 'Europe\'s leading hackathon gathering 1,500 hackers in Helsinki', location: 'Helsinki, Finland', url: null, start_date: '2026-11-07', end_date: '2026-11-09', image_url: null, latitude: 60.17, longitude: 24.94, companies: [{ id: 'c6', name: 'Nokia', role: 'sponsor' }], avg_rating: 4.6, review_count: 78, created_at: '' },
	];

	const events = $derived(data.events.length > 0 ? data.events : demoEvents);
	const eventCount = $derived(data.totalEvents || 10380);
</script>

<svelte:head>
	<title>RateMyHackathons</title>
	<meta name="description" content="The internet's honest record of hackathon experiences." />
</svelte:head>

<!-- ═══════ HERO ═══════ -->
<section class="relative flex min-h-[100svh] items-center overflow-hidden">
	<!-- Globe: big, bright, bleeds right -->
	<div class="pointer-events-none absolute -right-[5%] top-1/2 w-[85vw] max-w-[1000px] -translate-y-1/2 opacity-70 sm:pointer-events-auto sm:opacity-80 lg:w-[60vw]" use:fadeIn={{ y: 0, duration: 1.5 }}>
		<Globe markers={data.markers} />
	</div>

	<div class="relative z-10 mx-auto w-full max-w-[1400px] px-6 py-24">
		<div class="max-w-3xl" use:fadeIn={{ y: 50, duration: 1 }}>
			<h1 class="font-display text-[clamp(3.5rem,11vw,10rem)] italic leading-[0.85] tracking-tight">
				Every<br />hackathon,<br />
				<span class="text-muted">rated.</span>
			</h1>

			<div class="mt-8 h-px w-24 bg-dim"></div>

			<p class="mt-8 max-w-md text-sm leading-relaxed text-muted">
				No sponsored placements. No corporate filters. Just thousands of
				honest reviews from people who were actually there.
			</p>

			<!-- Search bar: minimal underline style -->
			<form onsubmit={handleHeroSearch} class="mt-10 max-w-sm">
				<div class="group flex items-center border-b border-dim transition-colors focus-within:border-text">
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
				<span class="mx-8 text-[11px] uppercase tracking-[0.3em] text-dim">{name}</span>
				<span class="mx-2 text-border">&bull;</span>
			{/each}
		{/each}
	</div>
</div>

<!-- ═══════ STATS ═══════ -->
<section class="py-24">
	<div class="mx-auto max-w-[1400px] px-6">
		<div class="grid grid-cols-2 border border-border md:grid-cols-4" use:staggerChildren={{ stagger: 0.15, y: 20 }}>
			<div class="border-b border-border p-8 text-center md:border-b-0 md:border-r">
				<div class="font-display text-5xl italic sm:text-6xl lg:text-7xl" use:countUp={{ target: eventCount }}>0</div>
				<p class="mt-2 text-[11px] uppercase tracking-[0.3em] text-muted">Events tracked</p>
			</div>
			<div class="border-b border-border p-8 text-center md:border-b-0 md:border-r">
				<div class="font-display text-5xl italic sm:text-6xl lg:text-7xl" use:countUp={{ target: 45 }}>0</div>
				<p class="mt-2 text-[11px] uppercase tracking-[0.3em] text-muted">Cities</p>
			</div>
			<div class="p-8 text-center md:border-r border-border">
				<div class="font-display text-5xl italic sm:text-6xl lg:text-7xl" use:countUp={{ target: 30 }}>0</div>
				<p class="mt-2 text-[11px] uppercase tracking-[0.3em] text-muted">Companies</p>
			</div>
			<div class="p-8 text-center">
				<div class="font-display text-5xl italic sm:text-6xl lg:text-7xl" use:countUp={{ target: 4 }}>0</div>
				<p class="mt-2 text-[11px] uppercase tracking-[0.3em] text-muted">Sources</p>
			</div>
		</div>
	</div>
</section>

<!-- ═══════ FEATURED EVENTS ═══════ -->
<section class="border-t border-border bg-surface py-24">
	<div class="mx-auto max-w-[1400px] px-6">
		<div class="mb-14 flex items-end justify-between" use:fadeIn>
			<div>
				<span class="text-[11px] uppercase tracking-[0.3em] text-dim">Selection</span>
				<h2 class="mt-2 font-display text-5xl italic sm:text-6xl">Recent events</h2>
			</div>
			<a href="/events" class="hover-line hidden text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text sm:block">
				View all &rarr;
			</a>
		</div>

		<!-- Magazine layout: featured spans 2 rows, flat grid -->
		<div class="grid gap-4 md:grid-cols-2 md:grid-rows-2" use:staggerChildren={{ stagger: 0.08 }}>
			{#each events.slice(0, 1) as event (event.id)}
				<EventCard {event} featured={true} />
			{/each}
			{#each events.slice(1, 3) as event (event.id)}
				<EventCard {event} />
			{/each}
		</div>

		{#if events.length > 3}
			<div class="mt-4 grid gap-4 md:grid-cols-3" use:staggerChildren={{ stagger: 0.06 }}>
				{#each events.slice(3, 6) as event (event.id)}
					<EventCard {event} />
				{/each}
			</div>
		{/if}

		<div class="mt-10 text-center sm:hidden">
			<a href="/events" class="text-xs uppercase tracking-[0.2em] text-muted">View all events &rarr;</a>
		</div>
	</div>
</section>

<!-- ═══════ HOW IT WORKS ═══════ -->
<section class="py-24">
	<div class="mx-auto max-w-[1400px] px-6">
		<div class="mb-14" use:fadeIn>
			<span class="text-[11px] uppercase tracking-[0.3em] text-dim">Process</span>
			<h2 class="mt-2 font-display text-5xl italic sm:text-6xl">How it works</h2>
		</div>

		<div class="grid gap-4 md:grid-cols-3" use:staggerChildren={{ stagger: 0.15 }}>
			{#each [
				{ num: '01', title: 'Discover', text: 'Browse hackathons worldwide. Filter by city, date, company, or rating. See them plotted on an interactive globe.' },
				{ num: '02', title: 'Experience', text: 'Read honest reviews before you commit. Know the vibe, the prizes, the food, the WiFi — from people who were actually there.' },
				{ num: '03', title: 'Rate', text: 'Been there? Share what it was really like. No PR spin, no sponsored takes. Just your honest experience for the next hacker.' },
			] as step}
				<div class="group border border-border p-10 transition-all duration-500 hover:border-accent hover:bg-surface">
					<span class="font-display text-7xl italic text-border transition-colors duration-500 group-hover:text-accent lg:text-8xl">{step.num}</span>
					<h3 class="mt-6 text-sm font-bold uppercase tracking-[0.2em]">{step.title}</h3>
					<p class="mt-4 text-sm leading-relaxed text-muted">{step.text}</p>
				</div>
			{/each}
		</div>
	</div>
</section>

<!-- ═══════ PULL QUOTE ═══════ -->
<section class="bg-surface py-28">
	<div class="mx-auto max-w-[1000px] px-6 text-center" use:slideUp>
		<span class="font-display text-8xl italic text-border">&ldquo;</span>
		<p class="font-display text-3xl italic leading-snug sm:text-4xl lg:text-5xl">
			The only hackathon directory that cares about what actually happened, not what the sponsors say happened.
		</p>
		<div class="mx-auto mt-8 h-px w-24 bg-border"></div>
		<p class="mt-6 text-[11px] uppercase tracking-[0.3em] text-dim">What we believe</p>
	</div>
</section>

<!-- ═══════ CTA ═══════ -->
<section class="py-28">
	<div class="mx-auto max-w-[1400px] px-6 text-center" use:fadeIn>
		<h2 class="font-display text-6xl italic sm:text-7xl lg:text-8xl">Start exploring</h2>
		<p class="mx-auto mt-6 max-w-md text-sm leading-relaxed text-muted">
			Thousands of events. Real reviews. Zero agenda.
		</p>
		<div class="mt-12 flex flex-col items-center gap-4 sm:flex-row sm:justify-center">
			<a
				href="/events"
				class="border border-text bg-text px-12 py-4 text-xs uppercase tracking-[0.2em] text-bg transition-all duration-300 hover:bg-transparent hover:text-text"
			>
				Browse events
			</a>
			<a
				href="/search"
				class="border border-border px-12 py-4 text-xs uppercase tracking-[0.2em] text-muted transition-all duration-300 hover:border-text hover:text-text"
			>
				Search
			</a>
		</div>
	</div>
</section>
