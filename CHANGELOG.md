# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial backend scaffold with Rust/Actix Web
- PostgreSQL schema with full-text search (tsvector/tsquery)
- REST API endpoints: events, companies, users, reviews, search
- Many-to-many event ↔ company relationships
- Paginated list endpoints with filtering
- Full-text search across events, companies, and users
- Crawler metadata table (`crawl_sources`) with JSONB storage
- Database migration for initial schema
