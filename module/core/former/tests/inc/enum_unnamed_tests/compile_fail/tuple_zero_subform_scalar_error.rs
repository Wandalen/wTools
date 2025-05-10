// File: module/core/former/tests/inc/former_enum_tests/compile_fail/tuple_zero_subform_scalar_error.rs

// This file is a compile-fail test for the scenario where #[subform_scalar] is
// applied to a zero-field tuple variant (Matrix T0.5), which should result in a compile error.

use former::Former;

#[ derive( Former ) ]
#[ allow( dead_code ) ]
enum TestEnum
{
  #[ subform_scalar ] // Should cause an error
  VariantZero(),
}

fn main()
{
  // Attempting to use the generated code should also fail compilation
  let _ = TestEnum::variant_zero();
}