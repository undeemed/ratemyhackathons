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
	category_ratings: CategoryAvg[];
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
	would_return_pct: number | null;
	category_ratings: CategoryAvg[];
	top_tags: TagCount[];
	rating_distribution: RatingDistributionEntry[];
	sponsors: EventSponsorRef[];
}

export interface EventSponsorRef {
	id: string;
	name: string;
	logo_url: string | null;
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
	would_return: boolean | null;
	created_at: string;
	category_ratings: ReviewRating[];
}

export interface ReviewRating {
	category: string;
	score: number;
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
	event_count: number;
	avg_rating: number | null;
	review_count: number;
	latest_event_date: string | null;
	category_ratings: CategoryAvg[];
	created_at: string;
}

export interface CompanyDetail {
	id: string;
	name: string;
	logo_url: string | null;
	website: string | null;
	description: string | null;
	events: CompanyEventRef[];
	avg_rating: number | null;
	review_count: number;
	would_return_pct: number | null;
	category_ratings: CategoryAvg[];
	top_tags: TagCount[];
	rating_distribution: RatingDistributionEntry[];
	reviews: ReviewRef[];
}

export interface CompanyEventRef {
	id: string;
	name: string;
	role: string | null;
	start_date: string | null;
	avg_rating: number | null;
}

export interface CategoryAvg {
	category: string;
	avg: number;
}

export interface TagCount {
	name: string;
	count: number;
}

export interface RatingDistributionEntry {
	rating: number;
	count: number;
}

export interface Tag {
	id: string;
	name: string;
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

export interface SearchResult {
	id: string;
	name: string;
	rank: number;
	avg_rating: number | null;
	review_count: number;
	would_return_pct: number | null;
}

export interface UserSearchResult {
	id: string;
	name: string;
	rank: number;
}

export interface SearchResults {
	events: SearchResult[];
	companies: SearchResult[];
	users: UserSearchResult[];
	total: number;
}

export interface CompareEntity {
	id: string;
	name: string;
	avg_rating: number | null;
	review_count: number;
	would_return_pct: number | null;
	category_ratings: CategoryAvg[];
}

export interface CreateReviewPayload {
	event_id?: string;
	company_id?: string;
	title?: string;
	body: string;
	would_return?: boolean;
	category_ratings: Record<string, number>;
	tag_ids?: string[];
}

export const RATING_CATEGORIES = [
	'organization',
	'prizes',
	'mentorship',
	'judging',
	'venue',
	'food',
	'swag',
	'networking',
	'communication',
	'vibes',
] as const;

export type RatingCategory = (typeof RATING_CATEGORIES)[number];

export const CATEGORY_LABELS: Record<RatingCategory, string> = {
	organization: 'Organization',
	prizes: 'Prizes',
	mentorship: 'Mentorship',
	judging: 'Judging',
	venue: 'Venue',
	food: 'Food & Drinks',
	swag: 'Swag',
	networking: 'Networking',
	communication: 'Communication',
	vibes: 'Vibes',
};

export const CATEGORY_ICONS: Record<RatingCategory, string> = {
	organization: '&#9776;',
	prizes: '&#9733;',
	mentorship: '&#9998;',
	judging: '&#9878;',
	venue: '&#9962;',
	food: '&#9749;',
	swag: '&#127873;',
	networking: '&#128101;',
	communication: '&#128172;',
	vibes: '&#9889;',
};

export const SCORE_LABELS = ['', 'Awful', 'OK', 'Good', 'Great', 'Awesome'] as const;

export function scoreColor(score: number): string {
	if (score >= 4.0) return '#4caf50';
	if (score >= 3.0) return '#ffc107';
	return '#ef5350';
}

export function scoreBg(score: number): string {
	if (score >= 4.0) return 'bg-score-green';
	if (score >= 3.0) return 'bg-score-yellow';
	return 'bg-score-red';
}

export function scoreLabel(score: number): string {
	const rounded = Math.round(score);
	return SCORE_LABELS[Math.min(Math.max(rounded, 1), 5)];
}
