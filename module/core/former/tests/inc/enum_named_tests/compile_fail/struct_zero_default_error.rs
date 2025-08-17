//! Purpose: This is a compile-fail test designed to verify that a zero-field named (struct-like)
//! variant without the `#[ scalar ]` attribute results in a compilation error.
//!
//! Coverage:
//! - Rule 3c (Struct + Zero-Field + Default -> Error): Verifies that the macro correctly reports an error when `#[ scalar ]` is missing for a zero-field named variant.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithNamedFields` with a zero-field named variant `VariantZeroDefault {}`.
//! - Applies `#[ derive( Former ) ]` to the enum.
//! - No `#[ scalar ]` attribute is applied to `VariantZeroDefault`, which is an invalid state according to Rule 3c.
//! - This file is intended for use with `trybuild`. The test is accepted if `trybuild` confirms that this code fails to compile with an appropriate error message, thereby validating the macro's error handling for this specific invalid scenario.

#[ derive( Debug, PartialEq, former::Former ) ]
pub enum EnumWithNamedFields
{
  // S0.1: Zero-field struct variant with Default behavior (expected compile error)
  VariantZeroDefault {},
}

fn main() {} // Required for trybuild