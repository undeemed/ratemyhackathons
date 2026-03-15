import type { PageLoad } from './$types';
import { listEvents } from '$lib/api';

export const load: PageLoad = async ({ url }) => {
	const page = Number(url.searchParams.get('page')) || 1;
	try {
		const res = await listEvents({ page, per_page: 12 });
		return { events: res.data, total: res.total, page, perPage: 12 };
	} catch {
		return { events: [], total: 0, page, perPage: 12 };
	}
};
