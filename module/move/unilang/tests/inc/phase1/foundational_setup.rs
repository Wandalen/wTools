use super::*;

#[ test ]
fn try_build()
{
  let t = test_tools::compiletime::TestCases::new();
  t.pass( "tests/inc/phase1/try_build.rs" );
}