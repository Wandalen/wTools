# Fixtures

### Scope

Checked-in artifacts produced by a real sqlx build, used as test inputs. Never hand-written: each one is what sqlx itself wrote.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `query-bbf600f17712173206b754fd7c8f8f8fd46a03bf54e824ff8046c37a88407123.json` | sqlx 0.9.0's real cache entry for `SELECT 1 as one` |

### Provenance

The entry was written by sqlx 0.9.0 during a live `cargo build` against a real SQLite database, with `DATABASE_URL` and `SQLX_OFFLINE_DIR` set and no `sqlx-cli` involved. `golden_fixture_hash_canary_test.rs` re-derives its name independently, pinning the cache-file name formula as sqlx implemented it (`docs/invariant/001_query_dispatch_invariant.md`).

To regenerate it -- after a sqlx upgrade, for instance:

1. Create a throwaway crate that depends on `sqlx` (features `sqlite`, `macros`) and compiles `sqlx::query!( "SELECT 1 as one" )`.
2. Run `cargo build` with `DATABASE_URL` pointing at a SQLite file and `SQLX_OFFLINE_DIR` at an empty directory.
3. Replace the entry here with the file that appears there. If its name changed, sqlx changed the formula: update the canary test and `docs/invariant/001_query_dispatch_invariant.md` to match.
