#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{
  fn main()
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
}

//

tests_index!
{
  main,
}
