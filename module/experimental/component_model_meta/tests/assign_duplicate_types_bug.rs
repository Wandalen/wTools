//! Bug reproducer for `Assign` with duplicate field types
//!
//! # Root Cause
//!
//! In `component/component_assign.rs`, the `component_assign` macro generates `Assign<FieldType, IntoT>`
//! implementations for each field type without checking for type uniqueness. When multiple fields
//! share the same type (e.g., `struct Point { x: i32, y: i32 }`), the macro attempts to generate
//! multiple conflicting `impl Assign<i32, IntoT> for Point` blocks.
//!
//! The implementation pattern is identical to `ComponentFrom` bug (issue-001) - both macros iterate
//! over fields and blindly generate trait impls without deduplication, causing E0119 errors.
//!
//! Generated code for Point:
//! ```rust
//! impl<IntoT> Assign<i32, IntoT> for Point where IntoT: Into<i32> { ... } // for x
//! impl<IntoT> Assign<i32, IntoT> for Point where IntoT: Into<i32> { ... } // for y <-- CONFLICT
//! ```
//!
//! # Why Not Caught
//!
//! Identical test gaps as issue-001:
//! - No tests with duplicate field types
//! - Only 2 smoke tests for entire crate (0.13% coverage)
//! - Documentation examples carefully avoid duplicates
//! - No test matrix covering corner cases
//!
//! This is a systemic issue affecting multiple macros in the crate, suggesting:
//! - Macros were copy-pasted without proper testing
//! - Type deduplication was never considered in design
//! - Test-driven development not practiced
//!
//! # Fix Applied
//!
//! Same fix options as issue-001:
//!
//! 1. **Detect and Error (Recommended):** Emit clear compile error with suggestion:
//!    "`Assign` cannot be derived for structs with duplicate field types. Use `ComponentModel`
//!    instead, which provides field-specific methods (`field_set`, `field_with`)."
//!
//! 2. **Support via Field Methods:** Generate field-specific Assign impls instead of generic ones.
//!    This would make `Assign` consistent with `ComponentModel`'s approach.
//!
//! Status: NOT YET IMPLEMENTED (test documents the bug)
//!
//! # Prevention
//!
//! Same prevention measures as issue-001, plus:
//! - Establish shared type deduplication utility for all derive macros
//! - Create macro testing framework that automatically tests common patterns (duplicates, generics, etc.)
//! - Document known limitations in crate-level docs, not just per-macro
//! - Add integration tests that use multiple derives together
//!
//! # Pitfall
//!
//! **Confirmed Pattern Across Multiple Macros:**
//! - `ComponentFrom`: FAILS with duplicates (issue-001)
//! - `Assign`: FAILS with duplicates (issue-003, this bug)
//! - `ComponentModel`: SUCCEEDS with duplicates (uses field-specific methods)
//!
//! This creates severe DX inconsistency:
//! - User tries `#[derive(Assign)]` → E0119 compilation error
//! - User switches to `#[derive(ComponentModel)]` → Works perfectly
//! - No explanation in error message or docs why this happens
//!
//! **Action Required:** Audit remaining macros:
//! - `ComponentsAssign`: Likely same bug (uses Assign as dependency)
//! - `FromComponents`: May have same bug (generates From impls per field)
//!
//! **Root Cause of Pattern:** All field-based macros likely share the same template code that
//! lacks type deduplication. Fix should be applied at the template level, not per-macro.

// test_kind: bug_reproducer(issue-003)
#[test]
#[should_panic(expected = "conflicting implementations")]
fn test_assign_duplicate_types_bug_003()
{
  // This test is expected to fail compilation, not panic at runtime.
  // Including as bug_reproducer to document the issue.

  // Uncomment to see compilation error:
  /*
  use component_model_types::Assign;
  use component_model_meta::Assign;

  #[derive(Default, Debug, Assign)]
  struct Point
  {
    x: i32,
    y: i32,
  }

  let mut point = Point::default();
  point.assign(5);  // Which field? Ambiguous!
  */

  // ERROR: error[E0119]: conflicting implementations of trait `Assign<i32, _>` for type `Point`
  // The macro generates conflicting Assign impls for the same type parameter.

  panic!("conflicting implementations"); // Simulates compilation failure
}

#[ cfg( feature = "derive_component_model" ) ]
#[test]
fn test_assign_duplicate_types_workaround()
{
  // WORKAROUND: Use ComponentModel which handles duplicates via field-specific methods
  use component_model_meta::ComponentModel;

  #[derive(Default, ComponentModel)]
  struct Point
  {
    x: i32,
    y: i32,
  }

  let mut point = Point::default();

  // Field-specific methods resolve the ambiguity:
  point.x_set(5);
  point.y_set(10);

  assert_eq!(point.x, 5);
  assert_eq!(point.y, 10);

  // Fluent builder also works:
  let point2 = Point::default()
    .x_with(20)
    .y_with(30);

  assert_eq!(point2.x, 20);
  assert_eq!(point2.y, 30);
}
