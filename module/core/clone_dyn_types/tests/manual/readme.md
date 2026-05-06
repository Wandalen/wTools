# Manual Testing Procedures for clone_dyn_types

## Overview

This document describes manual testing procedures for the `clone_dyn_types` crate, which provides the `CloneDyn` trait and helper functions for cloning trait objects without requiring procedural macros. Manual testing complements automated tests by verifying example compilation, feature flag combinations, and manual Clone implementations.

## Prerequisites

- Rust toolchain (stable)
- Access to crate source code
- Terminal access for cargo commands

## Test Procedures

### Procedure 1: Example Compilation and Execution

**Purpose:** Verify the manual Clone implementation example compiles and runs correctly.

**Steps:**

1. **Test with all features enabled:**
   ```bash
   cd ../..
   cargo run --example clone_dyn_types_trivial --all-features
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
   cargo run --example clone_dyn_types_trivial --no-default-features
   ```

   **Expected Output:** No output (example main function is empty when features disabled).

   **Verification:** Compiles successfully, no runtime output.

3. **Test with enabled feature only:**
   ```bash
   cargo run --example clone_dyn_types_trivial --no-default-features --features enabled
   ```

   **Expected Output:** Same as step 1.

   **Verification:** Output shows numbers 1-3 printed twice.

### Procedure 2: Feature Flag Combinations

**Purpose:** Verify crate compiles with all valid feature flag combinations.

**Feature Flags:**
- `enabled` - Core functionality toggle

**Test Matrix:**

| Test | Features | Expected Result | Command |
|------|----------|-----------------|---------|
| P2.1 | None | Compiles, no functionality | `cargo build --no-default-features` |
| P2.2 | `enabled` | Full functionality | `cargo build --no-default-features --features enabled` |
| P2.3 | All features | Equivalent to P2.2 | `cargo build --all-features` |
| P2.4 | Default | Equivalent to P2.2 | `cargo build` |

**Steps:**

1. Execute each command in the table
2. Verify compilation succeeds
3. Check for warnings (should be zero)
4. Note any unexpected errors

### Procedure 3: Manual Clone Implementation Verification

**Purpose:** Verify manual Clone implementation for `Box<dyn Trait>` works without procedural macros.

**Key Difference from clone_dyn:**
- This crate demonstrates manual Clone implementation
- No `#[clone_dyn]` macro used
- Explicit `impl Clone for Box<dyn IterTrait>` provided
- Demonstrates lower-level usage for users who prefer explicit control

**Focus Areas:**

1. **CloneDyn Trait:**
   - Verify `CloneDyn` trait is in scope
   - Verify trait is used as bound on implementing types

2. **Manual Clone Implementation:**
   - Verify explicit `impl Clone for Box<dyn IterTrait>` compiles
   - Verify implementation calls `clone_into_box(&**self)`
   - Verify `#[allow(non_local_definitions)]` is present (required for trait object Clone impls)

3. **Trait Object Usage:**
   - Same as clone_dyn, but with manual Clone impl

**Steps:**

1. Read `examples/clone_dyn_types_trivial.rs` line by line
2. Compare with clone_dyn example to identify differences
3. Verify manual Clone impl section (lines 83-92):
   ```rust
   impl< 'c, T > Clone for Box< dyn IterTrait<'c, T > + 'c>
   {
     fn clone( &self ) -> Self
     {
       clone_dyn_types::clone_into_box( &**self )
     }
   }
   ```
4. Verify this approach works without macro

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

**Verification Method:** These are tested by automated tests. Manual verification involves:

1. Running tests: `w3 .test l::3` or `ctest3`
2. Checking test output for helper function tests
3. Verifying all tests pass

### Procedure 5: Fat Pointer Handling (Advanced)

**Purpose:** Understand and verify fat pointer manipulation in `clone_into_box`.

**Background:**
- Trait objects are fat pointers (data pointer + vtable pointer)
- `clone_into_box` uses fat pointer manipulation to clone trait objects
- This is the core mechanism enabling trait object cloning

**Steps:**

1. **Understand the mechanism:**
   - Fat pointer extracted from trait object reference
   - Underlying value cloned via `CloneDyn::__clone_dyn` (doc-hidden internal method)
   - New fat pointer constructed with same vtable

2. **Verify via documentation:**
   ```bash
   cargo doc --no-deps --open --all-features
   ```
   Read `clone_into_box` function documentation

3. **Check safety invariants:**
   - Function is safe (not `unsafe`)
   - Relies on `CloneDyn` trait for safety guarantees
   - No direct pointer manipulation exposed to users

### Procedure 6: CloneDyn Trait Verification

**Purpose:** Verify the CloneDyn trait is properly defined and usable.

**Steps:**

1. **Check trait definition:**
   ```bash
   cargo doc --no-deps --open --all-features
   ```
   Navigate to `CloneDyn` trait documentation

2. **Verify trait requirements:**
   - Trait is object-safe (can be used in trait objects)
   - Has doc-hidden internal method `__clone_dyn`; `clone_into_box` is a free function (not a method)
   - Types implementing CloneDyn can be cloned via trait objects

3. **Test trait bound enforcement:**
   - Verify example requires `CloneDyn` bound
   - Without CloneDyn, cloning should fail to compile

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

5. **Manual vs Macro Equivalence:**
   - Manual Clone impl should behave identically to macro-generated
   - No performance or correctness differences

## Common Issues and Troubleshooting

### Issue: Example doesn't compile

**Possible Causes:**
- Missing features (ensure --all-features or --features enabled)
- Outdated dependencies (run `cargo update`)
- Rust version incompatibility (requires stable Rust)

**Resolution:** Check feature flags and Rust version.

### Issue: Example runs but produces no output

**Possible Cause:** Features disabled (example main is empty without features).

**Resolution:** Add `--all-features` or `--features enabled`.

### Issue: Clone trait not implemented

**Possible Cause:** Missing manual Clone implementation or CloneDyn bound.

**Resolution:** Verify explicit `impl Clone for Box<dyn Trait>` is present and CloneDyn bound is specified.

## Test Completion Checklist

- [ ] P1: Example runs with --all-features (output: 1,2,3,1,2,3)
- [ ] P1: Example runs with --no-default-features (no output)
- [ ] P1: Example runs with --features enabled (output: 1,2,3,1,2,3)
- [ ] P2: All feature combinations compile successfully
- [ ] P3: Manual Clone implementation verified
- [ ] P4: Helper functions verified via automated tests
- [ ] P5: Fat pointer handling understood (optional, advanced)
- [ ] P6: CloneDyn trait verified via documentation

## Comparison with clone_dyn Crate

| Aspect | clone_dyn | clone_dyn_types |
|--------|-----------|-----------------|
| Clone Implementation | Automatic via `#[clone_dyn]` macro | Manual via explicit impl |
| Procedural Macro | Required | Not used |
| Use Case | Convenient, less boilerplate | Explicit control, no macros |
| Code Volume | Less (macro handles it) | More (manual impl required) |
| Flexibility | Standard usage | Custom Clone behavior possible |

**When to use clone_dyn_types directly:**
- Prefer explicit control over macro magic
- Need custom Clone behavior
- Avoiding proc macros for build time or policy reasons
- Learning how trait object cloning works under the hood

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
- Example Code: `../../examples/clone_dyn_types_trivial.rs`
- API Documentation: Run `cargo doc --no-deps --open --all-features`
- clone_dyn Crate: For comparison with macro-based approach
