//! Purpose: Provides a hand-written implementation of the `Former` pattern's standalone scalar constructor
//! for a multi-field tuple variant (`Variant(u32, String)`) within an enum that has
//! `#[ standalone_constructors ]` and fields with `#[ arg_for_constructor ]`. This file focuses on
//! demonstrating the manual implementation corresponding to the derived behavior.
//!
//! Coverage:
//! - Rule 4a (#[`standalone_constructors`]): Manually implements the top-level constructor function (`variant`).
//! - Rule 4b (Option 2 Logic): Manually implements the logic for a scalar standalone constructor that takes arguments for all fields in a multi-field tuple variant.
//! - Rule 3f (Tuple + Multi-Field + Default): Implicitly relevant as `Variant` is a multi-field tuple variant.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines the `TestEnum` enum with the `Variant(u32, String)` variant.
//! - Provides a hand-written `variant` function that takes `u32` and `String` as arguments and returns `TestEnum::Variant(u32, String)`. This mimics the behavior expected when `#[ standalone_constructors ]` is on the enum and `#[ arg_for_constructor ]` is on all fields of the variant.
//! - Includes shared test logic from `tuple_multi_standalone_args_only_test.rs`.
//! - The included test calls this manually implemented standalone constructor and asserts that the returned enum instance matches a manually constructed `TestEnum::Variant(value1, value2)`. This verifies the manual implementation of the scalar standalone constructor with field arguments.

// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_args_manual.rs

// Define the enum without the derive macro
#[ derive( Debug, PartialEq ) ]
pub enum TestEnum
{
  Variant( u32, String ),
}

/// Manually implemented standalone constructor for the Variant variant (scalar style with args).
/// This function is at module level to match the `#[ standalone_constructors ]` behavior.
#[ inline( always ) ]
pub fn variant( value1 : u32, value2 : String ) -> TestEnum
{
  TestEnum::Variant( value1, value2 )
}

include!( "tuple_multi_standalone_args_only_test.rs" );