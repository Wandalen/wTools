# Fix subform_entry_hashmap_custom Dependencies

## Issue  
The `subform_entry_hashmap_custom` test is blocked due to missing dependencies and attributes.

## Location
- **File**: `tests/inc/struct_tests/subform_entry_hashmap_custom.rs`
- **Module**: `tests/inc/struct_tests/mod.rs:218`

## Specific Errors
1. **Missing `subform_entry` attribute**: Cannot find attribute `subform_entry` in scope
2. **Missing `ParentFormer` type**: Cannot find type `ParentFormer` in scope
3. **Missing `ChildFormerStorage` type**: Cannot find type `ChildFormerStorage` in scope
4. **Missing subformer types**: Cannot find `ChildAsSubformer`, `ChildAsSubformerEnd`, `ChildFormer`

## Error Details
```rust
error: cannot find attribute `subform_entry` in this scope
  --> module/core/former/tests/inc/struct_tests/subform_entry_hashmap_custom.rs:24:5
   |
24 |   #[subform_entry(setter = false)]
   |     ^^^^^^^^^^^^^

error[E0412]: cannot find type `ParentFormer` in this scope
error[E0412]: cannot find type `ChildFormerStorage` in this scope
error[E0412]: cannot find type `ChildAsSubformer` in this scope
error[E0405]: cannot find trait `ChildAsSubformerEnd` in this scope
error[E0412]: cannot find type `ChildFormer` in this scope
```

## Root Cause
The test has extensive dependency issues:
- `subform_entry` attribute is not available in the current context
- Multiple generated types from other modules are not accessible
- Complex manual implementation requiring significant infrastructure

## Required Dependencies
The test requires access to:
```rust
use crate::inc::struct_tests::subform_all::ParentFormer;
use crate::inc::struct_tests::subform_all::ChildFormerStorage;
use crate::inc::struct_tests::subform_all::ChildAsSubformer;
use crate::inc::struct_tests::subform_all::ChildAsSubformerEnd;
use crate::inc::struct_tests::subform_all::ChildFormer;
```

## Complex Implementation Details
This test includes:
- Custom hashmap-specific entry handling
- Manual implementation of subform ending logic
- Complex closure-based form completion
- Custom storage manipulation

## Example of Complex Manual Code
```rust
let on_end = |substorage: ChildFormerStorage, super_former: core::option::Option<Self>| -> Self {
  let mut super_former = super_former.unwrap();
  let preformed = former::StoragePreform::preform(substorage);
  super_former.storage.children.get_or_insert_with(Default::default).insert(name.into(), preformed);
  super_former
};
```

## Additional Issues
- **EntityToStorage trait missing**: Multiple trait implementations required
- **Storage type complexity**: Manual storage handling that should be generated
- **Custom collection logic**: Complex hashmap-specific handling

## Recommended Solution
1. **Import missing dependencies**: Add all required type and trait imports

2. **Implement missing traits**: Add `EntityToStorage` and related implementations

3. **Review test architecture**: Consider whether this level of manual implementation is necessary

4. **Alternative approach**: Convert to use generated code with custom configuration instead of full manual implementation

## Current Status
- **Status**: BLOCKED
- **Priority**: Low (custom/advanced functionality)
- **Estimated Effort**: 6-8 hours

## Notes
- This is the most complex manual implementation test
- Tests custom hashmap entry handling functionality
- May be better implemented as a configuration test rather than full manual implementation
- Similar patterns could be extracted to reduce code duplication