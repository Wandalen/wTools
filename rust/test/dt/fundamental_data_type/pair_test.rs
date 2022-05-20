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
      pub use f64;
    }

    // trace_macros!( true );
    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      pair Pair : mod1::f32, mod1::f64;

    }
    // trace_macros!( false );

    trait Round { fn round( &self ) -> Self; };
    impl Round for Pair
    {
      fn round( &self ) -> Self
      {
        Pair( self.0.round(), self.1.round() )
      }
    }

    /* test.case( "from f32 / into Pair" ) */
    let instance1 : Pair = ( 13.0, 31.0 ).into();
    let instance2 = Pair::from( ( 13.0, 31.0 ) );
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
    assert_eq!( got.round(), Pair( 14.0, 32.0 ) );

  }

  //

  #[ test ]
  fn empty_parameter_test()
  {

    mod mod1
    {
      pub use f32;
      pub use f64;
    }

    // trace_macros!( true );
    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      pair Pair : mod1::f32<>, mod1::f64<>;

    }
    // trace_macros!( false );

    trait Round { fn round( &self ) -> Self; };
    impl Round for Pair
    {
      fn round( &self ) -> Self
      {
        Pair( self.0.round(), self.1.round() )
      }
    }

    /* test.case( "from f32 / into Pair" ) */
    let instance1 : Pair = ( 13.0, 31.0 ).into();
    let instance2 = Pair::from( ( 13.0, 31.0 ) );
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
    assert_eq!( got.round(), Pair( 14.0, 32.0 ) );

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

//     types!
//     {
//
//       ///
//       /// Attribute which is inner.
//       ///
//
//       #[ derive( Debug, Clone ) ]
//       #[ derive( PartialEq ) ]
//       pair Pair : mod1::Pair0< T1 : PartialEq + std::marker::Copy, T2 : Default >;
//
//     }

//     /* test.case( "from f32 / into Pair" ) */
//     let instance1 : Pair< f32, f64 > = ( mod1::Pair0::from( ( 13.0, 31.0 ) ) ).into();
//     let instance2 = Pair::< f32, f64 >::from( mod1::Pair0::from( ( 13.0, 31.0 ) ) );
//     assert_eq!( instance1.0.0, 13.0 );
//     assert_eq!( instance2.0.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from itself / into itself" ) */
//     let instance1 : Pair< f32, f64 > = ( Pair::from( mod1::Pair0::from( ( 13.0, 31.0 ) ) ) ).into();
//     let instance2 = Pair::< f32, f64 >::from( Pair::from( mod1::Pair0::from( ( 13.0, 31.0 ) ) ) );
//     assert_eq!( instance1.0.0, 13.0 );
//     assert_eq!( instance2.0.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from Pair / into f32" ) */
//     let instance1 : Pair< f32, f64 > = ( mod1::Pair0::from( ( 13.0, 31.0 ) ) ).into();
//     let got : mod1::Pair0< f32, f64 > = instance1.into();
//     assert_eq!( got.0, 13.0 );
//     let instance1 : Pair< f32, f64 > = ( mod1::Pair0::from( ( 13.0, 31.0 ) ) ).into();
//     let got = mod1::Pair0::< f32, f64 >::from( instance1 );
//     assert_eq!( got.0, 13.0 );
//
//     /* test.case( "clone / eq" ) */
//     let instance1 : Pair< f32, f64 > = ( mod1::Pair0::from( ( 13.0, 31.0 ) ) ).into();
//     let instance2 = instance1.clone();
//     assert_eq!( instance2.0, mod1::Pair0::from( ( 13.0, 31.0 ) ) );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "deref" ) */
//     let got : Pair< f32, f64 > = ( mod1::Pair0::from( 13.5 ) ).into();
//     assert_eq!( got.round(), 14.0 );

  }

  //

//   fn parameter_test()
//   {
//
//     types!
//     {
//
//       ///
//       /// Attribute which is inner.
//       ///
//
//       #[ derive( Debug, Clone ) ]
//       #[ derive( PartialEq, Default ) ]
//       pair Pair : < T >;
//
//     }
//
//     /* test.case( "from f32 / into Pair" ) */
//     let instance1 : Pair< f32 > = ( 13.0, 31.0 ).into();
//     let instance2 = Pair::< f32 >::from( ( 13.0, 31.0 ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from itself / into itself" ) */
//     let instance1 : Pair< f32 > = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
//     let instance2 = Pair::< f32 >::from( Pair::from( ( 13.0, 31.0 ) ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "clone / eq" ) */
//     let instance1 : Pair< f32 > = ( 13.0, 31.0 ).into();
//     let instance2 = instance1.clone();
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "deref" ) */
//     let got : Pair< f32 > = ( 13.5 ).into();
//     assert_eq!( got.round(), 14.0 );
//
//   }
//
//   //
//
//   #[ test ]
//   fn parameter_complex_test()
//   {
//
//     types!
//     {
//
//       ///
//       /// Attribute which is inner.
//       ///
//
//       #[ derive( Debug, Clone ) ]
//       #[ derive( PartialEq ) ]
//       pair Pair : < T : core::cmp::PartialEq + core::clone::Clone >;
//
//     }
//
//     /* test.case( "from f32 / into Pair" ) */
//     let instance1 : Pair< f32 > = ( 13.0, 31.0 ).into();
//     let instance2 = Pair::< f32 >::from( ( 13.0, 31.0 ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from itself / into itself" ) */
//     let instance1 : Pair< f32 > = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
//     let instance2 = Pair::< f32 >::from( Pair::from( ( 13.0, 31.0 ) ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "clone / eq" ) */
//     let instance1 : Pair< f32 > = ( 13.0, 31.0 ).into();
//     let instance2 = instance1.clone();
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "deref" ) */
//     let got : Pair< f32 > = ( 13.5 ).into();
//     assert_eq!( got.round(), 14.0 );
//
//   }
//
//   //
//
//   #[ test ]
//   fn multiple_test()
//   {
//     use core::fmt::Debug;
//
//     types!
//     {
//
//       pair Single1 : f32;
//
//       #[ derive( Debug ) ]
//       #[ derive( PartialEq, Clone ) ]
//       pair Single2 : f32;
//
//     }
//
//     /* test.case( "from f32 / into Single2" ) */
//     let instance1 : Single1 = ( 13.0, 31.0 ).into();
//     let instance2 = Single1::from( ( 13.0, 31.0 ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert!( !implements!( instance1 => PartialEq ) );
//     assert!( !implements!( instance1 => Clone ) );
//     assert!( !implements!( instance1 => Debug ) );
//
//     // xxx : redo implements
//
//     /* test.case( "from f32 / into Single2" ) */
//     let instance1 : Single2 = ( 13.0, 31.0 ).into();
//     let instance2 = Single2::from( ( 13.0, 31.0 ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from f32 / into Single2" ) */
//     let instance1 : Single2 = ( 13.0, 31.0 ).into();
//     let instance2 = Single2::from( ( 13.0, 31.0 ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from itself / into itself" ) */
//     let instance1 : Single2 = ( Single2::from( ( 13.0, 31.0 ) ) ).into();
//     let instance2 = Single2::from( Single2::from( ( 13.0, 31.0 ) ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from Single2 / into f32" ) */
//     let instance1 : Single2 = ( 13.0, 31.0 ).into();
//     let got : f32 = instance1.into();
//     assert_eq!( got, 13.0 );
//     let instance1 : Single2 = ( 13.0, 31.0 ).into();
//     let got = f32::from( instance1 );
//     assert_eq!( got, 13.0 );
//
//     /* test.case( "clone / eq" ) */
//     let instance1 : Single2 = ( 13.0, 31.0 ).into();
//     let instance2 = instance1.clone();
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "deref" ) */
//     let got : Single2 = ( 13.5 ).into();
//     assert_eq!( got.round(), 14.0 );
//
//   }
//
//   //
//
//   #[ test ]
//   fn struct_basic_test()
//   {
//
//     /* test.case( "from f32 / into Pair" ) */
//     let instance1 : Pair< f32 > = ( 13.0, 31.0 ).into();
//     let instance2 = Pair::< f32 >::from( ( 13.0, 31.0 ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "from itself / into itself" ) */
//     let instance1 : Pair< f32 > = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
//     let instance2 = Pair::< f32 >::from( Pair::from( ( 13.0, 31.0 ) ) );
//     assert_eq!( instance1.0, 13.0 );
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "clone / eq" ) */
//     let instance1 : Pair< f32 > = ( 13.0, 31.0 ).into();
//     let instance2 = instance1.clone();
//     assert_eq!( instance2.0, 13.0 );
//     assert_eq!( instance1, instance2 );
//
//     /* test.case( "deref" ) */
//     let got : Pair< f32 > = ( 13.5 ).into();
//     assert_eq!( got.round(), 14.0 );
//
//   }
//
//   //
//
//   #[ test ]
//   fn struct_deaf_test()
//   {
//
//     struct Pair0< T >( pub T );
//
//     impl< T > Pair0< T >
//     {
//       pub fn new( src : T ) -> Self
//       { Self( src ) }
//     }
//
//     /* test.case( "from f32 / into Pair" ) */
//     let instance1 : Pair< Pair0< f32 > > = ( Pair0( 13.0, 31.0 ) ).into();
//     let instance2 = Pair::< Pair0< f32 > >::from( Pair0( 13.0, 31.0 ) );
//     assert_eq!( instance1.0.0, 13.0 );
//     assert_eq!( instance2.0.0, 13.0 );
//
//     /* test.case( "from itself / into itself" ) */
//     let val = Pair0::< f32 >::new( 13.0, 31.0 );
//     let instance1 : Pair< Pair0< f32 > > = ( Pair::from( val ) ).into();
//     let instance2 = Pair::< Pair0< f32 > >::from( Pair::from( Pair0( 13.0, 31.0 ) ) );
//     assert_eq!( instance1.0.0, 13.0 );
//     assert_eq!( instance2.0.0, 13.0 );
//
//     /* test.case( "deref" ) */
//     let got : Pair< f32 > = ( 13.5 ).into();
//     assert_eq!( got.round(), 14.0 );
//
//   }
//
//   //
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
  empty_parameter_test,
  parametrized_multiple_test,
  // parameter_test,
  // parameter_complex_test,
  // multiple_test,
  // struct_basic_test,
  // struct_deaf_test,
  // samples_test,

}
