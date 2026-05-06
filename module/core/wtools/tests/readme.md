# tests/

Test suite for wtools aggregating crate validating public API re-exports and example quality.

## Organization

Tests are organized in flat structure focused on aggregating crate verification rather than functional domain testing (functional tests belong in the constituent crate: collection_tools).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify crate compiles and runs in local and published contexts |
| `example_quality_test.rs` | Validate example file meets documentation and formatting standards |
| `readme_accuracy_test.rs` | Verify readme.md contains accurate paths and execution instructions |
| `manual/` | Document manual testing procedures for crate functionality |

## Test Strategy

This is a thin re-export crate that forwards collection_tools macros and types (hmap!, hset!, bmap!, bset!, HashMap, HashSet, BTreeMap, BTreeSet, etc.).

**Testing Philosophy**:
- Minimal wrapper-specific tests only (smoke tests, example validation, readme accuracy)
- NO functional tests here (those belong in constituent crates)
- Tests verify aggregation correctness, not functionality
- Test count deliberately kept low (~10-15 tests) to avoid duplication

**Why Minimal Tests**:
Comprehensive functional testing happens in constituent crates. Testing here would duplicate coverage without adding value. See test_organization.rulebook.md § Testing Thin Wrappers and Delegation Layers for rationale.

## Adding Tests

Before adding new test:
1. **Question**: Does this test functionality or aggregation?
   - Functionality → Add to constituent crate (collection_tools/tests/)
   - Aggregation/re-export → May add here
2. **Check duplication**: Is this already tested in constituent crate?
   - Yes → Don't duplicate, reference constituent test
   - No → Verify it's actually aggregation-specific
3. **Consult table**: Does responsibility overlap with existing test?
   - Yes → Use existing test file
   - No → Add new row, create new file

## Known Issues

See individual test files for documented bug reproduction tests:
- `example_quality_test.rs`: Documents placeholder removal (issue-wtools-001), spacing fixes (issue-wtools-002)
- `readme_accuracy_test.rs`: Documents path correction (issue-wtools-003)
