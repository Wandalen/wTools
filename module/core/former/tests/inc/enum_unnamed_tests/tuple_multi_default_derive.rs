//! Purpose: Tests the `#[derive(Former)]` macro's generation of an implicit variant former for a multi-field tuple variant when no specific variant attribute (`#[scalar]` or `#[subform_scalar]`) is applied (default behavior). This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 3f (Tuple + Multi-Field + Default): Verifies that for a multi-field tuple variant without specific attributes, the derived constructor returns an implicit variant former with setters like ._`0()` and ._`1()`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a multi-field tuple variant `Variant(u32, String)`.
//! - Applies `#[derive(Former)]` to the enum.
//! - No variant attributes are applied to `Variant`.
//! - Includes shared test logic from `tuple_multi_default_only_test.rs`.
//! - The included test calls the derived static method `TestEnum::variant()` which returns a former, uses setters ._`0()` and ._`1()`, and calls .`form()`. This verifies that the default behavior for a multi-field tuple variant is an implicit variant former.

use former::Former;

#[ derive( Former, Debug, PartialEq ) ]
pub enum TestEnum
{
  Variant( u32, String ),
}

include!( "tuple_multi_default_only_test.rs" );