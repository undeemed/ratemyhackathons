<script lang="ts">
	import { X, MapPin, Locate } from 'lucide-svelte';
	import { Show, UserButton } from 'svelte-clerk';
	import { locationStore } from '$lib/stores/location.svelte';
	import { getUniqueLocations } from '$lib/api';

	let mobileOpen = $state(false);
	let searchQuery = $state('');
	let locOpen = $state(false);
	let locInput = $state('');
	let detecting = $state(false);
	let allLocations = $state<string[]>([]);

	const suggestions = $derived.by(() => {
		const q = locInput.trim().toLowerCase();
		if (!q) return allLocations.slice(0, 8);
		return allLocations.filter((l) => l.toLowerCase().includes(q)).slice(0, 8);
	});

	async function loadLocations() {
		if (allLocations.length > 0) return;
		try {
			allLocations = await getUniqueLocations();
		} catch {
			/* ignore — autocomplete is best-effort */
		}
	}

	function handleSearch(e: SubmitEvent) {
		e.preventDefault();
		if (searchQuery.trim()) {
			window.location.href = `/search?q=${encodeURIComponent(searchQuery.trim())}`;
		}
	}

	function selectLocation(label: string) {
		locationStore.set({ label });
		locInput = '';
		locOpen = false;
	}

	function setManualLocation() {
		const val = locInput.trim();
		if (val) {
			selectLocation(val);
		}
	}

	async function handleAutoDetect() {
		detecting = true;
		await locationStore.autoDetect();
		detecting = false;
		locOpen = false;
	}

	function clearLocation() {
		locationStore.clear();
		locOpen = false;
	}

	function openLocDropdown() {
		locOpen = !locOpen;
		if (locOpen) loadLocations();
	}
</script>

<nav class="fixed top-0 z-40 w-full border-b border-border bg-bg/90 backdrop-blur-sm">
	<div class="mx-auto flex h-14 max-w-[1400px] items-center justify-between px-6">
		<a href="/" class="font-display text-xl italic tracking-tight">RMH</a>

		<div class="hidden items-center gap-6 md:flex">
			<!-- Location filter -->
			<div class="relative">
				<button
					type="button"
					onclick={openLocDropdown}
					class="flex items-center gap-1.5 text-xs uppercase tracking-[0.2em] transition-colors {locationStore.value
						? 'text-text'
						: 'text-muted hover:text-text'}"
				>
					<MapPin class="h-3.5 w-3.5" />
					<span class="max-w-[140px] truncate">{locationStore.value?.label ?? 'All Locations'}</span>
				</button>

				{#if locOpen}
					<!-- svelte-ignore a11y_no_static_element_interactions -->
					<div
						class="absolute left-0 top-full mt-3 w-72 border border-border bg-bg p-4 shadow-lg"
					>
						<!-- Auto-detect -->
						<button
							type="button"
							onclick={handleAutoDetect}
							disabled={detecting}
							class="mb-3 flex w-full items-center gap-2 border border-border px-3 py-2 text-xs uppercase tracking-[0.15em] text-muted transition-colors hover:border-text hover:text-text disabled:opacity-50"
						>
							<Locate class="h-3.5 w-3.5" />
							{detecting ? 'Detecting...' : 'Auto-detect location'}
						</button>

						<!-- Manual input with autocomplete -->
						<form
							onsubmit={(e) => {
								e.preventDefault();
								setManualLocation();
							}}
							class="relative"
						>
							<input
								type="text"
								bind:value={locInput}
								oninput={loadLocations}
								placeholder="City, state, or country..."
								class="w-full border border-border bg-surface px-3 py-2 text-xs text-text placeholder:text-dim focus:border-text focus:outline-none"
							/>
						</form>

						<!-- Suggestions dropdown -->
						{#if suggestions.length > 0}
							<div class="mt-1 max-h-48 overflow-y-auto border border-border bg-surface">
								{#each suggestions as loc}
									<button
										type="button"
										onclick={() => selectLocation(loc)}
										class="block w-full px-3 py-2 text-left text-xs text-muted transition-colors hover:bg-elevated hover:text-text"
									>
										{loc}
									</button>
								{/each}
							</div>
						{/if}

						<!-- Clear -->
						{#if locationStore.value}
							<button
								type="button"
								onclick={clearLocation}
								class="mt-3 w-full text-left text-[10px] uppercase tracking-[0.2em] text-dim transition-colors hover:text-text"
							>
								Clear filter
							</button>
						{/if}
					</div>
				{/if}
			</div>

			<a
				href="/events"
				class="hover-line text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text"
				>Events</a
			>
			<a
				href="/companies"
				class="hover-line text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text"
				>Companies</a
			>
			<a
				href="/compare"
				class="hover-line text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text"
				>Compare</a
			>
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
					<a
						href="/sign-in"
						class="text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text"
						>Sign In</a
					>
				{/snippet}
			</Show>
			<Show when="signed-in">
				{#snippet children()}
					<UserButton />
				{/snippet}
			</Show>
		</div>

		<button class="text-muted md:hidden" onclick={() => (mobileOpen = !mobileOpen)}>
			{#if mobileOpen}<X class="h-5 w-5" />{:else}<span
					class="text-xs uppercase tracking-[0.2em]">Menu</span
				>{/if}
		</button>
	</div>

	{#if mobileOpen}
		<div class="border-t border-border bg-bg px-6 py-8 md:hidden">
			<div class="flex flex-col gap-6">
				<!-- Mobile location filter -->
				<div class="flex flex-col gap-3">
					<span class="text-[10px] uppercase tracking-[0.3em] text-dim">Location</span>
					<button
						type="button"
						onclick={handleAutoDetect}
						disabled={detecting}
						class="flex items-center gap-2 text-sm uppercase tracking-[0.2em] text-muted"
					>
						<Locate class="h-4 w-4" />
						{detecting ? 'Detecting...' : 'Auto-detect'}
					</button>
					<form
						onsubmit={(e) => {
							e.preventDefault();
							setManualLocation();
						}}
						class="relative"
					>
						<input
							type="text"
							bind:value={locInput}
							oninput={loadLocations}
							placeholder="City, state, or country..."
							class="w-full border-b border-border bg-transparent pb-2 text-sm text-text placeholder:text-dim focus:border-text focus:outline-none"
						/>
					</form>
					<!-- Mobile suggestions -->
					{#if locInput.trim() && suggestions.length > 0}
						<div class="max-h-40 overflow-y-auto border border-border bg-surface">
							{#each suggestions as loc}
								<button
									type="button"
									onclick={() => selectLocation(loc)}
									class="block w-full px-3 py-2 text-left text-sm text-muted transition-colors hover:bg-elevated hover:text-text"
								>
									{loc}
								</button>
							{/each}
						</div>
					{/if}
					{#if locationStore.value}
						<div class="flex items-center gap-2">
							<MapPin class="h-3.5 w-3.5 text-text" />
							<span class="text-sm text-text">{locationStore.value.label}</span>
							<button
								type="button"
								onclick={clearLocation}
								class="text-xs text-dim hover:text-text"
							>
								×
							</button>
						</div>
					{/if}
				</div>

				<a href="/events" class="text-sm uppercase tracking-[0.2em] text-muted">Events</a>
				<a href="/companies" class="text-sm uppercase tracking-[0.2em] text-muted"
					>Companies</a
				>
				<a href="/compare" class="text-sm uppercase tracking-[0.2em] text-muted"
					>Compare</a
				>
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
						<a href="/sign-in" class="text-sm uppercase tracking-[0.2em] text-muted"
							>Sign In</a
						>
						<a href="/sign-up" class="text-sm uppercase tracking-[0.2em] text-muted"
							>Sign Up</a
						>
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

<!-- Close dropdown on outside click -->
{#if locOpen}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="fixed inset-0 z-30" onclick={() => (locOpen = false)}></div>
{/if}
