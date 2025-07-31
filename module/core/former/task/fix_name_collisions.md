# Fix name_collisions Test

## Issue
Test is disabled due to: "Name collision with std types causes E0308 type conflicts"

## Files Involved
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/name_collisions.rs`
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/mod.rs` (line 104)

## Problem Description
Test fails with E0308 error: "expected `std::option::Option<_>`, found fn item `fn() {name_collisions::None}`"
This indicates a naming conflict with standard library types.

## Investigation Required
1. Examine the specific name collisions in the test
2. Identify how the macro generates code that conflicts with std types
3. Determine if macro should handle std name conflicts automatically

## Expected Outcome
Either fix the macro to avoid std name conflicts or document this as a known limitation with workarounds.

## Priority
Medium - edge case but represents important macro robustness

## Status
Blocked - E0308 type conflicts with std