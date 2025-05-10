// mod subform_collection_test;
// qqq : xxx : make it working

#[ cfg( feature = "derive_former" ) ]
#[ test_tools::nightly ]
#[ test ]
fn former_trybuild()
{

  println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
  let _t = test_tools::compiletime::TestCases::new();

  // assert!( false );

}
