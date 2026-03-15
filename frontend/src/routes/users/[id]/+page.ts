import type { PageLoad } from './$types';
import { getUser } from '$lib/api';

export const load: PageLoad = async ({ params }) => {
	const user = await getUser(params.id);
	return { user };
};
