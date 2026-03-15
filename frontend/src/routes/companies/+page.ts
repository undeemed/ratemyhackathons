import type { PageLoad } from './$types';
import { listCompanies } from '$lib/api';

export const load: PageLoad = async ({ url }) => {
	const page = Number(url.searchParams.get('page')) || 1;
	const res = await listCompanies({ page, per_page: 20 });
	return { companies: res.data, total: res.total, page, perPage: 20 };
};
