#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
use TheModule::*;

tests_impls!
{

  //

  #[ test ]
  fn basic()
  {
    use core::fmt;

    mod mod1
    {
      pub use f32;
    }

    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::f32;

    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = Single::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::from( Single::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single / into f32" ) */
    let instance1 : Single = ( 13.0 ).into();
    let got : f32 = instance1.into();
    a_id!( got, 13.0 );
    let instance1 : Single = ( 13.0 ).into();
    let got = f32::from( instance1 );
    a_id!( got, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    use core::ops::AddAssign;
    let mut got : Single = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );
    got.add_assign( 1.0 );
    a_id!( got.0, 14.5 );

  }

  //

  #[ test ]
  fn empty_parameter()
  {

    mod mod1
    {
      pub use f32;
    }

    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::f32<>;
    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = Single::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::from( Single::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single / into f32" ) */
    let instance1 : Single = ( 13.0 ).into();
    let got : f32 = instance1.into();
    a_id!( got, 13.0 );
    let instance1 : Single = ( 13.0 ).into();
    let got = f32::from( instance1 );
    a_id!( got, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn parametrized()
  {

    macro_rules! mk
    {
      (
        $( $Rest : tt )*
      )
      =>
      {
        mod1::Floats::from( $( $Rest )* )
      };
    }

    mod mod1
    {

      #[ derive( Debug, Clone, PartialEq ) ]
      pub struct Floats< T >
      (
        pub T,
      );

      impl< T > core::ops::Deref
      for Floats< T >
      {
        type Target = T;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< T > From< T > for Floats< T >
      {
        fn from( src : T ) -> Self
        {
          Self( src )
        }
      }

    }

    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::Floats< T >;
    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( mk!( 13.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( mk!( 13.0 ) ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( mk!( 13.0 ) ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single / into f32" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let got : mod1::Floats< f32 > = instance1.into();
    a_id!( got.0, 13.0 );
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let got = mod1::Floats::< f32 >::from( instance1 );
    a_id!( got.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( mk!( 13.5 ) ).into();
    a_id!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn parametrized_complex()
  {

    macro_rules! mk
    {
      (
        $( $Rest : tt )*
      )
      =>
      {
        mod1::Floats::from( $( $Rest )* )
      };
    }

    mod mod1
    {

      #[ derive( Debug, Clone, PartialEq ) ]
      pub struct Floats< T : PartialEq + Copy >
      (
        pub T,
      );

      impl< T : PartialEq + Copy > core::ops::Deref
      for Floats< T >
      {
        type Target = T;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< T : PartialEq + Copy > From< T > for Floats< T >
      {
        fn from( src : T ) -> Self
        {
          Self( src )
        }
      }

    }

    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::Floats< T : PartialEq + std::marker::Copy >;

    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( mk!( 13.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( mk!( 13.0 ) ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( mk!( 13.0 ) ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single / into f32" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let got : mod1::Floats< f32 > = instance1.into();
    a_id!( got.0, 13.0 );
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let got = mod1::Floats::< f32 >::from( instance1 );
    a_id!( got.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( mk!( 13.5 ) ).into();
    a_id!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn parametrized_multiple()
  {
    use core::fmt;

    macro_rules! mk
    {
      (
        $( $Rest : tt )*
      )
      =>
      {
        mod1::Floats::from( $( $Rest )* )
      };
    }

    mod mod1
    {

      #[ derive( Debug, Clone, PartialEq ) ]
      pub struct Floats< T1 : PartialEq + Copy, T2 : Default >
      (
        pub T1,
        pub T2,
      );

      impl< T1 : PartialEq + Copy, T2 : Default > core::ops::Deref
      for Floats< T1, T2 >
      {
        type Target = T1;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< T1 : PartialEq + Copy, T2 : Default > From< T1 >
      for Floats< T1, T2 >
      {
        fn from( src : T1 ) -> Self
        {
          Floats::< T1, T2 >( src, T2::default() )
        }
      }

    }

    // trace_macros!( true );
    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::Floats< T1 : PartialEq + std::marker::Copy, T2 : Default >;
    }
    // trace_macros!( false );

    /* test.case( "make1" ) */
    let got : Single< f32, f64 > = make!( mk!( 13.0 ) );
    let exp = Single::< f32, f64 >::from( mk!( 13.0 ) );
    a_id!( got, exp );

    /* test.case( "traits" ) */
    let instance1 = Single::< f32, f64 >::from( mk!( 13.0 ) );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );
    assert!( !implements!( instance1 => fmt::Display ) );

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32, f64 > = ( mk!( 13.0 ) ).into();
    let instance2 = Single::< f32, f64 >::from( mk!( 13.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32, f64 > = ( Single::from( mk!( 13.0 ) ) ).into();
    let instance2 = Single::< f32, f64 >::from( Single::from( mk!( 13.0 ) ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single / into f32" ) */
    let instance1 : Single< f32, f64 > = ( mk!( 13.0 ) ).into();
    let got : mod1::Floats< f32, f64 > = instance1.into();
    a_id!( got.0, 13.0 );
    let instance1 : Single< f32, f64 > = ( mk!( 13.0 ) ).into();
    let got = mod1::Floats::< f32, f64 >::from( instance1 );
    a_id!( got.0, 13.0 );

    /* test.case( "from tuple" ) */
    let got : Single< f32, f64 > = ( mk!( 13.0 ), ).into();
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );
    let got = Single::< f32, f64 >::from( ( mk!( 13.0 ), ) );
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );

    /* test.case( "from array" ) */
    let got : Single< f32, f64 > = [ mk!( 13.0 ), ].into();
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );
    let got = Single::< f32, f64 >::from( [ mk!( 13.0 ), ] );
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );

    /* test.case( "from slice" ) */
    let got : Single< f32, f64 > = ( &[ mk!( 13.0 ), ][ .. ] ).into();
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );
    let got = Single::< f32, f64 >::from( &[ mk!( 13.0 ), ][ .. ] );
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32, f64 > = ( mk!( 13.0 ) ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32, f64 > = ( mk!( 13.5 ) ).into();
    a_id!( got.round(), 14.0 );

    /* test.case( "clone_as_tuple" ) */
    let src : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    let got = src.clone_as_tuple();
    a_id!( got, ( mk!( 13.0 ), ) );
    assert!( !mem_same_ptr( &src, &got ) );

    /* test.case( "clone_as_array" ) */
    let src : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    let got = src.clone_as_array();
    a_id!( got, [ mk!( 13.0 ), ] );
    assert!( !mem_same_ptr( &src, &got ) );

    /* test.case( "as_tuple" ) */
    let src : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    let got = src.as_tuple();
    a_id!( got, &( mk!( 13.0 ), ) );
    assert!( mem_same_region( &src, got ) );

    /* test.case( "as_array" ) */
    let src : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    let got = src.as_array();
    a_id!( got, &[ mk!( 13.0 ), ] );
    assert!( mem_same_region( &src, got ) );

    /* test.case( "as_slice" ) */
    let src : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    let got = src.as_slice();
    a_id!( got, &[ mk!( 13.0 ), ][ .. ] );
    assert!( mem_same_region( &src, got ) );

  }

  //

  #[ test ]
  fn parametrized_no_derives()
  {

    mod mod1
    {
      pub struct Floats< T1, T2 >
      (
        pub T1,
        pub T2,
      );
    }

    // trace_macros!( true );
    types!
    {
      single Single : mod1::Floats< T1, T2 >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Single::< f32, f64 >( mod1::Floats( 13.0, 31.0 ) );

  }

  //

  #[ test ]
  fn parameter()
  {
    use core::fmt;

    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq, Default ) ]
      single Single : < T >;
    }

    /* test.case( "make1" ) */
    let got : Single< f32 > = make!( 13.0 );
    let exp = Single::< f32 >::from( 13.0 );
    a_id!( got, exp );

    /* test.case( "traits" ) */
    let instance1 = Single::< f32 >::from( 13.0 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( implements!( instance1 => Default ) );
    assert!( !implements!( instance1 => fmt::Display ) );

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = Single::< f32 >::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from tuple" ) */
    let got : Single< f32 > = ( 13.0, ).into();
    a_id!( got, Single( 13.0 ) );
    let got = Single::< f32 >::from( ( 13.0, ) );
    a_id!( got, Single( 13.0 ) );

    /* test.case( "to tuple" ) */
    let got : ( f32, ) = ( Single::< f32 >::from( 13.0 ) ).into();
    a_id!( got, ( 13.0, ) );
    let got = < ( f32, ) >::from( Single::< f32 >::from( ( 13.0, ) ) );
    a_id!( got, ( 13.0, ) );

    /* test.case( "from array" ) */
    let got : Single< f32 > = [ 13.0 ].into();
    a_id!( got, Single( 13.0 ) );
    let got = Single::< f32 >::from( [ 13.0 ] );
    a_id!( got, Single( 13.0 ) );

    /* test.case( "to array" ) */
    let got : [ f32 ; 1 ] = ( Single::< f32 >::from( 13.0 ) ).into();
    a_id!( got, [ 13.0 ] );
    let got = < [ f32 ; 1 ] >::from( Single::< f32 >::from( 13.0 ) );
    a_id!( got, [ 13.0 ] );

    /* test.case( "from slice" ) */
    let got : Single< f32 > = (&[ 13.0 ][ .. ]).into();
    a_id!( got, Single( 13.0 ) );
    let got = Single::< f32 >::from( (&[ 13.0 ][ .. ]) );
    a_id!( got, Single( 13.0 ) );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );

    /* test.case( "clone_as_tuple" ) */
    let src : Single< f32 > = ( 13.0, ).into();
    let got = src.clone_as_tuple();
    a_id!( got, ( 13.0, ) );
    assert!( !mem_same_ptr( &src, &got ) );

    /* test.case( "clone_as_array" ) */
    let src : Single< f32 > = ( 13.0, ).into();
    let got = src.clone_as_array();
    a_id!( got, [ 13.0, ] );
    assert!( !mem_same_ptr( &src, &got ) );

    /* test.case( "as_tuple" ) */
    let src : Single< f32 > = ( 13.0, ).into();
    let got = src.as_tuple();
    a_id!( got, &( 13.0, ) );
    assert!( mem_same_region( &src, got ) );

    /* test.case( "as_array" ) */
    let src : Single< f32 > = ( 13.0, ).into();
    let got = src.as_array();
    a_id!( got, &[ 13.0, ] );
    assert!( mem_same_region( &src, got ) );

    /* test.case( "as_slice" ) */
    let src : Single< f32 > = ( 13.0, ).into();
    let got = src.as_slice();
    a_id!( got, &[ 13.0, ][ .. ] );
    assert!( mem_same_region( &src, got ) );

  }

  //

  #[ test ]
  fn parameter_complex()
  {

    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : < T : core::cmp::PartialEq + core::clone::Clone >;
    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = Single::< f32 >::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    use core::ops::AddAssign;
    let mut got : Single< f32 > = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );
    got.add_assign( 1.0 );
    a_id!( got.0, 14.5 );

  }

  //

  #[ test ]
  fn parameter_no_derives()
  {

    mod mod1
    {
      pub struct Floats< T1, T2 >
      (
        pub T1,
        pub T2,
      );
    }

    // trace_macros!( true );
    types!
    {
      single Single : < T >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Single( mod1::Floats( 13.0, 31.0 ) );

  }

  //

  #[ test ]
  fn multiple()
  {
    use core::fmt;

    types!
    {

      single Single1 : f32;

      #[ derive( Debug ) ]
      #[ derive( PartialEq, Clone ) ]
      single Single2 : f32;

    }

    /* test.case( "from f32 / into Single2" ) */
    let instance1 : Single1 = ( 13.0 ).into();
    let instance2 = Single1::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    assert!( !implements!( instance1 => PartialEq ) );
    assert!( !implements!( instance1 => Clone ) );
    assert!( !implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from f32 / into Single2" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let instance2 = Single2::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from f32 / into Single2" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let instance2 = Single2::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single2 = ( Single2::from( 13.0 ) ).into();
    let instance2 = Single2::from( Single2::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single2 / into f32" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let got : f32 = instance1.into();
    a_id!( got, 13.0 );
    let instance1 : Single2 = ( 13.0 ).into();
    let got = f32::from( instance1 );
    a_id!( got, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single2 = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn struct_basic()
  {

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = Single::< f32 >::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "default" ) */
    let instance1 : Single< f32 > = Default::default();
    a_id!( instance1.0, 0.0 );

    /* test.case( "deref" ) */
    use core::ops::AddAssign;
    let mut got : Single< f32 > = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );
    got.add_assign( 1.0 );
    a_id!( got.0, 14.5 );

    /* test.case( "make0" ) */
    let got : Single< f32 > = make!();
    let exp = Single::< f32 >::from( 0.0 );
    a_id!( got, exp );

    /* test.case( "make1" ) */
    let got : Single< f32 > = make!( 13.0 );
    let exp = Single::< f32 >::from( 13.0 );
    a_id!( got, exp );

  }

  //

  #[ test ]
  fn struct_no_derives()
  {

    struct Floats< T >( pub T );

    impl< T > Floats< T >
    {
      pub fn new( src : T ) -> Self
      { Self( src ) }
    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< Floats< f32 > > = ( Floats( 13.0 ) ).into();
    let instance2 = Single::< Floats< f32 > >::from( Floats( 13.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );

    /* test.case( "from itself / into itself" ) */
    let val = Floats::< f32 >::new( 13.0 );
    let instance1 : Single< Floats< f32 > > = ( Single::from( val ) ).into();
    let instance2 = Single::< Floats< f32 > >::from( Single::from( Floats( 13.0 ) ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );

    /* test.case( "deref" ) */
    use core::ops::AddAssign;
    let mut got : Single< f32 > = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );
    got.add_assign( 1.0 );
    a_id!( got.0, 14.5 );

  }

  //

  #[ test ]
  fn samples()
  {

    /* test.case( "multiple" ) */
    {
      types!
      {

        single MySingle : f32;
        single SingleWithParametrized : std::sync::Arc< T : Copy >;
        single SingleWithParameter : < T >;

        pair MyPair : f32;
        pair PairWithParametrized : std::sync::Arc< T1 : Copy >, std::sync::Arc< T2 : Copy >;
        pair PairWithParameter : < T1, T2 >;

        pair MyHomoPair : f32;
        pair HomoPairWithParametrized : std::sync::Arc< T : Copy >;
        pair HomoPairWithParameter : < T >;

        many MyMany : f32;
        many ManyWithParametrized : std::sync::Arc< T : Copy >;
        many ManyWithParameter : < T >;

      }
    }

    /* test.case( "no macro" ) */
    {
      let i32_in_tuple = TheModule::Single::< i32 >::from( 13 );
      dbg!( i32_in_tuple );
      // i32_in_tuple = Single( 13 )
      let i32_and_f32_in_tuple = TheModule::Pair::< i32, f32 >::from( ( 13, 13.0 ) );
      dbg!( i32_and_f32_in_tuple );
      // vec_of_i32_in_tuple = Pair( 13, 13.0 )
      let two_i32_in_tuple = TheModule::HomoPair::< i32 >::from( ( 13, 31 ) );
      dbg!( two_i32_in_tuple );
      // vec_of_i32_in_tuple = HomoPair( 13, 31 )
      let vec_of_i32_in_tuple = TheModule::Many::< i32 >::from( [ 1, 2, 3 ] );
      dbg!( vec_of_i32_in_tuple );
      // vec_of_i32_in_tuple = Many([ 1, 2, 3 ])
    }

    /* test.case( "single-line" ) */
    {
      types!( single MySingle : i32 );
      let x = MySingle( 13 );
      println!( "x : {}", x.0 );
    }

    /* test.case( "derives and attributes" ) */
    {
      types!
      {
        /// This is also attribute and macro understands it.
        #[ derive( Debug ) ]
        single MySingle : i32;
      }
      let x = MySingle( 13 );
      dbg!( x );
    }

    /* test.case( "struct instead of macro" ) */
    {
      let x = Single::< i32 >( 13 );
      dbg!( x );
    }

    /* test.case( "parametrized element" ) */
    {
      types!
      {
        #[ derive( Debug ) ]
        single MySingle : std::sync::Arc< T : Copy >;
      }
      let x = MySingle( std::sync::Arc::new( 13 ) );
      dbg!( x );
    }

    /* test.case( "parametrized tuple" ) */
    {
      types!
      {
        #[ derive( Debug ) ]
        single MySingle : < T : Copy >;
      }
      let x = MySingle( 13 );
      dbg!( x );
    }

  }

}

//

tests_index!
{

  basic,
  empty_parameter,
  parametrized,
  parametrized_complex,
  parametrized_multiple,
  parametrized_no_derives,
  parameter,
  parameter_complex,
  parameter_no_derives,
  multiple,
  struct_basic,
  struct_no_derives,
  samples,

}
