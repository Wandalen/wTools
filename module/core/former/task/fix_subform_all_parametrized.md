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
Blocked - E0726 + E0277 FormerDefinition trait issues