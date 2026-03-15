<script lang="ts">
	import ScoreBadge from '$lib/components/ScoreBadge.svelte';
	import { fadeIn, staggerChildren } from '$lib/animations/gsap';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	let searchQuery = $state(data.q || '');
	let activeTab = $state<'events' | 'companies' | 'users'>('events');

	function handleSearch(e: SubmitEvent) {
		e.preventDefault();
		if (searchQuery.trim()) {
			window.location.href = `/search?q=${encodeURIComponent(searchQuery.trim())}`;
		}
	}

	const totalResults = $derived(data.results?.total ?? 0);
</script>

<svelte:head>
	<title>{data.q ? `"${data.q}"` : 'Search'} — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[1400px] px-6 py-24">
	<div class="mb-16" use:fadeIn>
		<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Search</span>
		<h1 class="mt-2 font-display text-6xl italic sm:text-7xl">
			{#if data.q}&ldquo;{data.q}&rdquo;{:else}Find anything{/if}
		</h1>
		{#if data.q && data.results}
			<p class="mt-3 text-xs text-muted">{totalResults} result{totalResults !== 1 ? 's' : ''} for &ldquo;{data.q}&rdquo;</p>
		{/if}

		<form onsubmit={handleSearch} class="mt-8 max-w-lg">
			<div class="flex items-center border-b border-border transition-colors focus-within:border-text">
				<input
					bind:value={searchQuery}
					type="text"
					placeholder="Search hackathons, companies, people..."
					class="w-full bg-transparent py-3 text-sm text-text placeholder:text-dim focus:outline-none"
				/>
			</div>
		</form>
	</div>

	{#if data.results}
		<!-- Tabs -->
		<div class="mb-10 flex gap-8 border-b border-border" use:fadeIn>
			{#each [
				{ key: 'events', label: 'Events', count: data.results.events.length },
				{ key: 'companies', label: 'Companies', count: data.results.companies.length },
				{ key: 'users', label: 'Users', count: data.results.users.length },
			] as tab}
				<button
					class="border-b-2 pb-3 text-xs uppercase tracking-[0.2em] transition-colors {activeTab === tab.key ? 'border-text text-text' : 'border-transparent text-dim hover:text-muted'}"
					onclick={() => (activeTab = tab.key as typeof activeTab)}
				>
					{tab.label} ({tab.count})
				</button>
			{/each}
		</div>

		{#if activeTab === 'events'}
			{#if data.results.events.length > 0}
				<div class="space-y-0 divide-y divide-border" use:staggerChildren={{ stagger: 0.06 }}>
					{#each data.results.events as result (result.id)}
						<a href="/events/{result.id}" class="group flex items-stretch transition-colors hover:bg-elevated">
							<div class="flex flex-col items-center justify-center p-4 sm:p-6">
								<span class="text-[9px] uppercase tracking-[0.2em] text-dim mb-1">Quality</span>
								<ScoreBadge score={result.avg_rating} size="md" />
								<span class="mt-1 text-[9px] text-dim">{result.review_count} review{result.review_count !== 1 ? 's' : ''}</span>
							</div>
							<div class="flex flex-1 flex-col justify-center border-l border-border p-4 sm:p-6">
								<h3 class="font-display text-2xl italic transition-colors group-hover:text-white sm:text-3xl">{result.name}</h3>
								{#if result.would_return_pct !== null}
									<p class="mt-1 text-xs text-muted">{result.would_return_pct.toFixed(0)}% would attend again</p>
								{/if}
							</div>
						</a>
					{/each}
				</div>
			{:else}
				<p class="py-16 text-center text-xs text-dim">No events match.</p>
			{/if}
		{/if}

		{#if activeTab === 'companies'}
			{#if data.results.companies.length > 0}
				<div class="space-y-0 divide-y divide-border" use:staggerChildren={{ stagger: 0.04 }}>
					{#each data.results.companies as result (result.id)}
						<a href="/companies/{result.id}" class="group flex items-stretch transition-colors hover:bg-elevated">
							<div class="flex flex-col items-center justify-center p-4 sm:p-6">
								<span class="text-[9px] uppercase tracking-[0.2em] text-dim mb-1">Quality</span>
								<ScoreBadge score={result.avg_rating} size="md" />
								<span class="mt-1 text-[9px] text-dim">{result.review_count} review{result.review_count !== 1 ? 's' : ''}</span>
							</div>
							<div class="flex flex-1 flex-col justify-center border-l border-border p-4 sm:p-6">
								<h3 class="font-display text-2xl italic transition-colors group-hover:text-white sm:text-3xl">{result.name}</h3>
								{#if result.would_return_pct !== null}
									<p class="mt-1 text-xs text-muted">{result.would_return_pct.toFixed(0)}% would attend again</p>
								{/if}
							</div>
						</a>
					{/each}
				</div>
			{:else}
				<p class="py-16 text-center text-xs text-dim">No companies match.</p>
			{/if}
		{/if}

		{#if activeTab === 'users'}
			{#if data.results.users.length > 0}
				<div class="space-y-0 divide-y divide-border" use:staggerChildren={{ stagger: 0.04 }}>
					{#each data.results.users as user (user.id)}
						<a href="/users/{user.id}" class="group flex items-center p-4 transition-colors hover:bg-elevated sm:p-6">
							<h3 class="font-display text-2xl italic transition-colors group-hover:text-white">{user.name}</h3>
						</a>
					{/each}
				</div>
			{:else}
				<p class="py-16 text-center text-xs text-dim">No users match.</p>
			{/if}
		{/if}
	{:else if data.q}
		<p class="py-16 text-center text-xs text-dim">Search failed. The backend may not be running.</p>
	{:else}
		<p class="py-16 text-center text-xs text-dim">Type something to search.</p>
	{/if}
</div>
