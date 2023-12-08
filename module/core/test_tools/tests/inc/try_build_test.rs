#[ allow( unused_imports ) ]
use super::*;

#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
#[ ::test_tools::nightly ]
#[ test ]
fn trybuild_test()
{
  // let t = trybuild::TestCases::new();
  let t = ::test_tools::compiletime::TestCases::new();
  t.pass( "tests/inc/dynamic/trybuild.rs" );
  t.compile_fail( "tests/inc/dynamic/namespace_does_not_exists.rs" );
}
