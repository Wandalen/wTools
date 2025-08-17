# Fix subform_entry_named_manual Dependencies

## Issue
The `subform_entry_named_manual` test is blocked due to missing dependencies and attributes.

## Location
- **File**: `tests/inc/struct_tests/subform_entry_named_manual.rs`
- **Module**: `tests/inc/struct_tests/mod.rs:206`

## Specific Errors
1. **Missing `subform_entry` attribute**: Cannot find attribute `subform_entry` in scope
2. **Missing `ParentFormer` type**: Cannot find type `ParentFormer` in scope
3. **Missing subformer types**: Cannot find `ChildAsSubformer`, `ChildAsSubformerEnd`, `ChildFormer`
4. **Missing end types**: Cannot find `ParentSubformEntryChildrenEnd`

## Error Details
```rust
error: cannot find attribute `subform_entry` in this scope
  --> module/core/former/tests/inc/struct_tests/subform_entry_named_manual.rs:22:5
   |
22 |   #[subform_entry]
   |     ^^^^^^^^^^^^^
   |
note: `subform_entry` is imported here, but it is a module, not an attribute

error[E0412]: cannot find type `ParentFormer` in this scope
error[E0412]: cannot find type `ChildAsSubformer` in this scope
error[E0405]: cannot find trait `ChildAsSubformerEnd` in this scope
error[E0412]: cannot find type `ChildFormer` in this scope
error[E0412]: cannot find type `ParentSubformEntryChildrenEnd` in this scope
```

## Root Cause
The test has multiple dependency issues:
- `subform_entry` is imported as a module but used as an attribute
- Multiple types exist in other test modules but are not accessible
- The test requires complex infrastructure not available in the current context

## Required Dependencies
The test requires access to:
```rust
use crate::inc::struct_tests::subform_all::ParentFormer;
use crate::inc::struct_tests::subform_all::ChildAsSubformer;
use crate::inc::struct_tests::subform_all::ChildAsSubformerEnd;
use crate::inc::struct_tests::subform_all::ChildFormer;
use crate::inc::struct_tests::subform_entry::ParentSubformEntryChildrenEnd;
```

## Additional Issues
- **EntityToStorage trait not implemented**: The `Parent` struct doesn't implement required traits
- **Attribute vs Module confusion**: `subform_entry` being used as both module and attribute
- **Complex manual boilerplate**: Significant amount of generated code being manually implemented

## Recommended Solution
1. **Resolve attribute issue**: 
   - Determine if `subform_entry` should be an attribute or module
   - Import the correct attribute or implement the attribute macro

2. **Import missing types**: Add proper imports for all required types and traits

3. **Implement missing traits**: Add `EntityToStorage` and related trait implementations

4. **Review test purpose**: Consider if this test should use generated code instead of manual implementation

## Alternative Approach
Convert this from a manual implementation test to a test that uses the generated code, which would eliminate most of the dependency issues.

## Current Status
- **Status**: ✅ RESOLVED
- **Priority**: Medium  
- **Actual Effort**: 2 hours

## Resolution Summary
**Successfully fixed and enabled** - `subform_entry_named_manual` now compiles and passes all tests.

### Key Changes Made:
1. **Complete manual implementation**: Provided all missing manual implementations including:
   - `ParentFormer` struct and implementations
   - `ChildFormer` struct and implementations  
   - `ParentFormerStorage` and `ChildFormerStorage`
   - All required trait implementations (`EntityToFormer`, `EntityToStorage`, `FormerDefinitionTypes`, etc.)
   - Subformer types (`ChildAsSubformer`, `ChildAsSubformerEnd`)
   - `ParentSubformEntryChildrenEnd` for entry handling

2. **Fixed lifetime issues**: 
   - Added lifetime parameter `'a` to `FormerBegin` trait usage
   - Added `Definition: 'a` lifetime bounds
   - Fixed `Default` implementation for `ParentSubformEntryChildrenEnd`

3. **Enabled test module**: Re-enabled the test in `mod.rs` and verified all tests pass

### Technical Achievement:
This fix demonstrates that complex manual implementations can be successfully created to replace generated code, providing a complete working example of how the `Former` pattern works under the hood.

## Notes
- Part of the entry subform test suite
- Now serves as a reference implementation for manual Former patterns
- Test passes and contributes to overall test coverage