# Tests Directory

This directory contains all functional tests for the `crates_tools` crate.

### Domain Map

| Domain | Test File | Tests What |
|--------|-----------|------------|
| Smoke | `smoke_test.rs` | Basic crate load and API availability |
| Download | `crates_tools_tests.rs` | Live download from crates.io |
| Corner Cases | `corner_cases_comprehensive.rs` | Archive edge cases and no-panic guarantees |
| Example Quality | `example_quality_tests.rs` | Example code error handling and API patterns |

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Validate basic crate functionality in local and published environments |
| `crates_tools_tests.rs` | Test CrateArchive download and file listing functionality |
| `example_quality_tests.rs` | Validate example code quality, error handling, and edge case coverage |
| `corner_cases_comprehensive.rs` | Validate archive edge cases and no-panic guarantees |
