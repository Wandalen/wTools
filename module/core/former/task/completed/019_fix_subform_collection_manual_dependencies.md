# Fix subform_collection_manual Dependencies

## Issue
The `subform_collection_manual` test is blocked due to missing dependencies and attributes.

## Location
- **File**: `tests/inc/struct_tests/subform_collection_manual.rs`
- **Module**: `tests/inc/struct_tests/mod.rs:176`

## Specific Errors
1. **Missing `ParentFormer` type**: Cannot find type `ParentFormer` in scope
2. **Missing `scalar` attribute**: Cannot find attribute `scalar` in scope

## Error Details
```rust
error: cannot find attribute `scalar` in this scope
error[E0412]: cannot find type `ParentFormer` in this scope
```

## Root Cause
The test depends on:
- `ParentFormer` type that exists in other test modules but is not imported/accessible
- `scalar` attribute that is not available in the current context

## Required Dependencies
The test requires access to:
```rust
use crate::inc::struct_tests::subform_all::ParentFormer;
// OR similar import from one of these modules:
// - subform_collection::ParentFormer
// - subform_collection_custom::ParentFormer  
// - subform_collection_implicit::ParentFormer
```

## Recommended Solution
1. **Import missing types**: Add proper imports for `ParentFormer` and related types
2. **Verify attribute availability**: Ensure `scalar` attribute is available in the test context
3. **Review test dependencies**: Check if the test requires specific feature flags or modules to be enabled

## Current Status
- **Status**: BLOCKED
- **Priority**: Medium
- **Estimated Effort**: 2-4 hours

## Notes
- This test is part of the manual implementation test suite
- Similar dependency issues affect multiple manual implementation tests
- May require refactoring of test module structure or imports