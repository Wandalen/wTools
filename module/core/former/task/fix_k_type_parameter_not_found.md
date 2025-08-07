# Fix "K type parameter not found in scope" Error

## Problem Description

The test `parametrized_struct_imm` is failing with a strange error where the type parameter `K` is reported as "not found in scope" at the struct definition line itself:

```
error[E0412]: cannot find type `K` in this scope
  --> module/core/former/tests/inc/struct_tests/parametrized_struct_imm.rs:33:18
   |
33 | pub struct Child<K: core::hash::Hash + core::cmp::Eq> {
   |                  ^ not found in this scope
```

This error is unusual because:
1. It occurs at the struct definition line, not in generated code
2. The type parameter K is clearly defined in the struct's generic parameters
3. The macro expansion shows correct handling of K in the generated code

## Current Status

The macro correctly:
- Classifies Child<K> as having only type parameters (`has_only_types: true`)
- Generates Former<Definition> without K (which is correct design)
- Passes K through Definition types (ChildFormerDefinitionTypes<K, ...>)

## Investigation Notes

1. The error persists even without the `#[subform_collection]` attribute
2. The error appears to be related to macro hygiene or AST manipulation
3. Simple generic structs (Test<T>) compile correctly
4. The issue might be specific to the type parameter name 'K' or the trait bounds

## Possible Causes

1. **Macro hygiene issue**: The derive macro might be interfering with type parameter resolution
2. **AST manipulation**: Some part of the macro might be incorrectly modifying the original AST
3. **Quote/unquote context**: Type parameters might not be properly preserved through quote! macros
4. **Trait bound complexity**: The combination of Hash + Eq bounds might trigger an edge case

## Next Steps

1. Create minimal reproduction without Former derive to isolate the issue
2. Check if renaming K to another letter (e.g., T) resolves the issue
3. Investigate if the trait bounds (Hash + Eq) are causing the problem
4. Review the macro expansion for any AST modifications that might affect the original struct
5. Check if this is related to the recent changes in how we handle generic parameters

## Related Code

- `/home/user1/pro/lib/wTools/module/core/former_meta/src/derive_former/former_struct.rs` - Main macro implementation
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/parametrized_struct_imm.rs` - Failing test

## Temporary Workaround

The test is currently disabled with the subform_collection attribute commented out. Once the root cause is identified and fixed, re-enable the full test.