"""Geocoding module — resolve location text to lat/lng using Nominatim.

Uses geopy with a local cache to avoid repeat lookups.
Nominatim policy: max 1 request per second, include User-Agent.
"""

import time
from functools import lru_cache

from geopy.geocoders import Nominatim
from geopy.exc import GeocoderTimedOut, GeocoderUnavailable

_geocoder = Nominatim(user_agent="RateMyHackathons/1.0", timeout=10)

# Seed cache for common hackathon cities (avoids API calls for known locations)
_SEED_CACHE: dict[str, tuple[float, float]] = {
    "San Francisco": (37.7749, -122.4194),
    "San Francisco, CA": (37.7749, -122.4194),
    "New York": (40.7128, -74.0060),
    "New York, NY": (40.7128, -74.0060),
    "London": (51.5074, -0.1278),
    "London, UK": (51.5074, -0.1278),
    "Berlin": (52.5200, 13.4050),
    "Berlin, Germany": (52.5200, 13.4050),
    "Paris": (48.8566, 2.3522),
    "Paris, France": (48.8566, 2.3522),
    "Tokyo": (35.6762, 139.6503),
    "Tokyo, Japan": (35.6762, 139.6503),
    "Singapore": (1.3521, 103.8198),
    "Toronto": (43.6532, -79.3832),
    "Toronto, Canada": (43.6532, -79.3832),
    "Mexico City": (19.4326, -99.1332),
    "Sydney": (-33.8688, 151.2093),
    "Sydney, Australia": (-33.8688, 151.2093),
    "Bangalore": (12.9716, 77.5946),
    "Bangalore, India": (12.9716, 77.5946),
    "Los Angeles": (34.0522, -118.2437),
    "Los Angeles, CA": (34.0522, -118.2437),
    "Seattle": (47.6062, -122.3321),
    "Seattle, WA": (47.6062, -122.3321),
    "Chicago": (41.8781, -87.6298),
    "Chicago, IL": (41.8781, -87.6298),
    "Austin": (30.2672, -97.7431),
    "Austin, TX": (30.2672, -97.7431),
    "Cambridge, MA": (42.3736, -71.1097),
    "Boston, MA": (42.3601, -71.0589),
    "Boston": (42.3601, -71.0589),
    "Miami, FL": (25.7617, -80.1918),
    "Denver, CO": (39.7392, -104.9903),
    "Atlanta, GA": (33.7490, -84.3880),
    "Washington, DC": (38.9072, -77.0369),
    "Vancouver": (49.2827, -123.1207),
    "Vancouver, Canada": (49.2827, -123.1207),
    "Amsterdam": (52.3676, 4.9041),
    "Mumbai": (19.0760, 72.8777),
    "Dubai": (25.2048, 55.2708),
    "Seoul": (37.5665, 126.9780),
    "Beijing": (39.9042, 116.4074),
    "Shanghai": (31.2304, 121.4737),
    "Hong Kong": (22.3193, 114.1694),
    "Tel Aviv": (32.0853, 34.7818),
    "Lisbon": (38.7223, -9.1393),
    "Dublin": (53.3498, -6.2603),
    "Stockholm": (59.3293, 18.0686),
    "Zurich": (47.3769, 8.5417),
    "Waterloo, ON": (43.4643, -80.5204),
    "Pittsburgh, PA": (40.4406, -79.9959),
    "Ann Arbor, MI": (42.2808, -83.7430),
}

# Runtime cache for API lookups
_runtime_cache: dict[str, tuple[float, float] | None] = {}


def geocode_location(location: str | None) -> tuple[float, float] | None:
    """Resolve a location string to (latitude, longitude).

    Returns None if geocoding fails or location is empty.
    """
    if not location or not location.strip():
        return None

    location = location.strip()

    # Check seed cache (case-insensitive)
    for key, coords in _SEED_CACHE.items():
        if location.lower() == key.lower():
            return coords

    # Check runtime cache
    if location in _runtime_cache:
        return _runtime_cache[location]

    # Query Nominatim
    try:
        time.sleep(1.1)  # Respect rate limit
        result = _geocoder.geocode(location)
        if result:
            coords = (result.latitude, result.longitude)
            _runtime_cache[location] = coords
            return coords
        else:
            _runtime_cache[location] = None
            return None
    except (GeocoderTimedOut, GeocoderUnavailable, Exception) as e:
        print(f"  [GEO] Failed to geocode '{location}': {e}")
        _runtime_cache[location] = None
        return None
