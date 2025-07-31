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
Blocked - requires investigation