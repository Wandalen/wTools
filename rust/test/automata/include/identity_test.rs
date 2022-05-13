use std::collections::HashSet;
use test_tools::*;
use wtools::prelude::*;

//

fn identity_with_int_test()
{
  use wautomata::*;

  let src1 = wautomata::IdentityWithInt::make( 3 );
  let src2 = wautomata::IdentityWithInt::make( 3 );
  is_identity( src1 );
  fn is_identity< T : wautomata::IdentityInterface >( _ : T ){}
  assert_eq!( src1, src2 );

  let src1 = wautomata::IdentityWithInt::make( 3 );
  let src2 = wautomata::IdentityWithInt::make( 1 );
  assert_ne!( src1, src2 );

  let src = wautomata::IdentityWithInt::make( 3 );
  fn check_into< Src >( src : Src ) -> wautomata::IdentityWithInt
  where Src : Into< wautomata::IdentityWithInt >,
  {
    src.into()
  }
  assert_eq!( src, check_into( 3 ) );
  assert_ne!( src, check_into( 1 ) );
  assert_eq!( src, check_into( wautomata::IdentityWithInt::make( 3 ) ) );
  assert_ne!( src, check_into( wautomata::IdentityWithInt::make( 1 ) ) );

}

//

test_suite!
{
  identity_with_int,
}
