# Tests Directory

This directory contains comprehensive tests for the `test_tools` crate, verifying all functional requirements, user stories, and implementation details.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `api_stability_facade_tests.rs` | Test API stability facade functionality (Task 011) |
| `behavioral_equivalence_tests.rs` | Test behavioral equivalence between direct and re-exported items (Task 032) |
| `behavioral_equivalence_verification_tests.rs` | Enhanced verification of behavioral equivalence with comprehensive coverage (Task 033) |
| `cargo_execution_tests.rs` | Test SmokeModuleTest cargo command execution functionality (Task 020) |
| `cargo_toml_config_tests.rs` | Test SmokeModuleTest Cargo.toml configuration generation (Task 017) |
| `cleanup_functionality_tests.rs` | Test SmokeModuleTest cleanup functionality and error handling (Task 023) |
| `conditional_execution_tests.rs` | Test conditional execution behavior under different feature flags (Task 026) |
| `debug_assertion_availability_test.rs` | Verify debug assertions are available in test_tools |
| `local_published_smoke_tests.rs` | Test SmokeModuleTest with both local and published crate versions (Task 035) |
| `macro_ambiguity_test.rs` | Document vec! macro ambiguity patterns and resolution strategies |
| `mod_interface_aggregation_tests.rs` | Test mod_interface pattern aggregation functionality (Task 008) |
| `single_dependency_access_tests.rs` | Test single-dependency access and isolation (Task 029) |
| `smoke_module_test_creation.rs` | Test SmokeModuleTest instance creation and initialization (Task 014) |
| `smoke_test.rs` | Comprehensive smoke testing of the test_tools crate |
| `smoke_test_form_main_bug.rs` | Bug reproducer: form() must wrap code lacking fn main() (issue-smoke-form-missing-main) |
| `standalone_basic_test.rs` | Basic standalone build verification without dependencies |
| `standalone_build_tests.rs` | Test standalone build mode functionality (Task 038) |
| `tests.rs` | Main test aggregation entry point |
| `inc/` | Aggregated tests from dependency crates (error_tools, collection_tools, etc.) |

## Test Organization

Tests are organized by functional area and task number:
- **FR-6 Cargo Execution**: `cargo_execution_tests.rs`
- **FR-5 Configuration**: `cargo_toml_config_tests.rs`
- **FR-7 Cleanup**: `cleanup_functionality_tests.rs`
- **US-2 Creation**: `smoke_module_test_creation.rs`
- **Task 011**: API stability facade
- **Task 032/033**: Behavioral equivalence verification
- **Task 035**: Local/published version testing
- **Task 038**: Standalone build mode

## Running Tests

```bash
# Level 3 testing (recommended)
w3 .test l::3
# or
ctest3

# Individual test file
cargo nextest run --test cargo_execution_tests

# All tests with full verification
clear && RUSTFLAGS="-D warnings" cargo nextest run --all-features && \
RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features && \
cargo clippy --all-targets --all-features -- -D warnings
```

## Test Coverage

- **197 total tests** across all test files
- **18 tests** for cargo execution (Task 020)
- **13 tests** for Cargo.toml configuration (Task 017)
- **11 tests** for cleanup functionality (Task 023)
- **8 tests** for SmokeModuleTest creation (Task 014)
- Additional tests for behavioral equivalence, API stability, and edge cases
