<script lang="ts">
	import { SignIn } from 'svelte-clerk';
</script>

<svelte:head>
	<title>Sign In — RateMyHackathons</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-3.5rem)] items-center justify-center px-4">
	<div class="w-full max-w-md">
		<h1 class="mb-8 text-center font-display text-4xl italic text-text">Sign In</h1>
		<SignIn />
	</div>
</div>
