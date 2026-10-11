# Invariant: Declared Rerun Inputs

### Scope

- **Purpose**: Keep Cargo from replaying a stale decision, without re-running the check on builds where nothing it reads has changed.
- **Responsibility**: Documents which inputs `sqlx_query` declares to Cargo, which paths it deliberately leaves out, and the gaps that remain.
- **In Scope**: Declared variables and paths, exclusions, the stand-aside exception, known gaps, and historical violations.
- **Out of Scope**: How sources and cache directories are found (→ `invariant/003`), and what is decided from them (→ `feature/001`).

### Invariant Statement

Cargo replays a build script's previous output unless an input it was told about has changed. So, for any two consecutive builds of a consumer crate:

1. **Complete.** If anything the check decided from has changed in between -- `DATABASE_URL` (or the variable `sqlx.toml` names instead), `SQLX_OFFLINE`, `SQLX_OFFLINE_DIR`, `SQLX_QUERY_SOFT_WARN`, `SQLX_QUERY_AUTO_REGEN`, a `.env` file or `sqlx.toml` the settings were read from, a scanned `.rs` file, a SQL file a `query_file` form reads, a cache directory that exists, or a missing `SQLX_OFFLINE_DIR` the warnings say to create -- the second build always re-runs the check.
2. **Quiet.** If only target directories, nested packages, or git-ignored files other than those settings files have changed, the second build never re-runs the check, just as without `sqlx_query` -- unless the warnings say to create a missing `SQLX_OFFLINE_DIR`, which re-runs it on every build until the directory exists.

#### Known Gaps

- **New entries in partly watched directories.** A file or directory created directly in the crate root -- or in any directory that is watched entry by entry -- is noticed only once something else re-runs the check. New modules are still caught, because declaring one edits a watched file.
- **Hidden files in watched directories.** An editor's swap file in `src/` re-runs the check, because Cargo watches the directory as a whole.
- **Cache directories created later.** A cache directory that did not exist at the last run was not declared, so creating one by hand is noticed only once something else re-runs the check. `cargo sqlx prepare` is always noticed, through the `SQLX_OFFLINE=false` it sets, and so is a missing `SQLX_OFFLINE_DIR` the warnings say to create.
- **Settings files created later.** Likewise, a `.env` file -- in the crate directory or any ancestor -- or a `sqlx.toml` that did not exist at the last run is noticed only once something else re-runs the check. Editing or deleting one that existed is always noticed.

### Enforcement Mechanism

Any `cargo:rerun-if-*` directive switches Cargo from watching the whole package to watching only what is declared, so `sqlx_query` declares everything itself, first, in every outcome:

- `cargo:rerun-if-env-changed` for each of the five variables, with the variable `sqlx.toml` names in place of `DATABASE_URL`.
- `cargo:rerun-if-changed` for every `.env` file and `sqlx.toml` the settings were read from (`invariant/003`), each declared once even when the package walk lists it too.
- `cargo:rerun-if-changed` for the package walk's watch list: each top-level entry the walk accepts (`invariant/003`), plus every SQL file a `query_file` form reads, even one outside the package. Cargo watches a directory recursively, by the newest modification time inside it, so a directory with an excluded entry anywhere below it is replaced by its own entries, recursively: an excluded entry is never declared, not even through a parent.
- `cargo:rerun-if-changed` for every cache directory in the lookup order that exists. A missing path counts as changed on every build, so missing directories are left out -- except a `SQLX_OFFLINE_DIR` the warnings say to create, which only stale sites make them do: re-running until it exists is the point, and every build compiling a stale query fails anyway.

The package walk skips every cache directory, and none is declared when `SQLX_OFFLINE` reads as online: such a build compiles live and rewrites each entry it compiles, so declaring them would make every build re-run the check on the next one.

### Violation Consequences

- **An undeclared input** leaves the previous decision in force. Seen before the fix: a `SQLX_QUERY_SOFT_WARN=1` retry after a failed build still had `SQLX_OFFLINE=true` forced, and failed again; entries written by `cargo sqlx prepare` went unnoticed; a deleted entry went unnoticed too -- nothing was rebuilt -- so local builds kept passing while a clean build failed.
- **A declared path that changes during the build** -- a target directory inside the package, a log file -- re-runs the check on every build, and the crate is never fresh.

### Historical Violations

| Violation | Regression test |
|-----------|-----------------|
| No rerun directives at all: environment changes and `.sqlx/` edits never re-ran the check, and Cargo replayed the old decision | `rerun_triggers_mre_cover_every_input` |
| A target directory inside the package, not named `target`, was scanned and watched, re-running the check on every build | `target_dir_with_cachedir_tag_mre_is_skipped` |

### APIs

| File | Relationship |
|------|--------------|
| [api/001_ensure_offline_mode.md](../api/001_ensure_offline_mode.md) | Rerun directives every operation emits first |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_offline_mode_enforcement.md](../feature/001_offline_mode_enforcement.md) | Decision that must never be replayed stale |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/003_sqlx_resolution_parity.md](../invariant/003_sqlx_resolution_parity.md) | Finds the sources and cache directories declared here |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs` | Variables and cache directories declared; the stand-aside exception |
| `src/options.rs` | The `.env` files and `sqlx.toml` recorded for declaration |
| `src/scan.rs` | Watch list and its exclusions |

### Tests

| File | Relationship |
|------|--------------|
| `tests/build_script_directives_test.rs` | Every input declared; no cache directory declared when standing aside; a missing `SQLX_OFFLINE_DIR` declared only while named |
| `tests/options_test.rs` | Which `.env` files and `sqlx.toml` are recorded |
| `tests/package_walk_test.rs` | Watch list: exclusions, directories watched entry by entry |
| `tests/offline_stale_warning_test.rs` | Real builds: a soft-warn retry, a deleted entry and a created `SQLX_OFFLINE_DIR` act on the next build |
| `tests/manual/readme.md` | Cargo's own freshness, observed build by build |
