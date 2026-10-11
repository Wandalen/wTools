# Feature: Auto Regeneration

### Scope

- **Purpose**: Repair stale `.sqlx` entries during the build that found them, instead of only reporting them.
- **Responsibility**: Documents the `auto_regen` feature, its runtime opt-in, when it runs or is skipped, and the nested build it relies on.
- **In Scope**: Compile-time and runtime gating, skip conditions and their notices, the isolated nested `cargo check` and what it compiles, its reports, and its cost.
- **Out of Scope**: The decision that follows regeneration (→ `feature/001`), and the hazards the nested build's isolation prevents (→ `invariant/002`).

### Design

`auto_regen` is opt-in twice: the consumer must enable the `auto_regen` Cargo feature of its `sqlx_query` build-dependency, **and** the build must run with `SQLX_QUERY_AUTO_REGEN=1`. Either alone does nothing.

With both, a build that finds stale sites runs one real `cargo check --all-targets --all-features` of the consumer crate against the live database before anything else is decided. It checks every target with every feature because the build script, which Cargo runs per package rather than per target, cannot tell what the outer build compiles; these are also the flags the fail-fast summary recommends for `cargo sqlx prepare`.

sqlx writes an entry for every query the nested check compiles, and the sites are diffed again: whatever it repaired never reaches the fail-fast path. The report comes from that second diff -- ``regenerated stale .sqlx entries via an isolated live `cargo check` `` when no stale site is left, or ``regenerated 1 of 2 stale .sqlx entries ...`` when a query sits in code that no configuration compiles, such as `#[ cfg( any() ) ]` or another platform's `cfg`.

The nested check gets its own `--target-dir` inside `OUT_DIR`, `SQLX_OFFLINE=false`, `SQLX_OFFLINE_DIR` naming the cache directory, and no `SQLX_QUERY_AUTO_REGEN`; it inherits the database URL, or sqlx reads it from `.env` again, and the variables it is given beat any `.env` value. `invariant/002` explains why each of these is required, not merely tidy. `SQLX_QUERY_AUTO_REGEN` is never read from `.env`, so removing it stays a recursion guard.

**Regeneration does not run when:**
- No site is stale, or no `.sqlx/` directory exists -- there is nothing to repair, or nowhere to write.
- `SQLX_OFFLINE` is set: `true` forbids compiling live, and any other value makes the whole check stand aside (`feature/001`).
- The database URL is unset or empty, as sqlx reads it: `DATABASE_URL`, or the variable the crate's `sqlx.toml` names, from the environment or else a `.env` file (`invariant/003`). The build reports `auto_regen skipped: DATABASE_URL is unset or empty`, naming that variable.
- `SQLX_OFFLINE_DIR` names something that is not a directory. sqlx never creates it, so a nested check could only fail; the build reports `auto_regen skipped: SQLX_OFFLINE_DIR names <path>, which is not a directory`.
- `CARGO` is unset, which is reported: the nested check always runs the cargo that runs the outer build, never one found on `PATH`.

A nested check that fails -- in a crate whose features cannot all be enabled together, for instance -- or cannot start, is reported too; whatever is still stale afterwards is handled as if regeneration had not run. The check's own output, compile errors included, is passed on line by line to the build script's stderr, which Cargo shows when the build script fails, or as it runs under `-vv`.

**Cost, by design:** every attempt adds a nested `cargo check` to the build, in a target directory of its own, and the first attempt compiles the crate's whole dependency tree, dev-dependencies included, there from scratch. That is why `auto_regen` is opt-in: for most teams, failing fast and running `cargo sqlx prepare` by hand is cheaper.

### APIs

| File | Relationship |
|------|--------------|
| [api/001_ensure_offline_mode.md](../api/001_ensure_offline_mode.md) | `Options::auto_regen` and the directives that report regeneration |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_offline_mode_enforcement.md](../feature/001_offline_mode_enforcement.md) | The decision regeneration runs ahead of |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/002_consumer_integration_contract.md](../invariant/002_consumer_integration_contract.md) | Target directory, environment and `SQLX_OFFLINE_DIR` the nested check must use |

### Sources

| File | Relationship |
|------|--------------|
| `src/regen.rs` | The nested `cargo check` and every regeneration message |
| `src/lib.rs` | The gate, and the second diff after an attempt |
| `src/options.rs` | The database URL the gate checks, read where sqlx reads it |
| `Cargo.toml` | `auto_regen` feature declaration |

### Tests

| File | Relationship |
|------|--------------|
| `tests/offline_regen_deadlock_guard_test.rs` | Real regeneration, bounded against a deadlock |
| `tests/offline_regen_coverage_test.rs` | Real regeneration of a test's query, and the partial report |
| `tests/build_script_directives_test.rs` | Skip conditions, the skip notices, a `.env` database URL opening the gate, and the attempt's output staying captured |
