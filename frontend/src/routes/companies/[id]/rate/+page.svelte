<script lang="ts">
	import { Show, SignIn } from 'svelte-clerk';
	import RatingSlider from '$lib/components/RatingSlider.svelte';
	import { fadeIn } from '$lib/animations/gsap';
	import { createReview, createTag } from '$lib/api';
	import { RATING_CATEGORIES, CATEGORY_LABELS, type RatingCategory, type Tag } from '$lib/types';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const entity = $derived(data.company);

	let categoryRatings = $state<Record<string, number>>(
		Object.fromEntries(RATING_CATEGORIES.map((c) => [c, 0]))
	);
	let wouldReturn = $state<boolean | null>(null);
	let reviewBody = $state('');
	let reviewTitle = $state('');
	let selectedTagIds = $state<string[]>([]);
	let newTagName = $state('');
	let submitting = $state(false);
	let error = $state('');
	let success = $state(false);
	// svelte-ignore state_referenced_locally
	let availableTags = $state<Tag[]>([...(data.tags ?? [])]);

	const allRated = $derived(RATING_CATEGORIES.every((c) => categoryRatings[c] > 0));
	const bodyLength = $derived(reviewBody.length);
	const canSubmit = $derived(allRated && bodyLength >= 350 && bodyLength <= 5000 && !submitting);

	async function addTag() {
		if (!newTagName.trim()) return;
		const tag = await createTag(newTagName.trim());
		if (!availableTags.find((t) => t.id === tag.id)) {
			availableTags = [...availableTags, tag];
		}
		if (!selectedTagIds.includes(tag.id)) {
			selectedTagIds = [...selectedTagIds, tag.id];
		}
		newTagName = '';
	}

	function toggleTag(id: string) {
		if (selectedTagIds.includes(id)) {
			selectedTagIds = selectedTagIds.filter((t) => t !== id);
		} else {
			selectedTagIds = [...selectedTagIds, id];
		}
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!canSubmit || !entity) return;

		submitting = true;
		error = '';

		try {
			await createReview('me', {
				company_id: entity.id,
				title: reviewTitle || undefined,
				body: reviewBody,
				would_return: wouldReturn ?? undefined,
				category_ratings: categoryRatings,
				tag_ids: selectedTagIds.length > 0 ? selectedTagIds : undefined,
			});
			success = true;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to submit review';
		} finally {
			submitting = false;
		}
	}
</script>

<svelte:head>
	<title>Rate {entity?.name ?? 'Company'} — RateMyHackathons</title>
</svelte:head>

<div class="mx-auto max-w-[800px] px-6 py-24">
	<a href="/companies/{data.company?.id}" class="hover-line text-[10px] uppercase tracking-[0.3em] text-dim transition-colors hover:text-muted">&larr; Back</a>

	{#if !entity}
		<div class="mt-24 text-center">
			<p class="font-display text-3xl italic text-muted">Company not found</p>
		</div>
	{:else if success}
		<div class="mt-24 text-center" use:fadeIn>
			<p class="font-display text-4xl italic">Thank you!</p>
			<p class="mt-4 text-sm text-muted">Your review of {entity.name} has been submitted.</p>
			<a href="/companies/{entity.id}" class="mt-8 inline-block border border-text px-6 py-2 text-xs uppercase tracking-[0.2em] text-text transition-colors hover:bg-text hover:text-black">
				View Company
			</a>
		</div>
	{:else}
		<div class="mt-12" use:fadeIn>
			<h1 class="font-display text-4xl italic sm:text-5xl">Rate {entity.name}</h1>
			<p class="mt-2 text-xs text-muted">Share your experience to help others</p>
		</div>

		<Show when="signed-out">
			{#snippet children()}
				<div class="mt-12 border border-border p-8 text-center">
					<p class="mb-6 text-sm text-muted">You need to sign in to write a review.</p>
					<SignIn />
				</div>
			{/snippet}
		</Show>

		<Show when="signed-in">
			{#snippet children()}
				<form onsubmit={handleSubmit} class="mt-12 space-y-10">
					<!-- Category Ratings -->
					<div>
						<h2 class="mb-6 text-[10px] uppercase tracking-[0.3em] text-dim">Rate each category (1-5)</h2>
						<div class="grid gap-6 sm:grid-cols-2">
							{#each RATING_CATEGORIES as cat}
								<RatingSlider
									bind:value={categoryRatings[cat]}
									label={CATEGORY_LABELS[cat as RatingCategory]}
								/>
							{/each}
						</div>
					</div>

					<!-- Would Return -->
					<div>
						<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">Would you attend again?</h2>
						<div class="flex gap-3">
							<button
								type="button"
								class="border px-6 py-2 text-xs uppercase tracking-[0.2em] transition-colors
									{wouldReturn === true ? 'border-score-green bg-score-green text-black' : 'border-border text-dim hover:border-muted'}"
								onclick={() => { wouldReturn = wouldReturn === true ? null : true; }}
							>
								Yes
							</button>
							<button
								type="button"
								class="border px-6 py-2 text-xs uppercase tracking-[0.2em] transition-colors
									{wouldReturn === false ? 'border-score-red bg-score-red text-black' : 'border-border text-dim hover:border-muted'}"
								onclick={() => { wouldReturn = wouldReturn === false ? null : false; }}
							>
								No
							</button>
						</div>
					</div>

					<!-- Tags -->
					<div>
						<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">Tags (optional)</h2>
						<div class="flex flex-wrap gap-2 mb-3">
							{#each availableTags as tag}
								<button
									type="button"
									class="border px-3 py-1 text-[10px] uppercase tracking-[0.15em] transition-colors
										{selectedTagIds.includes(tag.id) ? 'border-text text-text bg-elevated' : 'border-border text-dim hover:border-muted'}"
									onclick={() => toggleTag(tag.id)}
								>
									{tag.name}
								</button>
							{/each}
						</div>
						<div class="flex gap-2">
							<input
								bind:value={newTagName}
								type="text"
								placeholder="Suggest a new tag..."
								class="flex-1 border-b border-border bg-transparent py-2 text-xs text-text placeholder:text-dim focus:border-text focus:outline-none"
							/>
							<button
								type="button"
								class="text-xs text-muted hover:text-text"
								onclick={addTag}
							>
								Add
							</button>
						</div>
					</div>

					<!-- Title -->
					<div>
						<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">Title (optional)</h2>
						<input
							bind:value={reviewTitle}
							type="text"
							placeholder="Summarize your experience..."
							maxlength={200}
							class="w-full border-b border-border bg-transparent py-2 text-sm text-text placeholder:text-dim focus:border-text focus:outline-none"
						/>
					</div>

					<!-- Review Body -->
					<div>
						<h2 class="mb-4 text-[10px] uppercase tracking-[0.3em] text-dim">Your Review</h2>
						<textarea
							bind:value={reviewBody}
							placeholder="Write your detailed review (minimum 350 characters)..."
							rows={8}
							maxlength={5000}
							class="w-full border border-border bg-surface p-4 text-sm text-text placeholder:text-dim focus:border-text focus:outline-none resize-none"
						></textarea>
						<div class="mt-1 flex justify-between text-[10px] text-dim">
							<span class={bodyLength < 350 ? 'text-score-red' : 'text-score-green'}>{bodyLength} / 350 min</span>
							<span>{bodyLength} / 5000</span>
						</div>
					</div>

					{#if error}
						<div class="border border-score-red p-4 text-xs text-score-red">{error}</div>
					{/if}

					<button
						type="submit"
						disabled={!canSubmit}
						class="w-full border border-text py-3 text-xs uppercase tracking-[0.2em] transition-colors
							{canSubmit ? 'text-text hover:bg-text hover:text-black' : 'border-border text-dim cursor-not-allowed'}"
					>
						{#if submitting}Submitting...{:else}Submit Review{/if}
					</button>
				</form>
			{/snippet}
		</Show>
	{/if}
</div>
