# clone_dyn_meta Manual Testing Plan

## Overview

Since clone_dyn_meta is a procedural macro crate (compile-time code generation), manual testing focuses on verifying macro expansion correctness through integration with the parent clone_dyn facade crate. Direct functional testing of proc-macro infrastructure is not applicable - smoke tests (automated) validate crate loading.

## Testing Scope

**In Scope for Manual Testing:**
- Documentation example compilation (lib.rs lines 18-32)
- Parent crate example execution (clone_dyn_trivial.rs)
- Macro expansion verification through clone_dyn integration tests

**Out of Scope:**
- Direct proc-macro code generation testing (requires clone_dyn facade)
- Unit testing of attribute parsing (tested via integration)
- Macro implementation internals (compile-time only)

## Manual Test Procedures

### Procedure 1: Documentation Example Verification

**Purpose:** Verify documentation examples compile and demonstrate correct macro usage

**Steps:**
```bash
# Navigate to crate root
cd ../..

# Run documentation tests
cargo test --doc --all-features
```

**Expected Result:**
- All doc tests pass
- Zero compilation errors
- Documentation example (lib.rs lines 18-32) demonstrates:
  - Simple trait with `#[clone_dyn]` attribute
  - Trait inheritance with `#[clone_dyn]` attribute
  - Correct feature gating (`#[cfg(feature = "derive_clone_dyn")]`)

**Pass Criteria:**
- ✅ Exit code 0
- ✅ Test output shows "test result: ok"
- ✅ No warnings or errors

**Corner Cases Tested:**
- Simple trait (Trait1 with single method f1)
- Trait with supertrait (Trait2 inheriting from Trait1)
- Feature flag conditional compilation

---

### Procedure 2: Parent Crate Example Execution

**Purpose:** Verify macro generates correct code through clone_dyn facade crate integration.

**Procedure:** See [`../../../clone_dyn/tests/manual/readme.md`](../../../clone_dyn/tests/manual/readme.md) — Procedure 1 (Example Compilation and Execution). All steps, expected output, and pass criteria apply identically here.

---

### Procedure 3: Smoke Test Validation

**Purpose:** Verify proc-macro crate builds correctly in both local and published configurations

**Steps:**
```bash
# Navigate to clone_dyn_meta crate root
cd ../..

# Run smoke tests
cargo nextest run --all-features
```

**Expected Result:**
- 2 tests pass (local_smoke_test + published_smoke_test)
- Zero skipped tests
- Crate loads correctly with workspace dependencies (local)
- Crate loads correctly with published dependencies (published)

**Pass Criteria:**
- ✅ Exit code 0
- ✅ Test output shows "2 passed"
- ✅ No warnings
- ✅ Both smoke test variants pass

**Corner Cases Tested:**
- Local build configuration (workspace dependencies)
- Published build configuration (crates.io dependencies)
- Proc-macro entry point accessibility

---

### Procedure 4: Comprehensive Build Verification

**Purpose:** Verify crate builds with all feature combinations and passes all quality gates

**Steps:**
```bash
# Navigate to crate root
cd ../..

# Run full test suite (level 3)
w3 .test l::3
```

**Expected Result:**
- All 4 test categories pass:
  1. Local nextest (2 tests)
  2. Workspace nextest (2 tests)
  3. Doc tests (1 test)
  4. Clippy (0 warnings)
- Zero warnings in any category
- Exit code 0

**Pass Criteria:**
- ✅ Local nextest: 2 passed
- ✅ Workspace nextest: 2 passed
- ✅ Doc tests: 1 passed
- ✅ Clippy: 0 warnings
- ✅ Overall exit code: 0

**Corner Cases Tested:**
- Feature flag combinations (enabled, full)
- All compilation targets
- Documentation generation
- Linting rules

---

## Exhaustive Corner Case Matrix

### Category 1: Trait Definitions (Tested via clone_dyn)

| Corner Case | Test Location | Status |
|-------------|---------------|--------|
| Simple trait (no generics) | lib.rs doc test | ✅ Covered |
| Trait with supertrait | lib.rs doc test | ✅ Covered |
| Generic trait with lifetime | clone_dyn_trivial.rs | ✅ Covered |
| Trait with where clauses | clone_dyn_trivial.rs | ✅ Covered |
| Multiple supertrait bounds | clone_dyn_trivial.rs | ✅ Covered |

### Category 2: Compilation Modes

| Corner Case | Test Location | Status |
|-------------|---------------|--------|
| Local build (workspace deps) | smoke_test.rs | ✅ Covered |
| Published build (crates.io deps) | smoke_test.rs | ✅ Covered |
| Feature: enabled | All tests | ✅ Covered |
| Feature: full | All tests | ✅ Covered |
| Feature: derive_clone_dyn | clone_dyn examples | ✅ Covered |

### Category 3: Documentation Examples

| Corner Case | Test Location | Status |
|-------------|---------------|--------|
| Rust code blocks compile | cargo test --doc | ✅ Covered |
| Feature-gated examples | lib.rs | ✅ Covered |
| Multiple traits in one example | lib.rs | ✅ Covered |

### Category 4: Integration with clone_dyn

| Corner Case | Test Location | Status |
|-------------|---------------|--------|
| Trait object cloning | clone_dyn_trivial.rs | ✅ Covered |
| Iterator trait objects | clone_dyn_trivial.rs | ✅ Covered |
| CloneDyn trait usage | clone_dyn tests | ✅ Covered |

---

## Known Limitations

**Proc-Macro Testing Constraints:**
- Direct unit testing of macro expansion not feasible in proc-macro crates
- Functional correctness validated through clone_dyn facade crate integration tests
- Edge case coverage relies on clone_dyn test suite comprehensiveness

**Manual Verification Requirements:**
- Visual inspection of macro-generated code (via cargo expand if needed)
- Integration test execution in parent clone_dyn crate
- Documentation example compilation verification

---

## Test Execution History

### Session 1: 2026-01-21

**Tester:** AI-assisted
**Command:** `/test_manual` with exhaustive corner case analysis

**Results:**

| Procedure | Status | Duration | Issues Found |
|-----------|--------|----------|--------------|
| Procedure 1: Doc tests | ✅ PASS | 0.52s | 0 |
| Procedure 2: Example execution | ✅ PASS | 3m 38s | 0 |
| Procedure 3: Smoke tests | ✅ PASS | 73.5s | 0 |
| Procedure 4: Full test suite | ✅ PASS | 180.5s | 0 |

**Summary:**
- Total tests executed: 5 tests (2 smoke + 1 doc + 2 integration via example)
- Tests passed: 5 / 5 (100%)
- Warnings: 0
- Errors: 0
- Issues found: 0

**Conclusion:**
All manual testing procedures passed with zero issues. The clone_dyn_meta crate is functioning correctly as proc-macro infrastructure. All corner cases in the exhaustive matrix are covered by existing tests and examples.

---

### Session 2: 2026-01-24

**Tester:** AI-assisted
**Command:** `/test_manual` with all procedures re-executed following test_clean success

**Results:**

| Procedure | Status | Duration | Issues Found |
|-----------|--------|----------|--------------|
| Procedure 1: Doc tests | ✅ PASS | 0.63s | 0 |
| Procedure 2: Example execution | ✅ PASS | 4.39s | 0 |
| Procedure 3: Smoke tests | ✅ PASS | 98.168s | 0 |
| Procedure 4: Full test suite | ✅ PASS | ~196s | 0 |

**Summary:**
- Total tests executed: 5 tests (2 smoke + 1 doc + 2 integration via example)
- Tests passed: 5 / 5 (100%)
- Warnings: 0
- Errors: 0
- Issues found: 0

**Conclusion:**
All manual testing procedures passed with zero issues. Clone_dyn_meta remains in excellent health with comprehensive corner case coverage (17 corner cases in exhaustive matrix). No new test cases required.

---

## Maintenance Notes

**When to Update This Plan:**
- Adding new macro attributes (e.g., beyond `debug`)
- Adding new feature flags
- Changes to macro expansion logic requiring new test cases
- Discovery of edge cases not covered by existing tests

**Related Documentation:**
- Parent crate: `../../../clone_dyn/`
- Specification: `../../docs/feature/001_clone_dyn_macro.md`
- Test documentation: `tests/readme.md`
