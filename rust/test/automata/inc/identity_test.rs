use test_tools::*;

//

tests_impls!
{
  #[ test ]
  fn identity_with_int()
  {
    use wautomata::exposed::*;

    let src1 = IdentityWithInt::make( 3 );
    let src2 = IdentityWithInt::make( 3 );
    is_identity( src1 );
    fn is_identity< T : IdentityInterface >( _ : T ){}
    a_id!( src1, src2 );

    let src1 = IdentityWithInt::make( 3 );
    let src2 = IdentityWithInt::make( 1 );
    assert_ne!( src1, src2 );

    let src = IdentityWithInt::make( 3 );
    fn check_into< Src >( src : Src ) -> IdentityWithInt
    where Src : Into< IdentityWithInt >,
    {
      src.into()
    }
    a_id!( src, check_into( 3 ) );
    assert_ne!( src, check_into( 1 ) );
    a_id!( src, check_into( IdentityWithInt::make( 3 ) ) );
    assert_ne!( src, check_into( IdentityWithInt::make( 1 ) ) );
  }
}

//

tests_index!
{
  identity_with_int,
}
