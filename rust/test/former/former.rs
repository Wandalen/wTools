
use std::env;

#[test]
fn trybuild_tests()
{
  println!( "current_dir : {:?}", env::current_dir().unwrap() );
  let t = trybuild::TestCases::new();
  t.compile_fail( "../../../rust/test/former/all/bad_attr.rs" );
  t.compile_fail( "../../../rust/test/former/all/vector_without_parameter.rs" );
  t.compile_fail( "../../../rust/test/former/all/hashmap_without_parameter.rs" );
}

include!( "./all/mod.rs" );
