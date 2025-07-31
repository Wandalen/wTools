# Fix Lifetime-Only Structs Missing Lifetime Specifier

## Issue Description
Lifetime-only structs are generating E0106 "missing lifetime specifier" errors across multiple test files.

## Error Details
```
error[E0106]: missing lifetime specifier
  --> module/core/former/tests/inc/struct_tests/a_basic.rs:13:28
   |
13 | #[derive(Debug, PartialEq, former::Former)]
   |                            ^ expected named lifetime parameter

error[E0106]: missing lifetime specifier
 --> module/core/former/tests/inc/struct_tests/test_lifetime_only.rs:9:28
  |
9 | #[derive(Debug, PartialEq, the_module::Former)]
  |                            ^ expected named lifetime parameter
```

## Affected Test Files
- `a_basic.rs`
- `test_lifetime_only.rs` 
- `test_lifetime_minimal.rs`
- `minimal_lifetime.rs`
- `debug_lifetime_minimal.rs`
- `debug_simple_lifetime.rs`
- `parametrized_slice.rs`

## Root Cause
The lifetime-only handling logic in the macro is broken. The classification system correctly identifies lifetime-only structs, but the generics generation is not producing the proper lifetime parameters.

## Investigation Points
1. Check the `classification.has_only_lifetimes` branch in `former_struct.rs:166-202`
2. Verify that lifetime parameters are being included in generated structs
3. Ensure FormerBegin implementation includes proper lifetime handling

## Files to Modify
- `/home/user1/pro/lib/wTools/module/core/former_meta/src/derive_former/former_struct.rs`

## Test Cases
All the affected test files should compile without E0106 errors.

## Priority
High - This affects multiple test files and represents a core functionality regression.