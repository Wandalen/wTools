// mod unit_subform_scalar_error;

#[ cfg( feature = "derive_former" ) ]
#[ test_tools::nightly ]
#[ test ]
fn former_trybuild()
{

  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  let t = test_tools::compiletime::TestCases::new();

  // Compile-fail tests for tuple variants (Increment 9)
  // Removed tuple variant compile-fail test references as they were moved

  // assert!( false );

}
