<script lang="ts">
	import EventCard from '$lib/components/EventCard.svelte';
	import { fadeIn, staggerChildren } from '$lib/animations/gsap';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const totalPages = $derived(Math.ceil(data.total / data.perPage));
</script>

<svelte:head>
	<title>Events — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1400px] px-6 py-24">
	<div class="mb-16" use:fadeIn>
		<a href="/" class="mb-6 inline-block text-xs uppercase tracking-[0.2em] text-dim transition-colors hover:text-text">&larr; Back</a>
		<h1 class="mt-2 font-display text-6xl italic sm:text-7xl">All events</h1>
		<p class="mt-4 text-xs text-muted">{data.total} hackathons and counting</p>
		<div class="mt-6 h-px w-24 bg-border"></div>
	</div>

	<div class="grid gap-px border border-border sm:grid-cols-2 lg:grid-cols-3" use:staggerChildren={{ stagger: 0.06 }}>
		{#each data.events as event (event.id)}
			<EventCard {event} />
		{/each}
	</div>

	{#if data.events.length === 0}
		<div class="border border-border py-24 text-center">
			<p class="font-display text-2xl italic text-muted">Nothing here yet.</p>
		</div>
	{/if}

	{#if totalPages > 1}
		<nav class="mt-12 flex items-center justify-center gap-6" use:fadeIn>
			{#if data.page > 1}
				<a href="/events?page={data.page - 1}" class="text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text">&larr; Prev</a>
			{/if}
			<span class="text-[10px] tracking-[0.3em] text-dim">{data.page} / {totalPages}</span>
			{#if data.page < totalPages}
				<a href="/events?page={data.page + 1}" class="text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text">Next &rarr;</a>
			{/if}
		</nav>
	{/if}
</div>
