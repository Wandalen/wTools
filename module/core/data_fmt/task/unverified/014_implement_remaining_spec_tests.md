# 014 — Implement test code for remaining 168 spec cases

## MOST Goal

Implement Rust test functions for 168 test cases defined in 43 spec files across 6 entity types (data_structure/1, input_model/2, input_type/2, pattern/4, trait/1, variant/33), bringing these spec cases from documented-but-unimplemented to exercised-and-passing.

- **Motivated:** Task 013 implemented 88 tests for builder/formatter/trait specs and proved the workflow effective. These 168 cases cover variant-level output contracts (132 cases across 33 config variants), input model invariants (8), input type contracts (8), architectural patterns (12), data structure properties (4), and trait migration semantics (4). Variant tests are the most numerous because each formatter config variant has its own spec file with 4 cases.
- **Observable:** Each spec file's Case Index `Status` column changes from `⏳` to `✅` for every implemented case. `cargo nextest run --all-features` includes new test functions. No new `⏳` markers remain in any of the 43 spec files.
- **Scoped:** Only creates/modifies `.rs` test files under `tests/` and updates `⏳`→`✅` in the 43 `tests/docs/` spec files. No `src/` changes, no `docs/` changes.
- **Testable:** `cargo nextest run --all-features` passes with new tests included; `grep -r '⏳' tests/docs/data_structure/ tests/docs/input_model/ tests/docs/input_type/ tests/docs/pattern/ tests/docs/trait/002_table_shaped_formatter.md tests/docs/variant/` returns 0 matches after completion.

## In Scope

Paths relative to crate root (`module/core/data_fmt/`).

**Data structure specs (4 cases):**

| Spec File | Cases | Test File |
|-----------|------:|-----------|
| `tests/docs/data_structure/001_variant_attributes.md` | 4 | `tests/data_structure_test.rs` (new) |

**Input model specs (8 cases):**

| Spec File | Cases | Test File |
|-----------|------:|-----------|
| `tests/docs/input_model/001_tabular.md` | 4 | `tests/input_model_tabular_test.rs` (new) |
| `tests/docs/input_model/002_hierarchical.md` | 4 | `tests/input_model_hierarchical_test.rs` (new) |

**Input type specs (8 cases):**

| Spec File | Cases | Test File |
|-----------|------:|-----------|
| `tests/docs/input_type/001_table_view.md` | 4 | `tests/input_type_table_view_test.rs` (new) |
| `tests/docs/input_type/002_tree_node.md` | 4 | `tests/input_type_tree_node_test.rs` (new) |

**Pattern specs (12 cases):**

| Spec File | Cases | Test File |
|-----------|------:|-----------|
| `tests/docs/pattern/001_three_layer_architecture.md` | 3 | `tests/pattern_architecture_test.rs` (new) |
| `tests/docs/pattern/002_design_principles.md` | 3 | `tests/pattern_design_test.rs` (new) |
| `tests/docs/pattern/003_formatter_design.md` | 3 | `tests/pattern_formatter_test.rs` (new) |
| `tests/docs/pattern/004_config_builder_pattern.md` | 3 | `tests/pattern_config_builder_test.rs` (new) |

**Trait specs (4 cases):**

| Spec File | Cases | Test File |
|-----------|------:|-----------|
| `tests/docs/trait/002_table_shaped_formatter.md` | 4 | `tests/trait_table_shaped_formatter_test.rs` (new) |

**Variant specs (132 cases, 33 files × 4 cases each):**

| Spec File | Cases | Test File |
|-----------|------:|-----------|
| `tests/docs/variant/001_table_plain.md` | 4 | `tests/variant_001_table_plain_test.rs` |
| `tests/docs/variant/002_table_minimal.md` | 4 | `tests/variant_002_table_minimal_test.rs` |
| `tests/docs/variant/003_table_bordered.md` | 4 | `tests/variant_003_table_bordered_test.rs` |
| `tests/docs/variant/004_table_markdown.md` | 4 | `tests/variant_004_table_markdown_test.rs` |
| `tests/docs/variant/005_table_grid.md` | 4 | `tests/variant_005_table_grid_test.rs` |
| `tests/docs/variant/006_table_unicode_box.md` | 4 | `tests/variant_006_table_unicode_box_test.rs` |
| `tests/docs/variant/007_table_csv.md` | 4 | `tests/variant_007_table_csv_test.rs` |
| `tests/docs/variant/008_table_tsv.md` | 4 | `tests/variant_008_table_tsv_test.rs` |
| `tests/docs/variant/009_table_compact.md` | 4 | `tests/variant_009_table_compact_test.rs` |
| `tests/docs/variant/010_expanded_postgres_style.md` | 4 | `tests/variant_010_expanded_postgres_test.rs` |
| `tests/docs/variant/011_expanded_property_style.md` | 4 | `tests/variant_011_expanded_property_test.rs` |
| `tests/docs/variant/012_tree_hierarchical.md` | 4 | `tests/variant_012_tree_hierarchical_test.rs` |
| `tests/docs/variant/013_tree_aligned.md` | 4 | `tests/variant_013_tree_aligned_test.rs` |
| `tests/docs/variant/014_tree_aggregated.md` | 4 | `tests/variant_014_tree_aggregated_test.rs` |
| `tests/docs/variant/015_logfmt_standard.md` | 4 | `tests/variant_015_logfmt_test.rs` |
| `tests/docs/variant/016_json_pretty.md` | 4 | `tests/variant_016_json_pretty_test.rs` |
| `tests/docs/variant/017_json_compact.md` | 4 | `tests/variant_017_json_compact_test.rs` |
| `tests/docs/variant/018_yaml_standard.md` | 4 | `tests/variant_018_yaml_test.rs` |
| `tests/docs/variant/019_toml_standard.md` | 4 | `tests/variant_019_toml_test.rs` |
| `tests/docs/variant/020_html_minimal.md` | 4 | `tests/variant_020_html_minimal_test.rs` |
| `tests/docs/variant/021_html_bootstrap.md` | 4 | `tests/variant_021_html_bootstrap_test.rs` |
| `tests/docs/variant/022_html_tailwind.md` | 4 | `tests/variant_022_html_tailwind_test.rs` |
| `tests/docs/variant/023_html_custom.md` | 4 | `tests/variant_023_html_custom_test.rs` |
| `tests/docs/variant/024_sql_ansi.md` | 4 | `tests/variant_024_sql_ansi_test.rs` |
| `tests/docs/variant/025_sql_postgresql.md` | 4 | `tests/variant_025_sql_postgresql_test.rs` |
| `tests/docs/variant/026_sql_mysql.md` | 4 | `tests/variant_026_sql_mysql_test.rs` |
| `tests/docs/variant/027_sql_sqlite.md` | 4 | `tests/variant_027_sql_sqlite_test.rs` |
| `tests/docs/variant/028_text_bullets.md` | 4 | `tests/variant_028_text_bullets_test.rs` |
| `tests/docs/variant/029_text_numbered.md` | 4 | `tests/variant_029_text_numbered_test.rs` |
| `tests/docs/variant/030_text_sections.md` | 4 | `tests/variant_030_text_sections_test.rs` |
| `tests/docs/variant/031_text_keyvalue.md` | 4 | `tests/variant_031_text_keyvalue_test.rs` |
| `tests/docs/variant/032_text_compact.md` | 4 | `tests/variant_032_text_compact_test.rs` |
| `tests/docs/variant/033_text_cli_help.md` | 4 | `tests/variant_033_text_cli_help_test.rs` |

Note: Variant tests are highly repetitive — each config variant tests 4 standard scenarios (basic output, key structural property, characteristic feature, empty table). Test files may be grouped by formatter family if 33 separate files proves excessive during implementation.

## Out of Scope

- Creating new spec files (already done by Task 012)
- Source code changes to `src/`
- Documentation changes to `docs/`
- Spec files for builder/, formatter/ (001-010), trait/001, trait/003 (already implemented by Task 013)
- Bug fixes discovered during testing (separate BUG-NNN if found)

## Requirements

- Each test function name includes its spec file number and case ID (e.g., `fn variant_001_vt_01_single_space_separation()`, `fn input_model_001_im_01_headers_define_schema()`)
- Each test function contains a `// test_kind: spec_case(VT-1)` marker comment (case IDs are file-scoped)
- Tests use real implementations — no mocking
- Tests compile and pass under `--all-features`
- Feature-gated formatters (json, yaml, toml, html, sql, text, expanded, logfmt) require the corresponding feature flag in test cfg attributes
- After each spec file's cases are fully implemented, update the spec's Case Index `Status` column from `⏳` to `✅`

## Work Procedure

1. **Read spec files** — read each of the 43 spec files to understand Given/When/Then contracts
2. **Implement by entity type** — work through data_structure, input_model, input_type, pattern, trait/002, then variant specs
3. **Variant batch strategy** — group variant tests by formatter family (table/9, expanded/2, tree/3, logfmt/1, json/2, yaml/1, toml/1, html/4, sql/4, text/6) to maximize code reuse
4. **Update spec statuses** — change `⏳` to `✅` after each spec file's cases are all implemented
5. **Full test suite** — run Level 3 verification after completion
6. **Verify zero remaining** — confirm `grep -r '⏳' tests/docs/data_structure/ tests/docs/input_model/ tests/docs/input_type/ tests/docs/pattern/ tests/docs/trait/002_table_shaped_formatter.md tests/docs/variant/` returns empty

## Acceptance Criteria

- AC-001: All 168 test cases have corresponding Rust test functions (1:1 mapping)
- AC-002: `cargo nextest run --all-features` passes with all new tests included
- AC-003: `grep -r '⏳' tests/docs/data_structure/ tests/docs/input_model/ tests/docs/input_type/ tests/docs/pattern/ tests/docs/trait/002_table_shaped_formatter.md tests/docs/variant/` returns 0 matches
- AC-004: Each test function has a `// test_kind: spec_case(XX-N)` marker comment
- AC-005: No `src/` files are modified
- AC-006: No mocking is used in any test

## Validation

### Checklist

Desired answer for every question is YES.

**Completeness**
- [ ] Does each of the 168 spec cases have a corresponding test function?
- [ ] Do all 43 spec files show `✅` for every case in their Case Index?
- [ ] Does `cargo nextest run --all-features` pass?

**Traceability**
- [ ] Does every new test function contain a `// test_kind: spec_case(XX-N)` marker?
- [ ] Does the test function name include its case ID?

**No side effects**
- [ ] Are no files under `src/` modified?
- [ ] Are no files under `docs/` modified (except `tests/docs/` status updates)?
- [ ] Are no mocks used anywhere?

**Feature gating**
- [ ] Are serde-dependent formatter tests gated on the specific format feature?
- [ ] Do all tests compile with `--all-features` and fail gracefully without?

### Measurements

**M1 — New test function count**
Command: `grep -r 'test_kind: spec_case' tests/ | wc -l`
Before: 88 (from Task 013). Expected: 256 (88 + 168).

**M2 — Remaining pending markers in target specs**
Command: `grep -r '⏳' tests/docs/data_structure/ tests/docs/input_model/ tests/docs/input_type/ tests/docs/pattern/ tests/docs/trait/002_table_shaped_formatter.md tests/docs/variant/ | wc -l`
Before: 168. Expected: 0.

**M3 — Full test suite**
Command: `cargo nextest run --all-features`
Expected: all pass, 0 failures.

**M4 — No source changes**
Command: `git diff --name-only src/`
Expected: empty (beyond prior task changes).

### Invariants

- [ ] I1 — spec parity: every `⏳` in the 43 spec files has exactly one test function
- [ ] I2 — no mocks: `grep -r 'mock\|Mock\|#\[mock\]' tests/ | grep -v '// ' | wc -l` returns 0

### Anti-faking checks

- AF1: Run `grep -c 'test_kind: spec_case(VT-' tests/variant_*.rs | awk -F: '{s+=$2} END{print s}'` — must show ≥132
- AF2: Run `grep -c 'test_kind: spec_case(IM-' tests/input_model_*.rs | awk -F: '{s+=$2} END{print s}'` — must show ≥8
- AF3: Run `grep -c 'test_kind: spec_case(PT-\|PA-' tests/pattern_*.rs | awk -F: '{s+=$2} END{print s}'` — must show ≥12
- AF4: Spot-check 3 random variant test functions — each must assert on the specific behavior described in its spec case's Then clause

## Related Documentation

- Related: [013 — Implement builder/formatter/trait tests](../completed/013_implement_builder_formatter_trait_tests.md) (predecessor: 88 tests for builder/formatter/trait)
- Related: [012 — Create test surface specs](../completed/012_create_test_surface_specs.md) (created the 43 spec files this task implements)

## Execution State

- **State:** ❓ (Unverified)
- **ID:** 014
- **Slug:** implement_remaining_spec_tests
- **Executor:** any
- **Priority:** 4
- **Value:** 8
- **Easiness:** 5
- **Safety:** 9
- **Dir:** `module/core/data_fmt`
- **Closes:** null
- **Reopen Count:** 0

## History

- **[2026-06-23]** `CREATED` — 43 spec files (data_structure/1, input_model/2, input_type/2, pattern/4, trait/1, variant/33) define 168 test cases with zero backing Rust code; this task implements all 168.
