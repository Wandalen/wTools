# Invariant: Isolated Target-Dir Regeneration

### Scope

- **Purpose**: Let every live repair -- `auto_regen`'s nested `cargo check`, or a soft-warn build -- finish, write its entries, and stop.
- **Responsibility**: Documents the three hazards of compiling live from inside a build script, the rule that prevents each, and what breaks when one is dropped.
- **In Scope**: The target-directory deadlock, unbounded recursion, and live builds that write nothing, with the enforcement and tests for each.
- **Out of Scope**: When regeneration runs (→ `feature/002`), and how sqlx's directories and variables are resolved (→ `invariant/003`).

### Invariant Statement

1. **Isolation.** Every nested `cargo check` that regeneration starts uses a `--target-dir` of its own, never the target directory of the build that is running the consumer's build script.
2. **No recursion.** Every nested check runs with `SQLX_OFFLINE=false` and without `SQLX_QUERY_AUTO_REGEN`, whatever the outer environment holds -- so the consumer's build script, which the nested check runs again, always stands aside instead of regenerating.
3. **Live writes land.** Every build `sqlx_query` lets compile live to repair the cache has an explicit `SQLX_OFFLINE_DIR`: the nested check always gets one naming the cache directory, and a build left live by `SQLX_QUERY_SOFT_WARN=1` gets one exported unless one is already set. Entries are promised, and regeneration attempted, only where that directory exists: one already set that is not a directory is named as the one to create instead.

### Enforcement Mechanism

`src/regen.rs` builds the nested command in one place, so every rule holds for every attempt:

- `--target-dir` is `$OUT_DIR/sqlx_query-offline-regen`. Cargo locks a build directory by the path a build is given, not by nesting on disk, so a directory inside the outer build's tree still takes a separate lock.
- `SQLX_OFFLINE=false` makes the nested run of the consumer's build script stand aside entirely (`invariant/003`): it neither regenerates nor forces offline mode, so sqlx compiles live. Removing `SQLX_QUERY_AUTO_REGEN` is a second guard that does not depend on the first.
- `SQLX_OFFLINE_DIR` names the cache directory; the database URL is inherited, or read from `.env` by sqlx itself.

For soft-warn builds, `src/lib.rs` exports `SQLX_OFFLINE_DIR` with `cargo:rustc-env` whenever none is set -- the only case in which sqlx would otherwise have none. Before regenerating or promising entries, `src/lib.rs` checks once that the cache directory is a directory; when one already set is not, regeneration is skipped with a notice, and the summaries say to create it.

`tests/offline_regen_deadlock_guard_test.rs` runs a real regeneration under a bounded wait and requires the build to pass, which it does only once the entry is really written: a deadlock fails the test at the bound instead of hanging the suite, and a regeneration that writes nothing fails it too. `tests/offline_stale_warning_test.rs` requires soft-warn builds to write their entries.

### Violation Consequences

- **Shared target directory:** the nested check waits forever for the lock the outer build holds, and the outer build waits forever for the nested check. Reproduced directly: no error, no timeout, a build that never ends.
- **Recursion:** each nested build script inherits the opt-in, finds the same stale site -- nothing is repaired until the deepest check finishes -- and starts another check one level down. Reproduced to 11 levels and 6.8 GiB of nested target directories in under six minutes before it was killed; on a runner without a build timeout it ends in a full disk.
- **No explicit `SQLX_OFFLINE_DIR`:** sqlx 0.9 checks each query against the database but writes no entry unless `SQLX_OFFLINE_DIR` is set explicitly; an existing `.sqlx/` is not enough (confirmed by experiment). The build passes, nothing is written, and the next build reports the same stale queries -- regeneration and the soft-warn repair silently do nothing.
- **A `SQLX_OFFLINE_DIR` that is not a directory:** sqlx never creates it. Compiling live, it fails every query with "sqlx offline path does not exist", while an offline build skips the directory silently. A nested check against it can only fail -- after compiling the crate's dependencies, and reporting nothing but its exit status -- and a summary promising entries there is contradicted by the error right after it (reproduced with a real build).

### APIs

| File | Relationship |
|------|--------------|
| [api/001_ensure_offline_mode.md](../api/001_ensure_offline_mode.md) | Operations that start the nested check and export `SQLX_OFFLINE_DIR` |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_offline_mode_enforcement.md](../feature/001_offline_mode_enforcement.md) | Soft-warn repair, which depends on rule 3 |
| [feature/002_auto_regeneration.md](../feature/002_auto_regeneration.md) | Nested check that rules 1 to 3 constrain |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/003_sqlx_resolution_parity.md](../invariant/003_sqlx_resolution_parity.md) | Standing aside on an explicit `SQLX_OFFLINE=false`, which rule 2 relies on |

### Sources

| File | Relationship |
|------|--------------|
| `src/regen.rs` | Target directory and environment of the nested check |
| `src/lib.rs` | `SQLX_OFFLINE_DIR` export for soft-warn builds, and the check that it is a directory |

### Tests

| File | Relationship |
|------|--------------|
| `tests/offline_regen_deadlock_guard_test.rs` | Real regeneration finishes within a bound and writes its entry |
| `tests/offline_stale_warning_test.rs` | Real soft-warn builds write their entries, or fail as reported without a directory |
| `tests/build_script_directives_test.rs` | `SQLX_OFFLINE_DIR` exported only when none is set; nothing promised into one that is not a directory |
