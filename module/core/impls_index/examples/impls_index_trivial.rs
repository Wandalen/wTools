//! doc
fn main()
{
  use ::impls_index::*;

  impls1!
  {
    fn f1() -> i32
    {
      println!( "f1() : 13" );
      13
    }
  }

  index!
  {
    f1,
  }
  assert_eq!( f1(), 13 );
  /* print : f1() : 13 */
}

