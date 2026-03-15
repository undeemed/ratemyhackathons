<script lang="ts">
	import '../app.css';
	let { children } = $props();

	const nav = [
		{ href: '/', label: 'Overview', icon: '◈' },
		{ href: '/events', label: 'Events', icon: '⚡' },
		{ href: '/crawl', label: 'Crawl Sources', icon: '↓' },
		{ href: '/reviews', label: 'Reviews', icon: '★' },
	];
</script>

<svelte:head>
	<title>RateMyHackathons Admin</title>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet" />
</svelte:head>

<div class="flex min-h-screen bg-bg">
	<!-- Sidebar -->
	<aside class="sticky top-0 flex h-screen w-56 flex-col border-r border-border bg-surface">
		<div class="flex items-center gap-3 border-b border-border px-5 py-4">
			<div class="flex h-8 w-8 items-center justify-center rounded-lg bg-accent text-sm font-bold text-white">R</div>
			<div>
				<p class="text-sm font-semibold text-text">Admin</p>
				<p class="text-[10px] text-text-muted">ratemyhackathons</p>
			</div>
		</div>
		<nav class="flex-1 px-3 py-4">
			{#each nav as item}
				<a href={item.href}
					class="mb-1 flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm text-text-muted transition-colors hover:bg-surface-hover hover:text-text">
					<span class="text-base">{item.icon}</span>
					{item.label}
				</a>
			{/each}
		</nav>
		<div class="border-t border-border px-5 py-3">
			<div class="flex items-center gap-2">
				<span class="h-2 w-2 animate-pulse rounded-full bg-success"></span>
				<span class="font-mono text-[10px] text-text-muted">LIVE</span>
			</div>
		</div>
	</aside>

	<!-- Main -->
	<main class="flex-1 overflow-y-auto p-6">
		{@render children()}
	</main>
</div>
