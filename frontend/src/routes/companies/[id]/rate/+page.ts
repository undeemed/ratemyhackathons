import type { PageLoad } from './$types';
import { getCompany, listTags } from '$lib/api';

export const load: PageLoad = async ({ params }) => {
	try {
		const [company, tags] = await Promise.all([getCompany(params.id), listTags()]);
		return { company, tags, entityType: 'company' as const };
	} catch {
		return { company: null, tags: [], entityType: 'company' as const };
	}
};
