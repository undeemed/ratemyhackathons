<script lang="ts">
	import { X } from 'lucide-svelte';
	import { Show, UserButton } from 'svelte-clerk';

	let mobileOpen = $state(false);
	let searchQuery = $state('');

	function handleSearch(e: SubmitEvent) {
		e.preventDefault();
		if (searchQuery.trim()) {
			window.location.href = `/search?q=${encodeURIComponent(searchQuery.trim())}`;
		}
	}
</script>

<nav class="fixed top-0 z-40 w-full border-b border-border bg-bg/90 backdrop-blur-sm">
	<div class="mx-auto flex h-14 max-w-[1400px] items-center justify-between px-6">
		<a href="/" class="font-display text-xl italic tracking-tight">RMH</a>

		<div class="hidden items-center gap-8 md:flex">
			<a href="/events" class="hover-line text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text">Events</a>
			<a href="/companies" class="hover-line text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text">Companies</a>
			<a href="/compare" class="hover-line text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text">Compare</a>
			<form onsubmit={handleSearch} class="relative">
				<input
					bind:value={searchQuery}
					type="text"
					placeholder="Search"
					class="h-8 w-40 border-b border-border bg-transparent text-xs uppercase tracking-widest text-text placeholder:text-dim focus:border-text focus:outline-none"
				/>
			</form>
			<Show when="signed-out">
				{#snippet children()}
					<a href="/sign-in" class="text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text">Sign In</a>
				{/snippet}
			</Show>
			<Show when="signed-in">
				{#snippet children()}
					<UserButton />
				{/snippet}
			</Show>
		</div>

		<button class="text-muted md:hidden" onclick={() => (mobileOpen = !mobileOpen)}>
			{#if mobileOpen}<X class="h-5 w-5" />{:else}<span class="text-xs uppercase tracking-[0.2em]">Menu</span>{/if}
		</button>
	</div>

	{#if mobileOpen}
		<div class="border-t border-border bg-bg px-6 py-8 md:hidden">
			<div class="flex flex-col gap-6">
				<a href="/events" class="text-sm uppercase tracking-[0.2em] text-muted">Events</a>
				<a href="/companies" class="text-sm uppercase tracking-[0.2em] text-muted">Companies</a>
				<a href="/compare" class="text-sm uppercase tracking-[0.2em] text-muted">Compare</a>
				<form onsubmit={handleSearch}>
					<input
						bind:value={searchQuery}
						type="text"
						placeholder="Search"
						class="w-full border-b border-border bg-transparent pb-2 text-sm text-text placeholder:text-dim focus:border-text focus:outline-none"
					/>
				</form>
				<Show when="signed-out">
					{#snippet children()}
						<a href="/sign-in" class="text-sm uppercase tracking-[0.2em] text-muted">Sign In</a>
						<a href="/sign-up" class="text-sm uppercase tracking-[0.2em] text-muted">Sign Up</a>
					{/snippet}
				</Show>
				<Show when="signed-in">
					{#snippet children()}
						<UserButton />
					{/snippet}
				</Show>
			</div>
		</div>
	{/if}
</nav>
