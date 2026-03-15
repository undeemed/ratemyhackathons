import type { PageLoad } from './$types';
import { getCompany, listEvents } from '$lib/api';

export const load: PageLoad = async ({ params }) => {
	try {
		const [company, eventsRes] = await Promise.all([
			getCompany(params.id),
			listEvents({ company_id: params.id, per_page: 12 }),
		]);
		return { company, events: eventsRes.data, totalEvents: eventsRes.total };
	} catch {
		return { company: null, events: [], totalEvents: 0 };
	}
};
