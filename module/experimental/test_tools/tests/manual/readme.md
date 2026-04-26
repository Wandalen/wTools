# Manual Testing Plan for test_tools

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test_corner_cases.rs` | Manual smoke test corner case explorations |
| `test_empty_code.rs` | Manual test for empty code string edge case |

This document describes comprehensive manual testing procedures for the test_tools crate.

## Test Coverage Areas

### 1. Example Testing
- ✅ `examples/test_tools_trivial.rs` - Verified compiles and runs (empty main)

### 2. SmokeModuleTest Corner Cases

#### 2.1 Dependency Configuration Edge Cases
- [ ] Non-existent dependency name
- [ ] Non-existent version number
- [ ] Malformed version strings (e.g., "invalid", "1.x.y")
- [ ] Very old versions (e.g., "0.1.0")
- [ ] Pre-release versions (e.g., "1.0.0-alpha.1")
- [ ] Wildcard versions (e.g., "*", "1.*")
- [ ] Multiple dependencies with version conflicts
- [ ] Dependency with features enabled
- [ ] Git dependencies
- [ ] Path dependencies

#### 2.2 Code Generation Edge Cases
- [ ] Empty code string
- [ ] Code with syntax errors
- [ ] Code with compile errors (type mismatches)
- [ ] Code with runtime panics
- [ ] Code with infinite loops
- [ ] Code accessing non-existent dependency items
- [ ] Code with multiple modules
- [ ] Code with macros
- [ ] Code with unsafe blocks
- [ ] Very large code strings (>10KB)

#### 2.3 Project Structure Edge Cases
- [ ] Test name with special characters
- [ ] Test name with unicode characters
- [ ] Test name with very long string (>255 chars)
- [ ] Multiple smoke tests running concurrently (already tested - fixed)
- [ ] Nested temporary directory creation
- [ ] Permission issues in /tmp directory
- [ ] Disk space exhaustion during build

#### 2.4 Cargo Operation Edge Cases
- [ ] Cargo test with no tests defined
- [ ] Cargo test with failing tests
- [ ] Cargo run with no main function
- [ ] Cargo run with failing main function
- [ ] Cargo build with warnings
- [ ] Cargo build with features
- [ ] Network failures during dependency download
- [ ] Interrupted cargo operations

#### 2.5 Cleanup Edge Cases
- [ ] Cleanup with non-existent directory
- [ ] Cleanup with read-only files
- [ ] Cleanup with nested directory structure
- [ ] Cleanup without prior form() call
- [ ] Multiple cleanup() calls on same instance
- [ ] Cleanup failure handling

### 3. Macro Re-exports (tests_impls!, tests_index!)

#### 3.1 tests_impls! Edge Cases
- [ ] Empty tests_impls! block
- [ ] Single test function
- [ ] Tests with attributes (#[should_panic], #[ignore])
- [ ] Tests with generic parameters
- [ ] Tests with lifetimes
- [ ] Nested module structures
- [ ] Tests with doc comments
- [ ] Tests with cfg attributes

#### 3.2 tests_index! Edge Cases
- [ ] Empty tests_index! block
- [ ] Single test reference
- [ ] Non-existent test function reference
- [ ] Duplicate test names
- [ ] Tests defined in external modules
- [ ] Mix of local and imported tests

### 4. Collection Constructor Macros

#### 4.1 heap! Macro
- [ ] Empty heap!()
- [ ] Single element heap!(42)
- [ ] Multiple elements heap!(1, 2, 3)
- [ ] Large collection (>1000 elements)
- [ ] Nested collections heap!(vec![1,2], vec![3,4])
- [ ] Different types (String, struct, enum)

#### 4.2 vec! Macro Ambiguity
- [ ] std::vec! vs collection_tools::vec!
- [ ] Disambiguation with fully qualified paths
- [ ] Use in different module contexts

#### 4.3 Other Collection Constructors
- [ ] hmap! (hash map)
- [ ] hset! (hash set)
- [ ] list! (linked list)
- [ ] bmap! (BTreeMap)
- [ ] bset! (BTreeSet)

### 5. Error Handling Utilities

#### 5.1 ErrWith Trait
- [ ] err_with() on Ok(T)
- [ ] err_with() on Err(E)
- [ ] Nested Results
- [ ] Chain multiple err_with() calls
- [ ] Different error types
- [ ] Custom error messages

#### 5.2 Debug Assertions
- [ ] Assertions in debug builds
- [ ] Assertions in release builds
- [ ] Custom assertion messages

## Manual Test Execution Log

### Session 1: 2026-01-21

#### Test: examples/test_tools_trivial.rs
- **Status**: ✅ PASSED
- **Command**: `cargo run --example test_tools_trivial`
- **Result**: Compiles and runs successfully (empty output as expected)
- **Issues Found**: None

#### Test: SmokeModuleTest - Non-existent dependency
- **Status**: Pending
- **Plan**: Test with dependency name that doesn't exist on crates.io

#### Test: SmokeModuleTest - Malformed version
- **Status**: Pending
- **Plan**: Test with invalid version strings

## Issues Found

_No issues found yet during manual testing phase_

## Test Automation Candidates

After manual verification, the following scenarios should be automated:
1. Non-existent dependency handling (if behavior is deterministic)
2. Malformed version string handling
3. Empty code string handling
4. Code with syntax errors (already automated in cargo_execution_tests.rs)
