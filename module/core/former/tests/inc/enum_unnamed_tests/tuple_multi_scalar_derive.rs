//! Purpose: Tests the `#[ derive( Former ) ]` macro's generation of a scalar constructor for a multi-field tuple variant when it is explicitly marked with the `#[ scalar ]` attribute. This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 1f (Tuple + Multi-Field + `#[ scalar ]`): Verifies that for a multi-field tuple variant with the `#[ scalar ]` attribute, the derived constructor is scalar, taking arguments for each field and returning the enum instance.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a multi-field tuple variant `Variant(u32, String)`.
//! - Applies `#[ derive( Former ) ]` to the enum.
//! - Applies `#[ scalar ]` to the `Variant` variant.
//! - Includes shared test logic from `tuple_multi_scalar_only_test.rs`.
//! - The included test calls the derived static method `TestEnum::variant(value1, value2)` and asserts that the returned enum instance matches a manually constructed `TestEnum::Variant(value1, value2)`. This verifies that the `#[ scalar ]` attribute forces scalar behavior for a multi-field tuple variant.

use former::Former;

#[ derive( Former, Debug, PartialEq ) ]
pub enum TestEnum
{
  #[ scalar ]
  Variant( u32, String ),
}

include!( "tuple_multi_scalar_only_test.rs" );