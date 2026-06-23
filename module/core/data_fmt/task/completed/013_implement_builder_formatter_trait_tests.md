# 013 — Implement test code for builder, formatter, and trait specs

## MOST Goal

Implement Rust test functions for 88 test cases defined in 14 newly created test spec files (builder/001-002, formatter/001-010, trait/001,003), bringing these spec cases from documented-but-unimplemented to exercised-and-passing.

- **Motivated:** 14 test spec files were created during the docs normalization session but have zero backing Rust test code. Task 002 proved that spec-driven test implementation catches real bugs (BUG-001 through BUG-013 were found during that effort). These 88 cases cover formatter output contracts, builder API correctness, and trait dispatch — the three behavioral domains with the least automated coverage.
- **Observable:** Each spec file's Case Index `Status` column changes from `⏳` to `✅` for every implemented case. `cargo nextest run --all-features` includes new test functions. No new `⏳` markers remain in any of the 14 spec files.
- **Scoped:** Only creates/modifies `.rs` test files under `tests/` and updates `⏳`→`✅` in the 14 `tests/docs/` spec files. No `src/` changes, no `docs/` changes.
- **Testable:** `cargo nextest run --all-features` passes with new tests included; `grep -r '⏳' tests/docs/builder/[0-9]*.md tests/docs/formatter/[0-9]*.md tests/docs/trait/001_format.md tests/docs/trait/003_table_shaped_view.md` returns 0 matches after completion.

## In Scope

Paths relative to crate root (`module/core/data_fmt/`).

**Builder specs (16 cases):**

| Spec File | Cases | Test File |
|-----------|------:|-----------|
| `tests/docs/builder/001_row_builder.md` | BL-1..BL-8 | `tests/builder_row_test.rs` (new or extend existing) |
| `tests/docs/builder/002_tree_builder.md` | BL-9..BL-16 | `tests/builder_tree_test.rs` (new or extend existing) |

**Formatter specs (60 cases):**

| Spec File | Cases | Test File |
|-----------|------:|-----------|
| `tests/docs/formatter/001_table_formatter.md` | 7 cases (FM-1..FM-7) | `tests/formatters.rs` or dedicated file |
| `tests/docs/formatter/002_expanded_formatter.md` | 5 cases (FM-5..FM-9) | `tests/expanded_tests.rs` |
| `tests/docs/formatter/003_tree_formatter.md` | 5 cases (FM-9..FM-13) | `tests/tree_tests.rs` |
| `tests/docs/formatter/004_logfmt_formatter.md` | 5 cases (FM-13..FM-17) | `tests/logfmt_tests.rs` |
| `tests/docs/formatter/005_json_formatter.md` | 5 cases (FM-17..FM-21) | `tests/json_tests.rs` |
| `tests/docs/formatter/006_yaml_formatter.md` | 5 cases (FM-22..FM-26) | `tests/yaml_tests.rs` |
| `tests/docs/formatter/007_toml_formatter.md` | 5 cases (FM-27..FM-31) | `tests/toml_tests.rs` |
| `tests/docs/formatter/008_html_formatter.md` | 7 cases (FM-32..FM-38) | `tests/html_tests.rs` |
| `tests/docs/formatter/009_sql_formatter.md` | 8 cases (FM-39..FM-46) | `tests/sql_tests.rs` |
| `tests/docs/formatter/010_text_formatter.md` | 8 cases (FM-47..FM-54) | `tests/text_tests.rs` |

Note: FM-N IDs overlap across specs 001-005 (each spec was authored independently). Each spec's cases are unique within that spec; the FM-N prefix is file-scoped, not globally unique. Implementers should use the spec filename + case ID as the compound key (e.g., `formatter_001_fm_03`).

**Trait specs (12 cases):**

| Spec File | Cases | Test File |
|-----------|------:|-----------|
| `tests/docs/trait/001_format.md` | TR-1..TR-6 | `tests/unified_format_trait.rs` (extend) |
| `tests/docs/trait/003_table_shaped_view.md` | TR-7..TR-12 | `tests/data.rs` or `tests/formatters.rs` (extend) |

## Out of Scope

- Creating new spec files (already done; Task 012 scope for remaining 43)
- Source code changes to `src/` (this is a test-only task)
- Documentation changes to `docs/` (already normalized)
- Spec files in algorithm/, api/, feature/, invariant/ (already implemented by Task 002)
- Spec files for data_structure/, input_model/, input_type/, pattern/, variant/ (not yet created; Task 012 scope)
- Bug fixes discovered during testing (separate BUG-NNN if found)

## Requirements

- Each test function name includes its spec file number and case ID as compound key (e.g., `fn builder_001_bl_01_single_row()`, `fn formatter_006_fm_22_yaml_standard()`)
- Each test function contains a `// test_kind: spec_case(BL-1)` marker comment for traceability
- Tests use real implementations — no mocking
- Tests compile and pass under `--all-features`
- Feature-gated formatters (yaml, toml, json, html, sql, text, logfmt) require the corresponding feature flag in test cfg attributes
- After each spec file's cases are fully implemented, update the spec's Case Index `Status` column from `⏳` to `✅`

## Work Procedure

1. **Read spec files** — read each of the 14 test spec files to understand the exact Given/When/Then contracts
2. **Implement builder tests** — create or extend test files for BL-1..BL-16; run `cargo nextest run -E 'test(bl_)' --all-features`
3. **Implement formatter tests** — for each formatter 001-010: read spec, create/extend test file, implement all FM-N cases, run targeted nextest filter
4. **Implement trait tests** — extend existing test files for TR-1..TR-12; run targeted nextest filter
5. **Update spec statuses** — for each implemented case, change `⏳` to `✅` in the spec's Case Index table
6. **Full test suite** — run `cargo nextest run --all-features` to confirm no regressions; run `cargo test --doc --all-features` for doc tests
7. **Verify zero remaining** — `grep -r '⏳' tests/docs/builder/[0-9]*.md tests/docs/formatter/[0-9]*.md tests/docs/trait/001_format.md tests/docs/trait/003_table_shaped_view.md` returns empty

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| 2-col, 1-row TableView | `TableConfig::plain()` via Format trait | Output contains headers and cell values, no borders |
| 2-col, 1-row TableView | `TableConfig::bordered()` via Format trait | Output contains box-drawing border characters |
| TreeNode with nested children | `TreeFormatter::hierarchical()` | Output contains box-drawing connectors and indentation |
| Single header, 1-row TableView | `LogfmtFormatter::new()` | Output is `key=value` on single line |
| 2-col, 1-row TableView | `JsonFormatter::pretty()` | Output is valid JSON array with indentation |
| 2-col, 1-row TableView | `YamlFormatter::new()` | Output is YAML sequence of mappings |
| 2-col, 1-row TableView | `TomlFormatter::new()` | Output is TOML array of inline tables |
| 2-col, 1-row TableView | `HtmlFormatter::with_variant(Minimal)` | Output contains `<table>`, `<th>`, `<td>` elements |
| 2-col, 1-row TableView | `SqlFormatter::new()` | Output is `INSERT INTO` with quoted identifiers |
| 2-col, 2-row TableView | `TextFormatter::new(Bullets)` | Output has bullet prefixes per row |
| RowBuilder fluent chain | `RowBuilder::new().add_headers(...).add_row(...)` | `.build()` produces correct `TableView` |
| TreeBuilder nested paths | `TreeBuilder::new().add_path(...)` | Intermediate nodes auto-created |
| FormatError::InvalidData | Error variant construction | `Display` output contains "Invalid data:" |
| TreeNode uniform structure | `is_table_shaped()` trait method | Returns `true` for uniform column names |
| Empty table (0 rows) | Any formatter | No panic; output is empty or header-only |
| Special chars in cell values | JSON/SQL/HTML formatters | Proper escaping (JSON `\"`, SQL `''`, HTML `&amp;`) |

## Acceptance Criteria

- AC-001: All 88 test cases have corresponding Rust test functions (1:1 mapping)
- AC-002: `cargo nextest run --all-features` passes with all new tests included
- AC-003: `grep -r '⏳' tests/docs/builder/[0-9]*.md tests/docs/formatter/[0-9]*.md tests/docs/trait/001_format.md tests/docs/trait/003_table_shaped_view.md` returns 0 matches
- AC-004: Each test function has a `// test_kind: spec_case(XX-N)` marker comment
- AC-005: No `src/` files are modified
- AC-006: No mocking is used in any test

## Validation

### Checklist

Desired answer for every question is YES.

**Completeness**
- [x] Does each of the 88 spec cases have a corresponding test function?
- [x] Do all 14 spec files show `✅` for every case in their Case Index?
- [x] Does `cargo nextest run --all-features` pass?

**Traceability**
- [x] Does every new test function contain a `// test_kind: spec_case(XX-N)` marker?
- [x] Does the test function name include its case ID?

**No side effects**
- [x] Are no files under `src/` modified?
- [x] Are no files under `docs/` modified (except `tests/docs/` status updates)?
- [x] Are no mocks used anywhere?

**Feature gating**
- [x] Are serde-dependent formatter tests gated on `#[cfg(feature = "serde_support")]` or the specific format feature?
- [x] Do all tests compile with `--all-features` and fail gracefully without?

### Measurements

**M1 — New test function count**
Command: `grep -r 'test_kind: spec_case' tests/ | wc -l`
Before: 0 (for builder/formatter/trait specs). Expected: 88.

**M2 — Remaining pending markers in target specs**
Command: `grep -r '⏳' tests/docs/builder/[0-9]*.md tests/docs/formatter/[0-9]*.md tests/docs/trait/001_format.md tests/docs/trait/003_table_shaped_view.md | wc -l`
Before: 88. Expected: 0.

**M3 — Full test suite**
Command: `cargo nextest run --all-features`
Expected: all pass, 0 failures.

**M4 — No source changes**
Command: `git diff --name-only src/`
Expected: empty.

### Invariants

- [x] I1 — spec parity: every `⏳` in the 14 spec files has exactly one test function
- [x] I2 — decisions: `task/decisions.md` exists
- [x] I3 — no mocks: `grep -r 'mock\|Mock\|#\[mock\]' tests/ | grep -v '// ' | wc -l` returns 0

### Anti-faking checks

- AF1: Run `cargo nextest run -E 'test(bl_)' --all-features` — must show ≥16 test results
- AF2: Run `cargo nextest run -E 'test(fm_)' --all-features` — must show ≥60 test results
- AF3: Run `cargo nextest run -E 'test(tr_)' --all-features` — must show ≥12 test results
- AF4: Spot-check 3 random test functions — each must assert on the specific behavior described in its spec case's Then clause

## Related Documentation

- [`tests/docs/builder/001_row_builder.md`](../tests/docs/builder/001_row_builder.md) — BL-1..BL-8 spec
- [`tests/docs/builder/002_tree_builder.md`](../tests/docs/builder/002_tree_builder.md) — BL-9..BL-16 spec
- [`tests/docs/formatter/001_table_formatter.md`](../tests/docs/formatter/001_table_formatter.md) — FM-1..FM-7 spec
- [`tests/docs/formatter/002_expanded_formatter.md`](../tests/docs/formatter/002_expanded_formatter.md) — FM-5..FM-9 spec
- [`tests/docs/formatter/003_tree_formatter.md`](../tests/docs/formatter/003_tree_formatter.md) — FM-9..FM-13 spec
- [`tests/docs/formatter/004_logfmt_formatter.md`](../tests/docs/formatter/004_logfmt_formatter.md) — FM-13..FM-17 spec
- [`tests/docs/formatter/005_json_formatter.md`](../tests/docs/formatter/005_json_formatter.md) — FM-17..FM-21 spec
- [`tests/docs/formatter/006_yaml_formatter.md`](../tests/docs/formatter/006_yaml_formatter.md) — FM-22..FM-26 spec
- [`tests/docs/formatter/007_toml_formatter.md`](../tests/docs/formatter/007_toml_formatter.md) — FM-27..FM-31 spec
- [`tests/docs/formatter/008_html_formatter.md`](../tests/docs/formatter/008_html_formatter.md) — FM-32..FM-38 spec
- [`tests/docs/formatter/009_sql_formatter.md`](../tests/docs/formatter/009_sql_formatter.md) — FM-39..FM-46 spec
- [`tests/docs/formatter/010_text_formatter.md`](../tests/docs/formatter/010_text_formatter.md) — FM-47..FM-54 spec
- [`tests/docs/trait/001_format.md`](../tests/docs/trait/001_format.md) — TR-1..TR-6 spec
- [`tests/docs/trait/003_table_shaped_view.md`](../tests/docs/trait/003_table_shaped_view.md) — TR-7..TR-12 spec
- Related: [002 — Fill test coverage gaps](../completed/002_fill_test_coverage_gaps.md) (Case E: 002 implemented tests from algorithm/invariant/feature specs; this task covers builder/formatter/trait specs)
- Related: [012 — Create test surface specs](012_create_test_surface_specs.md) (Case E: 012 creates spec files; this task implements Rust test code from those specs)

## Execution State

- **State:** ✅ (Done)
- **ID:** 013
- **Slug:** implement_builder_formatter_trait_tests
- **Executor:** any
- **Priority:** 4
- **Value:** 8
- **Easiness:** 6
- **Safety:** 9
- **Dir:** `module/core/data_fmt`
- **Closes:** null
- **Reopen Count:** 0

## History

- **[2026-06-23]** `CREATED` — 14 test spec files (builder/2, formatter/10, trait/2) created during docs normalization session define 88 test cases with zero backing Rust code; this task implements all 88.
- **[2026-06-23]** `SUBMITTED` — MAAV Verification Gate passed (4/4 dimensions).
- **[2026-06-23]** `DONE` — All 88 tests implemented across 14 test files; Level 3 PASS 722+74+0; M1=88, M2=0, I3=0.

## Verification Record

| Dimension | Result | Agent |
|-----------|--------|-------|
| Scope Coherence | PASS | sonnet (re-run after FM-N overlap fix) |
| MOST Goal Quality | PASS | sonnet |
| Value / YAGNI | PASS | sonnet |
| Implementation Readiness | PASS | sonnet (re-run after FM-N overlap fix + I2 confirmed) |

Initial run found 2 failures: (1) FM-N case ID ranges in In Scope table appeared to overlap globally — fixed by adding per-file counts and compound-key note clarifying IDs are file-scoped; (2) I2 invariant flagged as unsatisfiable — confirmed `task/decisions.md` already exists. Re-run: all 4 PASS.
