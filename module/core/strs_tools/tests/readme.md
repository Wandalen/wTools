# Tests Directory

Comprehensive test suite for the strs_tools core crate.

## Organization

| File | Responsibility |
|------|----------------|
| `ansi_truncate_tests.rs` | ANSI truncation with boundary detection tests |
| `issue_001_mre.rs` | Minimal reproducible example for escaped quotes bug (ISSUE-001) |
| `issue_002_example_feature_guards.rs` | Feature guard correctness for the `enabled` feature |
| `namespace_verification_test.rs` | Namespace pattern verification tests |
| `parser_integration_comprehensive_test.rs` | Comprehensive parser integration functionality tests |
| `smoke_test.rs` | Basic package smoke tests |
| `strs_tools_tests.rs` | Main test suite entry point importing all submodules |
| `inc/` | Test submodules (indentation, isolate, parse, split, etc.) |

## Test Strategy

The strs_tools crate uses a comprehensive testing approach:
- **ANSI handling tests:** Verify ANSI escape sequence processing (truncation, detection, parsing, stripping)
- **Split functionality:** Extensive split tests covering edge cases, quoting, delimiters, Unicode
- **Parser integration:** Command-line parsing, validation, error handling
- **Bug reproduction tests:** Minimal reproducible examples for documented issues
- **Compile-time optimization:** Procedural macro generation verification
- **Namespace verification:** Ensures proper module organization patterns

This strategy ensures robust validation across all string manipulation features.
