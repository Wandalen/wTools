# Fix parametrized_struct_imm Test

## Issue
Test is disabled due to: "E0277 Hash/Eq trait bound issues with Definition"

## Files Involved
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/parametrized_struct_imm.rs`
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/mod.rs` (line 118)

## Problem Description
The test has a Child struct with generic K parameter that requires Hash + Eq bounds, but the macro-generated code doesn't properly handle these trait bounds.

## Investigation Required
1. Run the test to see specific E0277 trait bound errors
2. Examine how the macro handles generic parameters with trait bounds
3. Identify if Definition type needs Hash/Eq constraints propagated

## Expected Outcome
Enable the test by fixing trait bound propagation in parametrized structs.

## Priority
High - generic parameter support is core functionality

## Status
INVESTIGATED - Multiple macro issues identified

## Investigation Results
The test fails with multiple compilation errors indicating fundamental issues with generic parameter handling in the macro:

**Error 1: Generic Arguments Order**
```
error: generic arguments must come before the first constraint
pub struct Child<K: core::hash::Hash + core::cmp::Eq> {
```

**Error 2: Undeclared Lifetime**
```
error[E0261]: use of undeclared lifetime name `'a`
```
The macro is trying to use lifetime `'a` that doesn't exist in the struct definition.

**Error 3: Generic Parameter Not Found** 
```
error[E0412]: cannot find type `K` in this scope
```
The macro isn't properly handling the generic parameter `K`.

**Error 4: Trait Bounds Not Propagated**
```
error[E0277]: the trait bound `K: Hash` is not satisfied
```
The `K: core::hash::Hash + core::cmp::Eq` constraints aren't being propagated to generated code.

**Root Causes:**
1. Macro's generic parameter parsing doesn't handle trait bounds properly
2. Lifetime inference is incorrectly trying to inject `'a` 
3. Generic parameters with constraints are not being recognized in scope
4. Trait bounds from struct definition not propagated to macro-generated code

**Solution Required:**
Fix the macro's generic parameter parsing to:
1. Properly handle `<K: Trait + Trait>` syntax
2. Not inject spurious lifetimes
3. Propagate trait bounds to generated FormerDefinition types
4. Ensure generic parameters are in scope for generated code

## Status
Blocked - requires macro-level fix for generic parameter parsing and trait bound propagation