# Manual Testing Procedures for clone_dyn

## Overview

This document describes manual testing procedures for the `clone_dyn` crate, which provides procedural macros and helper functions for cloning trait objects. Manual testing complements automated tests by verifying example compilation, feature flag combinations, and real-world usage patterns.

## Prerequisites

- Rust toolchain (stable)
- Access to crate source code
- Terminal access for cargo commands

## Test Procedures

### Procedure 1: Example Compilation and Execution

**Purpose:** Verify the main example compiles and runs correctly with different feature combinations.

**Steps:**

1. **Test with all features enabled:**
   ```bash
   cd ../..
   cargo run --example clone_dyn_trivial --all-features
   ```

   **Expected Output:**
   ```
   1
   2
   3
   1
   2
   3
   ```

   **Verification:** Output shows numbers 1-3 printed twice (once from cloned iterator, once from original).

2. **Test with no features:**
   ```bash
   cargo run --example clone_dyn_trivial --no-default-features
   ```

   **Expected Output:** No output (example main function is empty when features disabled).

   **Verification:** Compiles successfully, no runtime output.

3. **Test with default features:**
   ```bash
   cargo run --example clone_dyn_trivial
   ```

   **Expected Output:** Same as step 1 (default includes all necessary features).

   **Verification:** Output shows numbers 1-3 printed twice.

### Procedure 2: Feature Flag Combinations

**Purpose:** Verify crate compiles with all valid feature flag combinations.

**Feature Flags:**
- `enabled` - Core functionality toggle
- `clone_dyn_types` - Re-exports from clone_dyn_types crate
- `derive_clone_dyn` - Procedural macro for deriving Clone on trait objects

**Test Matrix:**

| Test | Features | Expected Result | Command |
|------|----------|-----------------|---------|
| P2.1 | None | Compiles, no functionality | `cargo build --no-default-features` |
| P2.2 | `enabled` only | Compiles, limited functionality | `cargo build --no-default-features --features enabled` |
| P2.3 | `enabled,clone_dyn_types` | Compiles, no macro | `cargo build --no-default-features --features enabled,clone_dyn_types` |
| P2.4 | `enabled,derive_clone_dyn` | Compiles, macro available | `cargo build --no-default-features --features enabled,derive_clone_dyn` |
| P2.5 | All features | Full functionality | `cargo build --all-features` |
| P2.6 | Default | Equivalent to P2.5 | `cargo build` |

**Steps:**

1. Execute each command in the table
2. Verify compilation succeeds
3. Check for warnings (should be zero)
4. Note any unexpected errors

### Procedure 3: Real-World Usage Verification

**Purpose:** Manually verify the crate works for realistic use cases by examining example code.

**Focus Areas:**

1. **Trait Definition:**
   - Verify `#[clone_dyn]` macro can be applied to traits
   - Verify trait has associated bounds (Iterator, ExactSizeIterator, DoubleEndedIterator)
   - Verify CloneDyn bound is required on implementing types

2. **Trait Implementation:**
   - Verify blanket impl works for types meeting requirements
   - Verify CloneDyn bound is enforced

3. **Trait Object Usage:**
   - Verify `Box<dyn Trait>` can be created
   - Verify `.clone()` works on trait object
   - Verify cloned object behaves identically to original
   - Verify original object remains usable after cloning

**Steps:**

1. Read `examples/clone_dyn_trivial.rs` line by line
2. For each code section, verify behavior matches documentation
3. Check that example demonstrates:
   - Trait definition with `#[clone_dyn]`
   - Creating trait objects
   - Cloning trait objects
   - Using both original and cloned objects

### Procedure 4: Helper Function Verification

**Purpose:** Verify `clone()` and `clone_into_box()` helper functions work correctly.

**Test Cases:**

1. **clone_into_box with copyable types (i32):**
   - Function should box a copy of the value
   - Original value should remain unchanged

2. **clone_into_box with clonable types (String):**
   - Function should box a clone of the value
   - Original value should remain valid

3. **clone_into_box with slices (&str, &[i32]):**
   - Function should box the slice contents
   - Handles unsized types correctly

**Verification Method:** These are tested by automated tests in `tests/inc/only_test/basic.rs`. Manual verification involves:

1. Running tests: `w3 .test l::3` or `ctest3`
2. Checking test output for `clone_into_box` tests
3. Verifying all tests pass

### Procedure 5: Macro Expansion Verification (Advanced)

**Purpose:** Verify the `#[clone_dyn]` macro generates correct Clone implementations.

**Steps:**

1. Expand macros for inspection:
   ```bash
   cargo expand --example clone_dyn_trivial --features derive_clone_dyn
   ```

2. **Verify Generated Code:** The macro should generate 4 Clone implementations:
   - `impl Clone for Box<dyn Trait + '_>`
   - `impl Clone for Box<dyn Trait + Send + '_>`
   - `impl Clone for Box<dyn Trait + Sync + '_>`
   - `impl Clone for Box<dyn Trait + Send + Sync + '_>`

3. **Check Implementation:** Each impl should call `clone_into_box(&**self)`

**Note:** Requires `cargo-expand` tool: `cargo install cargo-expand`

### Procedure 6: Cross-Crate Integration

**Purpose:** Verify clone_dyn correctly re-exports from clone_dyn_types and clone_dyn_meta.

**Steps:**

1. **Check module structure:**
   ```bash
   cargo doc --no-deps --open --all-features
   ```

2. **Verify re-exports:**
   - `CloneDyn` trait should be available from `clone_dyn`
   - `clone()` function should be available
   - `clone_into_box()` function should be available
   - `clone_dyn` attribute macro should be available

3. **Test import paths:**
   ```rust
   use clone_dyn :: { CloneDyn, clone, clone_into_box, clone_dyn };
   ```
   All imports should work without ambiguity.

## Corner Cases to Verify Manually

1. **Empty Iterators:**
   - Example uses `Some(&data)` - verify behavior
   - Manually test with `get_iter(None)` scenario
   - Expected: Empty iterator clones correctly

2. **Iterator Position:**
   - After cloning, original iterator position should be preserved
   - Cloned iterator should start from same position
   - Both can be consumed independently

3. **Multiple Clones:**
   - Verify cloning a clone works correctly
   - No degradation or corruption

4. **Drop Behavior:**
   - Original and clones should drop independently
   - No double-free or memory leaks

## Common Issues and Troubleshooting

### Issue: Example doesn't compile

**Possible Causes:**
- Missing features (ensure --all-features or correct feature flags)
- Outdated dependencies (run `cargo update`)
- Rust version incompatibility (requires stable Rust)

**Resolution:** Check feature flags and Rust version.

### Issue: Example runs but produces no output

**Possible Cause:** Features disabled (example main is empty without features).

**Resolution:** Add `--all-features` or `--features enabled,derive_clone_dyn`.

### Issue: Macro not found

**Possible Cause:** `derive_clone_dyn` feature not enabled.

**Resolution:** Enable feature: `--features derive_clone_dyn`.

## Test Completion Checklist

- [ ] P1: Example runs with --all-features (output: 1,2,3,1,2,3)
- [ ] P1: Example runs with --no-default-features (no output)
- [ ] P1: Example runs with default features (output: 1,2,3,1,2,3)
- [ ] P2: All feature combinations compile successfully
- [ ] P3: Real-world usage patterns work as documented
- [ ] P4: Helper functions verified via automated tests
- [ ] P5: Macro expansion produces correct Clone impls (optional)
- [ ] P6: Cross-crate integration verified via documentation

## Reporting Issues

If manual testing reveals issues:

1. Document exact steps to reproduce
2. Note expected vs actual behavior
3. Record feature flags used
4. Include relevant error messages
5. Create reproducing test in `tests/` if applicable
6. Report via project issue tracker

## Related Documentation

- Main README: `../../readme.md`
- Automated Tests: `../readme.md`
- Example Code: `../../examples/clone_dyn_trivial.rs`
- API Documentation: Run `cargo doc --no-deps --open --all-features`
