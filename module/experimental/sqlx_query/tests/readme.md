# Tests

### Scope

#### Responsibilities

Prove that `sqlx_query` finds every sqlx query site, keys it and looks it up exactly as sqlx does, declares every input to Cargo, and emits the right directives for each environment -- through fast checks on temporary packages, and through real nested Cargo builds against a real SQLite database.

#### In Scope

- Site scanning, cache-file naming, cache-directory lookup, reading settings from the environment, `.env` files and `sqlx.toml`, and the package walk with its watch list.
- Every directive and warning `build_script_directives` produces.
- Real builds: a fresh cache, fail-fast, the soft-warn repair and a missing cache directory, `cargo sqlx prepare`'s environment, and regeneration without deadlock.
- The manual plan for what automated tests cannot drive: `sqlx-cli`, sqlx itself reading `.env` and `sqlx.toml`, workspace layouts, and Cargo's own freshness.

#### Out of Scope

- sqlx's macro behavior and SQL validation, which sqlx's own suite covers.
- Performance: the crate has no benchmarks.

### Organization Principles

Tests are grouped by domain -- the part of the decision they pin -- never by method. Every test file opens with a Test Matrix of its cases, including the ones deliberately covered elsewhere, followed by `#![ cfg( feature = "enabled" ) ]`: the workspace CI also builds the tests with `--no-default-features`, where the crate exports nothing. Two kinds of file share the directory:

- **Fast files** build throwaway packages in temporary directories and call `build_script_directives` directly; the only processes they start are `cargo locate-project`, `git`, and -- in one `auto_regen` test -- a nested `cargo check` of a dependency-less package, which fails within a second; another re-runs that test through the same test binary, to see what it prints.
- **Real-build files** (`offline_*_test.rs`) compile a fixture crate that depends on this crate and on sqlx, against a real SQLite database. They are `#[ignore]`d, and run with `--run-ignored all`.

### Directory Structure

```text
tests/
├── readme.md
├── build_script_directives_test.rs
├── cache_dir_test.rs
├── call_site_scan_test.rs
├── golden_fixture_hash_canary_test.rs
├── hash_test.rs
├── offline_fresh_cache_test.rs
├── offline_prepare_compat_test.rs
├── offline_regen_coverage_test.rs
├── offline_regen_deadlock_guard_test.rs
├── offline_stale_warning_test.rs
├── options_test.rs
├── package_walk_test.rs
├── smoke_test.rs
├── fixtures/
├── manual/
└── support/
```

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Public API shape; missing cache degrades to warning |
| `options_test.rs` | Reading settings from environment, `.env` and `sqlx.toml` |
| `hash_test.rs` | Cache-entry names from independently computed digests |
| `golden_fixture_hash_canary_test.rs` | Real sqlx-written entry name, re-derived independently |
| `call_site_scan_test.rs` | Which macro calls are sites, keyed by which SQL |
| `package_walk_test.rs` | Which files are scanned and which paths watched |
| `cache_dir_test.rs` | Where entries are looked up and written |
| `build_script_directives_test.rs` | Directives and warnings for every decision |
| `offline_fresh_cache_test.rs` | Real build: a fresh cache builds offline, silently |
| `offline_stale_warning_test.rs` | Real builds: fail-fast, soft-warn repair, deleted entry, missing cache dir |
| `offline_prepare_compat_test.rs` | Real build under `cargo sqlx prepare`'s environment |
| `offline_regen_deadlock_guard_test.rs` | Real regeneration finishes within a bound |
| `offline_regen_coverage_test.rs` | Real regeneration repairs queries outside the library |
| `fixtures/` | Real sqlx cache entry for the hash canary |
| `manual/` | Manual test plan and fixture-crate script |
| `support/` | Temporary packages, fixture crates, shared assertions |

### Domain Map

| Domain | Files | Doc |
|--------|-------|-----|
| Public API | `smoke_test.rs` | `docs/api/001` |
| Settings and where they are read | `options_test.rs` | `docs/api/001`, `docs/invariant/003`, `docs/invariant/004` |
| Cache-file names | `hash_test.rs`, `golden_fixture_hash_canary_test.rs` | `docs/invariant/001` |
| Sites and SQL text | `call_site_scan_test.rs` | `docs/invariant/003` |
| Package walk and watch list | `package_walk_test.rs` | `docs/invariant/003`, `docs/invariant/004` |
| Cache lookup | `cache_dir_test.rs` | `docs/invariant/003` |
| Decisions and messages | `build_script_directives_test.rs` | `docs/feature/001`, `docs/feature/002`, `docs/invariant/004` |
| Real builds | `offline_*_test.rs` | `docs/feature/001`, `docs/feature/002`, `docs/invariant/002` |
| Real tools and layouts | `manual/readme.md` | all of the above |

### Adding Tests

1. Does the behavior only show with a real compiler -- sqlx writing an entry, a build failing on sqlx's error? Put it in an `offline_*_test.rs` file, `#[ignore]`d, using `support::FixtureCrate`. Otherwise write a fast test with `support::PackageDir` and `build_script_directives`.
2. Pick the file from the Domain Map, and add a row to its Test Matrix.
3. Only `sqlx-cli`, sqlx itself reading a `.env` file or `sqlx.toml`, a multi-crate layout, or Cargo's freshness across builds can show it? Add a scenario to `manual/readme.md`.
4. No domain fits? Add a `<domain>_test.rs`, with rows here and in the Domain Map.

### Naming Conventions

- Files are `<domain>_test.rs`; real-build files start with `offline_`.
- Test names state the expected behavior: `stale_site_still_forces_offline`.
- A regression test has `_mre_` in its name and a `// test_kind: mre` marker, and its doc comment has five sections: Root Cause, Why Not Caught, Fix Applied, Prevention, Pitfall. The code it pins carries a `Fix(<test name>)` comment.

### Special Considerations

- **Running.** `cargo nextest run -p sqlx_query --all-features` runs the fast tests; add `--run-ignored all` for the real builds, which take minutes on a cold cache. The deadlock guard needs the `auto_regen` feature, which `--all-features` enables.
- **No mocks.** Real temporary packages, a real SQLite database, and real `cargo` and `git` processes -- never stand-ins.
- **Bounded builds.** Every real build runs under `support::BUILD_BOUND`: a build that outlives it, such as a nested-build deadlock, is killed and fails its test instead of hanging the suite.
- **Environment.** No test changes its own process environment; settings are passed as values (`Options::resolve`) or to child processes, so tests run in parallel safely.
- **git.** `package_walk_test.rs` creates a repository in a temporary directory to exercise git-ignore handling, so `git` must be on `PATH`.
