#[ allow( unused_imports ) ]
use super::*;

#[ cfg( feature = "enabled" ) ]
#[ test_tools::rustversion::nightly ]
#[ test ]
fn trybuild_test()
{
  let t = trybuild::TestCases::new();
  t.pass( "tests/inc/dynamic/trybuild.rs" );
  t.compile_fail( "tests/inc/dynamic/namespace_does_not_exists.rs" );
}
