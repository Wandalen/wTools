//! Purpose: This is a compile-fail test designed to verify that applying the `#[ subform_scalar ]` attribute
//! to a zero-field named (struct-like) variant results in a compilation error.
//!
//! Coverage:
//! - Rule 2c (Struct + Zero-Field + `#[ subform_scalar ]` -> Error): Verifies that the macro correctly reports an error for this invalid attribute usage.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `EnumWithNamedFields` with a zero-field named variant `VariantZeroSubformScalar {}`.
//! - Applies `#[ derive( Former ) ]` to the enum.
//! - Applies `#[ subform_scalar ]` to the `VariantZeroSubformScalar` variant, which is an invalid combination according to Rule 2c.
//! - This file is intended for use with `trybuild`. The test is accepted if `trybuild` confirms that this code fails to compile with an appropriate error message, thereby validating the macro's error handling for this specific invalid scenario.

#[ derive( Debug, PartialEq, former::Former ) ]
pub enum EnumWithNamedFields
{
  // S0.5: Zero-field struct variant with #[ subform_scalar ] (expected compile error)
  #[ subform_scalar ]
  VariantZeroSubformScalar {},
}

fn main() {} // Required for trybuild