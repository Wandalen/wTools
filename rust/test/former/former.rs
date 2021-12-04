
use std::env;

#[test]
fn trybuild_tests()
{
  println!( "current_dir : {:?}", env::current_dir().unwrap() );
  // let t = trybuild::TestCases::new();
  // t.pass( "rust/test/former/test/basic_former.rs" );
}

include!( "./test/former.rs" );
