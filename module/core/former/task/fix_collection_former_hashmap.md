# Fix collection_former_hashmap Test

## Issue
Test is disabled due to: "Complex collection type mismatch issues with subform"

## Files Involved
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/collection_former_hashmap.rs`
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/mod.rs` (line 151)

## Problem Description
The test has Former derives enabled (lines 162, 169) but is blocked due to subform collection type mismatch issues.

## Investigation Required
1. Run the test to see specific compilation errors
2. Examine the subformer function with HashMap and subform_collection
3. Compare with working collection tests to identify differences

## Expected Outcome
Resolve type mismatch issues to get HashMap working with subform collections.

## Priority
High - HashMap is a critical collection type

## Status
INVESTIGATED - Root cause identified

## Investigation Results
The issue is in the macro's type parameter generation for `HashMapDefinition` with `subform_collection`.

**Error Details:**
- Expected: `ParentFormer<Definition>`
- Found: `Child`
- The macro generates `FormingEnd` implementations that expect `ParentFormer<Definition>` in the collection but the actual collection contains `Child` objects

**Root Cause:**
`HashMapDefinition` with `subform_collection` has incompatible type parameter mapping. The macro expects:
```rust
FormingEnd<HashMapDefinitionTypes<_, ParentFormer<Definition>, _, Hmap<u32, ParentFormer<_>>>>
```
But it finds:
```rust
FormingEnd<HashMapDefinitionTypes<_, Child, _, ParentFormer<_>>
```

**Solution Required:**
This appears to be a fundamental issue in the macro's handling of HashMap with subform_collection. The type parameter mapping needs to be fixed at the macro generation level.

## Status
Blocked - requires macro-level fix for HashMapDefinition type parameter mapping