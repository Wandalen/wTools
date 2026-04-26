# Invariant Doc Entity

### Scope

- **Purpose**: Document the behavioral guarantees that unitore's storage layer maintains unconditionally.
- **Responsibility**: Lists all correctness properties, their enforcement mechanisms, and violation consequences.
- **In Scope**: Frame deduplication and feed URL uniqueness invariants.
- **Out of Scope**: HTTP-level reliability and retry behavior; TOML parsing validation rules.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Frame Deduplication](001_frame_deduplication.md) | Frames are never duplicated when a feed is re-fetched | ✅ |
| 002 | [Feed URL Primary Key](002_feed_url_primary_key.md) | Each feed is stored exactly once, keyed by URL | ✅ |
