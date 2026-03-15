<script lang="ts">
	import { onMount } from 'svelte';
	import { gsap } from 'gsap';
	import { ScrollTrigger } from 'gsap/ScrollTrigger';

	interface Section {
		id: string;
		label: string;
		numeral: string;
	}

	let {
		sections = [] as Section[]
	}: {
		sections: Section[];
	} = $props();

	let activeIndex = $state(0);
	let progresses = $state<number[]>([]);
	let visible = $state(false);

	onMount(() => {
		gsap.registerPlugin(ScrollTrigger);
		progresses = sections.map(() => 0);

		// Show nav after scrolling past the hero
		ScrollTrigger.create({
			trigger: `#${sections[0]?.id}`,
			start: 'top 80%',
			onEnter: () => (visible = true),
			onLeaveBack: () => (visible = false),
		});

		// Track each section
		sections.forEach((section, i) => {
			ScrollTrigger.create({
				trigger: `#${section.id}`,
				start: 'top center',
				end: 'bottom center',
				onUpdate: (self) => {
					progresses[i] = self.progress;
				},
				onEnter: () => (activeIndex = i),
				onEnterBack: () => (activeIndex = i),
			});
		});

		return () => {
			ScrollTrigger.getAll().forEach((t) => t.kill());
		};
	});

	function scrollTo(id: string) {
		const el = document.getElementById(id);
		if (el) el.scrollIntoView({ behavior: 'smooth' });
	}
</script>

<nav
	class="fixed left-0 top-0 z-30 hidden h-screen w-[160px] flex-col justify-end pb-24 pl-6 transition-opacity duration-700 lg:flex {visible ? 'opacity-100' : 'opacity-0 pointer-events-none'}"
>
	<div class="flex flex-col gap-1">
		{#each sections as section, i}
			<button
				class="group flex items-center gap-2 py-1 text-left transition-colors"
				onclick={() => scrollTo(section.id)}
			>
				<span
					class="text-[11px] font-bold uppercase tracking-[0.15em] transition-colors duration-300
						{activeIndex === i ? 'text-text' : 'text-dim hover:text-muted'}"
				>
					{section.label}
				</span>
				<span class="flex-1">
					<span class="block h-px w-full overflow-hidden bg-border/30">
						<span
							class="block h-full transition-all duration-100"
							style="width: {activeIndex === i ? progresses[i] * 100 : 0}%; background: {activeIndex === i ? 'var(--color-gold)' : 'var(--color-border)'}"
						></span>
					</span>
				</span>
				<span
					class="text-[10px] transition-colors duration-300
						{activeIndex === i ? 'text-muted' : 'text-border'}"
				>
					{section.numeral}
				</span>
			</button>
		{/each}
	</div>

	<div class="mt-8 text-[9px] uppercase tracking-[0.3em] text-border">
		&copy; {new Date().getFullYear()}
	</div>
</nav>
