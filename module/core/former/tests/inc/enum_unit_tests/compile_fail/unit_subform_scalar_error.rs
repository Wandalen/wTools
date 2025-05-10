//! Purpose: Tests that applying `#[subform_scalar]` to a unit variant results in a compile-time error.
//!
//! Coverage:
//! - Rule 2a (Unit + `#[subform_scalar]` -> Error): Verifies that the macro correctly reports an error for this invalid attribute combination.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a unit variant `UnitVariant` annotated with `#[subform_scalar]`.
//! - This file is intended to be compiled using `trybuild`. The test is accepted if `trybuild` confirms
//!   that this code fails to compile with a relevant error message, thereby validating the macro's
//!   error reporting for this specific invalid scenario.
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former; // Import derive macro

// === Enum Definition ===

#[ derive( Debug, PartialEq, Clone, Former ) ]
#[ standalone_constructors ]
pub enum TestEnum
{
  #[ subform_scalar ] // This should cause a compile error
  UnitVariant,
}

// No include! or test functions needed for a compile-fail test file.