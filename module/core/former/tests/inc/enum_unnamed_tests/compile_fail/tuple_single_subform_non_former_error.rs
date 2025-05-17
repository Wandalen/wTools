// File: module/core/former/tests/inc/former_enum_tests/compile_fail/tuple_single_subform_non_former_error.rs

// This file is a compile-fail test for the scenario where #[subform_scalar] is
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