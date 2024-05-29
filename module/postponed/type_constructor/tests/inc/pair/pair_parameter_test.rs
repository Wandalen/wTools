#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{


  fn empty_parameter()
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
    the_module::types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      pair Pair : mod1::f32<>, mod1::f64<>;

    }
    // trace_macros!( false );

    /* test.case( "from tuple into pair" ) */
    let instance1 : Pair = ( 13.0, 31.0 ).into();
    let instance2 = Pair::from( ( 13.0, 31.0 ) );
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

  }

  //

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
      pair Pair : mod1::Float;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Pair( mod1::Float( 13.0 ), mod1::Float( 31.0 ) );

  }

  //

  fn parameter_complex()
  {
    use core::fmt;

    the_module::types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      pair Pair : < T1 : core::cmp::PartialEq + core::clone::Clone, T2 : core::cmp::PartialEq + core::clone::Clone >;
    }

    /* test.case( "traits" ) */
    let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );
    assert!( !implements!( instance1 => fmt::Display ) );

    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      /* test.case( "make0" ) */
      let got : Pair< f32, f64 > = the_module::from!();
      let exp = Pair::< f32, f64 >( 0.0, 0.0 );
      a_id!( got, exp );

      /* test.case( "make2" ) */
      let got : Pair< f32, f64 > = the_module::from!( 13.0, 31.0 );
      let exp = Pair::< f32, f64 >( 13.0, 31.0 );
      a_id!( got, exp );
    }

    /* test.case( "from tuple into pair" ) */
    let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = Pair::< f32, f64 >::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Pair into tuple" ) */
    let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = Pair::< f32, f64 >::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Pair< f32, f64 > = ( Pair::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = Pair::< f32, f64 >::from( Pair::from( ( 13.0, 31.0 ) ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

//     /* test.case( "deref" ) */
//     let got : Pair< f32, f64 > = ( 13.5 ).into();
//     a_id!( got.round(), 14.0 );

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
      pair Pair : < T1, T2 >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Pair( mod1::Floats( 13.0, 31.0 ), mod1::Floats( 13.0, 31.0 ) );

  }

  //

  fn multiple()
  {
    use core::fmt;

    the_module::types!
    {

      pair Pair1 : f64, f32;

      #[ derive( Debug ) ]
      #[ derive( PartialEq, Clone ) ]
      pair Pair2 : f32, f64;

    }

    /* test.case( "from tuple into Pair2" ) */
    let instance1 : Pair1 = ( 13.0, 31.0 ).into();
    let instance2 = Pair1::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    assert!( !implements!( instance1 => PartialEq ) );
    assert!( !implements!( instance1 => Clone ) );
    assert!( !implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from tuple into Pair2" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let instance2 = Pair2::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from tuple into Pair2" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let instance2 = Pair2::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Pair2 = ( Pair2::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = Pair2::from( Pair2::from( ( 13.0, 31.0 ) ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Pair2 into tuple" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let got : ( _, _ ) = instance1.into();
    a_id!( got, ( 13.0, 31.0 ) );
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let got = <( f32, f64 )>::from( instance1 );
    a_id!( got, ( 13.0, 31.0 ) );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    // /* test.case( "deref" ) */
    // let got : Pair2 = ( 13.5, 15.5 ).into();
    // a_id!( got.round(), 14.0 );

  }

  //

  fn struct_basic()
  {

    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      /* test.case( "make0" ) */
      let got : the_module::Pair< f32, f64 > = the_module::from!();
      let exp = the_module::Pair::< f32, f64 >( 0.0, 0.0 );
      a_id!( got, exp );

      /* test.case( "make2" ) */
      let got : the_module::Pair< f32, f64 > = the_module::from!( 13.0, 31.0 );
      let exp = the_module::Pair::< f32, f64 >( 13.0, 31.0 );
      a_id!( got, exp );
    }

    /* test.case( "from tuple into pair" ) */
    let instance1 : the_module::Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = the_module::Pair::< f32, f64 >::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Pair into tuple" ) */
    let instance1 : the_module::Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = the_module::Pair::< f32, f64 >::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : the_module::Pair< f32, f64 > = ( the_module::Pair::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = the_module::Pair::< f32, f64 >::from( the_module::Pair::from( ( 13.0, 31.0 ) ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : the_module::Pair< f32, f64 > = ( 13.0, 31.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance1.0, 13.0 );
    a_id!( instance1.1, 31.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance2.1, 31.0 );
    a_id!( instance1, instance2 );

    /* test.case( "default" ) */
    let instance1 : the_module::Pair< f32, f64 > = Default::default();
    a_id!( instance1.0, 0.0 );
    a_id!( instance1.1, 0.0 );

//     /* test.case( "deref" ) */
//     let got : the_module::Pair< f32, f64 > = ( 13.5 ).into();
//     a_id!( got.round(), 14.0 );

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

    /* test.case( "from tuple into pair" ) */
    let instance1 : the_module::Pair< Floats< f32, f64 >, f32 > = ( Floats( 13.0, 31.0 ), 131.0 ).into();
    let instance2 = the_module::Pair::< Floats< f32, f64 >, f32 >::from( ( Floats( 13.0, 31.0 ), 131.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance1.0.1, 31.0 );
    a_id!( instance1.1, 131.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance2.0.1, 31.0 );
    a_id!( instance2.1, 131.0 );

  }

  //

  fn struct_transitive_from()
  {
    // use the_module::{ From_2 };

    /* test.case( "from tuple" ) */
    {
      // the_module::types!
      // {
      //   #[ derive( PartialEq, Debug ) ]
      //   single MySingle : i32
      // };
      #[ derive( PartialEq, Debug ) ]
      struct MySingle
      (
        pub i32,
      );

      impl From< i32 >
      for MySingle
      {
        fn from( src : i32 ) -> Self
        {
          MySingle( src )
        }
      }

      let src = ( 1, 3 );
      let got : the_module::Pair< MySingle, MySingle > = src.into();
      let exp = the_module::Pair::from( ( MySingle::from( 1 ), MySingle::from( 3 ) ) );
      a_id!( got, exp );
    }
    // zzz : implement similar test for other type constructors

    // /* test.case( "from pair" ) */
    // {
    //   // trace_macros!( true );
    //   the_module::types!
    //   {
    //     #[ derive( PartialEq, Debug ) ]
    //     single MySingle : i32
    //   };
    //   // trace_macros!( false );
    //   let src = the_module::Pair::from_2( 1, 3 );
    //   // let got : the_module::Pair< MySingle, MySingle > = src.into();
    //   let exp = the_module::Pair::from_2( MySingle::from_1( 1 ), MySingle::from_1( 3 ) );
    //   // a_id!( got, exp );
    // }

  }
}

//

tests_index!
{
  empty_parameter,
  no_parameter_no_derive,
  parameter_complex,
  parameter_no_derives,
  multiple,
  struct_basic,
  struct_no_derives,
  struct_transitive_from,
}
