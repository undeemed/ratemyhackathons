<script lang="ts">
	import { fadeIn } from '$lib/animations/gsap';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const joinDate = $derived(
		data.user ? new Date(data.user.created_at).toLocaleDateString('en-US', { month: 'long', year: 'numeric' }).toUpperCase() : ''
	);
</script>

<svelte:head>
	<title>{data.user?.username ?? 'User'} — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[900px] px-6 py-24">
	{#if data.user}
		<div use:fadeIn>
			<h1 class="font-display text-6xl italic sm:text-7xl">{data.user.username}</h1>
			{#if data.user.bio}
				<p class="mt-4 text-sm text-muted">{data.user.bio}</p>
			{/if}
			<p class="mt-4 text-[10px] uppercase tracking-[0.3em] text-dim">Joined {joinDate}</p>
			<div class="mt-8 h-px w-24 bg-border"></div>
		</div>
	{:else}
		<div class="mt-24 text-center">
			<p class="font-display text-3xl italic text-muted">User not found</p>
			<p class="mt-4 text-xs text-dim">The backend may not be running.</p>
		</div>
	{/if}
</div>
