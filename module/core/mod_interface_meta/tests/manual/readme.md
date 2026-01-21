# Manual Testing Plan for mod_interface_meta

## Overview

This document describes the manual testing plan and records findings from systematic corner case testing of the `mod_interface` procedural macro.

## Testing Sessions

### Session 1: Corner Case Discovery (2026-01-21)

**Test Scope**: Systematic testing of use statement variations, micro-modules, namespace combinations, and integration patterns.

**Method**: Created comprehensive test file `tests/corner_cases_test.rs` with 15 test modules covering 30+ test cases.

**Critical Bugs Found**:

#### Bug 1: Bare `use` Statement Not Supported
- **Test**: UC-05 (Implicit use without layer keyword)
- **Input**: `mod_interface! { use my_fn; }`
- **Expected**: Export to all four layers (own, orphan, exposed, prelude)
- **Actual**: Compile error "unresolved import" and "expected type, found function"
- **Severity**: **HIGH** - This is documented in spec.md but completely non-functional
- **Root Cause**: Macro parser doesn't handle bare `use` statements, only layer-prefixed ones

#### Bug 2: `own use` Items Don't Propagate to Higher Layers
- **Test**: UC-01 (own use simple)
- **Input**: `mod_interface! { own use my_fn; }`
- **Expected**: Item available in own→orphan→exposed→prelude hierarchy (based on spec and examples)
- **Actual**: Item only available in `own` layer, missing from orphan/exposed/prelude
- **Severity**: **CRITICAL** - Violates the four-layer propagation architecture
- **Root Cause**: Macro's record handler for `own use` doesn't generate re-exports for higher layers

#### Bug 3: `orphan use` Items Don't Propagate to exposed/prelude
- **Test**: UC-02 (orphan use simple)
- **Input**: `mod_interface! { orphan use my_fn; }`
- **Expected**: Item available in orphan→exposed→prelude layers
- **Actual**: Item only available in `orphan` layer, missing from exposed/prelude
- **Severity**: **CRITICAL** - Violates propagation architecture
- **Root Cause**: Similar to Bug 2, missing re-export generation

#### Bug 4: `exposed use` Items Don't Propagate to prelude
- **Test**: UC-03 (exposed use simple)
- **Input**: `mod_interface! { exposed use my_fn; }`
- **Expected**: Item available in exposed→prelude layers
- **Actual**: Item only available in `exposed` layer, missing from prelude
- **Severity**: **HIGH** - Violates propagation architecture
- **Root Cause**: Similar to Bugs 2-3, missing re-export generation

#### Bug 5: Micro-Modules Require Filesystem Files
- **Test**: MM-01 through MM-04 (micro-module declarations)
- **Input**: `mod_interface! { own mod micro_own; }`
- **Expected**: Generate inline module or use existing module
- **Actual**: "file not found for module" error, requires actual .rs file
- **Severity**: **MEDIUM** - Not a bug if intended behavior, but unclear from documentation
- **Note**: This matches how Rust's `mod` declarations work, but spec doesn't clarify filesystem requirements

**Tests That Passed**:
- UC-04: `prelude use` works correctly (item appears in prelude layer)
- UC-09: Rename with `as` keyword works correctly
- UC-13: Multiple use statements in same layer work correctly
- IP-03: Empty mod_interface body compiles successfully
- IP-04: Private namespace with content doesn't interfere

**Incorrect Test Assumptions**:
- IP-01 trait test: Assumed trait could be used in bound, but trait has no implementors (test design issue, not macro bug)

**Documentation Issues**:
- spec.md shows bare `use` syntax but it doesn't work
- Propagation rules not clearly documented (which layers inherit from which)
- Micro-module filesystem requirements not documented

## Next Steps

1. Create focused bug reproducer tests with `bug_reproducer` attribute for each critical bug
2. Investigate macro source code in `src/impls.rs` and `src/record.rs` to understand propagation logic
3. Implement fixes for propagation bugs (Bugs 2-4)
4. Decide on bare `use` support (Bug 1) - either implement or remove from documentation
5. Clarify micro-module documentation (Bug 5)

## Test Coverage Matrix

Based on corner case matrix in `-corner_cases_matrix.md`:

| Category | Test ID | Status | Notes |
|----------|---------|--------|-------|
| Use Statements | UC-01 | **FAILED** | own use doesn't propagate |
| Use Statements | UC-02 | **FAILED** | orphan use doesn't propagate |
| Use Statements | UC-03 | **FAILED** | exposed use doesn't propagate |
| Use Statements | UC-04 | **PASSED** | prelude use works |
| Use Statements | UC-05 | **FAILED** | bare use not supported |
| Use Statements | UC-09 | **PASSED** | rename works |
| Use Statements | UC-13 | **PASSED** | multiple use works |
| Use Statements | UC-14 | **FAILED** | mixed fails (due to bare use bug) |
| Micro-Modules | MM-01-04 | **FAILED** | requires filesystem files |
| Micro-Modules | MM-07 | **FAILED** | requires filesystem files |
| Namespace Combinations | NC-06 | **FAILED** | propagation bug affects this |
| Namespace Combinations | NC-07 | **PASSED** | prelude-only works |
| Integration | IP-01 | **PARTIAL** | works except trait test design |
| Integration | IP-03 | **PASSED** | empty body works |
| Integration | IP-04 | **PASSED** | private content works |

### Session 2: Propagation Bug Fixes and Rename Support (2026-01-21)

**Test Scope**: Fixing propagation bugs discovered in Session 1 and verifying all working functionality.

**Method**: Applied TDD workflow - fixed `record_use_explicit` to propagate items through namespace hierarchy, added rename support, verified with comprehensive test suite.

**Fixes Applied**:

#### Fix 1: Namespace Propagation (Issue-001)
- **Location**: `src/impls.rs` lines 245-309 (`record_use_explicit` function)
- **Root Cause**: Function only added items to declared layer without generating re-exports for higher layers
- **Fix**: Added match statement to generate explicit re-exports using `super::[layer]::#final_ident` pattern
- **Verification**: All propagation tests pass (UC-01, UC-02, UC-03, propagation_bug_test.rs: 4/4)

#### Fix 2: Rename Support (Use ... as Alias)
- **Location**: `src/impls.rs` lines 220-236 (final_ident extraction)
- **Root Cause**: Propagation used original path instead of alias name
- **Fix**: Extract final identifier from UseTree (alias if present, otherwise last path segment)
- **Verification**: UC-09 rename test passes (2/2 tests)

#### Fix 3: Result Type Consistency
- **Location**: `src/impls.rs` lines 160-197 (`record_use_implicit`), lines 472-477 (call sites)
- **Root Cause**: `record_use_implicit` returned `()` while `record_use_explicit` returned `Result`, causing type mismatch in try_for_each closure
- **Fix**: Changed `record_use_implicit` signature to return `syn::Result<()>`, added `?` operators at call sites
- **Verification**: Compilation succeeds without type errors

**Test Results**: 32/32 passing tests
- Working corner cases: 22/22 ✅
- Integration tests: 5/5 ✅
- Propagation bug tests: 4/4 ✅
- Smoke test: 1/1 ✅

**Still Broken** (Unimplemented Features):
- UC-05: Bare `use` syntax (requires parser changes)
- MM-01-04, MM-07: Micro-modules require filesystem files (expected Rust behavior)
- UC-14: Mixed implicit/explicit use (depends on UC-05)

## Lessons Learned

1. **Propagation fix requires relative paths**: Using `super::[layer]::#ident` instead of `__all__::[layer]::#ident` avoids privacy violations and double-prefixing.

2. **Rename handling needs final identifier extraction**: Can't blindly re-export paths - must use the alias name for propagated imports.

3. **Result type consistency is critical**: When refactoring function signatures to return Result, update ALL call sites atomically to avoid type mismatches.

4. **Test methodology worked**: Creating comprehensive corner case matrix before testing revealed multiple critical bugs efficiently.

5. **Bare use syntax not yet supported**: Requires additional parser/record handling beyond current implementation.
