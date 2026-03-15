<script lang="ts">
	import { SignUp } from 'svelte-clerk';
</script>

<svelte:head>
	<title>Sign Up — RateMyHackathons</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-3.5rem)] flex-col items-center justify-center px-4">
	<h1 class="mb-8 font-display text-4xl italic text-text">Sign Up</h1>
	<SignUp />
</div>
