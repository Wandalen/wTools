#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
use TheModule::*;

tests_impls!
{

  //

  #[ test ]
  fn basic()
  {
    use core::fmt;

    mod mod1
    {
      pub use f32;
    }

    // trace_macros!( true );
    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      many Many : mod1::f32;

    }
    // trace_macros!( false );

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many = ( 13.0 ).into();
    let instance2 = Many::from( 13.0 );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Many = ( Many::from( 13.0 ) ).into();
    let instance2 = Many::from( Many::from( 13.0 ) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Many = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many = ( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );

  }

  //

  #[ test ]
  fn empty_parameter()
  {

    mod mod1
    {
      pub use f32;
    }

    // trace_macros!( true );
    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      many Many : mod1::f32<>;
    }
    // trace_macros!( false );

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many = ( 13.0 ).into();
    let instance2 = Many::from( 13.0 );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

  }

  //

  #[ test ]
  fn parametrized_multiple()
  {

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
    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      many Many : mod1::Floats< T1 : PartialEq + std::marker::Copy, T2 : Default >;
    }
    // trace_macros!( false );

    #[ cfg( feature = "types" ) ]
    {
      /* test.case( "make0" ) */
      let got : Many< f32, f64 > = make!();
      let exp = Many::< f32, f64 >( std::vec::Vec::new() );
      a_id!( got, exp );

      /* test.case( "make1" ) */
      let got : Many< f32, f64 > = make!( mk!( 1.0 ) );
      let exp = Many::< f32, f64 >( vec!( mk!( 1.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make2" ) */
      let got : Many< f32, f64 > = make!( mk!( 1.0 ), mk!( 1.0 ) );
      let exp = Many::< f32, f64 >( vec!( mk!( 1.0 ), mk!( 1.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make3" ) */
      let got : Many< f32, f64 > = make!( mk!( 1.0 ), mk!( 1.0 ), mk!( 1.0 ) );
      let exp = Many::< f32, f64 >( vec!( mk!( 1.0 ), mk!( 1.0 ), mk!( 1.0 ) ) );
      a_id!( got, exp );
    }

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many< f32, f64 > = ( mk!( 13.0 ) ).into();
    let instance2 = Many::< f32, f64 >::from( mk!( 13.0 ) );
    a_id!( instance1.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance2.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Many< f32, f64 > = ( Many::from( mk!( 13.0 ) ) ).into();
    let instance2 = Many::< f32, f64 >::from( Many::from( mk!( 13.0 ) ) );
    a_id!( instance1.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance2.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance1, instance2 );

    /* test.case( "from tuple" ) */
    let got : Many< f32, f64 > = ( mk!( 13.0 ), ).into();
    let exp : Many< f32, f64 > = Many::from( mk!( 13.0 ) );
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( ( mk!( 13.0 ), ) );
    let exp : Many< f32, f64 > = Many::from( mk!( 13.0 ) );
    a_id!( got, exp );

    /* test.case( "from array" ) */
    let got : Many< f32, f64 > = [ mk!( 13.0 ), ].into();
    let exp : Many< f32, f64 > = Many::from( mk!( 13.0 ) );
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( [ mk!( 13.0 ), ] );
    let exp : Many< f32, f64 > = Many::from( mk!( 13.0 ) );
    a_id!( got, exp );

    /* test.case( "from array" ) */
    let got : Many< f32, f64 > = [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ), ].into();
    let exp : Many< f32, f64 > = Many::from( [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ] );
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ] );
    let exp : Many< f32, f64 > = Many::from( [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ] );
    a_id!( got, exp );

    /* test.case( "from slice" ) */
    let got : Many< f32, f64 > = ( &[ mk!( 13.0 ), ][ .. ] ).into();
    let exp : Many< f32, f64 > = Many::from( mk!( 13.0 ) );
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( &[ mk!( 13.0 ), ][ .. ] );
    let exp : Many< f32, f64 > = Many::from( mk!( 13.0 ) );
    a_id!( got, exp );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32, f64 > = ( mk!( 13.0 ) ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many< f32, f64 > = ( mk!( 13.0 ) ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( mk!( 13.0 ) ) );

    /* test.case( "as_slice" ) */
    let src : Many< f32, f64 > = Many::from( mk!( 13.0 ) );
    let got = src.as_slice();
    a_id!( got, &[ mk!( 13.0 ), ][ .. ] );
    let got = &src[ .. ];
    a_id!( got, &[ mk!( 13.0 ), ][ .. ] );

  }

  //

  #[ test ]
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
    types!
    {
      many Many : mod1::Floats< T1, T2 >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Many::< f32, f64 >( vec![ mod1::Floats( 13.0, 31.0 ) ] );

  }

  //

  #[ test ]
  fn parameter()
  {
    use core::fmt;

    macro_rules! mk
    {
      (
        $( $Rest : tt )*
      )
      =>
      {
        $( $Rest )*
      };
    }

    // trace_macros!( true );
    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq, Default ) ]
      many Many : < T >;
    }
    // trace_macros!( false );

    /* test.case( "basic" ) */
    let instance1 = Many::< f32 >::from( 13.0 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( implements!( instance1 => Default ) );
    assert!( !implements!( instance1 => fmt::Display ) );

    #[ cfg( feature = "make" ) ]
    {
      /* test.case( "make0" ) */
      let got : Many< f32 > = make!();
      let exp = Many::< f32 >( std::vec::Vec::new() );
      a_id!( got, exp );

      /* test.case( "make1" ) */
      let got : Many< f32 > = make!( mk!( 1.0 ) );
      let exp = Many::< f32 >( vec!( mk!( 1.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make2" ) */
      let got : Many< f32 > = make!( mk!( 1.0 ), mk!( 1.0 ) );
      let exp = Many::< f32 >( vec!( mk!( 1.0 ), mk!( 1.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make3" ) */
      let got : Many< f32 > = make!( mk!( 1.0 ), mk!( 1.0 ), mk!( 1.0 ) );
      let exp = Many::< f32 >( vec!( mk!( 1.0 ), mk!( 1.0 ), mk!( 1.0 ) ) );
      a_id!( got, exp );
    }

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = Many::< f32 >::from( 13.0 );
    a_id!( instance1.0, vec!( 13.0 ) );
    a_id!( instance2.0, vec!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Many< f32 > = ( Many::from( 13.0 ) ).into();
    let instance2 = Many::< f32 >::from( Many::from( 13.0 ) );
    a_id!( instance1.0, vec!( 13.0 ) );
    a_id!( instance2.0, vec!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "from tuple" ) */
    let got : Many< f32 > = ( 13.0, ).into();
    a_id!( got, Many::from( 13.0 ) );
    let got = Many::< f32 >::from( ( 13.0, ) );
    a_id!( got, Many::from( 13.0 ) );

    /* test.case( "from array" ) */
    let got : Many< f32 > = [ 13.0 ].into();
    a_id!( got, Many::from( 13.0 ) );
    let got = Many::< f32 >::from( [ 13.0 ] );
    a_id!( got, Many::from( 13.0 ) );

    /* test.case( "from slice" ) */
    let got : Many< f32 > = (&[ 13.0 ][ .. ]).into();
    a_id!( got, Many::from( 13.0 ) );
    let got = Many::< f32 >::from( (&[ 13.0 ][ .. ]) );
    a_id!( got, Many::from( 13.0 ) );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many< f32 > = ( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );
    a_id!( got.0, std::vec::Vec::< f32 >::new() );

    /* test.case( "as_slice" ) */
    let src : Many< f32 > = ( 13.0, ).into();
    let got = src.as_slice();
    a_id!( got, &[ 13.0, ][ .. ] );
    assert!( !mem_same_ptr( &src, got ) );
    let got = &src[ .. ];
    a_id!( got, &[ 13.0, ][ .. ] );
    assert!( !mem_same_ptr( &src, got ) );

  }

  //

  #[ test ]
  fn parameter_complex()
  {

    types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      many Many : < T : core::cmp::PartialEq + core::clone::Clone >;
    }

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = Many::< f32 >::from( 13.0 );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Many< f32 > = ( Many::from( 13.0 ) ).into();
    let instance2 = Many::< f32 >::from( Many::from( 13.0 ) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many< f32 > = ( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );
    a_id!( got.0, std::vec::Vec::< f32 >::new() );

  }

  //

  #[ test ]
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
    types!
    {
      many Many : < T >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Many( vec![ mod1::Floats( 13.0, 31.0 ) ] );

  }

  //

  #[ test ]
  fn multiple()
  {
    use core::fmt;

    types!
    {

      many Many1 : f32;

      #[ derive( Debug ) ]
      #[ derive( PartialEq, Clone ) ]
      many Many2 : f32;

    }

    /* test.case( "from f32 / into Many2" ) */
    let instance1 : Many1 = ( 13.0 ).into();
    let instance2 = Many1::from( 13.0 );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    assert!( !implements!( instance1 => PartialEq ) );
    assert!( !implements!( instance1 => Clone ) );
    assert!( !implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from f32 / into Many2" ) */
    let instance1 : Many2 = ( 13.0 ).into();
    let instance2 = Many2::from( 13.0 );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "clone / eq" ) */
    let instance1 : Many2 = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

  }

  //

  #[ test ]
  fn struct_basic()
  {

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = Many::< f32 >::from( 13.0 );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Many< f32 > = ( Many::from( 13.0 ) ).into();
    let instance2 = Many::< f32 >::from( Many::from( 13.0 ) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "default" ) */
    let instance1 : Many< f32 > = Default::default();
    a_id!( instance1.0, std::vec::Vec::< f32 >::new() );

    /* test.case( "deref" ) */
    let mut got : Many< f32 > = ( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );

  }

  //

  #[ test ]
  fn struct_no_derives()
  {

    mod mod1
    {
      pub struct Floats< T >( pub T );
      impl< T > Floats< T >
      {
        pub fn new( src : T ) -> Self
        { Self( src ) }
      }
    }

    /* test.case( "from f32 / into Many" ) */
    let instance1 : Many< mod1::Floats< f32 > > = ( mod1::Floats( 13.0 ) ).into();
    let instance2 = Many::< mod1::Floats< f32 > >::from( mod1::Floats( 13.0 ) );
    a_id!( instance1.0[ 0 ].0, 13.0 );
    a_id!( instance1.len(), 1 );
    a_id!( instance2.0[ 0 ].0, 13.0 );
    a_id!( instance2.len(), 1 );

    /* test.case( "deref" ) */
    let mut got : Many< f32 > = ( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );

  }

  //

  #[ test ]
  fn samples()
  {

    /* test.case( "single-line" ) */
    {
      types!( many MyMany : i32 );
      let x = MyMany::from( [ 1, 2, 3 ] );
      println!( "x : {:?}", x.0 );
    }

  }
}

//

tests_index!
{
  basic,
  empty_parameter,
  parametrized_multiple,
  parametrized_no_derives,
  parameter,
  parameter_complex,
  parameter_no_derives,
  multiple,
  struct_basic,
  struct_no_derives,
  samples,
}
