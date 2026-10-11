# Invariant: Resolution Parity with sqlx

### Scope

- **Purpose**: Make every verdict and every setting `sqlx_query` emits agree with what sqlx's own macros do in the same build.
- **Responsibility**: Documents which of sqlx's resolution rules `sqlx_query` reproduces, how, where parity is known to stop, and the regressions that broke it before.
- **In Scope**: Query sites and their SQL text, cache-directory lookup, where variables are read from and how they are parsed, precedence of explicit settings, known gaps, and historical violations.
- **Out of Scope**: The cache-file name formula (→ `invariant/001`), the nested check's environment (→ `invariant/002`), and rerun triggers (→ `invariant/004`).

### Invariant Statement

Within its inputs -- the build's process environment, the `.env` files and `sqlx.toml` sqlx reads, and the package's own source files -- `sqlx_query` always resolves exactly what sqlx 0.9's macros resolve:

1. **Sites.** A call needs a cache entry in `sqlx_query`'s view if and only if sqlx looks one up for it: every call of the twelve query macros (`query!`, `query_as!`, `query_scalar!`, their `query_file` forms, and the `_unchecked` variant of each), at any depth -- inside other macros' arguments and `macro_rules!` bodies included -- and never a call that belongs to another package.
2. **SQL text.** A site is keyed by the text sqlx hashes: its string literals joined by `+` with nothing in between, or the exact contents of the file a `query_file` form names, relative to the crate root.
3. **Lookup.** An entry counts as present exactly when sqlx would find it, searching for each query separately in `SQLX_OFFLINE_DIR` (a relative path starting at the workspace root), then `<crate>/.sqlx`, then `<workspace root>/.sqlx` -- the workspace root being the one Cargo reports.
4. **Variables.** Each variable is taken from where sqlx takes it: `SQLX_OFFLINE`, `SQLX_OFFLINE_DIR`, and the database URL -- in `DATABASE_URL`, or in the variable `[common] database-url-var` names in the crate's own `sqlx.toml` -- come from the environment, where even an empty value counts, or else from the `.env` files in the crate directory and its ancestors, the outermost winning. `SQLX_OFFLINE` means offline exactly when sqlx reads it so -- `true` in any case, or `1` -- and the database URL counts only when non-empty, as sqlx counts it.
5. **Precedence.** No directive ever replaces an explicit setting, whether in the environment or a `.env` file: any `SQLX_OFFLINE` value sqlx reads as online makes `sqlx_query` stand aside, an explicit offline value is never set again, and `SQLX_OFFLINE_DIR` is exported only when unset.

#### Known Gaps

Outside those inputs, parity is not guaranteed:

- **Recognition by name.** Any macro named like a sqlx query macro is taken for one, whatever it expands to, and a sqlx macro invoked under another name (`use sqlx::query as q;`) is missed.
- **SQL through a `macro_rules!` parameter.** The call inside the macro body carries no literal, and the expanded call never appears in source, so such queries are not checked.
- **Sources outside the package walk.** `.rs` files that are git-ignored, generated into `OUT_DIR`, or pulled in from outside the package (`#[path]`, `include!`) are not scanned.

### Enforcement Mechanism

- **Sites:** `src/scan.rs` walks raw token trees, not a syntax tree, and descends into every group, so a query inside `vec![]`, `assert!()`, `tokio::join!()` or a `macro_rules!` body is found. One table lists all twelve macro names, each with the argument sqlx reads its SQL from. The package walk leaves out what Cargo's own package listing leaves out: hidden and git-ignored entries, target directories (`target/` at the root, or any directory holding the `CACHEDIR.TAG` Cargo writes) and nested packages.
- **SQL text:** `+`-joined literal pieces are decoded and concatenated with no separator; a `query_file` path is resolved against the crate root and read byte for byte, and absolute paths and bare file names, which sqlx rejects, are skipped.
- **Lookup:** `src/cache_dir.rs` asks Cargo for the workspace root (`cargo locate-project --workspace`) instead of searching for a `[workspace]` table, and resolves a relative `SQLX_OFFLINE_DIR` against it; `src/report.rs` checks every lookup directory for each query.
- **Variables:** `src/options.rs` reads the database URL's variable from the crate's `sqlx.toml` with a TOML parser, then walks from the crate directory up, reading each `.env` with `dotenvy` -- the parser sqlx uses -- and keeping the last value of each variable, before letting the environment override them all. It parses `SQLX_OFFLINE` with sqlx's own rule and requires a non-empty database URL. The crate's own `SQLX_QUERY_*` switches are never read from `.env`, which sqlx does not read them from either.
- **Precedence:** `src/lib.rs` stops right after the rerun triggers when `SQLX_OFFLINE` reads as online, reports without re-setting it when it is explicitly offline, and exports `SQLX_OFFLINE_DIR` only when none is set.

Every rule was read from sqlx 0.9's sources (`sqlx-macros-core`, and `sqlx-core` for `sqlx.toml`) and, wherever a build can show it, confirmed by the scenarios in `tests/manual/readme.md`.

### Violation Consequences

- **A missed site** looks fresh. The forced offline build fails on sqlx's error with no warning naming the query, and `SQLX_QUERY_SOFT_WARN=1` cannot repair it: with nothing stale in `sqlx_query`'s view, offline mode is forced again.
- **A phantom site** -- another package's query, or an entry sought in the wrong place -- warns about queries that are fine, and with `SQLX_QUERY_SOFT_WARN=1` keeps builds live for no reason.
- **The wrong workspace root** makes soft-warn builds write their entries into another project's `.sqlx/`, where this crate's offline builds never look.
- **An overridden `SQLX_OFFLINE=false`** breaks `cargo sqlx prepare`: it deletes every entry, fails to compile, and leaves `.sqlx/` empty, while `cargo sqlx prepare --check` compiles from the old entries and passes however far they have drifted.
- **An empty `DATABASE_URL` counted as a database** sends `auto_regen` into a nested check that cannot connect.
- **A setting sqlx reads but `sqlx_query` misses** -- kept in `.env`, or under the variable `sqlx.toml` names -- splits their views: `auto_regen` is skipped although sqlx has a database, a `SQLX_OFFLINE=false` is overridden, soft-warn entries are written into another directory than the one sqlx was told to use, and changing the setting never re-runs the check.

### Historical Violations

Each is fixed, and pinned by the regression test named beside it.

| Clause | Violation | Regression test |
|--------|-----------|-----------------|
| Sites | `_unchecked` macros were ignored | `unchecked_variants_mre_are_scanned` |
| Sites | `query_file` macros were ignored | `query_file_variants_mre_are_scanned` |
| Sites | Queries inside other macros were invisible | `nested_macro_sites_mre_are_scanned` |
| Sites | A nested package's queries were reported as this crate's | `nested_package_mre_is_not_scanned` |
| SQL text | `"a" + "b"` literals were not recognized | `concatenated_literals_mre_are_scanned` |
| Lookup | Only the first existing cache directory was searched | `workspace_entry_mre_counts_as_fresh_beside_a_local_cache_dir` |
| Lookup | A relative `SQLX_OFFLINE_DIR` started at the crate, not the workspace root | `relative_offline_dir_mre_resolves_from_workspace_root` |
| Lookup | The outermost `[workspace]` ancestor was taken for the workspace root | `nested_workspace_mre_resolves_the_nearest_root` |
| Variables | An empty `DATABASE_URL` counted as a database | `empty_database_url_mre_counts_as_unset` |
| Variables | Soft-warn builds required `DATABASE_URL` in the process environment, though sqlx also reads `.env` | `soft_warn_mre_needs_no_database_url_in_process_env` |
| Variables | `.env` files were not read, so settings kept there were ignored or overridden | `dotenv_mre_supplies_what_sqlx_reads_from_it` |
| Variables | `DATABASE_URL` was hard-coded, ignoring the variable `sqlx.toml` names | `database_url_var_mre_follows_sqlx_toml` |
| Precedence | `cargo sqlx prepare`'s `SQLX_OFFLINE=false` was overridden | `explicit_offline_false_mre_is_never_overridden` |

### APIs

| File | Relationship |
|------|--------------|
| [api/001_ensure_offline_mode.md](../api/001_ensure_offline_mode.md) | Settings parsed, and lookups performed, as sqlx performs them |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_offline_mode_enforcement.md](../feature/001_offline_mode_enforcement.md) | Decision whose verdicts and settings depend on this parity |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_query_dispatch_invariant.md](../invariant/001_query_dispatch_invariant.md) | Name formula applied to the SQL text resolved here |
| [invariant/002_consumer_integration_contract.md](../invariant/002_consumer_integration_contract.md) | Nested check that relies on standing aside for `SQLX_OFFLINE=false` |
| [invariant/004_declared_rerun_inputs.md](../invariant/004_declared_rerun_inputs.md) | Declares the sources and cache directories resolved here |

### Sources

| File | Relationship |
|------|--------------|
| `src/scan.rs` | Sites, SQL text, and the package walk |
| `src/cache_dir.rs` | Lookup order and workspace root |
| `src/report.rs` | Per-query lookup across every directory |
| `src/options.rs` | Variable sources, precedence and parsing |
| `src/lib.rs` | Precedence of explicit settings |

### Tests

| File | Relationship |
|------|--------------|
| `tests/call_site_scan_test.rs` | Sites and the SQL text they are keyed by |
| `tests/package_walk_test.rs` | Which files are scanned |
| `tests/cache_dir_test.rs` | Lookup order, relative paths, nested workspaces |
| `tests/options_test.rs` | Variable sources, precedence and parsing |
| `tests/build_script_directives_test.rs` | Precedence of explicit settings |
| `tests/offline_prepare_compat_test.rs` | Real build under `cargo sqlx prepare`'s environment |
| `tests/manual/readme.md` | Real `sqlx-cli`, `.env` files and workspace layouts |
