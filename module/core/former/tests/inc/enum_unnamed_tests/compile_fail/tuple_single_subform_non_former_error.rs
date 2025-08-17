//! Purpose: This is a compile-fail test designed to verify that applying the `#[ subform_scalar ]` attribute
//! to a single-field tuple variant whose inner type does *not* derive `Former` results in a compilation error.
//!
//! Coverage:
//! - Rule 2d (Tuple + Single-Field + `#[ subform_scalar ]` -> InnerFormer): Verifies that the macro correctly reports an error when the requirement for the inner type to derive `Former` is not met in conjunction with `#[ subform_scalar ]`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a struct `NonFormerInner` that does *not* derive `Former`.
//! - Defines an enum `TestEnum` with a single-field tuple variant `VariantSingle(NonFormerInner)`.
//! - Applies `#[ derive( Former ) ]` to the enum.
//! - Applies `#[ subform_scalar ]` to the `VariantSingle` variant, which is an invalid combination because `NonFormerInner` does not derive `Former`.
//! - This file is intended for use with `trybuild`. The test is accepted if `trybuild` confirms that this code fails to compile with an appropriate error message, thereby validating the macro's error handling for this specific invalid scenario.

// File: module/core/former/tests/inc/former_enum_tests/compile_fail/tuple_single_subform_non_former_error.rs

// This file is a compile-fail test for the scenario where #[ subform_scalar ] is
// applied to a single-field tuple variant where the inner type does NOT derive Former
// (Matrix T1.5), which should result in a compile error.

use former::Former;

// This struct does NOT derive Former
#[ allow( dead_code ) ]
#[ derive( Debug, PartialEq, Clone ) ]
struct NonFormerInner
{
  value: i32,
}

#[ derive( Former ) ]
#[ allow( dead_code ) ]
enum TestEnum
{
  #[ subform_scalar ] // Should cause an error because NonFormerInner does not derive Former
  VariantSingle( NonFormerInner ),
}

fn main()
{
  // Attempting to use the generated code should also fail compilation
  // let _ = TestEnum::variant_single(); // This line is commented out as the derive itself should fail
}