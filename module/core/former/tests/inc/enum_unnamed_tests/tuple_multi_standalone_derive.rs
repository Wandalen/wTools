#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Purpose: Tests the `#[ derive( Former ) ]` macro's generation of a standalone former builder for a multi-field tuple variant when the enum has `#[ standalone_constructors ]` and no fields within the variants have the `#[ arg_for_constructor ]` attribute. This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 4a (#[`standalone_constructors`]): Verifies the generation of the top-level constructor function (`variant`).
//! - Rule 4b (Option 2 Logic): Verifies that when no fields in a multi-field tuple variant have `#[ arg_for_constructor ]`, the standalone constructor returns a former builder for the variant.
//! - Rule 3f (Tuple + Multi-Field + Default): Implicitly relevant as `Variant` is a multi-field tuple variant.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a multi-field tuple variant `Variant(u32, String)`.
//! - Applies `#[ derive( Former ) ]` and `#[ standalone_constructors ]` to the enum.
//! - No `#[ arg_for_constructor ]` attributes are applied to fields.
//! - Includes shared test logic from `tuple_multi_standalone_only_test.rs`.
//! - The included test calls the derived standalone constructor function `variant()`, uses the returned former builders' setters (`._0()`, `._1()`), and calls `.form()`.
//! - Asserts that the resulting enum instance matches a manually constructed `TestEnum::Variant(value1, value2)`. This verifies that the standalone constructor is generated correctly as a former builder when no field arguments are specified.

use former::Former;

#[ derive( Former, Debug, PartialEq ) ]
#[ former( standalone_constructors ) ]
pub enum TestEnum
{
  Variant( u32, String ),
}

include!( "tuple_multi_standalone_only_test.rs" );