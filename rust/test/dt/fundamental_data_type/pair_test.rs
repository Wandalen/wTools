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

    trait Round { fn round( &self ) -> Self; };
    impl Round for ( f32, f64 )
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
      pair Pair : mod1::f32, mod1::f64;

    }
    // trace_macros!( false );

    /* test.case( "from tuple / into Pair" ) */
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

    // /* test.case( "deref" ) */
    // let got : Pair = ( 13.5, 31.5 ).into();
    // assert_eq!( got.round(), ( 14.0, 32.0 ) );

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

    trait Round { fn round( &self ) -> Self; };
    impl Round for ( f32, f64 )
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
      pair Pair : mod1::f32<>, mod1::f64<>;

    }
    // trace_macros!( false );

    /* test.case( "from tuple / into Pair" ) */
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

    // struct Struct1( i32, i32 );
    // let e = Struct1( 1, 2 );
    // let e : Struct1 = ( 1, 2 );

    // /* test.case( "deref" ) */
    // let got : Pair = ( 13.5, 31.5 ).into();
    // assert_eq!( got.round(), ( 14.0, 32.0 ) );

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
        std::sync::Arc< T : Copy >,
      ;

    }
    // trace_macros!( false );

    /* test.case( "from tuple / into Pair" ) */
    let instance1 : Pair< f32, f64, f32 > =
    (
      mod1::Pair0::from( 13.0 ),
      std::sync::Arc::new( 31.0 ),
    ).into();
    let instance2 = Pair::< f32, f64, f32 >::from
    ((
      mod1::Pair0::from( 13.0 ),
      std::sync::Arc::new( 31.0 ),
    ));
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance2.0.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Pair / into tuple" ) */
    let instance1 : Pair< f32, f64, f32 > = ( mod1::Pair0::from( 13.0 ), std::sync::Arc::new( 31.0 ) ).into();
    let got : ( mod1::Pair0< f32, f64 >, _ ) = instance1.into();
    assert_eq!( got.0.0, 13.0 );
    let instance1 : Pair< f32, f64, f32 > = ( mod1::Pair0::from( 13.0 ), std::sync::Arc::new( 31.0 ) ).into();
    let got = < ( mod1::Pair0::< f32, f64 >, _ ) >::from( instance1 );
    assert_eq!( got.0.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair< f32, f64, f32 > = ( mod1::Pair0::from( 13.0 ), std::sync::Arc::new( 31.0 ) ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, mod1::Pair0::from( 13.0 ) );
    assert_eq!( instance1, instance2 );

    // /* test.case( "deref" ) */
    // let got : Pair< f32, f64, f32 > = ( mod1::Pair0::from( 13.5 ), std::sync::Arc::new( 31.0 ) ).into();
    // assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn parametrized_mixed_test()
  {

    /* test.case( "control case" ) */
    {

      // trace_macros!( true );
      types!
      {

        ///
        /// Attribute which is inner.
        ///

        #[ derive( Debug, Clone ) ]
        #[ derive( PartialEq ) ]
        pair Pair :
          std::sync::Arc< T : Copy >,
          f32<>,
        ;

      }
      // trace_macros!( false );

      let instance1 : Pair< f64 > =
      (
        std::sync::Arc::new( 13.0 ),
        31.0,
      ).into();
      let instance2 = Pair::< f64 >::from
      ((
        std::sync::Arc::new( 13.0 ),
        31.0,
      ));
      assert_eq!( instance1, instance2 );

    }

    /* test.case( "second without <> with comma" ) */
    {

      // trace_macros!( true );
      types!
      {

        ///
        /// Attribute which is inner.
        ///

        #[ derive( Debug, Clone ) ]
        #[ derive( PartialEq ) ]
        pair Pair :
          std::sync::Arc< T : Copy >,
          f32,
        ;

      }
      // trace_macros!( false );

      let instance1 : Pair< f64 > =
      (
        std::sync::Arc::new( 13.0 ),
        31.0,
      ).into();
      let instance2 = Pair::< f64 >::from
      ((
        std::sync::Arc::new( 13.0 ),
        31.0,
      ));
      assert_eq!( instance1, instance2 );

    }

    /* test.case( "second without <> without comma" ) */
    {

      // trace_macros!( true );
      types!
      {

        ///
        /// Attribute which is inner.
        ///

        #[ derive( Debug, Clone ) ]
        #[ derive( PartialEq ) ]
        pair Pair :
          std::sync::Arc< T : Copy >,
          f32
        ;

      }
      // trace_macros!( false );

      let instance1 : Pair< f64 > =
      (
        std::sync::Arc::new( 13.0 ),
        31.0,
      ).into();
      let instance2 = Pair::< f64 >::from
      ((
        std::sync::Arc::new( 13.0 ),
        31.0,
      ));
      assert_eq!( instance1, instance2 );

    }

    /* test.case( "first without <> with comma" ) */
    {

      // trace_macros!( true );
      types!
      {

        ///
        /// Attribute which is inner.
        ///

        #[ derive( Debug, Clone ) ]
        #[ derive( PartialEq ) ]
        pair Pair :
          f32,
          std::sync::Arc< T : Copy >,
        ;

      }
      // trace_macros!( false );

      let instance1 : Pair< f64 > =
      (
        31.0,
        std::sync::Arc::new( 13.0 ),
      ).into();
      let instance2 = Pair::< f64 >::from
      ((
        31.0,
        std::sync::Arc::new( 13.0 ),
      ));
      assert_eq!( instance1, instance2 );

    }

    /* test.case( "first without <> without comma" ) */
    {

      // trace_macros!( true );
      types!
      {

        ///
        /// Attribute which is inner.
        ///

        #[ derive( Debug, Clone ) ]
        #[ derive( PartialEq ) ]
        pair Pair :
          f32,
          std::sync::Arc< T : Copy >
        ;

      }
      // trace_macros!( false );

      let instance1 : Pair< f64 > =
      (
        31.0,
        std::sync::Arc::new( 13.0 ),
      ).into();
      let instance2 = Pair::< f64 >::from
      ((
        31.0,
        std::sync::Arc::new( 13.0 ),
      ));
      assert_eq!( instance1, instance2 );

    }

  }

  //

  #[ test ]
  fn parameter_complex_test()
  {

    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      pair Pair : < T1 : core::cmp::PartialEq + core::clone::Clone, T2 : core::cmp::PartialEq + core::clone::Clone >;

    }

    /* test.case( "from tuple / into Pair" ) */
    let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = Pair::< f32, f64 >::from( ( 13.0, 31.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Pair / into tuple" ) */
    let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = Pair::< f32, f64 >::from( ( 13.0, 31.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Pair< f32, f64 > = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = Pair::< f32, f64 >::from( Pair::from( ( 13.0, 31.0 ) ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

//     /* test.case( "deref" ) */
//     let got : Pair< f32, f64 > = ( 13.5 ).into();
//     assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn multiple_test()
  {
    use core::fmt::Debug;

    types!
    {

      pair Pair1 : f64, f32;

      #[ derive( Debug ) ]
      #[ derive( PartialEq, Clone ) ]
      pair Pair2 : f32, f64;

    }

    /* test.case( "from tuple / into Pair2" ) */
    let instance1 : Pair1 = ( 13.0, 31.0 ).into();
    let instance2 = Pair1::from( ( 13.0, 31.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert!( !implements!( instance1 => PartialEq ) );
    assert!( !implements!( instance1 => Clone ) );
    assert!( !implements!( instance1 => Debug ) );

    // xxx : redo implements

    /* test.case( "from tuple / into Pair2" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let instance2 = Pair2::from( ( 13.0, 31.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from tuple / into Pair2" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let instance2 = Pair2::from( ( 13.0, 31.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Pair2 = ( Pair2::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = Pair2::from( Pair2::from( ( 13.0, 31.0 ) ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Pair2 / into tuple" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let got : ( _, _ ) = instance1.into();
    assert_eq!( got, ( 13.0, 31.0 ) );
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let got = <( f32, f64 )>::from( instance1 );
    assert_eq!( got, ( 13.0, 31.0 ) );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    // /* test.case( "deref" ) */
    // let got : Pair2 = ( 13.5, 15.5 ).into();
    // assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn struct_basic_test()
  {

    /* test.case( "from tuple / into Pair" ) */
    let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = Pair::< f32, f64 >::from( ( 13.0, 31.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Pair / into tuple" ) */
    let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = Pair::< f32, f64 >::from( ( 13.0, 31.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Pair< f32, f64 > = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = Pair::< f32, f64 >::from( Pair::from( ( 13.0, 31.0 ) ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance1.1, 31.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance2.1, 31.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "default" ) */
    let instance1 : Pair< f32, f64 > = Default::default();
    assert_eq!( instance1.0, 0.0 );
    assert_eq!( instance1.1, 0.0 );

//     /* test.case( "deref" ) */
//     let got : Pair< f32, f64 > = ( 13.5 ).into();
//     assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn struct_deaf_test()
  {

    struct Pair0< T1, T2 >( pub T1, pub T2 );

    impl< T1, T2 > Pair0< T1, T2 >
    {
      pub fn new( src : ( T1, T2 ) ) -> Self
      { Self( src.0, src.1 ) }
    }

    /* test.case( "from tuple / into Pair" ) */
    let instance1 : Pair< Pair0< f32, f64 >, f32 > = ( Pair0( 13.0, 31.0 ), 131.0 ).into();
    let instance2 = Pair::< Pair0< f32, f64 >, f32 >::from( ( Pair0( 13.0, 31.0 ), 131.0 ) );
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance1.0.1, 31.0 );
    assert_eq!( instance1.1, 131.0 );
    assert_eq!( instance2.0.0, 13.0 );
    assert_eq!( instance2.0.1, 31.0 );
    assert_eq!( instance2.1, 131.0 );

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
  empty_parameter_test,
  parametrized_multiple_test,
  parametrized_mixed_test,
  parameter_complex_test,
  multiple_test,
  struct_basic_test,
  struct_deaf_test,
  // samples_test,

}
