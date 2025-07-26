// mod tuple_multi_subform_scalar_error;
// mod tuple_single_subform_non_former_error;
mod tuple_zero_subform_scalar_error; // Comment out to avoid compilation issues

#[ cfg( feature = "derive_former" ) ]
#[ test_tools::nightly ]
#[ test ]
fn former_trybuild()
{

  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  let t = test_tools::compiletime::TestCases::new();

  // Compile-fail tests for tuple variants (Increment 9)
  t.compile_fail( "tests/inc/enum_unnamed_tests/compile_fail/tuple_zero_subform_scalar_error.rs" ); // T0.5
  t.compile_fail( "tests/inc/enum_unnamed_tests/compile_fail/tuple_single_subform_non_former_error.rs" ); // T1.5
  t.compile_fail( "tests/inc/enum_unnamed_tests/compile_fail/tuple_multi_subform_scalar_error.rs" ); // TN.3

  // assert!( false );

}
