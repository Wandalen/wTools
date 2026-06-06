# Invariant: Frame Deduplication

### Scope

- **Purpose**: Guarantee that re-fetching a feed never creates duplicate frame entries in storage.
- **Responsibility**: Documents the deduplication contract and the three-way diff that enforces it.
- **In Scope**: Behavior of feeds_process when frames for a feed already exist in storage.
- **Out of Scope**: Feed-level uniqueness (→ `invariant/002`); HTTP error handling and retry behavior.

### Invariant Statement

For any feed, repeated calls to feeds_process with the same feed content produce exactly the same set of frame rows in storage — no two frame rows share the same id value, and processing a feed twice inserts no additional rows for entries already present.

### Enforcement Mechanism

feeds_process queries storage for all frames matching the current feed's URL, collecting their id and published values. For each entry in the fetched feed:

- If the entry's id is absent from storage: categorized as new — queued for insertion via frames_save.
- If the entry's id is present and its published date differs: categorized as modified — queued for update via frames_update.
- If the entry's id is present and published is unchanged: skipped with no storage write.

The frame table schema declares id as TEXT PRIMARY KEY, providing a second line of defense — GlueSQL rejects any duplicate id insertion at the SQL engine level.

### Violation Consequences

Any regression that causes duplicate ids to be inserted would produce duplicate rows in `.frame.list` output, incorrect frame counts in download reports, and primary key constraint violations from GlueSQL on the second insertion attempt.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/sled_adapter/feed.rs` | feeds_process — three-way diff algorithm implementation |
| source | `src/sled_adapter/mod.rs` | frame table schema declaring id TEXT PRIMARY KEY |
| test | `tests/frames_download.rs` | test_save verifies 10 frames inserted on first fetch; test_update verifies no duplicates on re-fetch |
| doc | [feature/002_content_retrieval.md](../feature/002_content_retrieval.md) | Content retrieval feature subject to this invariant |
