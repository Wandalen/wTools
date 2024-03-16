#[ allow( unused_imports ) ]
use super::*;
// // use test_tools::exposed::*;

tests_impls!
{
  fn basic()
  {
    use the_module::{ VectorizedInto, VectorizedFrom };
    the_module::types!
    {
      #[ derive( Debug, PartialEq, Clone ) ]
      single Single1 : i32;
      #[ derive( Debug, PartialEq, Clone ) ]
      single Single2 : i32;
      #[ derive( Debug, PartialEq, Clone ) ]
      single Single3 : i32;
    }

    /* test.case( "from/into x0 tupple" ) */
    {
      let src = ();
      let got : () = src.vectorized_into();
      let exp = ();
      a_id!( got, exp );

      let src = ();
      let got = <()>::vectorized_from( src );
      let exp = ();
      a_id!( got, exp );
    }

    /* test.case( "from itself x1 tupple" ) */
    {
      let src = ( 1, );
      let got : ( i32, ) = src.vectorized_into();
      let exp = ( 1, );
      a_id!( got, exp );

      let src = ( 1, );
      let got = <( i32, )>::vectorized_from( src );
      let exp = ( 1, );
      a_id!( got, exp );
    }

    /* test.case( "from x1 tupple" ) */
    {
      let src = ( 1, );
      let got : ( Single1, ) = src.vectorized_into();
      let exp = ( Single1::from( 1 ), );
      a_id!( got, exp );

      let src = ( 1, );
      let got = <( Single1, )>::vectorized_from( src );
      let exp = ( Single1::from( 1 ), );
      a_id!( got, exp );
    }

    /* test.case( "into x1 tupple" ) */
    {
      let src = ( Single1::from( 1 ), );
      let got : ( i32, ) = src.vectorized_into();
      let exp = ( 1, );
      a_id!( got, exp );

      let src = ( Single1::from( 1 ), );
      let got = <( i32, )>::vectorized_from( src );
      let exp = ( 1, );
      a_id!( got, exp );
    }

    /* test.case( "from x2 tupple" ) */
    {
      let src = ( 1, 3 );
      let got : ( Single1, Single1 ) = src.vectorized_into();
      let exp = ( Single1::from( 1 ), Single1::from( 3 ) );
      a_id!( got, exp );

      let src = ( 1, 3 );
      let got = <( Single1, Single1 )>::vectorized_from( src );
      let exp = ( Single1::from( 1 ), Single1::from( 3 ) );
      a_id!( got, exp );
    }

    /* test.case( "into x2 tupple" ) */
    {
      let src = ( Single1::from( 1 ), Single2::from( 3 ) );
      let got : ( i32, i32 ) = src.vectorized_into();
      let exp = ( 1, 3 );
      a_id!( got, exp );

      let src = ( Single1::from( 1 ), Single2::from( 3 ) );
      let got = <( i32, i32 )>::vectorized_from( src );
      let exp = ( 1, 3 );
      a_id!( got, exp );
    }

    /* test.case( "from x3 tupple" ) */
    {
      let src = ( 1, 2, 3 );
      let got : ( Single1, Single2, Single3 ) = src.vectorized_into();
      let exp = ( Single1::from( 1 ), Single2::from( 2 ), Single3::from( 3 ) );
      a_id!( got, exp );

      let src = ( 1, 2, 3 );
      let got = <( Single1, Single2, Single3 )>::vectorized_from( src );
      let exp = ( Single1::from( 1 ), Single2::from( 2 ), Single3::from( 3 ) );
      a_id!( got, exp );
    }

    /* test.case( "into x3 tupple" ) */
    {
      let src = ( Single1::from( 1 ), Single2::from( 2 ), Single3::from( 3 ) );
      let got : ( i32, i32, i32 ) = src.vectorized_into();
      let exp = ( 1, 2, 3 );
      a_id!( got, exp );

      let src = ( Single1::from( 1 ), Single2::from( 2 ), Single3::from( 3 ) );
      let got = <( i32, i32, i32 )>::vectorized_from( src );
      let exp = ( 1, 2, 3 );
      a_id!( got, exp );
    }

    /* test.case( "from/into x0 array" ) */
    {
      let src : [ i32 ; 0 ] = [];
      let got : [ i32 ; 0 ] = src.vectorized_into();
      let exp : [ i32 ; 0 ] = [];
      a_id!( got, exp );

      let src : [ i32 ; 0 ] = [];
      let got = <[ i32 ; 0 ]>::vectorized_from( src );
      let exp : [ i32 ; 0 ] = [];
      a_id!( got, exp );
    }

    /* test.case( "from itself x1 array" ) */
    {
      let src = [ Single1::from( 1 ) ];
      let got : [ Single1 ; 1 ] = src.vectorized_into();
      let exp = [ Single1::from( 1 ) ];
      a_id!( got, exp );

      let src = [ Single1::from( 1 ) ];
      let got = <[ Single1 ; 1 ]>::vectorized_from( src );
      let exp = [ Single1::from( 1 ) ];
      a_id!( got, exp );
    }

    /* test.case( "from x1 array" ) */
    {
      let src = [ 1 ];
      let got : [ Single1 ; 1 ] = src.vectorized_into();
      let exp = [ Single1::from( 1 ) ];
      a_id!( got, exp );

      let src = [ 1 ];
      let got = <[ Single1 ; 1 ]>::vectorized_from( src );
      let exp = [ Single1::from( 1 ) ];
      a_id!( got, exp );
    }

    /* test.case( "into x1 array" ) */
    {
      let src = [ Single1::from( 1 ) ];
      let got : [ i32 ; 1 ] = src.vectorized_into();
      let exp = [ 1 ];
      a_id!( got, exp );

      let src = [ Single1::from( 1 ) ];
      let got = <[ i32 ; 1 ]>::vectorized_from( src );
      let exp = [ 1 ];
      a_id!( got, exp );
    }

    /* test.case( "from x2 array" ) */
    {
      let src = [ 1, 3 ];
      let got : [ Single1 ; 2 ] = src.vectorized_into();
      let exp = [ Single1::from( 1 ), Single1::from( 3 ) ];
      a_id!( got, exp );

      let src = [ 1, 3 ];
      let got = <[ Single1 ; 2 ]>::vectorized_from( src );
      let exp = [ Single1::from( 1 ), Single1::from( 3 ) ];
      a_id!( got, exp );
    }

    /* test.case( "into x2 array" ) */
    {
      let src = [ Single1::from( 1 ), Single1::from( 3 ) ];
      let got : [ i32 ; 2 ] = src.vectorized_into();
      let exp = [ 1, 3 ];
      a_id!( got, exp );

      let src = [ Single1::from( 1 ), Single1::from( 3 ) ];
      let got = <[ i32 ; 2 ]>::vectorized_from( src );
      let exp = [ 1, 3 ];
      a_id!( got, exp );
    }

    /* test.case( "from x3 array" ) */
    {
      let src = [ 1, 2, 3 ];
      let got : [ Single1 ; 3 ] = src.vectorized_into();
      let exp = [ Single1::from( 1 ), Single1::from( 2 ), Single1::from( 3 ) ];
      a_id!( got, exp );

      let src = [ 1, 2, 3 ];
      let got = <[ Single1 ; 3 ]>::vectorized_from( src );
      let exp = [ Single1::from( 1 ), Single1::from( 2 ), Single1::from( 3 ) ];
      a_id!( got, exp );
    }

    /* test.case( "into x3 array" ) */
    {
      let src = [ Single1::from( 1 ), Single1::from( 2 ), Single1::from( 3 ) ];
      let got : [ i32 ; 3 ] = src.vectorized_into();
      let exp = [ 1, 2, 3 ];
      a_id!( got, exp );

      let src = [ Single1::from( 1 ), Single1::from( 2 ), Single1::from( 3 ) ];
      let got = <[ i32 ; 3 ]>::vectorized_from( src );
      let exp = [ 1, 2, 3 ];
      a_id!( got, exp );
    }

  }

}

//

tests_index!
{
  basic,
}
