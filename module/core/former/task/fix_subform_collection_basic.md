# Fix subform_collection_basic Test

## Issue
Test is disabled due to: "Complex collection type mismatch issues"

## Files Involved
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/subform_collection_basic.rs`
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/mod.rs` (line 70)

## Problem Description
The test has complex collection type mismatch issues that prevent it from compiling when Former derive is enabled.

## Investigation Required
1. Examine the specific type mismatches in the test
2. Identify root cause in macro generation
3. Determine if it's a fundamental limitation or fixable issue

## Expected Outcome
Enable the test by resolving type mismatch issues in collection handling within the Former macro.

## Priority
Medium - represents core collection functionality that should work

## Status
Blocked - requires investigation