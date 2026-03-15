<script lang="ts">
  import Globe from "$lib/components/Globe.svelte";
  import EventCard from "$lib/components/EventCard.svelte";
  import {
    fadeIn,
    slideUp,
    staggerChildren,
    countUp,
  } from "$lib/animations/gsap";
  import { ArrowRight } from "lucide-svelte";
  import { onMount } from "svelte";
  import gsap from "gsap";
  import { ScrollTrigger } from "gsap/ScrollTrigger";
  import { beforeNavigate } from "$app/navigation";
  import type { PageData } from "./$types";
  import type { EventSummary } from "$lib/types";

  let { data }: { data: PageData } = $props();

  let heroSearchQuery = $state("");
  let searchMode = $state<"events" | "companies">("events");
  let globeVisible = $state(true);

  // Sample markers to ~200 for GPU perf (1600+ is too many for cobe)
  const MAX_GLOBE_MARKERS = 200;
  const globeMarkers = $derived(
    data.markers.length <= MAX_GLOBE_MARKERS
      ? data.markers
      : data.markers.filter((_, i) => i % Math.ceil(data.markers.length / MAX_GLOBE_MARKERS) === 0)
  );

  function handleHeroSearch(e: SubmitEvent) {
    e.preventDefault();
    if (heroSearchQuery.trim()) {
      const type = searchMode === "companies" ? "&type=company" : "";
      window.location.href = `/search?q=${encodeURIComponent(heroSearchQuery.trim())}${type}`;
    }
  }

  const demoEvents: EventSummary[] = [
    {
      id: "1",
      name: "TreeHacks",
      description:
        "Stanford's flagship hackathon — 1,600 hackers, 36 hours, $250K+ in prizes",
      location: "Stanford, CA",
      url: null,
      start_date: "2026-02-14",
      end_date: "2026-02-16",
      image_url: null,
      latitude: 37.43,
      longitude: -122.17,
      companies: [{ id: "c1", name: "Google", role: "sponsor" }],
      avg_rating: 4.7,
      review_count: 89,
      created_at: "",
    },
    {
      id: "2",
      name: "HackMIT",
      description: "MIT's annual hackathon bringing together 1,000+ students",
      location: "Cambridge, MA",
      url: null,
      start_date: "2026-10-01",
      end_date: "2026-10-02",
      image_url: null,
      latitude: 42.36,
      longitude: -71.09,
      companies: [{ id: "c2", name: "Microsoft", role: "sponsor" }],
      avg_rating: 4.5,
      review_count: 124,
      created_at: "",
    },
    {
      id: "3",
      name: "ETHGlobal London",
      description: "Ethereum's premier web3 hackathon in the heart of London",
      location: "London, UK",
      url: null,
      start_date: "2026-03-28",
      end_date: "2026-03-30",
      image_url: null,
      latitude: 51.51,
      longitude: -0.13,
      companies: [{ id: "c3", name: "Ethereum Foundation", role: "organizer" }],
      avg_rating: 4.3,
      review_count: 67,
      created_at: "",
    },
    {
      id: "4",
      name: "CalHacks",
      description: "UC Berkeley's largest collegiate hackathon",
      location: "Berkeley, CA",
      url: null,
      start_date: "2026-06-20",
      end_date: "2026-06-22",
      image_url: null,
      latitude: 37.87,
      longitude: -122.26,
      companies: [{ id: "c4", name: "Meta", role: "sponsor" }],
      avg_rating: 4.1,
      review_count: 56,
      created_at: "",
    },
    {
      id: "5",
      name: "Hack the North",
      description:
        "Canada's biggest hackathon — 3,000+ hackers at the University of Waterloo",
      location: "Waterloo, ON",
      url: null,
      start_date: "2026-09-13",
      end_date: "2026-09-15",
      image_url: null,
      latitude: 43.47,
      longitude: -80.54,
      companies: [{ id: "c5", name: "Shopify", role: "sponsor" }],
      avg_rating: 4.8,
      review_count: 201,
      created_at: "",
    },
    {
      id: "6",
      name: "Junction",
      description:
        "Europe's leading hackathon gathering 1,500 hackers in Helsinki",
      location: "Helsinki, Finland",
      url: null,
      start_date: "2026-11-07",
      end_date: "2026-11-09",
      image_url: null,
      latitude: 60.17,
      longitude: 24.94,
      companies: [{ id: "c6", name: "Nokia", role: "sponsor" }],
      avg_rating: 4.6,
      review_count: 78,
      created_at: "",
    },
  ];

  const events = $derived(data.events.length > 0 ? data.events : demoEvents);
  const eventCount = $derived(data.totalEvents || 10380);

  // ── Single Globe: hero → showcase morph ──
  let sectionEl: HTMLElement;
  let globeContainerEl: HTMLElement;
  let heroTextEl: HTMLElement;
  let showcaseCardEls: HTMLElement[] = [];
  const globeFocus = { lat: 0, lng: 0 };

  const showcaseEvents = $derived(
    events.filter((e) => e.latitude != null && e.longitude != null).slice(0, 5),
  );

  function fmtDate(d: string | null) {
    if (!d) return "";
    return new Date(d).toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  }

  // GSAP context scopes all animations — ctx.revert() cleans up everything
  // on unmount without the expensive killTweensOf("*") nuclear option.
  let ctx: ReturnType<typeof gsap.context> | undefined;

  // Kill ScrollTrigger BEFORE SvelteKit swaps pages — otherwise the pin
  // spacer (~5000px) inflates the new page's document height, pushing
  // its content below a huge blank gap.
  beforeNavigate(() => {
    ctx?.revert();
    ctx = undefined;
  });

  onMount(() => {
    gsap.registerPlugin(ScrollTrigger);

    ctx = gsap.context(() => {
      const vw = window.innerWidth;
      const vh = window.innerHeight;

      // Hero: large, shifted right + up
      const heroW = Math.min(vw >= 1024 ? vw * 0.9 : vw * 1.2, 1440);
      const heroL = vw * 1.05 - heroW;

      // Showcase: visually smaller + centered (achieved via transforms, not layout)
      const maxShowSize = Math.min(vh * 0.9, vw * 0.9);
      const morphScale = maxShowSize / heroW; // shrink to fit viewport
      const heroCenterX = heroL + heroW / 2;
      const morphX = vw / 2 - heroCenterX; // translate to viewport center
      const heroCenterY = vh * 0.45; // matches top: '45%' with yPercent: -50
      const morphY = vh / 2 - heroCenterY; // translate to vertical center

      // Initial hero position — fixed layout, never changes during scroll.
      // All scroll animations use transforms only (x/scale) so offsetWidth
      // reads in Globe.svelte stay cheap (no forced layout reflow).
      gsap.set(globeContainerEl, {
        width: heroW,
        height: heroW,
        left: heroL,
        top: "45%",
        yPercent: -50,
      });

      if (showcaseEvents.length === 0) return;

      // Hide all event cards
      showcaseCardEls.forEach((el) => {
        if (el) gsap.set(el, { autoAlpha: 0, x: -60 });
      });

      const tl = gsap.timeline({
        scrollTrigger: {
          trigger: sectionEl,
          pin: true,
          start: "top top",
          end: "+=5000",
          scrub: 2,
          onLeave: () => { globeVisible = false; },
          onEnterBack: () => { globeVisible = true; },
        },
      });

      // Phase 1: Hold hero (globe auto-spins with dots visible)
      tl.addLabel("hero");
      tl.to({}, { duration: 0.15 });

      // Phase 2: Fade hero text, morph globe to center + zoom in
      // Uses ONLY transform properties (x, scale) — no layout changes
      tl.addLabel("morph");
      tl.to(heroTextEl, { autoAlpha: 0, y: -60, duration: 0.15 });
      tl.to(
        globeContainerEl,
        {
          x: morphX,
          y: morphY,
          scale: morphScale,
          duration: 0.2,
          ease: "power2.inOut",
        },
        "<",
      );

      // Phase 3: Cycle events — spin globe to each location
      showcaseEvents.forEach((event, i) => {
        const card = showcaseCardEls[i];
        if (!card) return;

        tl.addLabel(`event-${i}`);

        // Spin globe to this event's lat/lng
        tl.to(globeFocus, {
          lat: event.latitude!,
          lng: event.longitude!,
          duration: 0.3,
          ease: "power2.inOut",
        });

        // Event card slides in
        tl.fromTo(
          card,
          { autoAlpha: 0, x: -60 },
          { autoAlpha: 1, x: 0, duration: 0.15 },
          "-=0.1",
        );

        // Hold on this event
        tl.to({}, { duration: 0.35 });

        // Card exits
        tl.to(card, { autoAlpha: 0, x: 60, duration: 0.15 });
      });

      // Phase 4: Exit — fade out
      tl.addLabel("exit");
      tl.to(globeContainerEl, {
        opacity: 0,
        scale: 0.85,
        duration: 0.2,
      });

      tl.addLabel("end");
    });

    // Fallback cleanup if component unmounts without navigation (e.g. HMR)
    return () => {
      ctx?.revert();
      ctx = undefined;
    };
  });
</script>

<svelte:head>
  <title>RateMyHackathons</title>
  <meta
    name="description"
    content="The internet's honest record of hackathon experiences."
  />
</svelte:head>

<!-- ═══════ HERO + GLOBE SHOWCASE (single pinned section) ═══════ -->
<section bind:this={sectionEl} class="relative h-screen overflow-hidden">
  <!-- Globe container — ONE instance, GSAP morphs position/size -->
  <div bind:this={globeContainerEl} class="absolute aspect-square">
    <Globe markers={globeMarkers} focus={globeFocus} visible={globeVisible} />
  </div>

  <!-- Hero text — fades out during morph -->
  <div
    bind:this={heroTextEl}
    class="relative z-10 mx-auto flex h-screen w-full max-w-[1400px] items-center px-6"
  >
    <div class="max-w-5xl">
      <h1
        class="font-display text-[clamp(3.5rem,11vw,10rem)] italic leading-[0.85] tracking-tight"
      >
        Every hackathon,<br />
        <span
          style="background-image: linear-gradient(to right, var(--color-score-red), var(--color-score-yellow), var(--color-score-green)); -webkit-background-clip: text; background-clip: text; color: transparent;"
          >rated.</span
        >
      </h1>

      <div class="mt-10 flex flex-col gap-8 sm:flex-row sm:items-start sm:gap-12">
        <div class="shrink-0 sm:max-w-xs">
          <div class="mb-4 h-px w-24 bg-dim"></div>
          <p class="text-sm leading-relaxed text-muted">
            No sponsored placements. No corporate filters. Just thousands of honest
            reviews from people who were actually there.
          </p>
        </div>

        <form onsubmit={handleHeroSearch} class="flex-1 sm:max-w-lg">
          <div class="mb-4 flex gap-4 text-xs uppercase tracking-[0.2em]">
            <button
              type="button"
              class="transition-colors {searchMode === 'events'
                ? 'text-text'
                : 'text-dim hover:text-muted'}"
              onclick={() => (searchMode = "events")}>Hackathons</button
            >
            <button
              type="button"
              class="transition-colors {searchMode === 'companies'
                ? 'text-text'
                : 'text-dim hover:text-muted'}"
              onclick={() => (searchMode = "companies")}>Companies</button
            >
          </div>
          <div
            class="group flex items-center border-b-2 border-dim transition-colors focus-within:border-text"
          >
            <input
              bind:value={heroSearchQuery}
              type="text"
              placeholder={searchMode === "events"
                ? "Search hackathons..."
                : "Search companies..."}
              class="w-full bg-transparent py-4 text-lg text-text placeholder:text-dim focus:outline-none"
            />
            <button
              type="submit"
              class="text-dim transition-colors group-focus-within:text-text"
            >
              <ArrowRight class="h-6 w-6" />
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>

  <!-- Event cards — appear during showcase phase -->
  {#each showcaseEvents as event, i}
    <div
      bind:this={showcaseCardEls[i]}
      class="invisible absolute left-8 top-1/2 z-20 -translate-y-1/2 border border-border bg-surface/90 px-8 py-6 backdrop-blur-sm lg:left-20"
      style="max-width: 380px;"
    >
      <span class="text-[10px] uppercase tracking-[0.3em] text-dim">
        {String(i + 1).padStart(2, "0")} / {String(
          showcaseEvents.length,
        ).padStart(2, "0")}
      </span>
      <h3 class="mt-3 font-display text-3xl italic leading-tight lg:text-4xl">
        {event.name}
      </h3>
      {#if event.location}
        <p class="mt-2 text-sm text-muted">{event.location}</p>
      {/if}
      {#if event.start_date}
        <p class="mt-1 text-xs uppercase tracking-wider text-dim">
          {fmtDate(event.start_date)}
        </p>
      {/if}
      {#if event.description}
        <p class="mt-3 line-clamp-2 text-xs leading-relaxed text-muted">
          {event.description}
        </p>
      {/if}
      <a
        href="/events/{event.id}"
        class="mt-4 inline-block text-xs uppercase tracking-[0.2em] text-text transition-colors hover:text-accent"
      >
        View details &rarr;
      </a>
    </div>
  {/each}
</section>

<!-- ═══════ MARQUEE TICKER ═══════ -->
<div class="overflow-hidden border-y border-border py-4">
  <div class="animate-marquee flex whitespace-nowrap">
    {#each [...Array(2)] as _}
      {#each ["HackMIT", "TreeHacks", "PennApps", "CalHacks", "HackGT", "MHacks", "HackNY", "SFHacks", "ETHGlobal", "Junction", "HackZurich", "AngelHack", "DeveloperWeek", "Launch Hack", "Hack the North", "YC Hacks"] as name}
        <span class="mx-8 text-[11px] uppercase tracking-[0.3em] text-dim"
          >{name}</span
        >
        <span class="mx-2 text-border">&bull;</span>
      {/each}
    {/each}
  </div>
</div>

<!-- ═══════ STATS ═══════ -->
<section class="py-24">
  <div class="mx-auto max-w-[1400px] px-6">
    <div
      class="grid grid-cols-2 border border-border md:grid-cols-4"
      use:staggerChildren={{ stagger: 0.15, y: 20 }}
    >
      <div
        class="border-b border-border p-8 text-center md:border-b-0 md:border-r"
      >
        <div
          class="font-display text-5xl italic sm:text-6xl lg:text-7xl"
          use:countUp={{ target: eventCount }}
        >
          0
        </div>
        <p class="mt-2 text-[11px] uppercase tracking-[0.3em] text-muted">
          Events tracked
        </p>
      </div>
      <div
        class="border-b border-border p-8 text-center md:border-b-0 md:border-r"
      >
        <div
          class="font-display text-5xl italic sm:text-6xl lg:text-7xl"
          use:countUp={{ target: 45 }}
        >
          0
        </div>
        <p class="mt-2 text-[11px] uppercase tracking-[0.3em] text-muted">
          Cities
        </p>
      </div>
      <div class="p-8 text-center md:border-r border-border">
        <div
          class="font-display text-5xl italic sm:text-6xl lg:text-7xl"
          use:countUp={{ target: 30 }}
        >
          0
        </div>
        <p class="mt-2 text-[11px] uppercase tracking-[0.3em] text-muted">
          Companies
        </p>
      </div>
      <div class="p-8 text-center">
        <div
          class="font-display text-5xl italic sm:text-6xl lg:text-7xl"
          use:countUp={{ target: 4 }}
        >
          0
        </div>
        <p class="mt-2 text-[11px] uppercase tracking-[0.3em] text-muted">
          Sources
        </p>
      </div>
    </div>
  </div>
</section>

<!-- ═══════ FEATURED EVENTS ═══════ -->
<section class="border-t border-border bg-surface py-24">
  <div class="mx-auto max-w-[1400px] px-6">
    <div class="mb-14 flex items-end justify-between" use:fadeIn>
      <div>
        <span class="text-[11px] uppercase tracking-[0.3em] text-dim"
          >Selection</span
        >
        <h2 class="mt-2 font-display text-5xl italic sm:text-6xl">
          Recent events
        </h2>
      </div>
      <a
        href="/events"
        class="hover-line hidden text-xs uppercase tracking-[0.2em] text-muted transition-colors hover:text-text sm:block"
      >
        View all &rarr;
      </a>
    </div>

    <div
      class="grid gap-4 sm:grid-cols-2 md:grid-cols-3 will-change-transform"
      style="contain: layout style;"
      use:staggerChildren={{ stagger: 0.06 }}
    >
      {#each events.slice(0, 6) as event (event.id)}
        <EventCard {event} />
      {/each}
    </div>

    <div class="mt-10 text-center sm:hidden">
      <a href="/events" class="text-xs uppercase tracking-[0.2em] text-muted"
        >View all events &rarr;</a
      >
    </div>
  </div>
</section>

<!-- ═══════ HOW IT WORKS ═══════ -->
<section class="py-24">
  <div class="mx-auto max-w-[1400px] px-6">
    <div class="mb-14" use:fadeIn>
      <span class="text-[11px] uppercase tracking-[0.3em] text-dim"
        >Process</span
      >
      <h2 class="mt-2 font-display text-5xl italic sm:text-6xl">
        How it works
      </h2>
    </div>

    <div
      class="grid gap-4 md:grid-cols-3"
      use:staggerChildren={{ stagger: 0.15 }}
    >
      {#each [{ num: "01", title: "Discover", text: "Browse hackathons worldwide. Filter by city, date, company, or rating. See them plotted on an interactive globe." }, { num: "02", title: "Experience", text: "Read honest reviews before you commit. Know the vibe, the prizes, the food, the WiFi — from people who were actually there." }, { num: "03", title: "Rate", text: "Been there? Share what it was really like. No PR spin, no sponsored takes. Just your honest experience for the next hacker." }] as step}
        <div
          class="group border border-border p-10 transition-all duration-500 hover:border-accent hover:bg-surface"
        >
          <span
            class="font-display text-7xl italic text-border transition-colors duration-500 group-hover:text-accent lg:text-8xl"
            >{step.num}</span
          >
          <h3 class="mt-6 text-sm font-bold uppercase tracking-[0.2em]">
            {step.title}
          </h3>
          <p class="mt-4 text-sm leading-relaxed text-muted">{step.text}</p>
        </div>
      {/each}
    </div>
  </div>
</section>

<!-- ═══════ PULL QUOTE ═══════ -->
<section class="bg-[#111] py-28">
  <div class="mx-auto max-w-[1000px] px-6 text-center" use:slideUp>
    <span class="font-display text-8xl italic text-border">&ldquo;</span>
    <p
      class="font-display text-3xl italic leading-snug sm:text-4xl lg:text-5xl"
    >
      The only hackathon directory that cares about what actually happened, not
      what the sponsors say happened.
    </p>
    <div class="mx-auto mt-8 h-px w-24 bg-border"></div>
  </div>
</section>

<!-- ═══════ CTA ═══════ -->
<section class="py-28">
  <div class="mx-auto max-w-[1400px] px-6 text-center" use:fadeIn>
    <h2 class="font-display text-6xl italic sm:text-7xl lg:text-8xl">
      Start exploring
    </h2>
    <p class="mx-auto mt-6 max-w-md text-sm leading-relaxed text-muted">
      Thousands of events. Real reviews. Zero agenda.
    </p>
    <div
      class="mt-12 flex flex-col items-center gap-4 sm:flex-row sm:justify-center"
    >
      <a
        href="/events"
        class="border border-text bg-text px-12 py-4 text-xs uppercase tracking-[0.2em] text-bg transition-all duration-300 hover:bg-transparent hover:text-text"
      >
        Browse events
      </a>
      <a
        href="/search"
        class="border border-border px-12 py-4 text-xs uppercase tracking-[0.2em] text-muted transition-all duration-300 hover:border-text hover:text-text"
      >
        Search
      </a>
    </div>
  </div>
</section>
