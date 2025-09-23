# Fix collection_former_btree_map Test

## Issue
Test is disabled due to: "Complex collection type mismatch issues with subform"

## Files Involved
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/collection_former_btree_map.rs`
- `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/mod.rs` (line 143)

## Problem Description
The subformer test in this file (lines 160-195) has Former derives commented out due to complex collection type mismatch issues.

## Investigation Required
1. Examine the subformer function that uses BTreeMap with subform_collection
2. Identify the specific type mismatch between Parent and Child formers
3. Determine if it's related to BTreeMapDefinition handling

## Expected Outcome
Enable the Former derives and get the subformer test working with BTreeMap collections.

## Priority
Medium - BTreeMap is a standard collection that should work with subforms

## Status
Blocked - requires investigation