# Test Spec: Test Coverage

- **Source**: `docs/invariant/003_test_coverage.md`
- **Prefix**: `IN-03`
- **Min cases**: 2

## Cases

| ID | Name | Status |
|----|------|--------|
| IN-03-1 | tarpaulin_reports_coverage_at_or_above_threshold | ⏳ |
| IN-03-2 | coverage_tool_runs_without_error | ⏳ |

[PENDING — `cargo tarpaulin` CI integration not yet in place]

---

### IN-03-1: tarpaulin_reports_coverage_at_or_above_threshold

- **Given:** The full test suite in `tests/`
- **When:** `cargo tarpaulin --all-features` is run and the reported line coverage is inspected
- **Then:** Reported line coverage is ≥ 80%

[PENDING — requires `cargo tarpaulin` CI setup — see task for coverage infrastructure]

---

### IN-03-2: coverage_tool_runs_without_error

- **Given:** The `genfile_core` crate with all features enabled
- **When:** `cargo tarpaulin --all-features` is run
- **Then:** The command exits with code 0 and produces a coverage report without tool errors
