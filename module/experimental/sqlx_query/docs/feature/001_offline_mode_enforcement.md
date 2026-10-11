# Feature: Offline Mode Enforcement

### Scope

- **Purpose**: Catch a stale sqlx offline query cache (`.sqlx/`) on the machine that made it stale, not later in CI or on a teammate's machine without a database.
- **Responsibility**: Documents what `ensure_offline_mode` decides for each cache state and environment, what it reports, and the workflow it supports.
- **In Scope**: The live-database gap in sqlx, the decision table, the soft-warn opt-out, standing aside for an explicit `SQLX_OFFLINE`, rerun tracking, and known limitations.
- **Out of Scope**: Signatures and options (→ `api/001`), automatic repair (→ `feature/002`), and the parity and rerun properties the decision relies on (→ `invariant/`).

### Design

#### The Gap

sqlx's query macros compile against a live database whenever `SQLX_OFFLINE` is unset or false and `DATABASE_URL` is non-empty, and that path never reads `.sqlx/`. With a database at hand, a developer can change a query's SQL -- or lose its cache entry -- and every local build stays green. The drift surfaces only in a build without a database, in CI or on a teammate's machine, far from the change that caused it.

#### The Check

From the consumer's build script, the check finds every sqlx query macro call site in the crate and looks each one's `query-<hash>.json` up in every directory sqlx itself searches (`invariant/001`, `invariant/003`). A site whose entry is in none of them is stale, and gets one warning naming its file and line: `sqlx_query: src/db.rs:42 -- no .sqlx entry matches this query's SQL`.

Every `.rs` file in the crate counts, tests, examples and feature-gated code included, whether or not the current build compiles it. CI's build may compile it, and a build script -- which Cargo runs per package, not per target -- cannot tell which targets a command compiles.

#### The Decision

The first matching row applies; rerun triggers (`invariant/004`) are emitted in every row.

Each setting is read where sqlx reads it (`invariant/003`): `SQLX_OFFLINE`, `SQLX_OFFLINE_DIR` and the database URL from the environment, or else from the `.env` files in the crate directory and its ancestors, and the database URL's variable -- `DATABASE_URL` unless the crate's `sqlx.toml` names another -- from `sqlx.toml`. A value in `.env` is as explicit as one in the environment.

| Environment | Stale sites | `sqlx_query` emits | Build |
|---|---|---|---|
| `SQLX_OFFLINE` set, but not to `true` (any case) or `1` -- `cargo sqlx prepare` sets `false` | any | nothing more | as sqlx decides: live with a database |
| No `SQLX_OFFLINE_DIR`, and no `.sqlx/` in the crate or at the workspace root | any | one warning | unchanged |
| `SQLX_OFFLINE=true` | none | nothing more | offline, as configured |
| `SQLX_OFFLINE=true` | some | site warnings, one summary | fails on them, as configured |
| `SQLX_OFFLINE` unset, `SQLX_QUERY_SOFT_WARN=1` | some | site warnings, one summary, `SQLX_OFFLINE_DIR` | live with a database, writing the missing entries -- or failing, while `SQLX_OFFLINE_DIR` is not a directory; otherwise fails offline |
| `SQLX_OFFLINE` unset | some | site warnings, two summary lines, `SQLX_OFFLINE=true` | **fails fast**, as CI would |
| `SQLX_OFFLINE` unset | none | `SQLX_OFFLINE=true` | offline -- the build CI runs |

A build fails on a stale query, or writes its entry, only if it compiles that query: `cargo build` with the query in `tests/` prints the warnings, then succeeds. The summaries therefore speak of wherever the query is compiled, never of the current build.

**Fail fast by default.** A stale site still forces `SQLX_OFFLINE=true`, database or not, so the local build fails on exactly what CI would. Two summary lines follow the site warnings: who forced offline mode, and both ways out -- `cargo sqlx prepare` (with `-- --all-targets --all-features` for queries in tests or behind features) or a rebuild with `SQLX_QUERY_SOFT_WARN=1`. Without them, the developer would see only sqlx's error, which suggests unsetting `SQLX_OFFLINE` -- a variable they never set.

**Soft-warn opt-out.** `SQLX_QUERY_SOFT_WARN=1` keeps the warnings but does not force offline mode, so sqlx compiles live whenever it finds a database and writes the entry of each query it compiles. For those writes to land, `sqlx_query` exports the cache directory as `SQLX_OFFLINE_DIR` unless one is already set, in the environment or a `.env` file (why: `invariant/002`); the summary names that directory. sqlx never creates it: while a `SQLX_OFFLINE_DIR` already set is not a directory, sqlx fails every query it compiles live, so the summary -- like the fail-fast advice -- says to create it instead of promising entries there. With every site fresh, the opt-out changes nothing.

**An explicit `SQLX_OFFLINE` is never overridden.** `cargo sqlx prepare` deletes every entry, then runs `cargo check` with `SQLX_OFFLINE=false`; forcing `true` over it would fail every query, leave `.sqlx/` empty, and make `prepare --check` compare nothing. Any value sqlx reads as online makes `sqlx_query` stand aside -- no warnings, no cache directory watched. An explicit `true` is reported on but never set again.

**No cache, no forcing.** Without any `.sqlx/` directory there is nothing to compare against, so one warning says to run `cargo sqlx prepare` once. Forcing offline mode would break every build of a project that never adopted the offline cache, so `sqlx_query` never creates `.sqlx/` either.

**Rerun tracking.** Cargo replays a build script's previous output unless a declared input changed, so every input of this decision is declared (`invariant/004`). A `SQLX_QUERY_SOFT_WARN=1` retry, a `cargo sqlx prepare`, a deleted entry, or creating the `SQLX_OFFLINE_DIR` a warning asked for takes effect on the very next build.

#### Workflow

1. Call `sqlx_query::ensure_offline_mode()` from `build.rs`.
2. Run `cargo sqlx prepare` once to create `.sqlx/`, and commit it.
3. When a build fails fast, run `cargo sqlx prepare` again -- or rebuild once with `SQLX_QUERY_SOFT_WARN=1` against a database -- and commit the new entries.

#### Known Limitations

- Only missing entries are detected. An entry whose recorded columns no longer match the schema is caught by `cargo sqlx prepare --check`, not here.
- Macros are recognized by name, only the package's own non-ignored `.rs` files are scanned, and SQL passed in through a `macro_rules!` parameter is not checked (`invariant/003`).
- A file created directly in the crate root, and a `.env` or `sqlx.toml` that did not exist at the last run, are noticed only once something else re-runs the check; hidden files inside watched directories re-run it (`invariant/004`).

### APIs

| File | Relationship |
|------|--------------|
| [api/001_ensure_offline_mode.md](../api/001_ensure_offline_mode.md) | Entry points and options implementing this decision |

### Features

| File | Relationship |
|------|--------------|
| [feature/002_auto_regeneration.md](../feature/002_auto_regeneration.md) | Opt-in repair that runs before this decision |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_query_dispatch_invariant.md](../invariant/001_query_dispatch_invariant.md) | Cache-key derivation every staleness verdict depends on |
| [invariant/002_consumer_integration_contract.md](../invariant/002_consumer_integration_contract.md) | Explicit `SQLX_OFFLINE_DIR` the soft-warn self-heal depends on |
| [invariant/003_sqlx_resolution_parity.md](../invariant/003_sqlx_resolution_parity.md) | Call sites, lookup and variables matched to sqlx's own |
| [invariant/004_declared_rerun_inputs.md](../invariant/004_declared_rerun_inputs.md) | Inputs declared to Cargo so no decision is replayed |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs` | The decision and the directives it emits |
| `src/options.rs` | Settings, read where sqlx reads them |
| `src/report.rs` | Stale-site diff and every warning text |
| `Cargo.toml` | `enabled` and `auto_regen` feature declarations |

### Tests

| File | Relationship |
|------|--------------|
| `tests/build_script_directives_test.rs` | Every decision row, warning text and rerun trigger |
| `tests/offline_fresh_cache_test.rs` | Real build: a fresh cache builds offline, silently |
| `tests/offline_stale_warning_test.rs` | Real builds: fail-fast, the soft-warn self-heal, a deleted entry, a missing `SQLX_OFFLINE_DIR` |
| `tests/offline_prepare_compat_test.rs` | Real build under `cargo sqlx prepare`'s environment |
| `tests/manual/readme.md` | Manual scenarios, including a real `cargo sqlx prepare` |
