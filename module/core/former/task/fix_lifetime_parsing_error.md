# Fix Lifetime Parsing Error for Lifetime-Only Structs

## Issue Description

When deriving `Former` for structs that only have lifetime parameters (e.g., `struct Simple<'a>`), the compiler produces a parsing error:

```
error: expected `while`, `for`, `loop` or `{` after a label
 --> tests/inc/struct_tests/minimal_lifetime.rs:8:28
  |
8 | #[derive(Debug, PartialEq, the_module::Former)]
  |                            ^^^^^^^^^^^^^^^^^^ expected `while`, `for`, `loop` or `{` after a label
  |
help: add `'` to close the char literal
  |
9 | pub struct Minimal<'a'> {
  |                      +
```

This error suggests that the parser is interpreting `'a` as an incomplete character literal or label instead of a lifetime parameter.

## What Has Been Fixed

1. **Double Definition Issue**: Fixed the perform impl which was generating `SimpleFormer < 'a, Definition, Definition >` instead of `SimpleFormer < 'a, Definition >`.

2. **FormerBegin Lifetime Bounds**: Added proper lifetime bounds (`Definition::Storage : 'a`, etc.) to the FormerBegin implementation.

3. **Generic Parameter Handling**: Improved handling of lifetime-only structs in various places throughout the code.

## Current State

The generated code appears syntactically correct when extracted and compiled separately. The main structures are properly generated:

- `SimpleFormer < 'a, Definition >` - correctly defined with two parameters
- All trait implementations use the correct number of generic parameters
- The perform impl now correctly uses `< 'a, Definition >`

## Remaining Issue

Despite these fixes, the parsing error persists. The error occurs during macro expansion, suggesting there's a subtle issue with how tokens are being generated or there's a problematic token sequence that only appears during macro expansion.

## Hypothesis

The issue might be related to:

1. **Token Stream Generation**: There might be an issue with how the quote! macro is generating tokens, possibly related to spacing or token adjacency.

2. **Trailing Comma Issues**: The `struct_generics_with_defaults` includes a trailing comma (`'a,`), which might cause issues in certain contexts.

3. **Lifetime Position**: There might be a place in the generated code where a lifetime appears without proper syntactic context.

## Minimal Reproduction

```rust
#[derive(Debug, PartialEq, former::Former)]
pub struct Minimal<'a> {
    value: &'a str,
}
```

## Next Steps

1. **Use cargo expand with more verbose output**: Try to get the exact token stream that's causing the issue.

2. **Check token adjacency**: Look for places where lifetimes might appear next to other tokens without proper spacing.

3. **Review all uses of struct_generics_with_defaults**: The trailing comma might be causing issues in specific contexts.

4. **Test with different lifetime names**: Check if the issue is specific to `'a` or affects all lifetimes.

5. **Consider alternative approaches**: If the issue persists, consider generating lifetime-only structs differently, perhaps by avoiding certain macro patterns that might confuse the parser.

## Related Files

- `/home/user1/pro/lib/wTools/module/core/former_meta/src/derive_former/former_struct.rs` - Main implementation
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/minimal_lifetime.rs` - Test case
- `/home/user1/pro/lib/wTools/module/core/macro_tools/src/generic_params.rs` - Generic parameter handling