# Invariant: Hash Algorithm Parity

### Scope

- **Purpose**: Keep every staleness verdict true by naming each query's cache entry exactly as sqlx names it.
- **Responsibility**: Documents the cache-file name sqlx derives from a query's SQL text, how `sqlx_query` stays identical to it, and the risk of depending on it.
- **In Scope**: The file-name formula, why nothing but the SQL text enters it, the tests that pin it, and its dependence on a private sqlx detail.
- **Out of Scope**: Which SQL text a call site is keyed by, and which directories are searched for the file (→ `invariant/003`).

### Invariant Statement

For every SQL text `s`, the cache entry `sqlx_query` expects is named `query-<h>.json`, where `<h>` is the lowercase hexadecimal SHA-256 digest of the UTF-8 bytes of `s` -- always the exact name sqlx 0.9 writes and reads for `s`. Nothing else ever enters the digest: two queries with the same SQL text and different bind arguments, record types or macros always name the same entry.

### Enforcement Mechanism

The hash function takes the SQL text as its only input, so bind arguments cannot be folded in by accident -- its signature rules that out. Three layers of tests pin the formula:

- `tests/hash_test.rs` drives the scan-to-warning path with digests computed outside this crate (Python's `hashlib.sha256`, `sha256sum`), including the empty string and non-ASCII SQL, and rejects near-miss names: no `query-` prefix, no `.json` extension, a directory instead of a file.
- `tests/golden_fixture_hash_canary_test.rs` re-derives, independently of the crate's own code, the name of an entry sqlx 0.9.0 actually wrote (kept in `tests/fixtures/`), pinning the formula as sqlx implemented it rather than as documented.
- The real-build tests (`tests/offline_*_test.rs`) let the workspace's own sqlx write entries and require `sqlx_query` to find them -- the tests that fail when a sqlx upgrade changes the formula.

The formula was read from sqlx's own source (`hash_string` in the query data module of `sqlx-macros-core` 0.9), not from documentation.

### Violation Consequences

The formula is a private sqlx implementation detail, not a semver-guaranteed contract. If it changes, every name `sqlx_query` predicts is wrong at once:

- **No entry in the old format left:** every query is reported stale on every build, the forced-offline summary blames queries sqlx accepts, and `SQLX_QUERY_SOFT_WARN=1` keeps every build live for no reason.
- **Old-format entries still on disk:** a query whose new entry is missing can look fresh. The build then fails on sqlx's error with no warning naming the query, and `SQLX_QUERY_SOFT_WARN=1` cannot repair it -- nothing is stale in `sqlx_query`'s view, so offline mode is forced again.

### APIs

| File | Relationship |
|------|--------------|
| [api/001_ensure_offline_mode.md](../api/001_ensure_offline_mode.md) | Operations whose staleness verdicts use this formula |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_offline_mode_enforcement.md](../feature/001_offline_mode_enforcement.md) | Staleness check keyed by this formula |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/003_sqlx_resolution_parity.md](../invariant/003_sqlx_resolution_parity.md) | Resolves the SQL text this formula hashes, and where the file is sought |

### Sources

| File | Relationship |
|------|--------------|
| `src/hash.rs` | The formula |
| `src/report.rs` | Looks each site's predicted file name up |

### Tests

| File | Relationship |
|------|--------------|
| `tests/hash_test.rs` | Independently computed digests, near-miss names rejected |
| `tests/golden_fixture_hash_canary_test.rs` | Name of a real sqlx-written entry, re-derived independently |
| `tests/offline_fresh_cache_test.rs` | Entries written by the workspace's sqlx are found fresh |
