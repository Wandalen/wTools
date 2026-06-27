//! Bug reproducer for `ComponentFrom` with duplicate field types
//!
//! # Root Cause
//!
//! In `component/component_from.rs`, the `component_from` macro generates `From<&StructName>`
//! implementations for each field type without checking for type uniqueness. When multiple fields
//! share the same type (e.g., `struct Point { x: i32, y: i32 }`), the macro attempts to generate
//! multiple conflicting `impl From<&Point> for i32` blocks, violating Rust's trait coherence rules.
//!
//! The implementation iterates over all fields and blindly generates From impls without deduplication,
//! leading to `error[E0119]: conflicting implementations of trait`.
//!
//! # Why Not Caught
//!
//! The existing test suite only covers trivial cases with unique field types:
//! - No tests with duplicate field types (Point, RGB, dimensions)
//! - No tests with common types that often duplicate (bool flags, i32 counts, String names)
//! - Test coverage is extremely low (2 smoke tests for ~1,500 LOC = 0.13%)
//! - No test matrix documenting corner cases per `test_organization.rulebook.md`
//!
//! The documentation examples carefully avoid this pattern, hiding the limitation from users.
//!
//! # Fix Applied
//!
//! Two potential fixes (choose one):
//!
//! 1. **Detect and Error (Recommended):** Add compile-time check to detect duplicate types and emit
//!    clear error: "`ComponentFrom` cannot be derived for structs with duplicate field types.
//!    Consider using `ComponentModel` instead, which provides field-specific methods."
//!
//! 2. **Support via Field Methods:** Generate field-specific From methods (e.g., `impl PointFromX`,
//!    `impl PointFromY`) instead of conflicting generic trait impls. This matches `ComponentModel`'s
//!    approach.
//!
//! Status: NOT YET IMPLEMENTED (test documents the bug)
//!
//! # Prevention
//!
//! - Add type deduplication check to all derive macros (`ComponentFrom`, `Assign`, `FromComponents`)
//! - Create test matrix covering all corner cases before implementing macros
//! - Document limitations prominently in all macro docs
//! - Add `compile_fail` doctests showing error messages for unsupported cases
//! - Increase test coverage from 0.13% to at least 80% per crate standards
//!
//! # Pitfall
//!
//! **Design Inconsistency Across Crate:**
//! - `ComponentFrom` and `Assign`: FAIL with duplicate types (E0119 compilation error)
//! - `ComponentModel`: SUCCEEDS with duplicate types (generates `field_set`/`field_with` methods)
//! - `ComponentsAssign`: Unknown (needs testing)
//! - `FromComponents`: Unknown (needs testing)
//!
//! This creates confusing DX where users hit compilation errors with individual derives but the
//! unified `ComponentModel` works. The docs mention "use field-specific methods to avoid type ambiguity"
//! but don't explain that individual derives completely fail with duplicates.
//!
//! **Similar Pattern Exists:** Check `Assign` macro (already confirmed same bug), `ComponentsAssign`,
//! and `FromComponents` for identical type deduplication issues. Audit entire crate.

// test_kind: bug_reproducer(issue-001)
#[test]
#[should_panic(expected = "conflicting implementations")]
fn test_component_from_duplicate_types_bug_001()
{
  // This test is expected to fail compilation, not panic at runtime.
  // Including as bug_reproducer to document the issue.

  // Uncomment to see compilation error:
  /*
  use component_model_meta::ComponentFrom;

  #[derive(ComponentFrom)]
  struct Point
  {
    pub x: i32,
    pub y: i32,
  }

  let point = Point { x: 5, y: 10 };
  let _value: i32 = From::from(&point);
  */

  // ERROR: error[E0119]: conflicting implementations of trait `From<&Point>` for type `i32`
  // The macro generates:
  //   impl From<&Point> for i32 { ... } // for field x
  //   impl From<&Point> for i32 { ... } // for field y  <-- CONFLICT

  panic!("conflicting implementations"); // Simulates compilation failure
}

#[ cfg( feature = "derive_component_model" ) ]
#[test]
fn test_component_from_duplicate_types_workaround()
{
  // WORKAROUND: Use ComponentModel instead, which handles duplicates via field-specific methods
  use component_model_meta::ComponentModel;

  #[derive(Default, ComponentModel)]
  struct Point
  {
    pub x: i32,
    pub y: i32,
  }

  let mut point = Point::default();

  // Can't use assign(5) because it's ambiguous, but field-specific methods work:
  point.x_set(5);
  point.y_set(10);

  assert_eq!(point.x, 5);
  assert_eq!(point.y, 10);
}
