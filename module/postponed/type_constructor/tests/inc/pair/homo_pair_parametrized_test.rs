#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{
  fn basic()
  {
    use core::fmt;

    mod mod1
    {
      pub use f32;
    }

    trait Round { fn round( &self ) -> Self; };
    impl Round for ( f32, f32 )
    {
      fn round( &self ) -> Self
      {
        ( self.0.round(), self.1.round() )
      }
    }

    trait RoundInplace { fn round_inplace( &mut self ); };
    impl RoundInplace for ( f32, f32 )
    {
      fn round_inplace( &mut self )
      {
        self.0 = self.0.round();
        self.1 = self.1.round();
      }
    }

    // trace_macros!( true );
    the_module::types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      pair Pair : mod1::f32;

    }
    // trace_macros!( false );

    /* test.case( "from array into pair" ) */
    let instance1 : Pair = [ 13.0, 31.0 ].into();
    let instance2 = Pair::from( [ 13.0, 31.0 ] );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from pair into array" ) */
    let instance1 : [ _ ; 2 ] = ( Pair::from( [ 13.0, 31.0 ] ) ).into();
    let instance2 = < [ _ ; 2] >::from( Pair::from( [ 13.0, 31.0 ] ) );
    a_id!( instance1[ 0 ], 13.0 );
    a_id!( instance1[ 1 ], 31.0 );
    a_id!( instance2[ 0 ], 13.0 );
    a_id!( instance2[ 1 ], 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from slice into pair" ) */
    let instance1 : Pair = ( &[ 13.0, 31.0 ][ .. ] ).into();
    let instance2 = Pair::from( ( &[ 13.0, 31.0 ][ .. ] ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from tuple into pair" ) */
    let instance1 : Pair = ( 13.0, 31.0 ).into();
    let instance2 = Pair::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from pair into tuple" ) */
    let instance1 : ( _, _ ) = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = < ( _, _ ) >::from( Pair::from( ( 13.0, 31.0 ) ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Pair = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = Pair::from( Pair::from( ( 13.0, 31.0 ) ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair = ( 13.0, 31.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Pair = ( 13.5, 31.5 ).into();
    a_id!( got.round(), ( 14.0, 32.0 ) );
    got.round_inplace();
    a_id!( got, Pair::from( ( 14.0, 32.0 ) ) );

  }

  //

  fn parametrized_multiple()
  {
    use the_module::
    {
      CloneAsTuple,
      CloneAsArray,
      AsTuple,
      AsArray,
      AsSlice,
    };

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
    the_module::types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      pair Pair :
        mod1::Floats< T1 : PartialEq + std::marker::Copy, T2 : Default >,
      ;
    }
    // trace_macros!( false );

    pub trait Round { fn round( &self ) -> Self; }
    impl Round
    for mod1::Floats< f32, f64 >
    {
      fn round( &self ) -> Self
      {
        mod1::Floats( self.0.round(), self.1.round() )
      }
    }
    impl Round
    for ( mod1::Floats< f32, f64 >, mod1::Floats< f32, f64 > )
    {
      fn round( &self ) -> Self
      {
        ( self.0.round(), self.1.round() )
      }
    }

    trait RoundInplace { fn round_inplace( &mut self ); };
    impl RoundInplace for mod1::Floats< f32, f64 >
    {
      fn round_inplace( &mut self )
      {
        self.0 = self.0.round();
        self.1 = self.1.round();
      }
    }
    impl RoundInplace for ( mod1::Floats< f32, f64 >, mod1::Floats< f32, f64 > )
    {
      fn round_inplace( &mut self )
      {
        self.0 = self.0.round();
        self.1 = self.1.round();
      }
    }

    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      /* test.case( "make1" ) */
      let got : Pair< f32, f64 > = the_module::from!( mk!( 13.0 ) );
      let exp = Pair::< f32, f64 >::from( ( mk!( 13.0 ), mk!( 13.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make2" ) */
      let got : Pair< f32, f64 > = the_module::from!( mk!( 13.0 ), mk!( 31.0 ) );
      let exp = Pair::< f32, f64 >::from( ( mk!( 13.0 ), mk!( 31.0 ) ) );
      a_id!( got, exp );
    }

    /* test.case( "from tuple into pair" ) */
    let instance1 : Pair< f32, f64 > =
    (
      mk!( 13.0 ),
      mk!( 31.0 ),
    ).into();
    let instance2 = Pair::< f32, f64 >::from
    ((
      mk!( 13.0 ),
      mk!( 31.0 ),
    ));
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Pair into tuple" ) */
    let instance1 : Pair< f32, f64 > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got : ( mod1::Floats< f32, f64 >, _ ) = instance1.into();
    a_id!( got.0.0, 13.0 );
    let instance1 : Pair< f32, f64 > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = < ( mod1::Floats::< f32, f64 >, _ ) >::from( instance1 );
    a_id!( got.0.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair< f32, f64 > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Pair< f32, f64 > = ( mk!( 13.5 ), mk!( 31.5 ) ).into();
    a_id!( got.round(), ( mk!( 14.0 ), mk!( 32.0 ) ) );
    got.round_inplace();
    a_id!( got, Pair::from( ( mk!( 14.0 ), mk!( 32.0 ) ) ) );

    /* test.case( "clone_as_tuple" ) */
    let src : Pair< f32, f64 > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = src.clone_as_tuple();
    a_id!( got, ( mk!( 13.0 ), mk!( 31.0 ) ) );
    assert!( !mem::same_ptr( &src, &got ) );

    /* test.case( "clone_as_array" ) */
    let src : Pair< f32, f64 > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = src.clone_as_array();
    a_id!( got, [ mk!( 13.0 ), mk!( 31.0 ) ] );
    assert!( !mem::same_ptr( &src, &got ) );

    /* test.case( "as_tuple" ) */
    let src : Pair< f32, f64 > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = src.as_tuple();
    a_id!( got, &( mk!( 13.0 ), mk!( 31.0 ) ) );
    assert!( mem::same_region( &src, got ) );

    /* test.case( "as_array" ) */
    let src : Pair< f32, f64 > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = src.as_array();
    a_id!( got, &[ mk!( 13.0 ), mk!( 31.0 ) ] );
    assert!( mem::same_region( &src, got ) );

    /* test.case( "as_slice" ) */
    let src : Pair< f32, f64 > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = src.as_slice();
    a_id!( got, &[ mk!( 13.0 ), mk!( 31.0 ) ][ .. ] );
    assert!( mem::same_region( &src, got ) );
  }

  //

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
    the_module::types!
    {
      pair Pair : mod1::Floats< T1, T2 >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Pair( mod1::Floats( 13.0, 31.0 ), mod1::Floats( 13.0, 31.0 ) );
  }
}

//

tests_index!
{
  basic,
  parametrized_multiple,
  parametrized_no_derives,
}
