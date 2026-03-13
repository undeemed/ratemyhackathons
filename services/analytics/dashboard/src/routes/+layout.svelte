<script>
	import '../app.css';
	let { children } = $props();
</script>

<svelte:head>
	<title>RateMyHackathons Analytics</title>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet" />
</svelte:head>

<div class="min-h-screen bg-bg">
	<header class="border-b border-border px-6 py-4 flex items-center justify-between">
		<div class="flex items-center gap-3">
			<div class="w-8 h-8 rounded-lg bg-accent flex items-center justify-center text-white font-bold text-sm">R</div>
			<h1 class="text-lg font-semibold text-text">Analytics Dashboard</h1>
		</div>
		<div class="flex items-center gap-2">
			<span id="live-dot" class="w-2 h-2 rounded-full bg-success animate-pulse"></span>
			<span class="text-xs text-text-muted font-mono">LIVE</span>
		</div>
	</header>
	<main class="p-6">
		{@render children()}
	</main>
</div>
