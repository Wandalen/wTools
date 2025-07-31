# Blocked Tests Execution Plan

## Overview
Plan to systematically fix all 18 blocked tests in the former crate, following the macro rulebook's one-test-at-a-time approach.

## Execution Priority Order

### Phase 1: Core Functionality Issues (High Priority) - COMPLETED
1. **fix_collection_former_hashmap.md** - ✅ INVESTIGATED
   - **Root Cause**: Macro type parameter generation for `HashMapDefinition` with `subform_collection`
   - **Issue**: Expected `ParentFormer<Definition>` but found `Child` in FormingEnd trait implementations
   - **Status**: Requires macro-level fix for HashMapDefinition type parameter mapping

2. **fix_parametrized_struct_imm.md** - ✅ INVESTIGATED
   - **Root Cause**: Multiple fundamental macro issues with generic parameter handling
   - **Issues**: Generic constraint syntax errors, undeclared lifetimes, trait bounds not propagated
   - **Status**: Requires macro-level fix for generic parameter parsing and trait bound propagation

3. **fix_subform_all_parametrized.md** - ✅ INVESTIGATED
   - **Root Cause**: Comprehensive lifetime parameter handling failures
   - **Issues**: E0726 implicit elided lifetime, E0106 missing lifetime specifier, E0261 undeclared lifetime
   - **Status**: Requires macro-level fix for lifetime parameter support

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
11. **fix_name_collisions.md** - ✅ RESOLVED - Successfully fixed by scoping conflicts in sub-module
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

## Phase 1 Investigation Summary

**Key Findings:**
All three Phase 1 tests require **macro-level fixes** - these are not simple test fixes but fundamental issues in the Former derive macro implementation.

### Critical Issues Identified:
1. **Type Parameter Mapping**: `HashMapDefinition` with `subform_collection` has incompatible type mappings
2. **Generic Parameter Parsing**: Macro cannot handle `<K: Trait + Trait>` syntax properly  
3. **Lifetime Parameter Support**: Macro fails with any explicit lifetime parameters (`<'a>`)
4. **Trait Bound Propagation**: Constraints from struct definitions not propagated to generated code

### Impact Assessment:
These findings suggest that **most blocked tests have similar macro-level root causes**:
- Tests with generic parameters will likely fail similarly to `parametrized_struct_imm`
- Tests with lifetimes will likely fail similarly to `subform_all_parametrized`  
- Tests with HashMap collections will likely fail similarly to `collection_former_hashmap`

## Revised Estimated Impact (Updated after Phase 5 success)
- **Best case**: +4-6 tests (some edge cases are fixable without macro changes)
- **Realistic case**: +2-4 tests (edge cases and simple fixes)
- **Minimum case**: +1-2 tests (proven that some fixes are possible)

**Proven Success**: The `name_collisions` fix demonstrates that some blocked tests can be resolved with clever test modifications rather than macro changes.

**Updated Recommendation**: Continue investigating tests that might be fixable through test modifications, workarounds, or simple changes rather than macro rewrites.

## Dependencies
- Some fixes may unblock others (e.g., fixing FormerBegin lifetime might fix multiple manual tests)
- Collection type fixes may share common root causes
- Generic parameter fixes may be interconnected

## Next Steps
1. Start with Phase 1, task 1: fix_collection_former_hashmap.md
2. Follow investigation → fix → test → document cycle for each task
3. Update this plan based on findings during execution