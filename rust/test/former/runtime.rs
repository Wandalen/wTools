
use std::env;

#[test]
fn tests()
{
  let t = trybuild::TestCases::new();
  println!( "current_dir : {:?}", env::current_dir().unwrap() );
  t.pass( "../../../rust/test/former/test/basic_manual.rs" );
}

// include!( "../../../rust/test/former/test/basic_manual.rs" );

