//! Bug reproducer for negative number parsing with separator conflict.
//!
//! This test demonstrates a critical issue in the `derive_tools_trivial.rs` example
//! where using `-` as a separator in Display format `"{a}-{b}"` creates parsing
//! ambiguity when values are negative.
//!
//! The issue occurs because the separator character `-` is identical to the
//! negative sign for integers, making it impossible for the `FromStr` parser to
//! correctly parse strings like "-5-10" (should be a=-5, b=10).
//!
//! ## Root Cause
//!
//! The Display format `"{a}-{b}"` uses hyphen `-` as the field separator. When field
//! `b` is negative, the output contains consecutive hyphens (e.g., `5--10` for
//! `Struct1{a:5, b:-10}`), creating parsing ambiguity. The `FromStr` parser cannot
//! distinguish between the separator `-` and the negative sign `-` in `-10`.
//!
//! ## Why Not Caught
//!
//! The original example only tested positive values (`1-3`), missing the edge case
//! of negative numbers. No comprehensive corner case testing was performed before
//! the example was written. The test suite lacked round-trip verification for
//! negative values.
//!
//! ## Fix Applied
//!
//! Changed the separator in `derive_tools_trivial.rs` from `-` to `:` in the Display
//! format attribute: `#[display("{a}:{b}")]`. The colon separator does not conflict
//! with any number representation characters, enabling correct round-trip conversion
//! for all i32 values including negatives.
//!
//! ## Prevention
//!
//! When choosing format separators for Display/FromStr derives:
//! - Avoid characters used in number representation: `-` (negative), `.` (decimal), `e` (scientific)
//! - Prefer unambiguous separators: `:`, `,`, `|`, `_`, or whitespace
//! - Always test round-trip conversion with edge cases: MIN, MAX, negative, zero
//! - Include comprehensive corner case testing before finalizing examples
//!
//! ## Pitfall
//!
//! **Never use number representation characters as format separators.** The hyphen `-`
//! is particularly dangerous as it's both a common separator choice and the negative
//! sign. Always verify round-trip conversion: `value → Display → FromStr → value` for
//! all edge cases including negative values, MIN, MAX, and zero.

#![cfg(all(
  feature = "derive_from",
  feature = "derive_display",
  feature = "derive_from_str"
))]

use derive_tools::*;
use core::str::FromStr;

/// Struct demonstrating the separator conflict issue.
/// Using `-` as separator creates ambiguity with negative number sign.
#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{a}-{b}")]
struct Struct1
{
  a: i32,
  b: i32,
}

/// Reproduces bug where `FromStr` fails to parse negative numbers with `-` separator.
///
/// This test documents the expected failure until the example is fixed.
// test_kind: bug_reproducer(example-negative-parsing)
#[test]
fn negative_number_parsing_with_conflicting_separator_fails()
{
  // Display works correctly - generates "-5-10"
  let original = Struct1 { a: -5, b: 10 };
  let displayed = format!("{original}");
  assert_eq!(displayed, "-5-10");

  // FromStr FAILS to parse the displayed value back
  // This is the BUG: round-trip conversion is broken for negative values
  let parsed = Struct1::from_str(&displayed);

  // Current behavior: parsing FAILS with error
  assert!(
    parsed.is_err(),
    "BUG: Parser cannot handle negative numbers with '-' separator. \
     Expected to parse '-5-10' as Struct1{{a: -5, b: 10}}, but parsing fails."
  );

  // This demonstrates the round-trip is broken:
  // Struct1{a: -5, b: 10} → Display → "-5-10" → FromStr → ERROR
}

/// Demonstrates the round-trip failure for both negative values.
// test_kind: bug_reproducer(example-negative-parsing)
#[test]
fn both_negative_values_round_trip_fails()
{
  let original = Struct1 { a: -5, b: -10 };
  let displayed = format!("{original}");
  assert_eq!(displayed, "-5--10"); // Triple minus!

  let parsed = Struct1::from_str(&displayed);

  // Parsing fails due to ambiguous separator
  assert!(
    parsed.is_err(),
    "BUG: Parser cannot handle '-5--10' (triple minus). Round-trip broken."
  );
}

/// Demonstrates successful round-trip with positive values only.
/// This test shows the format works correctly when no negative signs are involved.
#[test]
fn positive_values_round_trip_works()
{
  let original = Struct1 { a: 5, b: 10 };
  let displayed = format!("{original}");
  assert_eq!(displayed, "5-10");

  let parsed = Struct1::from_str(&displayed);
  assert_eq!(
    parsed,
    Ok(original),
    "Positive values should round-trip successfully"
  );
}
