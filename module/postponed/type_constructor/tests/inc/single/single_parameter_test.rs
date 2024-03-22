#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{

  fn parameter_complex()
  {

    the_module::types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : < T : core::cmp::PartialEq + core::clone::Clone >;
    }

    /* test.case( "from f32 into Single" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = Single::< f32 >::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
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
    the_module::types!
    {
      single Single : < T >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Single( mod1::Floats( 13.0, 31.0 ) );

  }

  //


  fn parameter_vis()
  {

    mod mod1
    {
      use super::*;
      the_module::types!
      {
        #[ derive( Debug, Clone ) ]
        pub single Public1 : < T >;
        #[ derive( Debug, Clone ) ]
        single Private1 : < T >;
      }
    }

    let instance1 : mod1::Public1< f32 > = ( 13.0 ).into();
    a_id!( instance1.0, 13.0 );
    // let instance1 : mod1::Private1< f32 > = ( 13.0 ).into();
    // a_id!( instance1.0, 13.0 );
    // qqq : add negative tests
    // qqq : add negative tests for pair, homopair and many

  }

  //

  fn struct_basic()
  {

    /* test.case( "from f32 into Single" ) */
    let instance1 : the_module::Single< f32 > = ( 13.0 ).into();
    let instance2 = the_module::Single::< f32 >::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : the_module::Single< f32 > = ( the_module::Single::from( 13.0 ) ).into();
    let instance2 = the_module::Single::< f32 >::from( the_module::Single::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : the_module::Single< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "default" ) */
    let instance1 : the_module::Single< f32 > = Default::default();
    a_id!( instance1.0, 0.0 );

    /* test.case( "deref" ) */
    use core::ops::AddAssign;
    let mut got : the_module::Single< f32 > = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );
    got.add_assign( 1.0 );
    a_id!( got.0, 14.5 );

    /* test.case( "make0" ) */
    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      let got : the_module::Single< f32 > = the_module::from!();
      let exp = the_module::Single::< f32 >::from( 0.0 );
      a_id!( got, exp );
    }

    /* test.case( "make1" ) */
    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      let got : the_module::Single< f32 > = the_module::Single::< f32 >::from( 13.0 );
      let exp = the_module::Single::< f32 >::from( 13.0 );
      a_id!( got, exp );
    }

  }

  //


  fn struct_no_derives()
  {

    struct Floats< T >( pub T );

    impl< T > Floats< T >
    {
      pub fn new( src : T ) -> Self
      { Self( src ) }
    }

    /* test.case( "from f32 into Single" ) */
    let instance1 : the_module::Single< Floats< f32 > > = ( Floats( 13.0 ) ).into();
    let instance2 = the_module::Single::< Floats< f32 > >::from( Floats( 13.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );

    /* test.case( "from itself into itself" ) */
    let val = Floats::< f32 >::new( 13.0 );
    let instance1 : the_module::Single< Floats< f32 > > = ( the_module::Single::from( val ) ).into();
    let instance2 = the_module::Single::< Floats< f32 > >::from( the_module::Single::from( Floats( 13.0 ) ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );

    /* test.case( "deref" ) */
    use core::ops::AddAssign;
    let mut got : the_module::Single< f32 > = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );
    got.add_assign( 1.0 );
    a_id!( got.0, 14.5 );

  }


}

//

tests_index!
{

  parameter_complex,
  parameter_no_derives,
  parameter_vis,
  struct_basic,
  struct_no_derives,

}
