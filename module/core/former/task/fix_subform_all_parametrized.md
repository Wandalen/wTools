# Fix subform_all_parametrized Test

## Issue
Test is disabled due to: "E0726 implicit elided lifetime not allowed here + E0277 FormerDefinition trait issues"

## Files Involved
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/subform_all_parametrized.rs`
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/mod.rs` (line 225)

## Problem Description
Complex test combining parametrized structs with all subform types (scalar, entry, collection) that encounters both lifetime and trait bound issues.

## Investigation Required
1. Examine the combination of parametrized + subform issues
2. Check FormerDefinition trait implementation for parametrized types
3. Identify interaction between lifetime and trait bound problems

## Expected Outcome
Enable the test by fixing both lifetime and FormerDefinition trait issues.

## Priority
High - represents full feature integration

## Status
INVESTIGATED - Lifetime parameter handling failures confirmed

## Investigation Results
The test fails with multiple E0726 and E0106 lifetime-related errors when Former derives are enabled:

**Error Details:**
```
error[E0726]: implicit elided lifetime not allowed here
error[E0106]: missing lifetime specifier
error[E0261]: use of undeclared lifetime name 'child
```

**Root Cause:**
The macro cannot properly handle:
1. **Lifetime parameters in struct definitions** (`Parent<'child>`, `Child<'child, T>`)
2. **Where clauses with lifetime bounds** (`T: 'child + ?Sized`)
3. **Lifetime parameter propagation** to generated FormerDefinition types
4. **Implicit lifetime elision** in macro-generated code

**Specific Issues:**
1. `pub struct Parent<'child>` - macro doesn't recognize `'child` lifetime
2. `data: &'child T` - references with explicit lifetimes break macro generation
3. `T: 'child + ?Sized` - where clause lifetime constraints aren't handled
4. Generated code tries to use undeclared lifetimes

**Test Structure:**
- `Child<'child, T>` with lifetime parameter and generic type parameter
- `Parent<'child>` containing `Vec<Child<'child, str>>`
- Multiple subform attributes on the same field
- Complex lifetime relationships between parent and child

This represents one of the most complex test cases combining:
- Lifetime parameters
- Generic type parameters  
- Where clauses
- Multiple subform attributes
- Parent-child lifetime relationships

## Status
Blocked - requires macro-level fix for comprehensive lifetime parameter support