# Fix FormerBegin Trait Bounds for Type-Only Structs

## Issue Description
Type-only structs like `Child<T>` are generating E0277 trait bound errors because the FormerBegin implementation is missing required trait bounds.

## Error Details
```
error[E0277]: the trait bound `T: Hash` is not satisfied
  --> module/core/former/tests/inc/struct_tests/parametrized_struct_imm.rs:31:28
   |
31 | #[derive(Debug, PartialEq, the_module::Former)]
   |                            ^^^^^^^^^^^^^^^^^^ the trait `Hash` is not implemented for `T`
   |
note: required by a bound in `parametrized_struct_imm::ChildFormerStorage`
```

## Root Cause
The FormerBegin implementation for type-only structs excludes the struct's where clause to avoid E0309 lifetime errors:

```rust
let former_begin_where_clause = if classification.has_only_types {
  quote! {}  // Missing trait bounds
} else {
  quote! { , #struct_generics_where }
};
```

## Solution
Include the struct's trait bounds in FormerBegin where clause for type-only structs, but ensure they don't cause lifetime constraint issues.

## Files to Modify
- `/home/user1/pro/lib/wTools/module/core/former_meta/src/derive_former/former_struct.rs`

## Test Case
- `cargo test parametrized_struct_imm` should compile without E0277 errors
- The `Child<T>` struct should work with `T: Hash + Eq` bounds

## Priority
Medium - This is a secondary issue after the main E0309 lifetime problem was resolved.