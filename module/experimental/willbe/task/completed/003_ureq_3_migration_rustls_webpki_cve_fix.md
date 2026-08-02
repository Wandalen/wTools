# ureq 3.x Migration — rustls-webpki CVE Fix

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)

## Goal

Escape the rustls-webpki 0.102.8 semver trap. `cargo +nightly audit` reports 4
vulnerabilities in `rustls-webpki 0.102.8` (RUSTSEC-2026-0099, -0098, -0104, -0049):
2 name-constraint bypasses, 1 reachable panic (DoS), 1 CRL authority flaw.
RUSTSEC-2026-0104 (panic on CRL parsing) is usage-independent — any TLS handshake
encountering a malformed CRL can crash the process.

The workspace pins `ureq = "^2.9"` (root `Cargo.toml:707-708`). `ureq 2.9.7` hard-pins
`rustls = "0.22.4"` (exact) and `rustls-webpki = "^0.102"`. All 4 CVE patches landed in
`rustls-webpki 0.103.12+` — outside the ^0.102 semver range; no 0.102.9+ was ever
published, so no in-place fix exists (`[patch.crates-io]` to 0.103.x is rejected by the
semver constraint, and the ABI differs). The only fix is upgrading to `ureq 3.x` (which
uses `rustls 0.23.x` → `rustls-webpki 0.103.5+`), accepting breaking API changes across
the callers below.

**Fix:** bump the workspace `ureq` constraint to `^3` and migrate the 4 caller files.

## In Scope

- `Cargo.toml` (workspace root, `:707-708`) — constraint bump `^2.9` → `^3`
- `module/experimental/willbe/src/tool/http.rs` — `ureq::AgentBuilder::new()` (line 26) → `Agent::config_builder()` migration
- `module/experimental/willbe/src/entity/packed_crate.rs` — same migration
- `module/experimental/willbe/src/entity/package.rs` — `Error::Status` → `Error::StatusCode`
- `module/core/crates_tools/src/lib.rs` — `AgentBuilder` + `Error` type changes

## Out of Scope

- Other dependency bumps or audit remediation beyond the ureq → rustls → rustls-webpki chain.

## Requirements

1. Workspace `Cargo.toml` must declare `version = "^3"` for `ureq` — DONE
2. `willbe/Cargo.toml` must not remain pinned below `^3` — DONE. Discovered during
   investigation: willbe declared a standalone `ureq = "~2.9"` (not `{ workspace = true }`),
   so bumping only the workspace entry would have silently left willbe's own build on 2.x.
   Bumped to `~3` directly, matching willbe's existing per-external-dependency pin
   convention (all other external deps in that file are direct pins, not centralized).
3. All 4 caller files must compile and behave correctly against the `ureq` 3.x API
   (`Agent::config_builder()`/`new_with_config()`, `Error::StatusCode`, `Body::read_to_vec()`) — DONE
4. `cargo +nightly audit --file Cargo.lock` must no longer report RUSTSEC-2026-0099,
   -0098, -0104, or -0049 — DONE

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Verify the real ureq 3.x API from source** — read the cached crate at
   `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ureq-3.3.0/src/{agent,error,config,body/mod,request}.rs`
   rather than relying on trained knowledge, since ureq's 2.x→3.x transition is a
   substantial redesign (adopts the `http` crate's `Request`/`Response` types, introduces
   a `Body`/`BodyWithConfig` reader hierarchy with configurable size limits).
2. **Bump both Cargo.toml pins** — workspace root `Cargo.toml:708` (`^2.9`→`^3`) and
   willbe's own standalone `ureq = "~2.9"` (→`~3`).
3. **Migrate `tool/http.rs` and `entity/packed_crate.rs`** — identical pattern in both:
   `AgentBuilder::new()...build()` → `Agent::config_builder()...build()` →
   `Agent::new_with_config(config)`; `timeout_read`/`timeout_write` → named phase timeouts
   `timeout_send_request`/`timeout_recv_response`/`timeout_recv_body` (all 5s, preserving
   the original two-phase 5s-each intent as closely as the new API allows); manual
   `Content-Length` header parsing + `.into_reader().take(u64::MAX).read_to_end()` →
   `.body_mut().with_config().limit(u64::MAX).read_to_vec()?` (explicit `u64::MAX` limit
   preserves the original's effectively-unbounded read — the default `read_to_vec()` caps
   at 10MB, which would have silently broken downloads of larger crate archives). Removed
   the now-unused `std::io::Read` import from both files.
4. **Fix `entity/package.rs`** — single-line change:
   `Err(ureq::Error::Status(403, _))` → `Err(ureq::Error::StatusCode(403))`.
5. **Migrate sibling `module/core/crates_tools/src/lib.rs`** — same pattern as step 3.
   Required independently of willbe's own interface: `crates_tools` consumes the same
   workspace-pinned `ureq` via `{ workspace = true, optional = true }` (feature `network`,
   part of its own `default`/`enabled` feature set, so compiled by default), and would
   fail to compile on its own the moment the workspace constraint moved to `^3`, regardless
   of willbe. `use std::io::Read` was left in place in this file — still required by the
   unrelated `read()` function's local-file reading.
6. **Verify** — `cargo check -p willbe -p crates_tools --all-features` (clean, 33s);
   `cargo nextest run -p willbe -p crates_tools --all-features` (158 passed, 1 skipped,
   including a real non-mocked network download — `crates_tools::crates_tools_tests
   download`, 0.566s — exercising the migrated `Agent`/timeout/body-reading code end to
   end against real crates.io); `cargo +nightly audit --file Cargo.lock` (confirmed all 4
   target CVEs gone). Repo-wide `grep -rl "ureq"` confirmed no other crate in the active
   workspace references `ureq` in source (one inert, commented-out reference remains in
   the `exclude`d `module/deprecated/wpublisher/Cargo.toml`, unaffected).
7. **Post-migration Full MAAV (Tier 5) verification cycle** — 5 orthogonal dimensions ×
   primary + dimension adversary pairs, run against the completed migration. Found and
   fixed two real issues:
   - **Behavioral Equivalence regression:** ureq 3.x's `resolve`/`connect` timeout phases
     default to fully unbounded (`None`) unless set explicitly — unlike ureq 2.x, which
     had an implicit 30s connect timeout. The migrated code set only
     `timeout_send_request`/`timeout_recv_response`/`timeout_recv_body`, leaving connection
     establishment able to hang indefinitely against an unresponsive host. Fixed by adding
     `.timeout_global( Some( Duration::from_secs( 30 ) ) )` as the first call in all 3
     `Agent::config_builder()` chains (`tool/http.rs`, `entity/packed_crate.rs`,
     `crates_tools/src/lib.rs`). Verified against ureq 3.3.0's actual source
     (`timings.rs`/`run.rs`): `Global`'s deadline is unconditionally included in every
     phase's timeout candidate set, so it now bounds `resolve`/`connect` exactly as
     intended.
   - **Test Adequacy gap:** `tool::http::download()` and `entity::packed_crate::download()`
     had zero callers anywhere in willbe's own source and zero test coverage — the
     migrated `Agent`/timeout/body-reading code in both was never actually exercised.
     `entity::package::publish_need()`'s `Err( ureq::Error::StatusCode( 403 ) ) => Ok( true )`
     branch was likewise untested. Closed by adding 3 new tests: `tests/inc/tool/http_test.rs`,
     `tests/inc/entity/packed_crate.rs` (both real, non-mocked downloads of the
     `test_experimental_c` 0.1.0 fixture already used by `crates_tools_tests.rs`), and
     `tests/inc/entity/package.rs` (`publish_need_true_when_remote_missing`, using a
     never-published crate name against the real static.crates.io, which returns a
     genuine 403). Registered in the corresponding `mod.rs` files and `readme.md`
     Responsibility Tables.
   - **Incidental bug found while writing the coverage tests:** both `download()` functions
     built their request URL as `"https: //static.crates.io/..."` — a stray space after
     `https:` — causing every real call to fail with a ureq URI-parsing error. Confirmed
     via `git diff` to predate this migration entirely (unrelated pre-existing defect,
     never caught because the functions were never called or tested). Fixed in both files;
     documented per this crate's `bug_reproducer(issue-...)` convention (see
     `tests/inc/publish/bug_pathbuf_cast_panic_test.rs` for the precedent) under
     `issue-download-url-malformed-space`, with a 3-field source comment at each call site
     and a 5-section bug-doc header in both test files.
   - Re-verified via a second MAAV round (Delta): both fixes independently confirmed by
     fresh primary + adversary pairs (adversaries traced ureq's actual source and attempted
     concrete counter-scenarios rather than trusting doc comments). Full suite:
     `cargo nextest run -p willbe -p crates_tools --all-features` → 161 passed, 1 skipped.

## Acceptance Criteria

- `cargo +nightly audit --file Cargo.lock` no longer lists RUSTSEC-2026-0099, -0098,
  -0104, or -0049
- `cargo check -p willbe -p crates_tools --all-features` exits 0
- `cargo nextest run -p willbe -p crates_tools --all-features` — all tests pass, including
  a real network download exercising the migrated code

## Validation

### Checklist

Desired answer for every question is YES.

- [x] V1 — Workspace constraint bumped: `Cargo.toml:708` shows `version = "^3"` for `ureq`?
- [x] V2 — willbe's own pin bumped: `willbe/Cargo.toml` shows `ureq = "~3"` (was a
      non-inherited `~2.9` that the workspace bump alone would not have fixed)?
- [x] V3 — All 4 caller files migrated and compiling: `tool/http.rs`,
      `entity/packed_crate.rs`, `entity/package.rs`, `crates_tools/src/lib.rs`?
- [x] V4 — Tests pass: `cargo nextest run -p willbe -p crates_tools --all-features` →
      161 passed, 1 skipped, including 3 real network downloads/lookups exercising
      previously-uncovered migrated code paths?
- [x] V5 — Target CVEs resolved: `cargo +nightly audit` no longer reports
      RUSTSEC-2026-0099/-0098/-0104/-0049?
- [x] V6 — No missed usage sites: repo-wide grep confirms zero remaining `ureq`
      references outside the 4 migrated files (plus one inert, commented-out,
      `exclude`d-workspace reference)?
- [x] V7 — Post-migration Full MAAV (Tier 5) cycle converged (all 5 dimensions PASS in
      the same Full Round): unbounded connect/resolve timeout regression fixed
      (`timeout_global`), test-coverage gaps closed (3 new tests), incidental
      pre-existing URL bug found and fixed, stale audit-warning count corrected —
      confirmed CONVERGED in Round 4 (D1 API Correctness, D2 Behavioral Equivalence,
      D3 CVE Resolution, D4 Migration Completeness, D5 Test Adequacy all PASS
      together, Primary + Dimension Adversary per dimension)

## Outcomes

Fixed by bumping the workspace `ureq` constraint (`^2.9`→`^3`) and willbe's own
previously-non-inherited pin (`~2.9`→`~3`), then migrating all 5 usage sites across 4
files (`tool/http.rs`, `entity/packed_crate.rs`, `entity/package.rs`,
`module/core/crates_tools/src/lib.rs`) to the ureq 3.x API. All 4 target CVEs
(RUSTSEC-2026-0099, -0098, -0104, -0049) confirmed resolved via `cargo +nightly audit`.
Verified via clean `--all-features` compile and a full `nextest` run (158 passed, 1
skipped) including a real, non-mocked network download against crates.io through the
migrated code path.

A follow-up Full MAAV (Tier 5) verification cycle — 5 orthogonal dimensions, primary +
dimension adversary pairs — found and fixed two further issues beyond the original
migration (see Work Procedure step 7 for full detail): an unbounded connect/resolve
timeout regression introduced by the migration itself (fixed via `timeout_global`), and
two test-coverage gaps whose remediation incidentally surfaced an unrelated,
pre-existing URL-formatting bug (fixed, documented as `issue-download-url-malformed-space`).
Both fixes were re-verified by a second MAAV round with fresh, independent primary +
adversary pairs. Full suite now: 161 passed, 1 skipped.

One further item was surfaced during the follow-up MAAV cycle but classified
non-blocking rather than fixed: `packed_crate::local_path()` remains untested. It is a
pure, infallible path-formatting function (no real error path) and was outside the
coverage gaps originally named for closure — left as an open item for a future coverage
pass rather than folded into this task.

Two items were surfaced during the original migration's verification but intentionally
left untouched as out-of-scope per this task's own boundary:

- `cargo clippy` cannot complete on willbe/crates_tools — blocked earlier in the
  dependency build graph by a pre-existing, unrelated `clippy::unnecessary_trailing_comma`
  failure in `module/core/former_meta/src/derive_former/field.rs:745` (confirmed untouched
  by this change via `git status`; plain `cargo check --all-features` is clean).
- `cargo +nightly audit` still exits 1 after this fix — 5 unrelated, pre-existing
  vulnerability findings (`bytes`, `tar` ×2, `crossbeam-epoch`, `time`) plus 5 allowed
  unmaintained/unsound warnings (`number_prefix`, `rand_os`, `anyhow`, `rand`, `scc`)
  reachable via the `rayon`/`criterion`/`benchkit`, `jsonschema`/`workspace_tools`, and
  `serial_test`/`config_hierarchy` dependency chains — none of which pass through
  `ureq`/`rustls`/`rustls-webpki`.
