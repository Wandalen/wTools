// use test_tools::exposed::*;
use super::*;

//

tests_impls!
{

  fn identity_with_int()
  {
    use the_module::exposed::*;

    /* test.case( "basic" ) */
    {
      let src1 = IdentityWithInt::from( 3 );
      let src2 = IdentityWithInt::from( 3 );
      // is_identity( src1 );
      // fn is_identity< T : IdentityInterface >( _ : T ){}
      a_true!( implements!( src1 => IdentityInterface ) );
      a_id!( src1, src2 );

      let src1 = IdentityWithInt::from( 3 );
      let src2 = IdentityWithInt::from( 1 );
      a_not_id!( src1, src2 );
    }

    /* test.case( "from" ) */
    {
      let src = IdentityWithInt::from( 3 );
      fn check_into< Src >( src : Src ) -> IdentityWithInt
      where Src : Into< IdentityWithInt >,
      {
        src.into()
      }
      a_id!( src, check_into( 3 ) );
      a_not_id!( src, check_into( 1 ) );
      a_id!( src, check_into( IdentityWithInt::from( 3 ) ) );
      a_not_id!( src, check_into( IdentityWithInt::from( 1 ) ) );
    }

    // zzz
    // /* test.case( "from pair" ) */
    // {
    //   let src = Pair::from_2( 1, 3 );
    //   let got : Pair< IdentityWithInt, IdentityWithInt > = src.into();
    //   let exp = Pair::from_2( IdentityWithInt::make( 1 ), IdentityWithInt::make( 3 ) );
    //   a_id!( got, exp );
    // }

    // /* test.case( "from x1 tupple" ) */
    // {
    //   let src = ( 1, );
    //   let got : ( IdentityWithInt, ) = src.into();
    //   let exp = ( IdentityWithInt::make( 1 ) );
    //   a_id!( got, exp );
    // }

    /* test.case( "from x2 tupple" ) */
    // {
    //   //use type_constructor::VectorizedInto;
    //   let src = ( 1, 3 );
    //   let got : ( IdentityWithInt, IdentityWithInt ) = src.into();
    //   let exp = ( IdentityWithInt::from( 1 ), IdentityWithInt::from( 3 ) );
    //   a_id!( got, exp );
    // }

    // /* test.case( "from x3 tupple" ) */
    // {
    //   let src = ( 1, 2, 3 );
    //   let got : ( IdentityWithInt, IdentityWithInt, IdentityWithInt ) = src.into();
    //   let exp = ( IdentityWithInt::make( 1 ), IdentityWithInt::make( 2 ), IdentityWithInt::make( 3 ) );
    //   a_id!( got, exp );
    // }

  }

  //

  fn identity_implemented_for_identity_by_pointer()
  {
    use the_module::exposed::*;

    let x = 1;
    let y = 1;
    let src1 = IdentityWithPointer::from( &x );
    let src2 = IdentityWithPointer::from( &y );
    check( src1 );
    fn check< T : IdentityInterface >( _ : T ){}
    a_not_id!( src1, src2 );
  }

  //

  fn identity_implemented_for_identity_by_name()
  {
    use the_module::exposed::*;

    let src1 = IdentityWithName::from( "abc" );
    let src2 = IdentityWithName::from( "abc" );
    check( src1 );
    fn check< T : IdentityInterface >( _ : T ){}
    assert_eq!( src1, src2 );
  }

  //


  fn identity_implemented_for_identity_by_int()
  {
    use the_module::exposed::*;

    let src1 = IdentityWithInt::from( 3 );
    let src2 = IdentityWithInt::from( 3 );
    check( src1 );
    fn check< T : IdentityInterface >( _ : T ){}
    assert_eq!( src1, src2 );
  }

}

//

tests_index!
{

  identity_with_int,

  identity_implemented_for_identity_by_pointer,
  identity_implemented_for_identity_by_name,
  identity_implemented_for_identity_by_int,

}
