#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{
  fn no_parameter_no_derive()
  {

    mod mod1
    {
      #[ derive( Default, Clone ) ]
      pub struct Float
      (
        pub f32,
      );
    }

    // trace_macros!( true );
    the_module::types!
    {
      pair Pair : mod1::Float, mod1::Float;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Pair( mod1::Float( 13.0 ), mod1::Float( 31.0 ) );

  }

  //

  fn parameter_with_derives()
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
        mod1::Float( $( $Rest )* )
      };
    }

    mod mod1
    {
      #[ derive( Debug, Default, Clone, PartialEq ) ]
      pub struct Float
      (
        pub f32,
      );
    }

    // trace_macros!( true );
    the_module::types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      pair Pair : < T1 : core::cmp::PartialEq + core::clone::Clone >;

    }
    // trace_macros!( false );

    pub trait Round { fn round( &self ) -> ( f32, f32 ); }
    impl Round
    for ( mod1::Float, mod1::Float )
    {
      fn round( &self ) -> ( f32, f32 )
      {
        ( self.0.0.round(), self.1.0.round() )
      }
    }

    trait RoundInplace { fn round_inplace( &mut self ); };
    impl RoundInplace for ( mod1::Float, mod1::Float )
    {
      fn round_inplace( &mut self )
      {
        self.0.0 = self.0.0.round();
        self.1.0 = self.1.0.round();
      }
    }

    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      /* test.case( "make1" ) */
      let instance1 : Pair< mod1::Float > = the_module::from!( mk!( 13.0 ) );
      let instance2 = Pair::< mod1::Float >::from( [ mk!( 13.0 ), mk!( 13.0 ) ] );
      a_id!( instance1, instance2 );

      /* test.case( "make2" ) */
      let instance1 : Pair< mod1::Float > = the_module::from!( mk!( 13.0 ), mk!( 31.0 ) );
      let instance2 = Pair::< mod1::Float >::from( [ mk!( 13.0 ), mk!( 31.0 ) ] );
      a_id!( instance1, instance2 );
    }

    /* test.case( "from array into pair" ) */
    let instance1 : Pair< mod1::Float > = [ mk!( 13.0 ), mk!( 31.0 ) ].into();
    let instance2 = Pair::< mod1::Float >::from( [ mk!( 13.0 ), mk!( 31.0 ) ] );
    a_id!( instance1.0, mk!( 13.0 ) );
    a_id!( instance1.1, mk!( 31.0 ) );
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance2.1, mk!( 31.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "from pair into array" ) */
    let instance1 : [ _ ; 2 ] = ( Pair::< mod1::Float >::from( [ mk!( 13.0 ), mk!( 31.0 ) ] ) ).into();
    let instance2 = < [ _ ; 2] >::from( Pair::< mod1::Float >::from( [ mk!( 13.0 ), mk!( 31.0 ) ] ) );
    a_id!( instance1[ 0 ], mk!( 13.0 ) );
    a_id!( instance1[ 1 ], mk!( 31.0 ) );
    a_id!( instance2[ 0 ], mk!( 13.0 ) );
    a_id!( instance2[ 1 ], mk!( 31.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "from slice into pair" ) */
    let instance1 : Pair< mod1::Float > = ( &[ mk!( 13.0 ), mk!( 31.0 ) ][ .. ] ).into();
    let instance2 = Pair::< mod1::Float >::from( ( &[ mk!( 13.0 ), mk!( 31.0 ) ][ .. ] ) );
    a_id!( instance1.0, mk!( 13.0 ) );
    a_id!( instance1.1, mk!( 31.0 ) );
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance2.1, mk!( 31.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "from tuple into pair" ) */
    let instance1 : Pair< mod1::Float > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let instance2 = Pair::< mod1::Float >::from( ( mk!( 13.0 ), mk!( 31.0 ) ) );
    a_id!( instance1.0, mk!( 13.0 ) );
    a_id!( instance1.1, mk!( 31.0 ) );
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance2.1, mk!( 31.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "from Pair into tuple" ) */
    let instance1 : Pair< mod1::Float > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let instance2 = Pair::< mod1::Float >::from( ( mk!( 13.0 ), mk!( 31.0 ) ) );
    a_id!( instance1.0, mk!( 13.0 ) );
    a_id!( instance1.1, mk!( 31.0 ) );
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance2.1, mk!( 31.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Pair< mod1::Float > = ( Pair::from( ( mk!( 13.0 ), mk!( 31.0 ) ) ) ).into();
    let instance2 = Pair::< mod1::Float >::from( Pair::from( ( mk!( 13.0 ), mk!( 31.0 ) ) ) );
    a_id!( instance1.0, mk!( 13.0 ) );
    a_id!( instance1.1, mk!( 31.0 ) );
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance2.1, mk!( 31.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair< mod1::Float > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let instance2 = instance1.clone();
    a_id!( instance1.0, mk!( 13.0 ) );
    a_id!( instance1.1, mk!( 31.0 ) );
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance2.1, mk!( 31.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Pair< mod1::Float > = ( mk!( 13.5 ), mk!( 31.5 ) ).into();
    a_id!( got.round(), ( 14.0, 32.0 ) );
    got.round_inplace();
    a_id!( got.0, mk!( 14.0 ) );
    a_id!( got.1, mk!( 32.0 ) );

    /* test.case( "clone_as_tuple" ) */
    let src : Pair< mod1::Float > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = src.clone_as_tuple();
    a_id!( got, ( mk!( 13.0 ), mk!( 31.0 ) ) );
    assert!( !mem::same_ptr( &src, &got ) );

    /* test.case( "clone_as_array" ) */
    let src : Pair< mod1::Float > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = src.clone_as_array();
    a_id!( got, [ mk!( 13.0 ), mk!( 31.0 ) ] );
    assert!( !mem::same_ptr( &src, &got ) );

    /* test.case( "as_tuple" ) */
    let src : Pair< mod1::Float > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = src.as_tuple();
    a_id!( got, &( mk!( 13.0 ), mk!( 31.0 ) ) );
    assert!( mem::same_region( &src, got ) );

    /* test.case( "as_array" ) */
    let src : Pair< mod1::Float > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = src.as_array();
    a_id!( got, &[ mk!( 13.0 ), mk!( 31.0 ) ] );
    assert!( mem::same_region( &src, got ) );

    /* test.case( "as_slice" ) */
    let src : Pair< mod1::Float > = ( mk!( 13.0 ), mk!( 31.0 ) ).into();
    let got = src.as_slice();
    a_id!( got, &[ mk!( 13.0 ), mk!( 31.0 ) ][ .. ] );
    assert!( mem::same_region( &src, got ) );

  }

  //

  fn parameter_no_derives()
  {

    macro_rules! mk
    {
      (
        $( $Rest : tt )*
      )
      =>
      {
        mod1::Float( $( $Rest )* )
      };
    }

    mod mod1
    {
      pub struct Float
      (
        pub f32,
      );
    }

    // trace_macros!( true );
    the_module::types!
    {
      pair Pair : < T1 >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Pair::< mod1::Float >( mk!( 13.0 ), mk!( 13.0 ) );

  }

  //

  fn struct_basic()
  {

    trait Round { fn round( &self ) -> Self; };
    impl Round for ( f32, f32 )
    {
      fn round( &self ) -> Self
      {
        // dbg!( &self );
        ( self.0.round(), self.1.round() )
      }
    }

    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      /* test.case( "make0" ) */
      let got : the_module::HomoPair< f32 > = the_module::from!();
      let exp = the_module::HomoPair::< f32 >( 0.0, 0.0 );
      a_id!( got, exp );

      /* test.case( "make2" ) */
      let got : the_module::HomoPair< f32 > = the_module::from!( 13.0, 31.0 );
      let exp = the_module::HomoPair::< f32 >( 13.0, 31.0 );
      a_id!( got, exp );
    }

    /* test.case( "from tuple into pair" ) */
    let instance1 : the_module::HomoPair< f32 > = ( 13.0, 31.0 ).into();
    let instance2 = the_module::HomoPair::< f32 >::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from the_module::HomoPair into tuple" ) */
    let instance1 : the_module::HomoPair< f32 > = ( 13.0, 31.0 ).into();
    let instance2 = the_module::HomoPair::< f32 >::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : the_module::HomoPair< f32 > = ( the_module::HomoPair::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = the_module::HomoPair::< f32 >::from( the_module::HomoPair::from( ( 13.0, 31.0 ) ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from scalar into the_module::HomoPair" ) */
    let instance1 : the_module::HomoPair< f32 > = ( the_module::HomoPair::from( 13.0 ) ).into();
    let instance2 = the_module::HomoPair::< f32 >::from( the_module::HomoPair::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : the_module::HomoPair< f32 > = ( 13.0, 31.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "default" ) */
    let instance1 : the_module::HomoPair< f32 > = Default::default();
    a_id!( instance1.0, 0.0 );
    a_id!( instance1.1, 0.0 );

    /* test.case( "deref" ) */
    let got : the_module::HomoPair< f32 > = ( 13.5, 31.5 ).into();
    a_id!( got.round(), ( 14.0, 32.0 ) );

  }

  //

  fn struct_no_derives()
  {

    struct Floats< T1, T2 >( pub T1, pub T2 );
    impl< T1, T2 > Floats< T1, T2 >
    {
      pub fn new( src : ( T1, T2 ) ) -> Self
      { Self( src.0, src.1 ) }
    }

    /* test.case( "smoke test" ) */
    let instance1 = the_module::HomoPair( Floats( 13.0, 31.0 ), Floats( 13.0, 31.0 ) );

  }

  //

  fn samples()
  {
    use the_module::
    {
      CloneAsTuple,
      CloneAsArray,
    };

    /* test.case( "single-line homopair" ) */
    {
      the_module::types!( pair MyHomoPair : i32 );
      let x = MyHomoPair( 13, 31 );
      println!( "x : ( {}, {} )", x.0, x.1 );
      // prints : x : ( 13, 31 )
    }

    /* test.case( "parametrized tuple" ) */
    {
      use core::fmt;
      the_module::types!
      {
        #[ derive( Debug ) ]
        pair MyHomoPair : < T : fmt::Debug >;
      }
      let x = MyHomoPair( 13, 31 );
      dbg!( &x );
      // prints : &x = MyHomoPair( 13, 31 )
      let clone_as_array : [ i32 ; 2 ] = x.clone_as_array();
      dbg!( &clone_as_array );
      // prints : &clone_as_array = [ 13, 31 ]
      let clone_as_tuple : ( i32 , i32 ) = x.clone_as_tuple();
      dbg!( &clone_as_tuple );
      // prints : &clone_as_tuple = ( 13, 31 )
    }
  }
}

//

tests_index!
{
  no_parameter_no_derive,
  parameter_with_derives,
  parameter_no_derives,
  struct_basic,
  struct_no_derives,
  samples,
}
