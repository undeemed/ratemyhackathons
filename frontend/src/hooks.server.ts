import { withClerkHandler } from 'svelte-clerk/server';
import { sequence } from '@sveltejs/kit/hooks';
import type { Handle } from '@sveltejs/kit';

const stripHeaders: Handle = async ({ event, resolve }) => {
	const response = await resolve(event);
	response.headers.delete('x-sveltekit-page');
	response.headers.delete('x-clerk-auth-status');
	response.headers.delete('x-clerk-auth-reason');
	return response;
};

export const handle = sequence(withClerkHandler(), stripHeaders);
