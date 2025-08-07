//! Purpose: This is a compile-fail test designed to verify that applying the `#[subform_scalar]` attribute
//! to a multi-field tuple variant results in a compilation error.
//!
//! Coverage:
//! - Rule 2f (Tuple + Multi-Field + `#[subform_scalar]` -> Error): Verifies that the macro correctly reports an error for this invalid attribute usage.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a multi-field tuple variant `VariantMulti(i32, bool)`.
//! - Applies `#[derive(Former)]` to the enum.
//! - Applies `#[subform_scalar]` to the `VariantMulti` variant, which is an invalid combination according to Rule 2f.
//! - This file is intended for use with `trybuild`. The test is accepted if `trybuild` confirms that this code fails to compile with an appropriate error message, thereby validating the macro's error handling for this specific invalid scenario.

// File: module/core/former/tests/inc/former_enum_tests/compile_fail/tuple_multi_subform_scalar_error.rs

// This file is a compile-fail test for the scenario where #[subform_scalar] is
// applied to a multi-field tuple variant (Matrix TN.3), which should result in a compile error.

use former::Former;

#[ derive( Former ) ]
#[ allow( dead_code ) ]
enum TestEnum
{
  #[ subform_scalar ] // Should cause an error
  VariantMulti( i32, bool ),
}

fn main()
{
  // Attempting to use the generated code should also fail compilation
  // let _ = TestEnum::variant_multi(); // This line is commented out as the derive itself should fail
}