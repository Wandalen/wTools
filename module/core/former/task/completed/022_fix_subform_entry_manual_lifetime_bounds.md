# Fix subform_entry_manual Higher-Ranked Lifetime Bounds

## Issue
The `subform_entry_manual` test is blocked due to complex lifetime errors with higher-ranked trait bounds.

## Location
- **File**: `tests/inc/struct_tests/subform_entry_manual.rs`
- **Module**: `tests/inc/struct_tests/mod.rs:201`

## Specific Errors
Complex lifetime errors involving higher-ranked trait bounds (`for<'a>`):

```rust
error: `Definition` does not live long enough
   --> module/core/former/tests/inc/struct_tests/subform_entry_manual.rs:64:10
    |
64  |     self._children_subform_entry::<ChildFormer<_>, _>().name(name)
    |          ^^^^^^^^^^^^^^^^^^^^^^^
    |
note: due to current limitations in the borrow checker, this implies a `'static` lifetime
   --> module/core/former/tests/inc/struct_tests/subform_entry_manual.rs:109:22
    |
109 |     for<'a> Former2: former::FormerBegin<'a, Definition2>,
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

## Root Cause
The issue stems from Rust's borrow checker limitations with higher-ranked trait bounds (HRTB). The `for<'a>` lifetime bound in the `FormerBegin` trait creates a constraint that the borrow checker cannot currently handle properly, causing it to infer a `'static` lifetime requirement.

## Technical Details
- **Affected methods**: `_children_subform_entry` calls
- **Trait bound**: `for<'a> Former2: former::FormerBegin<'a, Definition2>`
- **Borrow checker limitation**: Cannot properly handle the interaction between the generic `Definition` parameter and the higher-ranked lifetime bounds

## Error Pattern
```rust
// This pattern causes issues:
pub fn _children_subform_entry<Former2, Definition2>(self) -> Former2
where
  for<'a> Former2: former::FormerBegin<'a, Definition2>, // <- HRTB causing issues
  // ... other bounds
```

## Attempted Solutions
1. **Added explicit lifetime parameters**: Did not resolve the HRTB interaction
2. **Added `Definition: 'a` bounds**: Still conflicts with higher-ranked bounds
3. **Modified trait bounds**: The fundamental HRTB limitation persists

## Recommended Solution
This requires one of the following approaches:

### Option 1: Redesign Trait Bounds
- Remove higher-ranked trait bounds where possible
- Use explicit lifetime parameters instead of `for<'a>`
- May require changes to the `FormerBegin` trait design

### Option 2: Compiler Feature Dependency
- Wait for Rust compiler improvements to HRTB handling
- This is a known limitation in the current borrow checker

### Option 3: Alternative Implementation Pattern
- Restructure the manual implementation to avoid the problematic pattern
- Use different trait bounds that don't trigger the HRTB limitation

## Current Status
- **Status**: BLOCKED
- **Priority**: High (affects core functionality)
- **Estimated Effort**: 8-12 hours (requires trait redesign)

## Impact
- Blocks manual implementation patterns for entry subforms  
- May affect other similar patterns in the codebase
- Requires careful consideration of trait API design

## Notes
- This is a fundamental limitation of the current Rust borrow checker
- Similar patterns may exist in other manual implementation tests
- Resolution may require breaking changes to the trait API