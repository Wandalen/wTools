# Implement missing test cases for all ⏳ spec entries in config_hierarchy

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** 🎯 (Available)

## Goal

Implement 11 missing test functions so every spec case in `tests/docs/` has automated coverage and all ⏳ entries flip to ✅, leaving zero uncovered spec cases in the test surface (Motivated: gaps between spec and implementation mean behavioral regressions go undetected; Observable: 11 new tests appear in nextest output, all spec readme Status rows reflect ✅, `w3 .test level::3` stays green; Scoped: test files in `tests/` only — no source code changes, no documentation edits; Testable: `cargo nextest run --all-features 2>&1 | grep -E "PASS|FAIL"` → all PASS with 11 more passing tests than before).

All 11 missing tests correspond to spec cases in `tests/docs/` that were added during the test surface audit. The source code already implements every behavior described — the gap is test coverage only. Each test must match the exact function name in the spec's **Tests:** field (or use a descriptive name for specs with only a file-level reference) and assert the exact behavior in the spec's **Then:** clause. Note: two test functions cover two spec cases each — `test_backslash_in_app_name_rejected` covers both AP-09 and AN-03; `test_last_modified_updated_on_resave` covers both FM-08 and FP-02.

## In Scope

- `/home/user1/pro/lib/wip_core/wtools/dev/module/experimental/config_hierarchy/tests/type_detection_tests.rs` — add `test_zero_is_boolean_not_integer` (AC-07) and `test_integer_overflow_cascades_to_float` (AC-08)
- `/home/user1/pro/lib/wip_core/wtools/dev/module/experimental/config_hierarchy/tests/validator_tests.rs` — add `test_validate_all_collects_all_errors` (AP-07)
- `/home/user1/pro/lib/wip_core/wtools/dev/module/experimental/config_hierarchy/tests/edge_cases_tests.rs` — add `test_backslash_in_app_name_rejected` (AP-09), `test_missing_metadata_section_legacy_flat_format` (FM-07), `test_last_modified_updated_on_resave` (FM-08), `test_yaml_sequence_param_not_supported` (FM-09)
- `/home/user1/pro/lib/wip_core/wtools/dev/module/experimental/config_hierarchy/tests/basic_operations_tests.rs` — add `test_atomic_config_modify` (FT-08)
- `/home/user1/pro/lib/wip_core/wtools/dev/module/experimental/config_hierarchy/tests/dual_pattern_tests.rs` — add `test_temporary_beats_permanent_same_depth` (IN-05)
- `/home/user1/pro/lib/wip_core/wtools/dev/module/experimental/config_hierarchy/tests/configurability_tests.rs` — add `test_config_manager_is_zero_size` (CM-01)
- `/home/user1/pro/lib/wip_core/wtools/dev/module/experimental/config_hierarchy/tests/path_standards_tests.rs` — add `test_app_name_with_hyphens_and_underscores_accepted` (AN-06)
- After each test passes, flip its Case Index `⏳` → `✅` in the corresponding `tests/docs/` spec file

## Out of Scope

- Source code changes (all behaviors already implemented)
- Documentation edits (already completed)
- `tests/docs/api/002_config_defaults_trait.md` — already fully ✅
- `tests/docs/algorithm/002_resolution_waterfall.md` — already fully ✅ (all 7 RW cases have backing tests)
- CM-02 through CM-06 in `tests/docs/api/004_config_manager.md` — already ✅ (covered by existing tests in the referenced files)

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Each test function name must match exactly the **Tests:** field in the spec case; no renaming
- Tests must use real implementations — no mocks, no stubs, no `#[ignore]`
- Tests requiring filesystem operations must use `TempDir` for isolation
- Tests involving `env::current_dir()` changes must use `#[serial]` (see `dual_pattern_tests.rs` header for the known pitfall)
- Code style: 2-space indents, space inside braces for generics and fn args (follow existing test file style exactly)
- After all tests pass, each corresponding spec case's Status column entry must be updated from `⏳` to `✅`

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note `test_organization_universal.rulebook.md` constraints on test structure, `code_style.rulebook.md` for formatting.
2. **Read spec files** — Read each spec case in `tests/docs/` as source of truth for exact **Given/When/Then** behavior to assert:
   - `tests/docs/algorithm/001_type_detection.md` § AC-07, AC-08
   - `tests/docs/api/001_config_paths_trait.md` § AP-09
   - `tests/docs/api/003_config_validator_trait.md` § AP-07
   - `tests/docs/feature/001_config_hierarchy.md` § FT-08
   - `tests/docs/format/001_config_file_format.md` § FM-07, FM-08, FM-09
   - `tests/docs/invariant/001_resolution_hierarchy.md` § IN-05
   - `tests/docs/api/004_config_manager.md` § CM-01
3. **Read source** — Read `src/type_detection.rs`, `src/file_ops.rs`, `src/manager.rs` to understand the exact function signatures being tested.
4. **Read existing test files** — Read each target test file to understand imports, helpers, and code style before appending.
5. **Add AC-07 and AC-08** — Append to `tests/type_detection_tests.rs`. AC-07 asserts `detect_and_convert_value("0") == JsonValue::Bool(false)` (not Number). AC-08 asserts a string too large for i64 (e.g. `"99999999999999999999"`) returns `JsonValue::Number` containing a float — i64 parse fails, f64 parse succeeds.
6. **Add AP-07** — Append to `tests/validator_tests.rs`. Define a validator struct that rejects both negative `timeout` (via `validate_parameter`) and zero `retries` (via `validate_all` cross-param check). Build a config map with `timeout = -1` and `retries = 0`. Assert `validate_all_config` returns a Vec with `len() == 2` — both violations collected, no short-circuit.
7. **Add AP-09** — Append to `tests/edge_cases_tests.rs`. Define a `ConfigPaths` impl returning `"my\\app"` from `app_name()`. Call `get_local_config_path()` and assert the result is `Err` containing `"invalid characters"`. Follow the pattern of existing `test_slash_only_in_app_name_rejected` and `test_path_traversal_rejected` tests in the same file.
8. **Add FT-08** — Append to `tests/basic_operations_tests.rs`. Create a temp config file with `counter = 1` using `TestConfig::save_config_file`. Call `TestConfig::atomic_config_modify` with a closure that increments the counter. Load the file and assert `counter == 2`. Note: `atomic_config_modify` on the manager type uses `ConfigPaths::get_local_config_path()` for the path — use explicit `file_ops::atomic_config_modify(&path, |m| { ... })` instead, or set up the directory so the path resolves correctly.
9. **Add FM-07, FM-08, FM-09** — Append to `tests/edge_cases_tests.rs`:
   - FM-07: Write a YAML file with bare top-level key-value pairs (no `metadata:` key). Load via `TestConfig::load_config_file`. Assert all top-level keys appear in the map.
   - FM-08: Save a config file, sleep briefly (`Duration::from_millis(10)`), save again. Read raw YAML both times. Assert `last_modified` differs between the two saves (the test in `basic_operations_tests.rs::test_created_at_preserved_last_modified_updated` already does this — write a dedicated, focused test here).
   - FM-09: Write a YAML file with `parameters: { items: [a, b, c] }`. Load via `TestConfig::load_config_file`. Assert `map.get("items")` is either `None` or a `JsonValue::String` — sequences not parsed as arrays.
10. **Add IN-05** — Append to `tests/dual_pattern_tests.rs`. Create a temp dir, write both `-testapp/config.yaml` (temp pattern) and `.testapp/config.yaml` (permanent pattern) in the **same** directory with different values for `param1`. `cd` into the temp dir, call `resolve_config_value`. Assert the `-testapp` value is returned. Must use `#[serial]` attribute — see existing tests in that file for the pattern including the `cd /tmp` before `TempDir` drops.
11. **Add CM-01** — Append to `tests/configurability_tests.rs`. Assert `std::mem::size_of::<TestConfig>() == 0` (or the concrete manager type alias used in that file). This proves `ConfigManager<D, P, V>` is a ZST — no fields, no heap allocation from construction.
12. **Add AN-06** — Append to `tests/path_standards_tests.rs`. Define a `ConfigPaths` impl returning `"my-app_v2"` from `app_name()`. Call `get_local_config_path()` and assert the result is `Ok(_)` — hyphens and underscores are valid characters, not blocked by the validation.
13. **Validate** — Run `w3 .test level::3`. All tests must pass.
14. **Update spec Status fields** — For each newly passing test, edit its Case Index row from `⏳` to `✅` in the corresponding `tests/docs/` spec file; note that `test_backslash_in_app_name_rejected` covers both AP-09 and AN-03, `test_last_modified_updated_on_resave` covers both FM-08 and FP-02, and `test_config_manager_is_zero_size` covers both CM-01 (api/004 spec) and ZCC-01 (pattern spec).
15. **Walk Validation Checklist** — check every item. Every answer must be YES.

## Test Matrix

| Spec Case | Test Function | File | Input Scenario | Expected Behavior |
|-----------|---------------|------|----------------|-------------------|
| AC-07 | `test_zero_is_boolean_not_integer` | `type_detection_tests.rs` | `detect_and_convert_value("0")` | `JsonValue::Bool(false)` — boolean check runs before integer |
| AC-08 | `test_integer_overflow_cascades_to_float` | `type_detection_tests.rs` | `detect_and_convert_value("99999999999999999999")` | `JsonValue::Number` (f64 approximation) — i64 parse fails, f64 succeeds |
| AP-07 | `test_validate_all_collects_all_errors` | `validator_tests.rs` | Config with `timeout=-1, retries=0`; validator rejects both | `Vec` with `len() == 2` — both violations in result |
| AP-09 | `test_backslash_in_app_name_rejected` | `edge_cases_tests.rs` | `app_name()` returns `"my\\app"` | `Err` containing `"invalid characters"` |
| FT-08 | `test_atomic_config_modify` | `basic_operations_tests.rs` | Config file with `counter=1`; closure increments it | File contains `counter=2` after modify |
| FM-07 | `test_missing_metadata_section_legacy_flat_format` | `edge_cases_tests.rs` | YAML with bare top-level keys, no `metadata:` | All top-level keys in returned map |
| FM-08 | `test_last_modified_updated_on_resave` | `edge_cases_tests.rs` | Two saves with 10ms sleep between them | `last_modified` timestamp differs; `created_at` unchanged |
| FM-09 | `test_yaml_sequence_param_not_supported` | `edge_cases_tests.rs` | `parameters: { items: [a, b, c] }` in YAML | `items` key absent or is `String` — not `Array` |
| IN-05 | `test_temporary_beats_permanent_same_depth` | `dual_pattern_tests.rs` | Both `-app/config.yaml` and `.app/config.yaml` in same dir | Value from `-app` returned — temporary beats permanent at same depth |
| CM-01 | `test_config_manager_is_zero_size` | `configurability_tests.rs` | `std::mem::size_of::<TestConfig>()` | Returns `0` — ZST, no heap allocation from construction |
| AN-06 | `test_app_name_with_hyphens_and_underscores_accepted` | `path_standards_tests.rs` | `app_name()` returns `"my-app_v2"` | `Ok(_)` — hyphens and underscores not blocked by validation |

## Acceptance Criteria

- All 11 test functions listed in the Test Matrix exist and pass under `cargo nextest run --all-features`
- All spec files in `tests/docs/` have zero ⏳ entries (all Status values are ✅)
- `w3 .test level::3` exits 0 — no regressions introduced
- `test_temporary_beats_permanent_same_depth` uses `#[serial]` and correctly restores cwd (no subsequent test failures due to missing directory)
- `test_validate_all_collects_all_errors` asserts `Vec` length is exactly 2, not 1 — proving no short-circuit
- `test_zero_is_boolean_not_integer` asserts `Bool(false)`, not `Number(0)` — explicitly proving the type

## Validation

### Checklist

Desired answer for every question is YES.

**Test implementation**
- [ ] Do all 9 test function names exactly match the **Tests:** field in their spec cases?
- [ ] Does each test assert the exact **Then:** clause from the spec (not a weaker assertion)?
- [ ] Are all tests using real implementations (no mocks, no stubs)?
- [ ] Are all filesystem-touching tests using `TempDir` for isolation?
- [ ] Does `test_temporary_beats_permanent_same_depth` use `#[serial]` and restore cwd before drop?

**Spec consistency**
- [ ] Are all 9 previously-⏳ Case Index rows now ✅ in their spec files?
- [ ] Is `tests/docs/feature/001_config_hierarchy.md` FT-09 still ✅ (was already correct)?

**Test suite health**
- [ ] Does `w3 .test level::3` pass with 0 failures?
- [ ] Does `cargo clippy --all-targets --all-features -- -D warnings` produce 0 warnings?

**Out of Scope confirmation**
- [ ] Are all files under `src/` unchanged (no source modifications)?
- [ ] Are all files under `docs/` unchanged (documentation was updated separately)?

### Measurements

**M1 — New test count**
Command: `cargo nextest run --all-features 2>&1 | grep "tests run"`
Before: 126 tests run. Expected: 137 tests run (11 new). Deviation: count below 137.

**M2 — No failures**
Command: `w3 .test level::3`
Before: 0 failures. Expected: 0 failures. Deviation: any failure.

**M3 — Spec coverage**
Command: `grep -r "⏳" /home/user1/pro/lib/wip_core/wtools/dev/module/experimental/config_hierarchy/tests/docs/`
Before: 13 matches (AC-07, AC-08, AP-07, AP-09, FT-08, FM-07, FM-08, FM-09, IN-05, CM-01, FP-02, AN-03, AN-06). Expected: 0 matches. Deviation: any remaining match.

### Invariants

- [ ] I1 — test suite: `w3 .test level::3` → 0 failures

### Anti-faking checks

**AF1 — AC-07 asserts Bool not Number**
Check: `grep -A5 "fn test_zero_is_boolean_not_integer" tests/type_detection_tests.rs | grep "Bool"`
Expected: 1 match containing `Bool(false)`. Why: a passing test asserting `Number(0)` would miss the boolean-priority invariant.

**AF2 — AP-07 asserts 2 errors not 1**
Check: `grep -A10 "fn test_validate_all_collects_all_errors" tests/validator_tests.rs | grep "len()"`
Expected: 1 match with `== 2`. Why: asserting `!is_empty()` would pass even if implementation short-circuits after the first error.

**AF3 — IN-05 uses serial**
Check: `grep -B2 "fn test_temporary_beats_permanent_same_depth" tests/dual_pattern_tests.rs | grep "serial"`
Expected: 1 match. Why: without `#[serial]`, test may pass due to lucky scheduling but fail intermittently in CI.

**AF4 — FM-07 verifies legacy key in map**
Check: `grep -A15 "fn test_missing_metadata_section_legacy_flat_format" tests/edge_cases_tests.rs | grep "assert"`
Expected: ≥1 match asserting a key from the bare YAML is present. Why: an empty-check pass (`is_ok()`) would not prove legacy flat format loading.

## Outcomes

<!-- Populated upon task completion -->
