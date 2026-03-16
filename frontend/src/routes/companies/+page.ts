import { dev } from '$app/environment';
import type { PageLoad } from './$types';
import type { Company } from '$lib/types';
import { listCompanies } from '$lib/api';

const DEV_COMPANIES: Company[] = dev
	? [
			{
				id: 'dev-1',
				name: 'Backend Unreachable 1',
				logo_url: null,
				website: null,
				description: 'Placeholder company — backend is not running',
				event_count: 12,
				avg_rating: 4.3,
				review_count: 27,
				latest_event_date: '2025-11-15',
				category_ratings: [
					{ category: 'organization', avg: 4.5 },
					{ category: 'prizes', avg: 3.8 },
					{ category: 'mentorship', avg: 4.2 },
					{ category: 'judging', avg: 4.0 },
					{ category: 'venue', avg: 4.7 },
					{ category: 'food', avg: 3.5 },
					{ category: 'swag', avg: 4.1 },
					{ category: 'networking', avg: 4.6 },
					{ category: 'communication', avg: 4.3 },
					{ category: 'vibes', avg: 4.8 },
				],
				created_at: '2025-01-01T00:00:00Z',
			},
			{
				id: 'dev-2',
				name: 'Backend Unreachable 2',
				logo_url: null,
				website: null,
				description: 'Placeholder company — backend is not running',
				event_count: 5,
				avg_rating: 3.1,
				review_count: 8,
				latest_event_date: '2025-06-01',
				category_ratings: [
					{ category: 'organization', avg: 3.0 },
					{ category: 'prizes', avg: 2.5 },
					{ category: 'mentorship', avg: 3.8 },
					{ category: 'judging', avg: 3.2 },
					{ category: 'venue', avg: 2.9 },
					{ category: 'food', avg: 3.4 },
					{ category: 'swag', avg: 2.7 },
					{ category: 'networking', avg: 3.6 },
					{ category: 'communication', avg: 3.1 },
					{ category: 'vibes', avg: 3.0 },
				],
				created_at: '2025-02-15T00:00:00Z',
			},
			{
				id: 'dev-3',
				name: 'Backend Unreachable 3',
				logo_url: null,
				website: null,
				description: null,
				event_count: 1,
				avg_rating: null,
				review_count: 0,
				latest_event_date: null,
				category_ratings: [],
				created_at: '2025-06-01T00:00:00Z',
			},
		]
	: [];

export const load: PageLoad = async () => {
	try {
		const res = await listCompanies({ page: 1, per_page: 500 });
		const companies = [...DEV_COMPANIES, ...res.data];
		return { companies, total: res.total + DEV_COMPANIES.length };
	} catch {
		return { companies: DEV_COMPANIES, total: DEV_COMPANIES.length };
	}
};
