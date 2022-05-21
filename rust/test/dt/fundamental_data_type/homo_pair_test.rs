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

    mod mod1
    {
      pub use f32;
    }

    trait Round { fn round( &self ) -> Self; };
    impl Round for ( f32, f32 )
    {
      fn round( &self ) -> Self
      {
        dbg!( &self );
        ( self.0.round(), self.1.round() )
      }
    }

    // trace_macros!( true );
    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      pair Pair : mod1::f32;

    }
    // trace_macros!( false );

    /* test.case( "from array / into pair" ) */
    let instance1 : Pair = [ 13.0, 31.0 ].into();
    let instance2 = Pair::from( [ 13.0, 31.0 ] );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from pair / into array" ) */
    let instance1 : [ _ ; 2 ] = ( Pair::from( [ 13.0, 31.0 ] ) ).into();
    let instance2 = < [ _ ; 2] >::from( Pair::from( [ 13.0, 31.0 ] ) );
    assert_eq!( instance1[ 0 ], 13.0 );
    assert_eq!( instance1[ 1 ], 31.0 );
    assert_eq!( instance2[ 0 ], 13.0 );
    assert_eq!( instance2[ 1 ], 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from slice / into pair" ) */
    let instance1 : Pair = ( &[ 13.0, 31.0 ][ .. ] ).into();
    let instance2 = Pair::from( ( &[ 13.0, 31.0 ][ .. ] ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from tuple / into pair" ) */
    let instance1 : Pair = ( 13.0, 31.0 ).into();
    let instance2 = Pair::from( ( 13.0, 31.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from pair / into tuple" ) */
    let instance1 : ( _, _ ) = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = < ( _, _ ) >::from( Pair::from( ( 13.0, 31.0 ) ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Pair = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = Pair::from( Pair::from( ( 13.0, 31.0 ) ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair = ( 13.0, 31.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Pair = ( 13.5, 31.5 ).into();
    assert_eq!( got.round(), ( 14.0, 32.0 ) );

  }

  //

  #[ test ]
  fn parametrized_multiple_test()
  {

    mod mod1
    {

      #[ derive( Debug, Clone, PartialEq ) ]
      pub struct Pair0< T1 : PartialEq + Copy, T2 : Default >
      (
        pub T1,
        pub T2,
      );

      impl< T1 : PartialEq + Copy, T2 : Default > core::ops::Deref
      for Pair0< T1, T2 >
      {
        type Target = T1;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< T1 : PartialEq + Copy, T2 : Default > From< T1 >
      for Pair0< T1, T2 >
      {
        fn from( src : T1 ) -> Self
        {
          Pair0::< T1, T2 >( src, T2::default() )
        }
      }

    }

    // trace_macros!( true );
    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      pair Pair :
        mod1::Pair0< T1 : PartialEq + std::marker::Copy, T2 : Default >,
      ;

    }
    // trace_macros!( false );

    pub trait Round { fn round( &self ) -> Self; }
    impl Round
    for mod1::Pair0< f32, f64 >
    {
      fn round( &self ) -> Self
      {
        mod1::Pair0( self.0.round(), self.1.round() )
      }
    }

    impl Round
    for ( mod1::Pair0< f32, f64 >, mod1::Pair0< f32, f64 > )
    {
      fn round( &self ) -> Self
      {
        ( self.0.round(), self.1.round() )
      }
    }

    /* test.case( "from tuple / into pair" ) */
    let instance1 : Pair< f32, f64 > =
    (
      mod1::Pair0::from( 13.0 ),
      mod1::Pair0::from( 31.0 ),
    ).into();
    let instance2 = Pair::< f32, f64 >::from
    ((
      mod1::Pair0::from( 13.0 ),
      mod1::Pair0::from( 31.0 ),
    ));
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance2.0.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Pair / into tuple" ) */
    let instance1 : Pair< f32, f64 > = ( mod1::Pair0::from( 13.0 ), mod1::Pair0::from( 31.0 ) ).into();
    let got : ( mod1::Pair0< f32, f64 >, _ ) = instance1.into();
    assert_eq!( got.0.0, 13.0 );
    let instance1 : Pair< f32, f64 > = ( mod1::Pair0::from( 13.0 ), mod1::Pair0::from( 31.0 ) ).into();
    let got = < ( mod1::Pair0::< f32, f64 >, _ ) >::from( instance1 );
    assert_eq!( got.0.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair< f32, f64 > = ( mod1::Pair0::from( 13.0 ), mod1::Pair0::from( 31.0 ) ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, mod1::Pair0::from( 13.0 ) );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Pair< f32, f64 > = ( mod1::Pair0::from( 13.5 ), mod1::Pair0::from( 31.5 ) ).into();
    assert_eq!( got.round(), ( mod1::Pair0::from( 14.0 ), mod1::Pair0::from( 32.0 ) ) );

  }

  //

  #[ test ]
  fn parameter_complex_test()
  {

//     types!
//     {
//
//       ///
//       /// Attribute which is inner.
//       ///
//
//       #[ derive( Debug, Clone ) ]
//       #[ derive( PartialEq ) ]
//       pair Pair : < T1 : core::cmp::PartialEq + core::clone::Clone >;
//
//     }

//     /* test.case( "from tuple / into pair" ) */
//     let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
//     let instance2 = Pair::< f32, f64 >::from( ( 13.0, 31.0 ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance1.1, 31.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance2.1, 31.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from Pair / into tuple" ) */
//     let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
//     let instance2 = Pair::< f32, f64 >::from( ( 13.0, 31.0 ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance1.1, 31.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance2.1, 31.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from itself / into itself" ) */
//     let instance1 : Pair< f32, f64 > = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
//     let instance2 = Pair::< f32, f64 >::from( Pair::from( ( 13.0, 31.0 ) ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance1.1, 31.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance2.1, 31.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "clone / eq" ) */
//     let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
//     let instance2 = instance1.clone();
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance1.1, 31.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance2.1, 31.0 );
//     assert_eq!( instance1, instance2 );
//
// //     /* test.case( "deref" ) */
// //     let got : Pair< f32, f64 > = ( 13.5 ).into();
// //     assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn struct_basic_test()
  {

//     /* test.case( "from tuple / into pair" ) */
//     let instance1 : HomoPair< f32 > = ( 13.0, 31.0 ).into();
//     let instance2 = HomoPair::< f32, f64 >::from( ( 13.0, 31.0 ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance1.1, 31.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance2.1, 31.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from Pair / into tuple" ) */
//     let instance1 : HomoPair< f32 > = ( 13.0, 31.0 ).into();
//     let instance2 = HomoPair::< f32, f64 >::from( ( 13.0, 31.0 ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance1.1, 31.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance2.1, 31.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from itself / into itself" ) */
//     let instance1 : HomoPair< f32 > = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
//     let instance2 = HomoPair::< f32, f64 >::from( Pair::from( ( 13.0, 31.0 ) ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance1.1, 31.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance2.1, 31.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "clone / eq" ) */
//     let instance1 : HomoPair< f32 > = ( 13.0, 31.0 ).into();
//     let instance2 = instance1.clone();
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance1.1, 31.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance2.1, 31.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "default" ) */
//     let instance1 : HomoPair< f32 > = Default::default();
//     assert_eq!( instance1.0, 0.0 );
//     assert_eq!( instance1.1, 0.0 );
//
// //     /* test.case( "deref" ) */
// //     let got : Pair< f32, f64 > = ( 13.5 ).into();
// //     assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn struct_deaf_test()
  {

//     struct Pair0< T1, T2 >( pub T1, pub T2 );
//
//     impl< T1, T2 > Pair0< T1, T2 >
//     {
//       pub fn new( src : ( T1, T2 ) ) -> Self
//       { Self( src.0, src.1 ) }
//     }
//
//     /* test.case( "from tuple / into pair" ) */
//     let instance1 : HomoPair< Pair0< f32, f64 > > = ( Pair0( 13.0, 31.0 ), 131.0 ).into();
//     let instance2 = HomoPair::< Pair0< f32, f64 > >::from( ( Pair0( 13.0, 31.0 ), 131.0 ) );
//     assert_eq!( instance1.0.0, 13.0 );
//     assert_eq!( instance1.0.1, 31.0 );
//     assert_eq!( instance1.1, 131.0 );
//     assert_eq!( instance2.0.0, 13.0 );
//     assert_eq!( instance2.0.1, 31.0 );
//     assert_eq!( instance2.1, 131.0 );

  }

  //

//   #[ test ]
//   fn samples_test()
//   {
//
//     /* test.case( "pair-line" ) */
//     {
//       types!( pair MySingle : i32 );
//       let x = MySingle( 13 );
//       println!( "x : {}", x.0 );
//     }
//
//     /* test.case( "derives and attributes" ) */
//     {
//       types!
//       {
//         /// This is also attribute and macro understands it.
//         #[ derive( Debug ) ]
//         pair MySingle : i32;
//       }
//       let x = MySingle( 13 );
//       dbg!( x );
//     }
//
//     /* test.case( "struct instead of macro" ) */
//     {
//       let x = Pair::< i32 >( 13 );
//       dbg!( x );
//     }
//
//     /* test.case( "parametrized element" ) */
//     {
//       types!
//       {
//         #[ derive( Debug ) ]
//         pair MySingle : std::sync::Arc< T : Copy >;
//       }
//       let x = MySingle( std::sync::Arc::new( 13 ) );
//       dbg!( x );
//     }
//
//     /* test.case( "parametrized tuple" ) */
//     {
//       types!
//       {
//         #[ derive( Debug ) ]
//         pair MySingle : < T : Copy >;
//       }
//       let x = MySingle( 13 );
//       dbg!( x );
//     }
//
//   }

}

//

tests_index!
{

  basic_test,
  parametrized_multiple_test,
  parameter_complex_test,
  struct_basic_test,
  struct_deaf_test,
  // samples_test,

}
