import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

const API_URL = env.API_URL || 'http://127.0.0.1:8080';

const handler: RequestHandler = async ({ params, request }) => {
	const target = `${API_URL}/api/${params.path}${new URL(request.url).search}`;
	const res = await fetch(target, {
		method: request.method,
		headers: request.headers,
		body: request.method !== 'GET' && request.method !== 'HEAD' ? request.body : undefined,
		// @ts-expect-error -- Node fetch supports duplex
		duplex: 'half',
	});
	return new Response(res.body, {
		status: res.status,
		statusText: res.statusText,
		headers: res.headers,
	});
};

export const GET = handler;
export const POST = handler;
export const PUT = handler;
export const PATCH = handler;
export const DELETE = handler;
