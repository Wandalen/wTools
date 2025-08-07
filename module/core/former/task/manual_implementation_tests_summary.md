# Manual Implementation Tests - Blocked Issues Summary

## Overview
This document summarizes all blocked manual implementation tests and their dependencies. These tests were systematically analyzed and disabled due to various compilation issues.

## Successfully Fixed Tests ✅
1. **`subform_collection_basic_manual`** - Fixed lifetime parameter issues in `FormerBegin` trait usage
2. **`parametrized_struct_manual`** - Already working correctly

## Blocked Tests ❌

### 1. Missing Dependencies Pattern
**Affected Tests:**
- `subform_collection_manual` 
- `subform_scalar_manual`
- `subform_entry_named_manual`
- `subform_entry_hashmap_custom`

**Common Issues:**
- Missing `ParentFormer` type imports
- Missing attribute macros (`scalar`, `subform_entry`)
- Missing subformer types (`ChildAsSubformer`, `ChildAsSubformerEnd`, etc.)
- Missing trait implementations (`EntityToStorage`)

**Root Cause:** Test module isolation prevents access to types defined in other test modules.

### 2. Complex Lifetime Bounds Issue
**Affected Test:**
- `subform_entry_manual`

**Issue:** Higher-ranked trait bounds (`for<'a>`) conflict with borrow checker limitations.

**Root Cause:** Fundamental limitation in Rust's current borrow checker when handling HRTB with generic parameters.

## Resolution Strategy

### Short Term (2-4 hours each)
1. **Import Resolution**: Add proper imports for missing types
2. **Trait Implementation**: Implement missing traits like `EntityToStorage`
3. **Attribute Availability**: Ensure required attributes are available in test context

### Medium Term (4-8 hours)
1. **Test Architecture Review**: Restructure test modules for better type accessibility
2. **Generated vs Manual**: Evaluate which tests should use generated code instead
3. **Dependency Management**: Create shared test infrastructure

### Long Term (8+ hours)
1. **HRTB Issue Resolution**: Redesign trait bounds to avoid borrow checker limitations
2. **API Simplification**: Reduce complexity of manual implementation requirements

## Recommended Priority Order

### High Priority
1. `subform_entry_manual` - Core functionality, requires trait API changes
2. `subform_collection_manual` - Basic collection functionality

### Medium Priority  
3. `subform_scalar_manual` - Scalar subform functionality
4. `subform_entry_named_manual` - Named entry functionality

### Low Priority
5. `subform_entry_hashmap_custom` - Advanced/custom functionality

## Individual Task Files
- [fix_subform_collection_manual_dependencies.md](./fix_subform_collection_manual_dependencies.md)
- [fix_subform_scalar_manual_dependencies.md](./fix_subform_scalar_manual_dependencies.md)  
- [fix_subform_entry_manual_lifetime_bounds.md](./fix_subform_entry_manual_lifetime_bounds.md)
- [fix_subform_entry_named_manual_dependencies.md](./fix_subform_entry_named_manual_dependencies.md)
- [fix_subform_entry_hashmap_custom_dependencies.md](./fix_subform_entry_hashmap_custom_dependencies.md)

## Success Metrics
- All manual implementation tests compile successfully
- All manual implementation tests pass their test cases
- No reduction in test coverage
- Maintain backward compatibility of public APIs

## Notes
- All blocked tests are currently disabled with detailed comments in `mod.rs`
- The successful fix of `subform_collection_basic_manual` provides a pattern for lifetime parameter issues
- Some tests may be better converted to use generated code rather than full manual implementation