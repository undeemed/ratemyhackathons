import { dev } from '$app/environment';
import type { PageLoad } from './$types';
import type { EventSummary } from '$lib/types';
import { listEvents } from '$lib/api';

const DEV_EVENTS: EventSummary[] = dev
	? [
			{
				id: 'dev-1',
				name: 'Backend Unreachable 1',
				description: null,
				location: 'Nowhere',
				url: null,
				start_date: '2025-06-01',
				end_date: '2025-06-02',
				image_url: null,
				latitude: null,
				longitude: null,
				companies: [],
				avg_rating: 4.3,
				review_count: 12,
				category_ratings: [
					{ category: 'organization', avg: 4.5 },
					{ category: 'prizes', avg: 3.8 },
					{ category: 'mentorship', avg: 4.2 },
					{ category: 'judging', avg: 3.5 },
					{ category: 'venue', avg: 4.8 },
					{ category: 'food', avg: 4.0 },
					{ category: 'swag', avg: 3.2 },
					{ category: 'networking', avg: 4.6 },
					{ category: 'communication', avg: 4.1 },
					{ category: 'vibes', avg: 4.9 },
				],
				created_at: '2025-01-01T00:00:00Z',
			},
			{
				id: 'dev-2',
				name: 'Backend Unreachable 2',
				description: null,
				location: null,
				url: null,
				start_date: null,
				end_date: null,
				image_url: null,
				latitude: null,
				longitude: null,
				companies: [],
				avg_rating: 2.7,
				review_count: 5,
				category_ratings: [
					{ category: 'organization', avg: 2.5 },
					{ category: 'prizes', avg: 3.0 },
					{ category: 'mentorship', avg: 2.8 },
					{ category: 'judging', avg: 2.2 },
					{ category: 'venue', avg: 3.1 },
					{ category: 'food', avg: 2.0 },
					{ category: 'swag', avg: 1.5 },
					{ category: 'networking', avg: 3.3 },
					{ category: 'communication', avg: 2.9 },
					{ category: 'vibes', avg: 2.6 },
				],
				created_at: '2025-01-02T00:00:00Z',
			},
		]
	: [];

export const load: PageLoad = async () => {
	try {
		const res = await listEvents({ page: 1, per_page: 500 });
		const events = [...DEV_EVENTS, ...res.data];
		return { events, total: res.total + DEV_EVENTS.length };
	} catch {
		return { events: DEV_EVENTS, total: DEV_EVENTS.length };
	}
};
