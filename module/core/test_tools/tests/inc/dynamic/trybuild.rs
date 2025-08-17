use test_tools::*;

//

test_tools::tests_impls!
{
  fn pass()
  {
    assert_eq!( true, true );
  }
}

//

test_tools::tests_index!
{
  pass,
}

#[ allow( dead_code ) ]
fn main()
{
}
