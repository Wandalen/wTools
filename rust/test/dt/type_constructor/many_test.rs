#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
use TheModule::*;

tests_impls!
{

  //

  #[ test ]
  fn basic_test()
  {
    use core::fmt;

    mod mod1
    {
      pub use f32;
    }

    // trace_macros!( true );
    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      many Many : mod1::f32;

    }
    // trace_macros!( false );

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many = ( 13.0 ).into();
    let instance2 = Many::from( 13.0 );
    assert_eq!( instance1.0, vec![ 13.0 ] );
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Many = ( Many::from( 13.0 ) ).into();
    let instance2 = Many::from( Many::from( 13.0 ) );
    assert_eq!( instance1.0, vec![ 13.0 ] );
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Many = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many = ( 13.0 ).into();
    assert_eq!( got.len(), 1 );
    assert_eq!( got.pop(), Some( 13.0 ) );

  }

  //

  #[ test ]
  fn empty_parameter_test()
  {

    mod mod1
    {
      pub use f32;
    }

    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      many Many : mod1::f32<>;
    }

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many = ( 13.0 ).into();
    let instance2 = Many::from( 13.0 );
    assert_eq!( instance1.0, vec![ 13.0 ] );
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );

  }

  //

  #[ test ]
  fn parametrized_multiple_test()
  {

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
      many Many : mod1::Floats< T1 : PartialEq + std::marker::Copy, T2 : Default >;
    }
    // trace_macros!( false );

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many< f32, f64 > = ( mod1::Floats::from( 13.0 ) ).into();
    let instance2 = Many::< f32, f64 >::from( mod1::Floats::from( 13.0 ) );
    assert_eq!( instance1.0, vec![ mod1::Floats::from( 13.0 ) ] );
    assert_eq!( instance2.0, vec![ mod1::Floats::from( 13.0 ) ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Many< f32, f64 > = ( Many::from( mod1::Floats::from( 13.0 ) ) ).into();
    let instance2 = Many::< f32, f64 >::from( Many::from( mod1::Floats::from( 13.0 ) ) );
    assert_eq!( instance1.0, vec![ mod1::Floats::from( 13.0 ) ] );
    assert_eq!( instance2.0, vec![ mod1::Floats::from( 13.0 ) ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "from tuple" ) */
    let got : Many< f32, f64 > = ( mod1::Floats::from( 13.0 ), ).into();
    let exp : Many< f32, f64 > = Many::from( mod1::Floats::from( 13.0 ) );
    assert_eq!( got, exp );
    let got = Many::< f32, f64 >::from( ( mod1::Floats::from( 13.0 ), ) );
    let exp : Many< f32, f64 > = Many::from( mod1::Floats::from( 13.0 ) );
    assert_eq!( got, exp );

    /* test.case( "from array" ) */
    let got : Many< f32, f64 > = [ mod1::Floats::from( 13.0 ), ].into();
    let exp : Many< f32, f64 > = Many::from( mod1::Floats::from( 13.0 ) );
    assert_eq!( got, exp );
    let got = Many::< f32, f64 >::from( [ mod1::Floats::from( 13.0 ), ] );
    let exp : Many< f32, f64 > = Many::from( mod1::Floats::from( 13.0 ) );
    assert_eq!( got, exp );

    /* test.case( "from slice" ) */
    let got : Many< f32, f64 > = ( &[ mod1::Floats::from( 13.0 ), ][ .. ] ).into();
    let exp : Many< f32, f64 > = Many::from( mod1::Floats::from( 13.0 ) );
    assert_eq!( got, exp );
    let got = Many::< f32, f64 >::from( &[ mod1::Floats::from( 13.0 ), ][ .. ] );
    let exp : Many< f32, f64 > = Many::from( mod1::Floats::from( 13.0 ) );
    assert_eq!( got, exp );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32, f64 > = ( mod1::Floats::from( 13.0 ) ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, vec![ mod1::Floats::from( 13.0 ) ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many< f32, f64 > = ( mod1::Floats::from( 13.0 ) ).into();
    assert_eq!( got.len(), 1 );
    assert_eq!( got.pop(), Some( mod1::Floats::from( 13.0 ) ) );

    /* test.case( "as_slice" ) */
    let src : Many< f32, f64 > = Many::from( mod1::Floats::from( 13.0 ) );
    let got = src.as_slice();
    assert_eq!( got, &[ mod1::Floats::from( 13.0 ), ][ .. ] );
    let got = &src[ .. ];
    assert_eq!( got, &[ mod1::Floats::from( 13.0 ), ][ .. ] );

  }

  //

  #[ test ]
  fn parametrized_no_derives_test()
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
      many Many : mod1::Floats< T1, T2 >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Many::< f32, f64 >( vec![ mod1::Floats( 13.0, 31.0 ) ] );

  }

  //

  #[ test ]
  fn parameter_test()
  {
    use core::fmt;

    // trace_macros!( true );
    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq, Default ) ]
      many Many : < T >;
    }
    // trace_macros!( false );

    /* test.case( "basic" ) */
    let instance1 = Many::< f32 >::from( 13.0 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( implements!( instance1 => Default ) );
    assert!( !implements!( instance1 => fmt::Display ) );

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = Many::< f32 >::from( 13.0 );
    assert_eq!( instance1.0, vec!( 13.0 ) );
    assert_eq!( instance2.0, vec!( 13.0 ) );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Many< f32 > = ( Many::from( 13.0 ) ).into();
    let instance2 = Many::< f32 >::from( Many::from( 13.0 ) );
    assert_eq!( instance1.0, vec!( 13.0 ) );
    assert_eq!( instance2.0, vec!( 13.0 ) );
    assert_eq!( instance1, instance2 );

    /* test.case( "from tuple" ) */
    let got : Many< f32 > = ( 13.0, ).into();
    assert_eq!( got, Many::from( 13.0 ) );
    let got = Many::< f32 >::from( ( 13.0, ) );
    assert_eq!( got, Many::from( 13.0 ) );

    /* test.case( "from array" ) */
    let got : Many< f32 > = [ 13.0 ].into();
    assert_eq!( got, Many::from( 13.0 ) );
    let got = Many::< f32 >::from( [ 13.0 ] );
    assert_eq!( got, Many::from( 13.0 ) );

    /* test.case( "from slice" ) */
    let got : Many< f32 > = (&[ 13.0 ][ .. ]).into();
    assert_eq!( got, Many::from( 13.0 ) );
    let got = Many::< f32 >::from( (&[ 13.0 ][ .. ]) );
    assert_eq!( got, Many::from( 13.0 ) );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, vec!( 13.0 ) );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many< f32 > = ( 13.0 ).into();
    assert_eq!( got.len(), 1 );
    assert_eq!( got.pop(), Some( 13.0 ) );
    assert_eq!( got.0, std::vec::Vec::< f32 >::new() );

    /* test.case( "as_slice" ) */
    let src : Many< f32 > = ( 13.0, ).into();
    let got = src.as_slice();
    assert_eq!( got, &[ 13.0, ][ .. ] );
    assert!( !mem_same_ptr( &src, got ) );
    let got = &src[ .. ];
    assert_eq!( got, &[ 13.0, ][ .. ] );
    assert!( !mem_same_ptr( &src, got ) );

  }

  //

  #[ test ]
  fn parameter_complex_test()
  {

    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      many Many : < T : core::cmp::PartialEq + core::clone::Clone >;
    }

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = Many::< f32 >::from( 13.0 );
    assert_eq!( instance1.0, vec![ 13.0 ] );
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Many< f32 > = ( Many::from( 13.0 ) ).into();
    let instance2 = Many::< f32 >::from( Many::from( 13.0 ) );
    assert_eq!( instance1.0, vec![ 13.0 ] );
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many< f32 > = ( 13.0 ).into();
    assert_eq!( got.len(), 1 );
    assert_eq!( got.pop(), Some( 13.0 ) );
    assert_eq!( got.0, std::vec::Vec::< f32 >::new() );

  }

  //

  #[ test ]
  fn parameter_no_derives_test()
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
      many Many : < T >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Many( vec![ mod1::Floats( 13.0, 31.0 ) ] );

  }

  //

  #[ test ]
  fn multiple_test()
  {
    use core::fmt;

    types!
    {

      many Many1 : f32;

      #[ derive( Debug ) ]
      #[ derive( PartialEq, Clone ) ]
      many Many2 : f32;

    }

    /* test.case( "from f32 / into Many2" ) */
    let instance1 : Many1 = ( 13.0 ).into();
    let instance2 = Many1::from( 13.0 );
    assert_eq!( instance1.0, vec![ 13.0 ] );
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert!( !implements!( instance1 => PartialEq ) );
    assert!( !implements!( instance1 => Clone ) );
    assert!( !implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from f32 / into Many2" ) */
    let instance1 : Many2 = ( 13.0 ).into();
    let instance2 = Many2::from( 13.0 );
    assert_eq!( instance1.0, vec![ 13.0 ] );
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "clone / eq" ) */
    let instance1 : Many2 = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );

  }

  //

  #[ test ]
  fn struct_basic_test()
  {

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = Many::< f32 >::from( 13.0 );
    assert_eq!( instance1.0, vec![ 13.0 ] );
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Many< f32 > = ( Many::from( 13.0 ) ).into();
    let instance2 = Many::< f32 >::from( Many::from( 13.0 ) );
    assert_eq!( instance1.0, vec![ 13.0 ] );
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, vec![ 13.0 ] );
    assert_eq!( instance1, instance2 );

    /* test.case( "default" ) */
    let instance1 : Many< f32 > = Default::default();
    assert_eq!( instance1.0, std::vec::Vec::< f32 >::new() );

    /* test.case( "deref" ) */
    let mut got : Many< f32 > = ( 13.0 ).into();
    assert_eq!( got.len(), 1 );
    assert_eq!( got.pop(), Some( 13.0 ) );

  }

  //

  #[ test ]
  fn struct_no_derives_test()
  {

    mod mod1
    {
      pub struct Floats< T >( pub T );
      impl< T > Floats< T >
      {
        pub fn new( src : T ) -> Self
        { Self( src ) }
      }
    }

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many< mod1::Floats< f32 > > = ( mod1::Floats( 13.0 ) ).into();
    let instance2 = Many::< mod1::Floats< f32 > >::from( mod1::Floats( 13.0 ) );
    assert_eq!( instance1.0[ 0 ].0, 13.0 );
    assert_eq!( instance1.len(), 1 );
    assert_eq!( instance2.0[ 0 ].0, 13.0 );
    assert_eq!( instance2.len(), 1 );

    /* test.case( "deref" ) */
    let mut got : Many< f32 > = ( 13.0 ).into();
    assert_eq!( got.len(), 1 );
    assert_eq!( got.pop(), Some( 13.0 ) );

  }

//   //
//
//   #[ test ]
//   fn samples_test()
//   {
//
//     /* test.case( "many-line" ) */
//     {
//       types!( many MyMany : i32 );
//       let x = MyMany( 13 );
//       println!( "x : {}", x.0 );
//     }
//
//     /* test.case( "derives and attributes" ) */
//     {
//       types!
//       {
//         /// This is also attribute and macro understands it.
//         #[ derive( Debug ) ]
//         many MyMany : i32;
//       }
//       let x = MyMany( 13 );
//       dbg!( x );
//     }
//
//     /* test.case( "struct instead of macro" ) */
//     {
//       let x = Many::< i32 >( 13 );
//       dbg!( x );
//     }
//
//     /* test.case( "parametrized element" ) */
//     {
//       types!
//       {
//         #[ derive( Debug ) ]
//         many MyMany : std::sync::Arc< T : Copy >;
//       }
//       let x = MyMany( std::sync::Arc::new( 13 ) );
//       dbg!( x );
//     }
//
//     /* test.case( "parametrized tuple" ) */
//     {
//       types!
//       {
//         #[ derive( Debug ) ]
//         many MyMany : < T : Copy >;
//       }
//       let x = MyMany( 13 );
//       dbg!( x );
//     }
//
//   }

}

//

tests_index!
{

  basic_test,
  empty_parameter_test,
  parametrized_multiple_test,
  parametrized_no_derives_test,
  parameter_test,
  parameter_complex_test,
  parameter_no_derives_test,
  multiple_test,
  struct_basic_test,
  struct_no_derives_test,
  // samples_test,

}
