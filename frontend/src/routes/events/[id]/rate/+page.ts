import type { PageLoad } from './$types';
import { getEvent } from '$lib/api';
import { listTags } from '$lib/api';

export const load: PageLoad = async ({ params }) => {
	try {
		const [event, tags] = await Promise.all([getEvent(params.id), listTags()]);
		return { event, tags, entityType: 'event' as const };
	} catch {
		return { event: null, tags: [], entityType: 'event' as const };
	}
};
