
use wtest_basic as TheModule;
mod inc;

//

#[ test ]
fn trybuild_test()
{

  let t = trybuild::TestCases::new();
  t.pass( "../../../rust/test/test/dynamic/trybuild.rs" );

}
