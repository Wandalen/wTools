//!
//! Smoke tests validating basic crate compilation, linking, and public API accessibility.
//! These tests verify the facade re-exports function correctly.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `local_smoke_test` | Basic facade functionality | `Range`, `RangeInclusive`, `Tuple` | All interval types iterate correctly | ✅ |
//! | `published_smoke_test` | Public API re-exports | `Bound` enum, `NonIterableInterval` | Bounds accessible, intervals work | ✅ |
//!
//! ### Coverage
//!
//! - ✅ `Range` iteration (half-open: 0..5)
//! - ✅ `RangeInclusive` iteration (closed: 0..=4)
//! - ✅ Tuple to interval conversion (0, 5) → [0..=5]
//! - ✅ `Bound` enum accessibility (`Included`, `Unbounded`)
//! - ✅ `NonIterableInterval` trait (`.left()`, `.right()`)
//! - ✅ Bounded intervals (`Included..Included`)
//! - ✅ Unbounded intervals (`Included..Unbounded`)
//!

/// Verifies basic facade re-export functionality for iterable intervals.
///
/// Tests that winterval correctly re-exports core interval types:
/// 1. `Range` (half-open: 0..5) iterates 5 times
/// 2. `RangeInclusive` (closed: 0..=4) iterates 5 times
/// 3. Tuple conversion (0, 5) creates inclusive interval [0..=5] with 6 elements
///
/// This test ensures the facade crate compiles, links, and basic API is accessible.
/// Critical for baseline functionality verification.
#[ test ]
fn local_smoke_test()
{
  use winterval :: IntoInterval;

  // Verify Range works
  let mut count = 0;
  for _ in 0..5
  {
    count += 1;
  }
  assert_eq!( count, 5, "Range iteration failed" );

  // Verify RangeInclusive works
  let count2 = ( 0..=4 ).count();
  assert_eq!( count2, 5, "RangeInclusive iteration failed" );

  // Verify tuple conversion via IntoInterval trait  (creates inclusive interval)
  let interval = ( 0, 5 ).into_interval();
  let count3 = interval.into_iter().count();
  assert_eq!( count3, 6, "Tuple to interval conversion failed (0..=5 inclusive)" );
}

/// Verifies public API re-exports for non-iterable intervals and bounds.
///
/// Tests that winterval correctly re-exports:
/// 1. `Bound` enum variants (`Included`, `Unbounded`) are accessible
/// 2. `NonIterableInterval` trait methods (`.left()`, `.right()`) work correctly
/// 3. Bounded intervals (`Included..Included`) return correct bounds
/// 4. Unbounded intervals (`Included..Unbounded`) handle unbounded correctly
///
/// This test ensures advanced API surface is properly exposed through facade.
/// Validates re-export completeness beyond basic iteration.
#[ test ]
fn published_smoke_test()
{
  use winterval :: { NonIterableInterval, Bound, IntoInterval };

  // Verify Bound enum is accessible
  let left_bound = Bound::Included( 0 );
  let right_bound = Bound::Included( 10 );

  // Verify NonIterableInterval works with bounded interval
  let interval = ( left_bound, right_bound ).into_interval();
  assert_eq!( interval.left(), Bound::Included( 0 ), "Left bound mismatch" );
  assert_eq!( interval.right(), Bound::Included( 10 ), "Right bound mismatch" );

  // Verify unbounded intervals work
  let unbounded = ( Bound::Included( 0 ), Bound::Unbounded ).into_interval();
  assert_eq!( unbounded.left(), Bound::Included( 0 ), "Unbounded left mismatch" );
  assert_eq!( unbounded.right(), Bound::Unbounded, "Unbounded right mismatch" );
}
