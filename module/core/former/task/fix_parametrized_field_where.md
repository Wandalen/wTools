# Fix parametrized_field_where Test

## Issue
Test is disabled due to: "E0726 implicit elided lifetime not allowed here"

## Files Involved
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/parametrized_field_where.rs`
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/mod.rs` (line 116)

## Problem Description
Similar to parametrized_field but uses where clauses with lifetime constraints. The macro fails to handle implicit lifetime elision in where clauses.

## Investigation Required
1. Examine lifetime constraints in where clauses
2. Check macro's where clause lifetime parsing
3. Identify specific elision failures

## Expected Outcome
Enable the test by fixing lifetime elision in where clause handling.

## Priority
Medium - advanced lifetime + where clause combination

## Status
Blocked - E0726 implicit elided lifetime not allowed