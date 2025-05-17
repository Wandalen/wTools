// mod unit_subform_scalar_error;

#[ cfg( feature = "derive_former" ) ]
#[ test_tools::nightly ]
#[ test ]
fn subform_scalar_on_unit_compile_fail() // Renamed for clarity
{
  let t = test_tools::compiletime::TestCases::new();
  t.compile_fail("tests/inc/enum_unit_tests/compile_fail/subform_scalar_on_unit.rs");
}

// To keep other potential trybuild tests separate, you might add more functions
// or integrate into a single one if preferred by project structure.
// For now, focusing on the current increment's test.
