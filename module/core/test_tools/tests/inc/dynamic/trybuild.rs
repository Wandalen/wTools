use test_tools::*;

//

tests_impls!
{
  fn pass()
  {
    assert_eq!( true, true );
  }
}

//

tests_index!
{
  pass,
}

#[ allow( dead_code ) ]
fn main()
{
}
