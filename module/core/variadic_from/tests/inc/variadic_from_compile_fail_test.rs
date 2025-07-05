#[ test ]
fn compile_fail()
{
  let t = test_tools::compiletime::TestCases::new();
  t.compile_fail( "tests/inc/compile_fail/*.rs" );
}