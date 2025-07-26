# Rust Macro Development Rulebook

This rulebook provides comprehensive guidelines for developing Rust procedural macros based on the codegen_roo system prompt. It emphasizes strict adherence to Test-Driven Development (TDD), comprehensive testing strategies, and rigorous code quality standards.

## Table of Contents

1. [Core Principles](#core-principles)
2. [Test-Driven Development Requirements](#test-driven-development-requirements)
3. [Testing Strategy and Rules](#testing-strategy-and-rules)
4. [Macro-Specific Guidelines](#macro-specific-guidelines)
5. [Code Organization and Structure](#code-organization-and-structure)
6. [Development Workflow](#development-workflow)
7. [Quality Assurance and Verification](#quality-assurance-and-verification)
8. [Problem-Solving Heuristics](#problem-solving-heuristics)

## Core Principles

### Strict Test-Driven Development (TDD)
- **All development must be guided by tests**
- Never write production code without a corresponding automated test planned and implemented in the same increment
- Blind development without tests is strictly forbidden

### Focused, Tracked Debugging
- All test failures must be tracked individually in the plan's `### Tests` section
- Only one failing test may be addressed at a time
- If a test cannot be fixed with a simple, one-shot attempt, create a dedicated `Focused Debugging Increment`

### Context-Rich Planning
- Assume the Executor has no prior knowledge beyond what is explicitly provided
- All plans must be context-rich and self-contained
- Include relevant code snippets, dependency API signatures, and detailed explanations

### Prioritize Reuse and Minimal Change
- Look for opportunities to reuse existing code, patterns, components, and working pieces
- Do not reinvent solutions if suitable ones already exist
- Aim for the smallest possible change that meets requirements

## Test-Driven Development Requirements

### Mandatory Test Coverage
**All new or modified production code MUST be accompanied by automated tests within the same increment.**

```rust
// ❌ Bad: Adding a function without any corresponding test
// Increment Plan:
// 1. Add `fn calculate_total(price: f32, quantity: u32)` to `src/billing.rs`.
// 2. Refactor the main loop to use this new function.
// (No test step is planned for the new function)
```

```rust
// ✅ Good: Planning a test alongside the new function
// Increment Plan:
// 1. Add a new test file `tests/billing_tests.rs`.
// 2. In `billing_tests.rs`, write a test `test_calculate_total_with_zero_quantity` that asserts the result is 0. Expect it to fail.
// 3. Implement the `fn calculate_total` in `src/billing.rs` to make the test pass.
// 4. Add more test cases for `calculate_total` covering edge cases.
```

### Test Location Requirements
**All automated tests MUST be placed within the canonical `tests` directory at the crate root.**

```rust
// ❌ Bad: Unit tests inside src/lib.rs
// src/lib.rs
pub fn add( a: i32, b: i32 ) -> i32 { a + b }
#[cfg(test)]
mod tests
{
  use super::*;
  #[test]
  fn it_works()
  {
    assert_eq!( add( 2, 2 ), 4 );
  }
}
```

```rust
// ✅ Good: All tests in tests directory
// tests/my_feature_tests.rs
#[ test ]
fn test_addition()
{
  assert_eq!( my_crate::add( 2, 2 ), 4 );
}
```

## Testing Strategy and Rules

### One Aspect Per Test
Each test must verify only a single, specific aspect of behavior.

```rust
// ❌ Bad: Single test checking multiple aspects
#[ test ]
fn test_user_lifecycle()
{
  let mut user = User::new( "Alex" );
  assert_eq!( user.name(), "Alex" ); // Aspect 1: Name on creation
  user.set_name( "Bob" );
  assert_eq!( user.name(), "Bob" ); // Aspect 2: Name after update
  assert!( user.is_active() ); // Aspect 3: Default status
}
```

```rust
// ✅ Good: Decoupled tests with single responsibility
#[ test ]
fn test_user_creation_sets_name()
{
  let user = User::new( "Alex" );
  assert_eq!( user.name(), "Alex" );
}

#[ test ]
fn test_user_set_name_updates_name()
{
  let mut user = User::new( "Alex" );
  user.set_name( "Bob" );
  assert_eq!( user.name(), "Bob" );
}

#[ test ]
fn test_user_is_active_by_default()
{
  let user = User::new( "Alex" );
  assert!( user.is_active() );
}
```

### Explicit Parameters to Avoid Fragility
All functional tests must explicitly provide values for every parameter to prevent fragile tests.

```rust
// ❌ Bad: Fragile test relying on default parameter
#[ test ]
fn test_create_user_sets_name()
{
  // This test implicitly relies on `is_admin` being `false`.
  // If the default changes to `true`, this test will fail unexpectedly.
  let user = create_user( "Alex" );
  assert_eq!( user.name(), "Alex" );
  assert!( !user.is_admin() ); // This assertion breaks if default changes
}
```

```rust
// ✅ Good: Robust test with explicit parameters
#[ test ]
fn test_create_user_as_non_admin()
{
  // This test is robust. It explicitly states its assumptions.
  let user = create_user( "Alex", false ); // `is_admin` is explicit
  assert_eq!( user.name(), "Alex" );
  assert!( !user.is_admin() );
}
```

### Default Value Equivalence Testing
Create dedicated tests to verify that default parameter behavior works correctly.

```rust
// ✅ Good: Dedicated test for default value equivalence
#[ test ]
fn test_default_is_admin_is_equivalent_to_explicit_false()
{
  let user_default = create_user( "Default" );
  let user_explicit = create_user( "Explicit", false );
  
  // Verification: The resulting objects should be identical
  assert_eq!( user_default, user_explicit );
}
```

### Test Matrix Planning
When writing tests, create a Test Matrix to ensure comprehensive coverage.

```markdown
#### Test Matrix for `create_user(name: &str, is_admin: bool = false)`

**Test Factors:**
- `name`: The value of the user's name
- `is_admin`: The explicit value of the admin flag
- Parameter Style: Whether `is_admin` is explicit or uses the default

**Test Combinations:**

| ID   | Aspect Tested | `name` | `is_admin` | Parameter Style | Expected Behavior |
|------|---------------|--------|------------|-----------------|-------------------|
| T1.1 | Name setting  | "Alex" | `false`    | Explicit        | `user.name()` is "Alex" |
| T1.2 | Admin status  | "Alex" | `true`     | Explicit        | `user.is_admin()` is `true` |
| T1.3 | Default Equiv.| "User" | `false`    | Default vs Exp. | `create_user("User")` == `create_user("User", false)` |
```

### Test Documentation Requirements
**Every test file MUST begin with a file-level doc comment containing the relevant Test Matrix.**

```rust
// tests/my_feature_tests.rs

//! ## Test Matrix for My Feature
//!
//! | ID   | Input      | Expected Output |
//! |------|------------|-----------------|
//! | T1.1 | `Some(5)`  | `Ok(10)`        |
//! | T1.2 | `None`     | `Err(NotFound)` |

use my_crate::my_feature_func;

/// Tests that a valid input is processed correctly.
/// Test Combination: T1.1
#[ test ]
fn test_valid_input()
{
  assert_eq!( my_feature_func( Some( 5 ) ), Ok( 10 ) );
}

/// Tests that a missing input returns the expected error.
/// Test Combination: T1.2
#[ test ]
fn test_missing_input()
{
  assert_eq!( my_feature_func( None ), Err( "NotFound".to_string() ) );
}
```

### Test Kind Markers
Mark special tests to protect them from removal.

```rust
// test_kind: bug_reproducer(issue-123)
#[ test ]
fn test_specific_panic_on_empty_input()
{
  // ... test logic ...
}

// test_kind: mre
#[ test ]
fn test_minimal_case_for_feature_x()
{
  // ... test logic ...
}
```

## Macro-Specific Guidelines

### Dependencies: Prefer `macro_tools`
For procedural macro development, always prefer using the `macro_tools` crate over direct dependencies.

```toml
# ❌ Bad: Direct dependencies
[dependencies]
syn = { version = "1.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"
```

```toml
# ✅ Good: Using macro_tools
[dependencies]
macro_tools = "0.5"
```

```rust
// ✅ Good: Code usage
use macro_tools::
{
  proc_macro2, // Re-exported
  quote,       // Re-exported
  syn,         // Re-exported
  // ... and useful abstractions from macro_tools
};
```

### Mandatory Debug Attribute
All procedural macros MUST implement an item attribute named `debug`.

```rust
// When #[debug] is used, the macro should print:
// = context
//   derive : Deref
//   item : IsTransparentSimple
//   field_type : Type::Path { ... }
//   field_name : None
//
// = original
//   pub struct IsTransparentSimple(bool);
//
// = generated
//   #[ automatically_derived ]
//   impl  core::ops::Deref for IsTransparentSimple
//   {
//     type Target = bool;
//     #[ inline ]
//     fn deref( &self ) -> &bool
//     {
//       & self.0
//     }
//   }
```

### Path Resolution in Generated Code
Generated code must use paths that correctly resolve within the target crate.

```rust
// ✅ Good: Using crate::... for standard structure
quote!
{
  impl MyTrait for #struct_ident
  {
    type Assoc = crate::types::MyType;
    fn method() -> crate::definitions::MyDef { /* ... */ }
  }
}
```

```rust
// ❌ Bad: Absolute paths break with crate aliasing
quote!
{
  impl MyTrait for #struct_ident
  {
    type Assoc = ::crate1::types::MyType; // Breaks with aliasing
    fn method() -> ::crate1::definitions::MyDef { /* ... */ }
  }
}
```

## Code Organization and Structure

### Module Declaration Order
Always add module declarations before creating file content.

```text
// ✅ Good: Declaring module first
// Plan Step 3: Add `mod my_feature;` to `src/lib.rs`. // Declare module first
// Plan Step 4: Create file `src/my_feature.rs`.
// Plan Step 5: Add `pub fn feature_func() {}` to `src/my_feature.rs`.
```

### File Size Guidelines
- Strive to keep files under approximately 1000 lines
- For new features, proactively design structures that avoid large files
- Only split existing large files when explicitly requested

### Test Propagation Headers
Use standard headers for test file inclusion.

```rust
// Root test file: tests/tests.rs
#![ allow( unused_imports ) ]
use my_crate as the_module;

#[ path = "./inc/feature_a.rs" ]
mod feature_a;
```

```rust
// Included test file: tests/inc/feature_a.rs
use super::*; // Correctly propagates `the_module` and other items

#[ test ]
fn test_something()
{
  let _ = the_module::some_item();
}
```

## Development Workflow

### Increment-Based Development
1. **Initial Task Planning**: Create high-level task structure
2. **Detailed Increment Planning**: Refine specific increment details (minimum 3 iterations)
3. **Test Quality Evaluation**: Verify test coverage and adherence to rules
4. **Step-by-Step Implementation**: Follow the detailed plan meticulously
5. **Verification**: Run all checks and tests
6. **Commit**: Only after all verification passes

### Critical Log Analysis Process
When tests fail:

1. Identify the **first** failing test ID
2. Track status in the `### Tests` section:
   - `Failing (New)` → `Failing (Attempt 1)` → `Failing (Stuck)`
   - `Fixed (Monitored)` → `Failing (Regression)`
3. For `Failing (Stuck)`, create a Focused Debugging Increment
4. Address only **one** test at a time

### Focused Debugging Increment
For stuck tests, create a dedicated increment with:

- **Goal**: "Diagnose and fix the `Failing (Stuck)` test: `[Test ID]`"
- **Mandatory steps**:
  - Apply Problem Decomposition
  - Isolate the test case
  - Add targeted debug logging
  - Review related code changes
  - Formulate and test a hypothesis

## Quality Assurance and Verification

### Output Cleanliness Check
Ensure no unintended debug output from procedural macros:

1. Run `cargo clean`
2. Run build command
3. Analyze output for debug prints

### Crate Conformance Check
After each increment:

1. Run `timeout 90 cargo build`
2. Run `timeout 90 cargo test`
3. Run `cargo clippy` (without auto-fix flags)
4. Analyze all outputs for errors/warnings

### Test Count Monitoring
- Establish baseline test count at task start
- Monitor for unexplained decreases during conformance checks
- Investigate any discrepancies immediately

### Warning-Free Requirements
All test runs must complete without compiler warnings. Warnings must be treated as errors and fixed.

## Problem-Solving Heuristics

### Problem Reduction
1. Simplify the problem to its core
2. Solve the simplified version
3. Generalize the solution back to the original problem

### Problem Decomposition
1. Break large problems into smaller, independent sub-problems
2. Solve each sub-problem individually
3. Combine solutions systematically

### Isolate the Variable
1. Change only one factor at a time
2. Test the impact of each change
3. Build understanding incrementally

## Best Practices Summary

1. **Always start with tests** - Write failing tests before implementing features
2. **One test, one aspect** - Keep tests focused and specific
3. **Explicit parameters** - Avoid relying on defaults in functional tests
4. **Document everything** - Include Test Matrices and clear test documentation
5. **Use macro_tools** - Prefer it over direct syn/quote dependencies
6. **Implement debug attributes** - Mandatory for all procedural macros
7. **Plan thoroughly** - Use detailed, context-rich planning with multiple iterations
8. **Track failures** - Maintain detailed status of all test failures
9. **Verify comprehensively** - Run all checks after each increment
10. **Maintain quality** - Zero warnings, clean builds, complete test coverage

This rulebook serves as a comprehensive guide for developing high-quality Rust procedural macros with rigorous testing and quality assurance practices.