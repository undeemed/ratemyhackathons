import type { PageLoad } from './$types';
import { getCompany } from '$lib/api';

export const load: PageLoad = async ({ params }) => {
	try {
		const company = await getCompany(params.id);
		return { company };
	} catch {
		return { company: null };
	}
};
