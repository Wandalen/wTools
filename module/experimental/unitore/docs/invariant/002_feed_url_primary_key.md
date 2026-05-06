# Invariant: Feed URL Primary Key

### Scope

- **Purpose**: Guarantee that each feed is stored exactly once, identified by its URL.
- **Responsibility**: Documents the URL uniqueness constraint on the feed table and how the schema enforces it.
- **In Scope**: feed table PRIMARY KEY on link, feeds_save insertion behavior, config table path uniqueness.
- **Out of Scope**: Frame deduplication within a single feed (→ `invariant/001`).

### Invariant Statement

For all feeds stored in the database, no two rows share the same link (URL) value. The link column is the feed's canonical identifier. Attempting to insert a feed whose URL already exists is rejected at the storage level.

### Enforcement Mechanism

The feed table schema declares link as TEXT PRIMARY KEY. GlueSQL enforces this constraint on every INSERT — a second insertion of the same URL produces a primary key violation error rather than creating a duplicate row. feeds_save relies on this schema constraint and does not perform a pre-insert existence check.

The config table similarly declares path as TEXT PRIMARY KEY, preventing the same config file path from being registered twice.

### Violation Consequences

If two rows with the same feed URL were stored, frame rows (which carry a feed_link foreign key referencing feed.link) would be ambiguously attributed to both. Feed listing and frame download operations would produce incorrect results, and foreign key resolution in GlueSQL would be undefined.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/sled_adapter/mod.rs` | Schema definition — link TEXT PRIMARY KEY on feed table |
| source | `src/sled_adapter/feed.rs` | feeds_save — INSERT relying on PRIMARY KEY enforcement |
| doc | [feature/001_subscription_management.md](../feature/001_subscription_management.md) | Subscription management — context where feeds are added |
| doc | [feature/002_content_retrieval.md](../feature/002_content_retrieval.md) | Content retrieval — context where feed uniqueness is observed |
