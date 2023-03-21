
use wtest_basic as TheModule;
mod inc;

//

#[ test_tools::rustversion::nightly ]
#[ test ]
fn trybuild_test()
{
  let t = trybuild::TestCases::new();
  t.pass( "../../../rust/test/test/dynamic/trybuild.rs" );
  t.compile_fail( "../../../rust/test/test/dynamic/namespace_does_not_exists.rs" );
}
