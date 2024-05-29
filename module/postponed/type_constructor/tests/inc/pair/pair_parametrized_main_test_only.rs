#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{
  fn main()
  {
    macro_rules! mk1
    {
      (
        $( $Rest : tt )*
      )
      =>
      {
        mod1::Floats::from( $( $Rest )* )
      };
    }

    macro_rules! mk2
    {
      (
        $( $Rest : tt )*
      )
      =>
      {
        std::sync::Arc::new( $( $Rest )* )
      };
    }

    macro_rules! mk
    {
      (
        $( $Rest : tt )*
      )
      =>
      {
        (
          mk1!( $( $Rest )* ),
          mk2!( 31.0 ),
        )
      };
    }

    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      /* test.case( "make2" ) */
      let got : Pair< f32, f64, f32 > = the_module::from!( mk1!( 13.0 ), mk2!( 31.0 ) );
      let exp = Pair::< f32, f64, f32 >( mk1!( 13.0 ), mk2!( 31.0 ) );
      a_id!( got, exp );
    }

    /* test.case( "from tuple into pair" ) */
    let instance1 : Pair< f32, f64, f32 > = mk!( 13.0 ).into();
    let instance2 = Pair::< f32, f64, f32 >::from( mk!( 13.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Pair into tuple" ) */
    let instance1 : Pair< f32, f64, f32 > = mk!( 13.0 ).into();
    let got : ( mod1::Floats< f32, f64 >, _ ) = instance1.into();
    a_id!( got.0.0, 13.0 );
    let instance1 : Pair< f32, f64, f32 > = mk!( 13.0 ).into();
    let got = < ( mod1::Floats::< f32, f64 >, _ ) >::from( instance1 );
    a_id!( got.0.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair< f32, f64, f32 > = mk!( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, mk1!( 13.0 ) );
    a_id!( instance1, instance2 );


  }
}

//

tests_index!
{
  main,
}
