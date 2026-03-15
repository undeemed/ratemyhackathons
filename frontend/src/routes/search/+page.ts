import type { PageLoad } from './$types';
import { search } from '$lib/api';

export const load: PageLoad = async ({ url }) => {
	const q = url.searchParams.get('q') || '';
	if (!q.trim()) return { q, results: null };
	try {
		const results = await search(q);
		return { q, results };
	} catch {
		return { q, results: null };
	}
};
