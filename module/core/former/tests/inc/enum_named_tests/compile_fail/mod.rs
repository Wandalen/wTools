mod struct_zero_default_error;
mod struct_zero_subform_scalar_error;

#[ cfg( feature = "derive_former" ) ]
#[ test_tools::nightly ]
#[ test ]
fn former_trybuild()
{

  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  let t = test_tools::compiletime::TestCases::new();

  // Compile-fail tests for struct variants
  t.compile_fail( "tests/inc/enum_named_tests/compile_fail/struct_zero_default_error.rs" );
  t.compile_fail( "tests/inc/enum_named_tests/compile_fail/struct_zero_subform_scalar_error.rs" );

  // assert!( false );

}
