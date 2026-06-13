# Read `$COLUMNS` env var as terminal-width fallback before hardcoded default

## Execution State

- **Executor Type:** any
- **Actor:** dev
- **Claimed At:** null
- **Status:** ✅ (Completed)

## Goal

Make `resolve_terminal_width()` in `auto_fit.rs` read the `$COLUMNS` environment variable as an intermediate fallback between `terminal_size` detection and the hardcoded `120` default, so that scripts and CI pipelines can control auto-wrap width without a real TTY.
(Motivated: `resolve_terminal_width()` falls through directly to the `terminal_size` crate and then to `120` — `$COLUMNS` is never consulted; any script or CI job that sets `COLUMNS=80` gets ignored and wide tables silently overflow; Observable: when `COLUMNS=60` is set and auto-wrap is enabled, a table whose natural row width exceeds 60 characters is folded at the 60-column threshold rather than the 120-column default; Scoped: changes limited to `src/formatters/table/auto_fit.rs` — one `std::env::var("COLUMNS")` read with `parse::<u16>()` guard inserted before the `terminal_size` branch; no public API changes; existing behavior when `COLUMNS` is unset is identical to today; Testable: `w3 .test level::3` passes and a new test in `tests/table_auto_wrap.rs` sets `COLUMNS=40`, renders a table wider than 40 chars, and asserts the output is folded)

## In Scope

All paths relative to the crate root (`module/core/data_fmt/`).

**Source — `src/formatters/table/auto_fit.rs`:**
- In `resolve_terminal_width()`: before querying `terminal_size`, attempt `std::env::var("COLUMNS").ok().and_then(|v| v.parse::<u16>().ok())` and return that value if non-zero; otherwise fall through to existing logic

**Tests — `tests/table_auto_wrap.rs`:**
- Add `test_columns_env_var_respected`: serialize access with a mutex (env var mutation is process-global); call `std::env::set_var("COLUMNS", "40")`; build a `TableFormatter` with `auto_wrap(true)` and `TableConfig::unicode_box()`; render a five-column table whose natural row width is ~90 chars; assert the output contains a fold/continuation row or that no output line exceeds 40 chars; restore or remove `COLUMNS` in teardown

Note: `resolve_terminal_width` is `pub(super)` and cannot be called directly from `tests/`. The test must exercise the behavior indirectly through the public `Format::format()` call. Env-var mutation requires test serialization — use a `static Mutex` guard at the top of the test, or add `serial_test = "3.0"` to `[dev-dependencies]` and annotate with `#[serial]`.

## Out of Scope

- `terminal_size` crate call — preserved as-is when `COLUMNS` is unset or unparseable
- Hardcoded `120` fallback — preserved when both `COLUMNS` and `terminal_size` are unavailable
- Any formatter other than `TableFormatter` with `auto_wrap(true)`
- Public API surface — no new methods, no type changes

## Work Procedure

1. Read `src/formatters/table/auto_fit.rs`. Locate `resolve_terminal_width()`.
2. Insert env-var read: `if let Some(w) = std::env::var("COLUMNS").ok().and_then(|s| s.parse::<u16>().ok().filter(|&n| n > 0)) { return w as usize; }` at the top of the fallback chain, before the `terminal_size` branch.
3. Decide on serialization approach for the env-var test: either add `serial_test = "3.0"` to `Cargo.toml` `[dev-dependencies]` or use a `static Mutex` guard inline in the test.
4. Add `test_columns_env_var_respected` to `tests/table_auto_wrap.rs` (create file if it does not exist) using the indirect rendering approach described in In Scope.
5. Run `w3 .test level::3` to confirm clean pass.

## Test Matrix

| Scenario | Assertion |
|----------|-----------|
| `COLUMNS=40`, natural row width ~90 | no output line exceeds 40 chars |
| `COLUMNS` unset, same table | output line width governed by `terminal_size` or 120 (unchanged behavior) |
| `COLUMNS=garbage`, same table | parse fails silently; fallback to existing behavior |

## Closes

null

## Verification Findings

**Finding — Implementation Readiness:**
`resolve_terminal_width` is `pub(super)`, so it cannot be unit-tested by calling it directly from `tests/`. Additionally, env-var mutation is process-global and will race in a parallel test run if not serialized.

Resolution: Test uses the indirect path — call `Format::format()` with `auto_wrap(true)` and assert on rendered line lengths. Env-var mutation is serialized via a `static Mutex` guard or `#[serial]` from `serial_test`. Work Procedure step 3 covers the serialization decision. No rework of scope or goal required; the test approach is executable as-is.

## Verification Record

- **Date:** 2026-06-13
- **Ground truth:** 605/605 nextest pass, 0 clippy warnings
- **Confirming agent:** `src/formatters/table/auto_fit.rs:21-26` — `std::env::var("COLUMNS")` read at Tier 1 before `#[cfg(feature = "terminal_size")]` block and before `120` fallback; parses as `usize` with `trim()` and `n > 0` guard. `tests/terminal_width_test.rs:303-358` — `columns_env_var_controls_terminal_width_ft9` uses `static COLUMNS_TEST_MUTEX`, sets `COLUMNS=40`, renders table, removes var before assertion, asserts max line width ≤ 40; also tests empty/"0"/non-numeric cases all fall through to 120-col fallback. FT-9 ✅.
- **Adversarial agent:** No disqualifying gap found. Tier resolution order confirmed correct. Mutex covers full set/render/remove cycle. All edge cases tested.

| Dimension | Confirming Finding | Adversarial Finding | Verdict |
|-----------|-------------------|---------------------|---------|
| Scope Coherence | Single insertion in `auto_fit.rs:resolve_terminal_width` — one env-var read before `terminal_size` block; no public API changes; no other files modified | No scope boundary violations detected | PASS |
| MOST Goal Quality | Observable met: `COLUMNS=40` → all output lines ≤ 40 chars (asserted at test line 315-317); resolution order Tier 0→1→2→3 confirmed in source lines 16-35; FT-9 ✅ | MOST goal says `COLUMNS=60` but test uses `COLUMNS=40` — functionally equivalent, mechanism is identical; not a failure | PASS |
| Value/YAGNI | Enables scripts and CI pipelines to control auto-wrap width without a real TTY; no speculative additions; env-var parse is guarded (trim/parse/n>0); no new public API | No over-engineering detected | PASS |
| Implementation Readiness | `usize` parse (not `u16` per spec) — semantically more correct, avoids 65535 limit; `remove_var` called before assertion in each sub-case; all 4 cases (valid/empty/zero/non-numeric) tested; `COLUMNS_TEST_MUTEX` serializes all 4 cases within the single test function | No gaps found; env-var removal before assertion confirmed correct; race condition excluded by mutex + nextest process isolation | PASS |
