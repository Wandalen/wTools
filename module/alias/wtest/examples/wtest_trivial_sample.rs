//! qqq : write proper description
use test_tools::*;

tests_impls!
{

  fn pass1_test()
  {
    assert_eq!( true, true );
  }

  //

  fn pass2_test()
  {
    assert_eq!( 1, 1 );
  }

}

//

tests_index!
{
  pass1_test,
  pass2_test,
}

fn main()
{
}