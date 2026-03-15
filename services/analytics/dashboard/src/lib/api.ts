// ── Analytics API (port 8081) ──

const API = '';

export async function fetchCrawlStats() {
	const res = await fetch(`${API}/api/crawl/stats`);
	return res.json();
}

export async function fetchCrawlHistory(days = 30) {
	const res = await fetch(`${API}/api/crawl/history?days=${days}`);
	return res.json();
}

export async function fetchCrawlSources() {
	const res = await fetch(`${API}/api/crawl/sources`);
	return res.json();
}

export async function fetchTrending(days = 30, limit = 20) {
	const res = await fetch(`${API}/api/events/trending?days=${days}&limit=${limit}`);
	return res.json();
}

export async function fetchEventsTimeline() {
	const res = await fetch(`${API}/api/events/timeline`);
	return res.json();
}

export async function fetchRatingDistribution() {
	const res = await fetch(`${API}/api/reviews/stats`);
	return res.json();
}

export async function fetchRecentReviews(limit = 20) {
	const res = await fetch(`${API}/api/reviews/recent?limit=${limit}`);
	return res.json();
}

export function connectLiveFeed(onEvent: (data: any) => void): EventSource {
	const source = new EventSource(`${API}/api/live`);
	source.onmessage = (e) => {
		try {
			const data = JSON.parse(e.data);
			onEvent(data);
		} catch {}
	};
	return source;
}

// ── Main API (port 8080, proxied via /main-api) ──

const MAIN = '/main-api';

export async function fetchEvents(page = 1, perPage = 50) {
	const res = await fetch(`${MAIN}/events?page=${page}&per_page=${perPage}`);
	return res.json();
}

export async function fetchEvent(id: string) {
	const res = await fetch(`${MAIN}/events/${id}`);
	return res.json();
}

export async function fetchCompanies(page = 1, perPage = 50) {
	const res = await fetch(`${MAIN}/companies?page=${page}&per_page=${perPage}`);
	return res.json();
}

export async function fetchUsers(page = 1, perPage = 50) {
	const res = await fetch(`${MAIN}/users?page=${page}&per_page=${perPage}`);
	return res.json();
}

export async function fetchGlobeMarkers() {
	const res = await fetch(`${MAIN}/events/globe`);
	return res.json();
}

export async function fetchSearch(q: string, type?: string) {
	const params = new URLSearchParams({ q });
	if (type) params.set('type', type);
	const res = await fetch(`${MAIN}/search?${params}`);
	return res.json();
}
