import { browser } from '$app/environment';

const STORAGE_KEY = 'rmh-location';

export type LocationFilter = {
	label: string;
	lat?: number;
	lng?: number;
};

function load(): LocationFilter | null {
	if (!browser) return null;
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		return raw ? JSON.parse(raw) : null;
	} catch {
		return null;
	}
}

let current = $state<LocationFilter | null>(load());

export const locationStore = {
	get value() {
		return current;
	},
	set(loc: LocationFilter | null) {
		current = loc;
		if (browser) {
			if (loc) {
				localStorage.setItem(STORAGE_KEY, JSON.stringify(loc));
			} else {
				localStorage.removeItem(STORAGE_KEY);
			}
		}
	},
	clear() {
		this.set(null);
	},
	async autoDetect(): Promise<LocationFilter | null> {
		if (!browser || !navigator.geolocation) return null;

		return new Promise((resolve) => {
			navigator.geolocation.getCurrentPosition(
				async (pos) => {
					const { latitude, longitude } = pos.coords;
					try {
						const res = await fetch(
							`https://nominatim.openstreetmap.org/reverse?lat=${latitude}&lon=${longitude}&format=json&zoom=10`,
							{ headers: { 'User-Agent': 'RateMyHackathons/1.0' } },
						);
						const data = await res.json();
						const addr = data.address ?? {};
						const city = addr.city || addr.town || addr.village || addr.county || '';
						const state = addr.state || '';
						const country = addr.country || '';
						const parts = [city, state, country].filter(Boolean);
						const label = parts.slice(0, 2).join(', ') || 'Your Location';
						const loc: LocationFilter = { label, lat: latitude, lng: longitude };
						locationStore.set(loc);
						resolve(loc);
					} catch {
						const loc: LocationFilter = {
							label: 'Your Location',
							lat: latitude,
							lng: longitude,
						};
						locationStore.set(loc);
						resolve(loc);
					}
				},
				() => resolve(null),
				{ timeout: 8000 },
			);
		});
	},
};
