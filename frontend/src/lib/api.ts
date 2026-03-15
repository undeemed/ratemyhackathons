import type {
	PaginatedResponse,
	EventSummary,
	EventDetail,
	GlobeMarker,
	Company,
	User,
	Review,
	ReviewComment,
	SearchResults,
} from './types';

const BASE = '/api';

async function request<T>(path: string, init?: RequestInit): Promise<T> {
	const res = await fetch(`${BASE}${path}`, {
		headers: { 'Content-Type': 'application/json', ...init?.headers },
		...init,
	});
	if (!res.ok) {
		const body = await res.text();
		throw new Error(`API ${res.status}: ${body}`);
	}
	return res.json();
}

export function listEvents(params?: { page?: number; per_page?: number; company_id?: string }) {
	const q = new URLSearchParams();
	if (params?.page) q.set('page', String(params.page));
	if (params?.per_page) q.set('per_page', String(params.per_page));
	if (params?.company_id) q.set('company_id', params.company_id);
	return request<PaginatedResponse<EventSummary>>(`/events?${q}`);
}

export function getEvent(id: string) {
	return request<EventDetail>(`/events/${id}`);
}

export function getGlobeMarkers() {
	return request<GlobeMarker[]>('/events/globe');
}

export function listCompanies(params?: { page?: number; per_page?: number }) {
	const q = new URLSearchParams();
	if (params?.page) q.set('page', String(params.page));
	if (params?.per_page) q.set('per_page', String(params.per_page));
	return request<PaginatedResponse<Company>>(`/companies?${q}`);
}

export function getCompany(id: string) {
	return request<Company>(`/companies/${id}`);
}

export function getUser(id: string) {
	return request<User>(`/users/${id}`);
}

export function getReview(id: string) {
	return request<Review>(`/reviews/${id}`);
}

export function createReview(userId: string, data: { event_id: string; rating: number; title?: string; body?: string }) {
	return request<Review>(`/users/${userId}/reviews`, {
		method: 'POST',
		body: JSON.stringify(data),
	});
}

export function voteReview(id: string, vote: 'up' | 'down') {
	return request<Review>(`/reviews/${id}/vote`, {
		method: 'POST',
		body: JSON.stringify({ vote }),
	});
}

export function listComments(reviewId: string) {
	return request<ReviewComment[]>(`/reviews/${reviewId}/comments`);
}

export function createComment(reviewId: string, data: { user_id: string; body: string }) {
	return request<ReviewComment>(`/reviews/${reviewId}/comments`, {
		method: 'POST',
		body: JSON.stringify(data),
	});
}

export function search(q: string) {
	return request<SearchResults>(`/search?q=${encodeURIComponent(q)}`);
}
