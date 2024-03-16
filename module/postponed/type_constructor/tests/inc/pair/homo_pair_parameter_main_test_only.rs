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
        mod1::Float( $( $Rest )* )
      };
    }

    mod mod1
    {
      #[ derive( Debug, Clone, PartialEq ) ]
      pub struct Float
      (
        pub f32,
      );
    }

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
}

//

tests_index!
{
  main,
}
