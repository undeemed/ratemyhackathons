import type { PageLoad } from './$types';
import { getEvent } from '$lib/api';

export const load: PageLoad = async ({ params }) => {
	const event = await getEvent(params.id);
	return { event };
};
