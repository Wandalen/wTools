# Invariant Spec: Testing Coverage

### Scope

- **Element:** `invariant/005_testing_coverage`
- **Source:** `docs/invariant/005_testing_coverage.md`
- **Prefix:** `IN-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IN-01 | all_24_commands_have_integration_tests | nominal | ✅ |
| IN-02 | tests_use_manifest_directory_relative_paths | nominal | ✅ |

---

### IN-01: all 24 commands have at least one integration test

- **Given:** The full test suite is run
- **When:** Each of the 24 commands is checked for test coverage
- **Then:** Every command appears at least once as the subject of a test function
- **Tests:** none — see task/001_fill_test_surface_gaps.md (help commands IN-01 gap)

### IN-02: tests use manifest-directory-relative paths

- **Given:** The test suite is run on a different machine or CI environment
- **When:** Tests that reference files use `CARGO_MANIFEST_DIR`-based paths
- **Then:** All tests pass regardless of working directory at invocation time
- **Tests:** `tests/cli_runner.rs`
