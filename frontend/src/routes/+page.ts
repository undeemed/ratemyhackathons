import type { PageLoad } from './$types';
import { getGlobeMarkers, listEvents } from '$lib/api';

export const load: PageLoad = async () => {
	const [markers, eventsRes] = await Promise.all([
		getGlobeMarkers().catch(() => []),
		listEvents({ per_page: 6 }).catch(() => ({ data: [], total: 0, page: 1, per_page: 6 })),
	]);
	return { markers, events: eventsRes.data, totalEvents: eventsRes.total };
};
