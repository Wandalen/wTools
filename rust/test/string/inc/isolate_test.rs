
use super::*;

//

tests_impls!
{
  fn basic()
  {
    let src = "";
    let req = TheModule::string::isolate_left()
    .src( src )
    .perform();
    let mut exp = ( "", None, "" );
    assert_eq!( req, exp );
  }
}

//

tests_index!
{
  basic,
}
