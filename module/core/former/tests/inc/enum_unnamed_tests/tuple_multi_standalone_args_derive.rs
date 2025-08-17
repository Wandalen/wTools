//! Purpose: Tests the `#[ derive( Former ) ]` macro's generation of a standalone scalar constructor
//! for a multi-field tuple variant when the enum has `#[ standalone_constructors ]` and all fields
//! within the variant have `#[ arg_for_constructor ]`. This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 4a (#[`standalone_constructors`]): Verifies the generation of the top-level constructor function (`variant`).
//! - Rule 4b (Option 2 Logic): Verifies that when all fields in a multi-field tuple variant have `#[ arg_for_constructor ]`, the standalone constructor takes arguments for those fields and returns the final enum instance (scalar style).
//! - Rule 3f (Tuple + Multi-Field + Default): Implicitly relevant as `Variant` is a multi-field tuple variant.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a multi-field tuple variant `Variant(u32, String)`.
//! - Applies `#[ derive( Former ) ]` and `#[ standalone_constructors ]` to the enum.
//! - Applies `#[ arg_for_constructor ]` to both fields within the `Variant` variant.
//! - Includes shared test logic from `tuple_multi_standalone_args_only_test.rs`.
//! - The included test calls the derived standalone constructor function `variant(value1, value2)` and asserts that the returned enum instance matches a manually constructed `TestEnum::Variant(value1, value2)`. This verifies that the standalone constructor is generated correctly as a scalar function when all fields have `#[ arg_for_constructor ]`.

use former::Former;

#[ derive( Former, Debug, PartialEq ) ]
#[ former( standalone_constructors ) ]
pub enum TestEnum
{
  Variant( #[ arg_for_constructor ] u32, #[ arg_for_constructor ] String ),
}

include!( "tuple_multi_standalone_args_only_test.rs" );