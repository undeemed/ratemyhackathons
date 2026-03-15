import type { PageLoad } from './$types';
import { compare } from '$lib/api';

export const load: PageLoad = async ({ url }) => {
	const type = (url.searchParams.get('type') ?? 'event') as 'event' | 'company';
	const idsParam = url.searchParams.get('ids') ?? '';
	const ids = idsParam.split(',').filter(Boolean);

	if (ids.length < 2) {
		return { type, entities: [], ids };
	}

	try {
		const entities = await compare(type, ids);
		return { type, entities, ids };
	} catch {
		return { type, entities: [], ids };
	}
};
