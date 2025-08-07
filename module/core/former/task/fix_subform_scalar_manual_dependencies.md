# Fix subform_scalar_manual Dependencies

## Issue
The `subform_scalar_manual` test is blocked due to missing dependencies and attributes.

## Location
- **File**: `tests/inc/struct_tests/subform_scalar_manual.rs`
- **Module**: `tests/inc/struct_tests/mod.rs:191`

## Specific Errors
1. **Missing `ParentFormer` type**: Cannot find type `ParentFormer` in scope
2. **Missing `scalar` attribute**: Cannot find attribute `scalar` in scope
3. **Missing `ChildAsSubformer` type**: Cannot find type `ChildAsSubformer` in scope
4. **Missing `ChildAsSubformerEnd` trait**: Cannot find trait `ChildAsSubformerEnd` in scope

## Error Details
```rust
error: cannot find attribute `scalar` in this scope
  --> module/core/former/tests/inc/struct_tests/subform_scalar_manual.rs:24:5
   |
24 |   #[scalar(setter = false)]
   |     ^^^^^^

error[E0412]: cannot find type `ParentFormer` in this scope
error[E0412]: cannot find type `ChildAsSubformer` in this scope  
error[E0405]: cannot find trait `ChildAsSubformerEnd` in this scope
```

## Root Cause
The test depends on types and attributes from other modules that are not properly imported or accessible:
- `ParentFormer` exists in other test modules but is inaccessible
- `scalar` attribute is not available in the current context
- Subformer types are defined in other modules but not imported

## Required Dependencies
The test requires access to:
```rust
use crate::inc::struct_tests::subform_all::ParentFormer;
use crate::inc::struct_tests::subform_all::ChildAsSubformer;
use crate::inc::struct_tests::subform_all::ChildAsSubformerEnd;
```

## Additional Issues
- **EntityToStorage trait not implemented**: The `Parent` struct doesn't implement required traits
- **Complex manual implementation**: Requires significant boilerplate that should be generated

## Recommended Solution
1. **Import missing types**: Add proper imports for all required types and traits
2. **Verify attribute availability**: Ensure `scalar` attribute is available or implement alternative
3. **Implement missing traits**: Add `EntityToStorage` implementation for `Parent` struct
4. **Review test architecture**: Consider if this test should use generated code instead of manual implementation

## Current Status
- **Status**: BLOCKED
- **Priority**: Medium  
- **Estimated Effort**: 4-6 hours

## Notes
- This is a complex manual implementation test
- Similar dependency issues affect multiple manual implementation tests
- May require architectural changes to the test suite structure