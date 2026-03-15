<script lang="ts">
	import EventCard from '$lib/components/EventCard.svelte';
	import { fadeIn, staggerChildren } from '$lib/animations/gsap';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	let searchQuery = $state(data.q);
	let activeTab = $state<'events' | 'companies' | 'users'>('events');

	function handleSearch(e: SubmitEvent) {
		e.preventDefault();
		if (searchQuery.trim()) {
			window.location.href = `/search?q=${encodeURIComponent(searchQuery.trim())}`;
		}
	}
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
				<div class="grid gap-px border border-border sm:grid-cols-2 lg:grid-cols-3" use:staggerChildren={{ stagger: 0.06 }}>
					{#each data.results.events as event (event.id)}
						<EventCard {event} />
					{/each}
				</div>
			{:else}
				<p class="py-16 text-center text-xs text-dim">No events match.</p>
			{/if}
		{/if}

		{#if activeTab === 'companies'}
			{#if data.results.companies.length > 0}
				<div class="grid gap-px border border-border sm:grid-cols-2 lg:grid-cols-3" use:staggerChildren={{ stagger: 0.04 }}>
					{#each data.results.companies as company (company.id)}
						<a href="/companies/{company.id}" class="group p-6 transition-colors hover:bg-elevated">
							<h3 class="font-display text-2xl italic group-hover:text-white">{company.name}</h3>
							{#if company.description}
								<p class="mt-1 text-xs text-muted">{company.description}</p>
							{/if}
						</a>
					{/each}
				</div>
			{:else}
				<p class="py-16 text-center text-xs text-dim">No companies match.</p>
			{/if}
		{/if}

		{#if activeTab === 'users'}
			{#if data.results.users.length > 0}
				<div class="grid gap-px border border-border sm:grid-cols-2 lg:grid-cols-3" use:staggerChildren={{ stagger: 0.04 }}>
					{#each data.results.users as user (user.id)}
						<a href="/users/{user.id}" class="group p-6 transition-colors hover:bg-elevated">
							<h3 class="font-display text-2xl italic group-hover:text-white">{user.username}</h3>
							{#if user.bio}
								<p class="mt-1 text-xs text-muted">{user.bio}</p>
							{/if}
						</a>
					{/each}
				</div>
			{:else}
				<p class="py-16 text-center text-xs text-dim">No users match.</p>
			{/if}
		{/if}
	{:else if !data.q}
		<p class="py-16 text-center text-xs text-dim">Type something to search.</p>
	{/if}
</div>
