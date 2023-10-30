
use wtest_basic as TheModule;
mod inc;

//

#[ wtest_basic::rustversion::nightly ]
#[ test ]
fn trybuild_test()
{
  let t = trybuild::TestCases::new();
  t.pass( "tests/test/dynamic/trybuild.rs" );
  t.compile_fail( "tests/test/dynamic/namespace_does_not_exists.rs" );
}
