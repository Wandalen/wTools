# Fix Manual Tests with FormerBegin Lifetime Issues

## Issue
Multiple manual tests are disabled due to: "FormerBegin lifetime parameter in manual code"

## Files Involved
- `subform_collection_basic_manual.rs` (line 72)
- `parametrized_struct_manual.rs` (line 120)
- `subform_collection_manual.rs` (line 176)
- `subform_scalar_manual.rs` (line 191)
- `subform_entry_manual.rs` (line 201)
- `subform_entry_named_manual.rs` (line 206)
- `subform_entry_hashmap_custom.rs` (line 218)

## Problem Description
Manual implementations require explicit FormerBegin lifetime parameters, but the manual code doesn't specify them correctly, causing E0106 "missing lifetime specifier" errors.

## Investigation Required
1. Identify the correct FormerBegin lifetime signature
2. Update all manual implementations to use proper lifetime parameters
3. Ensure consistency between derive and manual implementations

## Expected Outcome
Enable all manual tests by fixing FormerBegin lifetime parameter specifications.

## Priority
Medium - manual tests verify derive macro correctness

## Status
Blocked - E0106 missing lifetime specifier for FormerBegin

## Batch Fix Approach
All these tests have the same root cause and can be fixed together by:
1. Determining the correct FormerBegin lifetime signature from working examples
2. Applying the same fix pattern to all manual implementations
3. Testing each one individually after the fix