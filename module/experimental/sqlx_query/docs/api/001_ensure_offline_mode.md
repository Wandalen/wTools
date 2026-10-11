# API: Ensure Offline Mode

### Scope

- **Purpose**: Give a consumer's build script one call that checks the sqlx offline cache and settles `SQLX_OFFLINE` for the build.
- **Responsibility**: Documents the crate's public operations, the settings they take, the effects they have, and what callers can rely on across releases.
- **In Scope**: The three operations, each setting and where it is read from, the directives produced, subprocesses run, failure behavior, and compatibility.
- **Out of Scope**: Why each decision is made (→ `feature/001`), automatic repair (→ `feature/002`), and the properties the operations maintain (→ `invariant/`).

### Abstract

`sqlx_query` is a build-time library: its operations run inside a consumer crate's build script, never in application code, and they neither execute queries nor depend on sqlx. Each call reads its settings, scans the crate's own sources, looks every sqlx query's cache entry up, and answers in Cargo build-script directives -- rerun triggers, warnings, and the `SQLX_OFFLINE` or `SQLX_OFFLINE_DIR` value sqlx's macros then compile with.

### Operations

**`ensure_offline_mode`** -- the call a build script makes. Takes nothing: reads the settings where sqlx reads them -- the build's environment, `.env` files, `sqlx.toml` -- and prints the directives for them. A build script that makes this call and nothing else is complete.

**`ensure_offline_mode_with`** -- the same, with settings supplied by the caller, for a configuration the environment cannot express: read the settings from the environment, adjust them, pass them in.

**`build_script_directives`** -- takes settings and returns, in order, the directives the other two would print, without printing them; for tools and tests that inspect a decision. It is not free of side effects: it runs the subprocesses below, including the nested `cargo check` when regeneration is enabled.

**Settings** (`Options`) -- read from the build's environment, `.env` files and `sqlx.toml` by `Options::from_env`, from any variable lookup plus those files by `Options::resolve`, or set field by field; `Options::default()` starts every field empty or off, with the database URL in `DATABASE_URL`.

| Setting | Read from | Meaning |
|---|---|---|
| `manifest_dir` | `CARGO_MANIFEST_DIR` | Root of the crate to scan; the current directory when unset |
| `cache_dir_override` | `SQLX_OFFLINE_DIR`\* | Searched first and written to by live builds, which fail while it is not a directory; a relative path starts at the workspace root |
| `offline` | `SQLX_OFFLINE`\* | Unset; offline (`true` in any case, or `1`); or online (any other value), which makes the check stand aside |
| `database_url_var` | `[common] database-url-var` in the crate's `sqlx.toml` | The variable holding the database URL; `DATABASE_URL` without one |
| `database_url_set` | that variable\* | Set and non-empty |
| `auto_regen` | `SQLX_QUERY_AUTO_REGEN` | `1` enables regeneration, when the `auto_regen` feature is compiled in |
| `soft_warn` | `SQLX_QUERY_SOFT_WARN` | `1` leaves a build with stale queries live instead of forcing offline mode |
| `config_files` | -- | Every `.env` file and `sqlx.toml` read, declared as rerun triggers |

\* From the environment, where even an empty value counts, or else from the `.env` files in the crate directory and its ancestors, the outermost winning -- exactly as sqlx reads them (`invariant/003`). The two `SQLX_QUERY_*` switches are read from the environment alone.

**Directives**, in this order:

1. Rerun triggers (`invariant/004`): `cargo:rerun-if-env-changed` for every variable above except `CARGO_MANIFEST_DIR`, and `cargo:rerun-if-changed` for the `config_files` and the crate's sources -- where standing aside stops -- then for each existing cache directory, and for a missing `SQLX_OFFLINE_DIR` the warnings say to create.
2. Without any cache directory: one warning, and nothing more.
3. At most one regeneration report (`feature/002`).
4. One `cargo:warning` per query without a cache entry.
5. The decision (`feature/001`): its summary warnings, and the `cargo:rustc-env` value it sets -- `SQLX_OFFLINE=true`, or the soft-warn `SQLX_OFFLINE_DIR` -- if any.

**Subprocesses** -- `cargo locate-project --workspace`, through the cargo running the build (or `cargo` on `PATH`), to find the workspace root; `git ls-files`, to find ignored files, skipped when git is missing or the crate is not in a repository; and, with regeneration, the nested `cargo check`.

### Error Handling

No operation returns an error, and none panics on what it finds. An unreadable file or directory, a source file that does not tokenize, a `.env` or `sqlx.toml` file that does not parse, a query whose SQL is not a literal, a failing `cargo` or `git` subprocess: each drops only what it affects -- that file or the rest of it, that query, the workspace-root or ignored-file lookup -- and the check goes on with the rest. A settings file sqlx cannot parse fails every query macro anyway, with sqlx's own error. Regeneration problems are reported as warnings (`feature/002`).

The one build failure `sqlx_query` causes is deliberate. With offline mode forced over a query that has no entry, sqlx's macro rejects that query, right after warnings that name it and both fixes (`feature/001`). The build recovers as soon as the entry exists -- after `cargo sqlx prepare`, or one build with `SQLX_QUERY_SOFT_WARN=1` against a database.

### Compatibility Guarantees

- **Versioning:** semantic versioning, pre-1.0 -- a minor release (`0.3` → `0.4`) may change the operations, the settings or a decision; a patch release changes none of them.
- **sqlx version:** follows sqlx 0.9's query macros, cache-file names, directory lookup and variable parsing (`invariant/001`, `invariant/003`). A sqlx release that changes any of them needs a matching `sqlx_query` release.
- **The cache stays sqlx's:** never creates `.sqlx/`, and never deletes or edits an entry; only sqlx writes entries -- in a build, in `cargo sqlx prepare`, or in the nested regeneration check.
- **Explicit settings win:** an explicit `SQLX_OFFLINE` is never overridden, and `SQLX_OFFLINE_DIR` is exported only when unset -- in the environment and in `.env` files alike (`invariant/003`).
- **Opt-ins stay off:** the `auto_regen` feature does nothing until `SQLX_QUERY_AUTO_REGEN=1` is also set, and `SQLX_QUERY_SOFT_WARN=1` changes nothing while every query has an entry.
- **Messages:** warning wording is not part of the API and may change in any release.

### Features

| File | Relationship |
|------|--------------|
| [feature/001_offline_mode_enforcement.md](../feature/001_offline_mode_enforcement.md) | Decision these operations implement |
| [feature/002_auto_regeneration.md](../feature/002_auto_regeneration.md) | Opt-in repair the `auto_regen` setting enables |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_query_dispatch_invariant.md](../invariant/001_query_dispatch_invariant.md) | Cache-file names every staleness verdict relies on |
| [invariant/002_consumer_integration_contract.md](../invariant/002_consumer_integration_contract.md) | Isolation of the nested `cargo check`, and where live builds write |
| [invariant/003_sqlx_resolution_parity.md](../invariant/003_sqlx_resolution_parity.md) | Sites, lookup, setting parsing and precedence matched to sqlx |
| [invariant/004_declared_rerun_inputs.md](../invariant/004_declared_rerun_inputs.md) | Rerun triggers emitted ahead of every decision |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs` | The three operations and the order of their directives |
| `src/options.rs` | Settings, and how the environment, `.env` files and `sqlx.toml` are read into them |
| `src/cache_dir.rs` | Workspace-root lookup through `cargo locate-project` |
| `src/scan.rs` | Source scan, and the `git ls-files` exclusion |
| `src/regen.rs` | Nested `cargo check` behind the `auto_regen` setting |

### Tests

| File | Relationship |
|------|--------------|
| `tests/smoke_test.rs` | Public signatures; a missing cache directory degrades to a warning |
| `tests/options_test.rs` | How each setting is read, and from where |
| `tests/build_script_directives_test.rs` | Directives produced for every decision |
