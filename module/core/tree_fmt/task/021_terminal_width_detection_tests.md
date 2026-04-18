# Test terminal width detection three-tier fallback

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** 🎯 (Available)

## Goal

Add comprehensive tests for the terminal width detection three-tier fallback (`resolve_terminal_width`) to verify that explicit override takes precedence, the `terminal_size` feature gate compiles correctly, the hardcoded 120-column fallback applies when no override is set, and edge cases (zero width clamping) are handled. (Motivated: terminal detection is the entry point for the entire auto-fit pipeline and has no dedicated tests; Observable: new test file `tests/terminal_width_test.rs` with passing tests; Scoped: only `resolve_terminal_width` behavior and its integration with `TableConfig::terminal_width`; Testable: `cargo nextest run --test terminal_width_test --all-features`)

## In Scope

- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/tests/terminal_width_test.rs` — new test file
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs` § `resolve_terminal_width` (lines 870-884) — function under test
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs` § `terminal_width` field and `term_width` accessor — config plumbing

## Out of Scope

- Documentation updates (already completed by doc_tsk)
- Auto-wrap budget allocation logic (covered by task 019)
- Column folding logic (covered by task 020)
- Modifying `resolve_terminal_width` implementation (test-only task)

## Description

The `resolve_terminal_width` method in `TableFormatter` implements a three-tier fallback for determining effective terminal width: (1) explicit `terminal_width` config value, (2) `terminal_size` crate query when the feature is enabled, (3) hardcoded 120-column default.

Currently no tests exercise this fallback chain. The explicit override path (Tier 1) and the hardcoded fallback (Tier 3) are testable without platform-specific mocking. The `terminal_size` feature gate (Tier 2) can be verified by compiling with `--features terminal_size` and checking that the code compiles — the actual TTY detection can't be unit-tested deterministically in CI since stdout is typically redirected.

The tests should verify:
- Default config (`terminal_width: None`, no TTY) produces output consistent with 120-column budget
- Explicit `terminal_width(Some(80))` constrains auto-wrap to 80 columns
- `terminal_width(Some(0))` clamps to 1 (not division-by-zero)
- `terminal_width(Some(40))` with wide content triggers wrapping at 40 columns
- Auto-wrap disabled (`auto_wrap(false)`) bypasses terminal detection entirely

## Requirements

- All work must strictly adhere to all applicable rulebooks
  (discover via `kbase .rulebooks`)
- Tests must use real `TableFormatter` rendering, not mock internals
- Tests must use explicit `terminal_width` for determinism (Tier 1), since Tier 2/3 depend on runtime environment
- Test file must have doc comment explaining scope and relationship to `feature/005_auto_fit.md`

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note test_organization.rulebook.md constraints on test file location, doc comments, and naming.
2. **Read documentation** — Read `docs/feature/005_auto_fit.md § Terminal Width Detection` as source of truth for expected behavior.
3. **Read source code** — Read `src/formatters/table.rs` lines 866-1006 (`resolve_terminal_width` through `compute_column_budgets`) and `src/config.rs` lines 230-268 (terminal_width field, default, and accessor).
4. **Write failing tests** — Create `tests/terminal_width_test.rs` with test cases from the Test Matrix below. All tests should use `TableFormatter::with_config(...)` with explicit `terminal_width` and real data, then assert on output characteristics (line count, max line width).
5. **Run tests** — `RUSTFLAGS="-D warnings" cargo nextest run --test terminal_width_test --all-features`. All tests must pass (they exercise existing working code).
6. **Validate** — Run `w3 .test level::3`. All tests must pass including new ones.
7. **Walk Validation Checklist** — check every item. Every answer must be YES.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| 5-column table, natural width ~150 chars | `terminal_width: None`, `auto_wrap: true` | Output lines ≤ 120 chars (Tier 3 fallback) |
| 5-column table, natural width ~150 chars | `terminal_width: Some(80)` | Output lines ≤ 80 chars (Tier 1 override) |
| 5-column table, natural width ~150 chars | `terminal_width: Some(40)` | Output lines ≤ 40 chars, cells wrap aggressively |
| 5-column table, natural width ~150 chars | `terminal_width: Some(0)` | No panic; width clamped to 1 |
| 5-column table, natural width ~60 chars | `terminal_width: Some(80)` | No wrapping needed; output unchanged |
| 5-column table, natural width ~150 chars | `auto_wrap: false` | Output lines may exceed any terminal width |
| CSV config with wide data | `terminal_width: Some(80)`, `csv()` | No wrapping (CSV bypasses auto-fit) |

## Acceptance Criteria

- `tests/terminal_width_test.rs` exists with all 7 test cases from Test Matrix
- All test cases pass under `cargo nextest run --test terminal_width_test --all-features`
- No test uses mocking — all tests render real tables via `TableFormatter`
- Each test asserts on measurable output properties (max line width, line count, or exact content)
- Full test suite passes at Level 3 (`w3 .test level::3`)

## Validation

### Checklist

Desired answer for every question is YES.

**Functional correctness**
- [ ] Does explicit `terminal_width(Some(80))` produce output with all lines ≤ 80 chars?
- [ ] Does default config (no terminal_width) produce output consistent with 120-column budget?
- [ ] Does `terminal_width(Some(0))` avoid panic?
- [ ] Does `auto_wrap(false)` bypass terminal-aware wrapping entirely?
- [ ] Does CSV config bypass auto-fit regardless of terminal_width?

**Test quality**
- [ ] Does `tests/terminal_width_test.rs` exist and contain all 7 test scenarios?
- [ ] Does each test use real `TableFormatter` rendering (no mocks)?
- [ ] Does the test file have a doc comment explaining its scope?

**Out of Scope confirmation**
- [ ] Are `docs/` files unchanged by this task?
- [ ] Is `src/formatters/table.rs` unchanged by this task?
- [ ] Is `src/config.rs` unchanged by this task?

### Measurements

**M1 — Test count**
Command: `cargo nextest run --test terminal_width_test --all-features 2>&1 | grep 'test result'`
Before: test file doesn't exist. Expected: `test result: ok. 7 passed`. Deviation: fewer tests or any failures.

**M2 — Full suite**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --all-features 2>&1 | tail -1`
Before: all existing tests pass. Expected: `test result: ok`. Deviation: any failure.

### Anti-faking checks

**AF1 — Tests use real formatter**
Check: `grep -c "TableFormatter" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/tests/terminal_width_test.rs`
Expected: ≥ 7. Why: each test must construct a real TableFormatter, not mock the detection.

**AF2 — Tests set explicit terminal_width**
Check: `grep -c "terminal_width" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/tests/terminal_width_test.rs`
Expected: ≥ 5. Why: most test scenarios require explicit width override for determinism.

**AF3 — No source code modifications**
Check: `git diff --name-only -- src/ | wc -l`
Expected: 0 (within this task's changes). Why: this is a test-only task; source changes indicate scope creep.

## Outcomes

[Empty — populated upon task completion]
