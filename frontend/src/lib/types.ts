export interface Event {
	id: string;
	name: string;
	description: string | null;
	location: string | null;
	url: string | null;
	start_date: string | null;
	end_date: string | null;
	image_url: string | null;
	latitude: number | null;
	longitude: number | null;
	created_at: string;
	updated_at: string;
}

export interface EventSummary {
	id: string;
	name: string;
	description: string | null;
	location: string | null;
	url: string | null;
	start_date: string | null;
	end_date: string | null;
	image_url: string | null;
	latitude: number | null;
	longitude: number | null;
	companies: CompanyRef[];
	avg_rating: number | null;
	review_count: number;
	created_at: string;
}

export interface EventDetail {
	id: string;
	name: string;
	description: string | null;
	location: string | null;
	url: string | null;
	start_date: string | null;
	end_date: string | null;
	image_url: string | null;
	latitude: number | null;
	longitude: number | null;
	companies: CompanyRef[];
	reviews: ReviewRef[];
	avg_rating: number | null;
	review_count: number;
}

export interface CompanyRef {
	id: string;
	name: string;
	role: string | null;
}

export interface ReviewRef {
	id: string;
	user_id: string;
	username: string;
	rating: number;
	title: string | null;
	body: string | null;
	created_at: string;
}

export interface GlobeMarker {
	id: string;
	name: string;
	latitude: number;
	longitude: number;
	start_date: string | null;
}

export interface Company {
	id: string;
	name: string;
	logo_url: string | null;
	website: string | null;
	description: string | null;
	created_at: string;
}

export interface User {
	id: string;
	username: string;
	avatar_url: string | null;
	bio: string | null;
	created_at: string;
}

export interface Review {
	id: string;
	event_id: string;
	user_id: string;
	rating: number;
	title: string | null;
	body: string | null;
	upvotes: number;
	downvotes: number;
	created_at: string;
}

export interface ReviewComment {
	id: string;
	review_id: string;
	user_id: string;
	username: string;
	body: string;
	created_at: string;
}

export interface PaginatedResponse<T> {
	data: T[];
	total: number;
	page: number;
	per_page: number;
}

export interface SearchResults {
	events: EventSummary[];
	companies: Company[];
	users: User[];
}
