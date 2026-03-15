import type {
	PaginatedResponse,
	EventSummary,
	EventDetail,
	GlobeMarker,
	Company,
	CompanyDetail,
	User,
	Review,
	ReviewComment,
	SearchResults,
	CompareEntity,
	Tag,
	CreateReviewPayload,
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

// ── Events ──

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

// Module-level cache — globe markers rarely change, no need to re-fetch
// on every SvelteKit client-side navigation back to the landing page.
let _globeCache: GlobeMarker[] | null = null;

export async function getGlobeMarkers() {
	if (_globeCache) return _globeCache;
	const markers = await request<GlobeMarker[]>('/events/globe');
	_globeCache = markers;
	return markers;
}

// ── Companies ──

export function listCompanies(params?: { page?: number; per_page?: number; search?: string }) {
	const q = new URLSearchParams();
	if (params?.page) q.set('page', String(params.page));
	if (params?.per_page) q.set('per_page', String(params.per_page));
	if (params?.search) q.set('search', params.search);
	return request<PaginatedResponse<Company>>(`/companies?${q}`);
}

export function getCompany(id: string) {
	return request<CompanyDetail>(`/companies/${id}`);
}

// ── Users ──

export function getUser(id: string) {
	return request<User>(`/users/${id}`);
}

// ── Reviews ──

export function getReview(id: string) {
	return request<Review>(`/reviews/${id}`);
}

export function createReview(userId: string, data: CreateReviewPayload) {
	return request<Review>(`/users/${userId}/reviews`, {
		method: 'POST',
		body: JSON.stringify(data),
	});
}

export function voteReview(id: string, data: { helpful: boolean }) {
	return request<void>(`/reviews/${id}/vote`, {
		method: 'POST',
		body: JSON.stringify(data),
	});
}

export function listComments(reviewId: string) {
	return request<ReviewComment[]>(`/reviews/${reviewId}/comments`);
}

export function createComment(reviewId: string, data: { body: string; parent_comment_id?: string }) {
	return request<ReviewComment>(`/reviews/${reviewId}/comments`, {
		method: 'POST',
		body: JSON.stringify(data),
	});
}

// ── Search ──

export function search(q: string, type?: string) {
	const params = new URLSearchParams({ q });
	if (type) params.set('type', type);
	return request<SearchResults>(`/search?${params}`);
}

// ── Tags ──

export function listTags() {
	return request<Tag[]>('/tags');
}

export function createTag(name: string) {
	return request<Tag>('/tags', {
		method: 'POST',
		body: JSON.stringify({ name }),
	});
}

// ── Compare ──

export function compare(type: 'event' | 'company', ids: string[]) {
	return request<CompareEntity[]>(`/compare?type=${type}&ids=${ids.join(',')}`);
}
