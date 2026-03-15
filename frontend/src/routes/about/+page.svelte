<script lang="ts">
	import { fadeIn, staggerChildren } from '$lib/animations/gsap';
</script>

<svelte:head>
	<title>About — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1400px] px-6 py-24">
	<!-- Header -->
	<div class="mb-20" use:fadeIn>
		<a
			href="/"
			class="mb-6 inline-block text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:text-text"
			>&larr; Back</a
		>
		<h1 class="mt-2 font-display text-6xl italic sm:text-7xl lg:text-8xl">About</h1>
		<div class="mt-6 h-px w-24 bg-border"></div>
	</div>

	<!-- Mission -->
	<section class="mb-36 grid grid-cols-1 gap-12 lg:grid-cols-2" use:fadeIn>
		<div>
			<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Mission</span>
			<h2 class="mt-3 font-display text-4xl italic sm:text-5xl">
				The internet's honest record of hackathon experiences.
			</h2>
		</div>
		<div class="flex flex-col justify-center">
			<p class="text-sm leading-relaxed text-muted">
				Hackathon culture has a transparency problem. Organizers self-report attendance, sponsors
				curate testimonials, and attendees have nowhere to share unfiltered feedback. RateMyHackathons
				changes that.
			</p>
			<p class="mt-4 text-sm leading-relaxed text-muted">
				We aggregate data from across the hackathon ecosystem — MLH, Luma, Cerebral Valley,
				Hackiterate — and pair it with real participant reviews. No sponsored reviews.
				No corporate filters. Just truth.
			</p>
		</div>
	</section>

	<!-- How It Works -->
	<section class="mb-36 border-t border-border pt-52" use:fadeIn>
		<span class="text-[10px] uppercase tracking-[0.3em] text-dim">How It Works</span>
		<h2 class="mt-3 font-display text-4xl italic">Three layers of data</h2>
		<div class="mt-12 grid grid-cols-1 gap-px border border-border md:grid-cols-3" use:staggerChildren>
			<div class="bg-surface p-8">
				<span class="font-display text-5xl italic text-dim">01</span>
				<h3 class="mt-4 text-xs uppercase tracking-[0.2em]">Crawl</h3>
				<p class="mt-3 text-xs leading-relaxed text-muted">
					Four automated spiders continuously scrape hackathon listings from MLH, Luma, Cerebral
					Valley, and Hackiterate. Cross-source deduplication merges events using URL normalization
					and fuzzy name matching.
				</p>
			</div>
			<div class="bg-surface p-8">
				<span class="font-display text-5xl italic text-dim">02</span>
				<h3 class="mt-4 text-xs uppercase tracking-[0.2em]">Enrich</h3>
				<p class="mt-3 text-xs leading-relaxed text-muted">
					Each event is geocoded for globe visualization, sponsors are extracted via CSS selectors and
					LLM fallback, and companies are matched against our database. Latitude/longitude coordinates
					power the interactive WebGL globe.
				</p>
			</div>
			<div class="bg-surface p-8">
				<span class="font-display text-5xl italic text-dim">03</span>
				<h3 class="mt-4 text-xs uppercase tracking-[0.2em]">Review</h3>
				<p class="mt-3 text-xs leading-relaxed text-muted">
					Participants rate hackathons across 10 categories — organization, prizes, mentorship,
					judging, venue, food, swag, networking, communication, and vibes. Reviews are threaded,
					votable, and tag-able.
				</p>
			</div>
		</div>
	</section>

	<!-- Stack -->
	<section class="mb-36 border-t border-border pt-52" use:fadeIn>
		<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Stack</span>
		<h2 class="mt-3 font-display text-4xl italic">Built with</h2>
		<div class="mt-12 grid grid-cols-1 gap-px border border-border md:grid-cols-2">
			{#each [
				{ layer: 'Frontend', tech: 'SvelteKit + Svelte 5 + Tailwind v4 + GSAP + cobe', desc: 'Editorial brutalist interface with WebGL globe and scroll-driven animations' },
				{ layer: 'Backend', tech: 'Rust + Actix-Web + SQLx', desc: 'Async REST API with compile-time checked SQL, input sanitization, UUIDv7' },
				{ layer: 'Crawler', tech: 'Python + Scrapling + geopy', desc: 'Adaptive scraping with stealth fetchers, proxy rotation, Nominatim geocoding' },
				{ layer: 'Database', tech: 'PostgreSQL', desc: 'Full-text search, tsvector indexing, correlated subqueries for N+1-free queries' },
				{ layer: 'Analytics', tech: 'Rust + SvelteKit', desc: 'Live dashboard with SSE feed, crawl stats, and review analytics' },
				{ layer: 'Auth', tech: 'Clerk + JWKS', desc: 'JWT verification with RS256 signatures, cached JWKS endpoint' },
			] as item}
				<div class="bg-surface p-6">
					<span class="mb-2 block text-[10px] uppercase tracking-[0.3em] text-dim">{item.layer}</span>
					<span class="text-sm text-text">{item.tech}</span>
					<p class="mt-1 text-xs text-muted">{item.desc}</p>
				</div>
			{/each}
		</div>
	</section>

	<!-- Data Sources -->
	<section class="mb-36 border-t border-border pt-52 grid grid-cols-1 gap-12 lg:grid-cols-2" use:fadeIn>
		<div>
			<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Data Sources</span>
			<h2 class="mt-3 font-display text-4xl italic">Four spiders, one truth</h2>
			<p class="mt-4 text-xs leading-relaxed text-muted">
				Our crawler runs continuously, pulling from every major hackathon listing platform. Source
				priority for dedup merging: Luma (richest data) &gt; Cerebral Valley &gt; MLH &gt;
				Hackiterate.
			</p>
		</div>
		<div class="flex flex-col gap-px border border-border" use:staggerChildren>
			{#each [
				{ name: 'MLH', method: 'HTML scraping', desc: 'The largest hackathon league' },
				{ name: 'Luma', method: 'API', desc: '15-city geo sweep + keyword filter' },
				{ name: 'Cerebral Valley', method: 'Public API', desc: 'AI/ML focused events + host enrichment' },
				{ name: 'Hackiterate', method: 'Playwright', desc: 'JS-rendered SPA scraping' },
			] as spider}
				<div class="flex items-baseline justify-between bg-surface px-6 py-4">
					<div>
						<span class="text-sm text-text">{spider.name}</span>
						<span class="ml-3 text-[10px] uppercase tracking-[0.2em] text-dim">{spider.method}</span>
					</div>
					<span class="text-xs text-muted">{spider.desc}</span>
				</div>
			{/each}
		</div>
	</section>

	<!-- Review Categories -->
	<section class="mb-36 border-t border-border pt-52" use:fadeIn>
		<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Rating System</span>
		<h2 class="mt-3 font-display text-4xl italic">10 categories, no faking it</h2>
		<p class="mt-4 max-w-2xl text-xs leading-relaxed text-muted">
			Every review scores a hackathon across 10 distinct categories on a 1–5 scale. This granularity
			prevents gaming — you can't hide bad food behind great prizes.
		</p>
		<div class="mt-12 grid grid-cols-2 gap-px border border-border sm:grid-cols-5" use:staggerChildren>
			{#each ['Organization', 'Prizes', 'Mentorship', 'Judging', 'Venue', 'Food', 'Swag', 'Networking', 'Communication', 'Vibes'] as cat}
				<div class="bg-surface p-4 text-center">
					<span class="text-[10px] uppercase tracking-[0.2em] text-muted">{cat}</span>
				</div>
			{/each}
		</div>
	</section>

	<!-- Open Source / Contact -->
	<section class="border-t border-border pt-52" use:fadeIn>
		<div class="grid grid-cols-1 gap-12 lg:grid-cols-2">
			<div>
				<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Open Source</span>
				<h2 class="mt-3 font-display text-4xl italic">Built in the open</h2>
				<p class="mt-4 text-xs leading-relaxed text-muted">
					RateMyHackathons is fully open source. The entire stack — frontend, backend, crawler,
					analytics — is available on GitHub. Contributions welcome.
				</p>
			</div>
			<div class="flex flex-col justify-center gap-4">
				<a
					href="https://github.com/undeemed/ratemyhackathons"
					target="_blank"
					rel="noopener noreferrer"
					class="group flex items-center justify-between border border-border px-6 py-4 transition-colors hover:bg-elevated"
				>
					<span class="text-xs uppercase tracking-[0.2em] text-muted group-hover:text-text">GitHub Repository</span>
					<span class="text-dim group-hover:text-text">&rarr;</span>
				</a>
				<a
					href="/api"
					class="group flex items-center justify-between border border-border px-6 py-4 transition-colors hover:bg-elevated"
				>
					<span class="text-xs uppercase tracking-[0.2em] text-muted group-hover:text-text">API Documentation</span>
					<span class="text-dim group-hover:text-text">&rarr;</span>
				</a>
			</div>
		</div>
	</section>
</div>
