<script lang="ts">
	import EventCard from '$lib/components/EventCard.svelte';
	import { fadeIn, staggerChildren } from '$lib/animations/gsap';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>{data.company?.name ?? 'Company'} — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1400px] px-6 py-24">
	<a href="/companies" class="hover-line text-[10px] uppercase tracking-[0.3em] text-dim transition-colors hover:text-muted">&larr; Back</a>

	{#if data.company}
		<div class="mt-12" use:fadeIn>
			<h1 class="font-display text-6xl italic sm:text-7xl">{data.company.name}</h1>
			{#if data.company.description}
				<p class="mt-4 max-w-lg text-sm text-muted">{data.company.description}</p>
			{/if}
			{#if data.company.website}
				<a href={data.company.website} target="_blank" rel="noopener" class="hover-line mt-4 inline-block text-xs text-muted transition-colors hover:text-text">
					{data.company.website} &nearr;
				</a>
			{/if}
			<div class="mt-8 h-px w-24 bg-border"></div>
		</div>

		<div class="mt-16">
			<h2 class="mb-8 text-[10px] uppercase tracking-[0.3em] text-dim">Events ({data.totalEvents})</h2>
			{#if data.events.length > 0}
				<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3" use:staggerChildren={{ stagger: 0.06 }}>
					{#each data.events as event (event.id)}
						<EventCard {event} />
					{/each}
				</div>
			{:else}
				<p class="py-12 text-center text-xs text-dim">No events linked yet.</p>
			{/if}
		</div>
	{:else}
		<div class="mt-24 text-center">
			<p class="font-display text-3xl italic text-muted">Company not found</p>
			<p class="mt-4 text-xs text-dim">The backend may not be running.</p>
		</div>
	{/if}
</div>
