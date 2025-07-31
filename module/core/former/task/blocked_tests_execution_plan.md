# Blocked Tests Execution Plan

## Overview
Plan to systematically fix all 18 blocked tests in the former crate, following the macro rulebook's one-test-at-a-time approach.

## Execution Priority Order

### Phase 1: Core Functionality Issues (High Priority)
1. **fix_collection_former_hashmap.md** - HashMap is critical collection type
2. **fix_parametrized_struct_imm.md** - Generic parameter support is core functionality  
3. **fix_subform_all_parametrized.md** - Full feature integration test

### Phase 2: Collection Type Mismatches (Medium Priority)
4. **fix_subform_collection_basic.md** - Basic subform collection functionality
5. **fix_collection_former_btree_map.md** - BTreeMap collection support
6. **fix_subform_collection_playground.md** - Experimental subform collections

### Phase 3: Generic Parameter & Trait Bounds (Medium Priority)
7. **fix_parametrized_struct_where.md** - Where clause trait bounds
8. **fix_parametrized_field.md** - Parametrized field support
9. **fix_parametrized_field_where.md** - Field where clause support

### Phase 4: Manual Implementation Consistency (Medium Priority)
10. **fix_manual_tests_formerbegin_lifetime.md** - Batch fix for 7 manual tests:
    - subform_collection_basic_manual.rs
    - parametrized_struct_manual.rs
    - subform_collection_manual.rs
    - subform_scalar_manual.rs
    - subform_entry_manual.rs
    - subform_entry_named_manual.rs
    - subform_entry_hashmap_custom.rs

### Phase 5: Edge Cases & Future Features (Low Priority)
11. **fix_name_collisions.md** - std type name conflicts
12. **fix_standalone_constructor_derive.md** - Unimplemented feature

## Execution Approach
1. **One test at a time** - Follow macro rulebook principles
2. **Investigate first** - Run each test to see actual errors before fixing
3. **Understand root cause** - Don't just patch symptoms
4. **Test thoroughly** - Ensure fix doesn't break other tests
5. **Document findings** - Update task files with investigation results

## Success Criteria
- All 18 blocked tests either enabled and passing, or properly documented as known limitations
- Total test count increased from current 147 to maximum possible
- No regressions in currently passing tests
- Clear documentation of any remaining limitations

## Estimated Impact
- **Best case**: +18 tests (165 total)
- **Realistic case**: +10-15 tests (most fixable issues resolved)
- **Minimum case**: +5-8 tests (critical issues only)

## Dependencies
- Some fixes may unblock others (e.g., fixing FormerBegin lifetime might fix multiple manual tests)
- Collection type fixes may share common root causes
- Generic parameter fixes may be interconnected

## Next Steps
1. Start with Phase 1, task 1: fix_collection_former_hashmap.md
2. Follow investigation → fix → test → document cycle for each task
3. Update this plan based on findings during execution