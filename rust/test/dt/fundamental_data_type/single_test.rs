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
    }

    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::f32;

    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = Single::from( 13.0 );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::from( Single::from( 13.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Single / into f32" ) */
    let instance1 : Single = ( 13.0 ).into();
    let got : f32 = instance1.into();
    assert_eq!( got, 13.0 );
    let instance1 : Single = ( 13.0 ).into();
    let got = f32::from( instance1 );
    assert_eq!( got, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single = ( 13.5 ).into();
    assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn empty_parameter_test()
  {

    mod mod1
    {
      pub use f32;
    }

    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::f32<>;

    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = Single::from( 13.0 );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::from( Single::from( 13.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Single / into f32" ) */
    let instance1 : Single = ( 13.0 ).into();
    let got : f32 = instance1.into();
    assert_eq!( got, 13.0 );
    let instance1 : Single = ( 13.0 ).into();
    let got = f32::from( instance1 );
    assert_eq!( got, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single = ( 13.5 ).into();
    assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn parametrized_test()
  {

    mod mod1
    {

      #[ derive( Debug, Clone, PartialEq ) ]
      pub struct Single0< T >
      (
        pub T,
      );

      impl< T > core::ops::Deref
      for Single0< T >
      {
        type Target = T;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< T > From< T > for Single0< T >
      {
        fn from( src : T ) -> Self
        {
          Self( src )
        }
      }

    }

    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::Single0< T >;

    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32 > = ( mod1::Single0::from( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( mod1::Single0::from( 13.0 ) );
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance2.0.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( mod1::Single0::from( 13.0 ) ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( mod1::Single0::from( 13.0 ) ) );
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance2.0.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Single / into f32" ) */
    let instance1 : Single< f32 > = ( mod1::Single0::from( 13.0 ) ).into();
    let got : mod1::Single0< f32 > = instance1.into();
    assert_eq!( got.0, 13.0 );
    let instance1 : Single< f32 > = ( mod1::Single0::from( 13.0 ) ).into();
    let got = mod1::Single0::< f32 >::from( instance1 );
    assert_eq!( got.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( mod1::Single0::from( 13.0 ) ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, mod1::Single0::from( 13.0 ) );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( mod1::Single0::from( 13.5 ) ).into();
    assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn parametrized_complex_test()
  {

    mod mod1
    {

      #[ derive( Debug, Clone, PartialEq ) ]
      pub struct Single0< T : PartialEq + Copy >
      (
        pub T,
      );

      impl< T : PartialEq + Copy > core::ops::Deref
      for Single0< T >
      {
        type Target = T;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< T : PartialEq + Copy > From< T > for Single0< T >
      {
        fn from( src : T ) -> Self
        {
          Self( src )
        }
      }

    }

    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::Single0< T : PartialEq + std::marker::Copy >;

    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32 > = ( mod1::Single0::from( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( mod1::Single0::from( 13.0 ) );
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance2.0.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( mod1::Single0::from( 13.0 ) ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( mod1::Single0::from( 13.0 ) ) );
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance2.0.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Single / into f32" ) */
    let instance1 : Single< f32 > = ( mod1::Single0::from( 13.0 ) ).into();
    let got : mod1::Single0< f32 > = instance1.into();
    assert_eq!( got.0, 13.0 );
    let instance1 : Single< f32 > = ( mod1::Single0::from( 13.0 ) ).into();
    let got = mod1::Single0::< f32 >::from( instance1 );
    assert_eq!( got.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( mod1::Single0::from( 13.0 ) ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, mod1::Single0::from( 13.0 ) );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( mod1::Single0::from( 13.5 ) ).into();
    assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn parametrized_multiple_test()
  {

    mod mod1
    {

      #[ derive( Debug, Clone, PartialEq ) ]
      pub struct Single0< T1 : PartialEq + Copy, T2 : Default >
      (
        pub T1,
        pub T2,
      );

      impl< T1 : PartialEq + Copy, T2 : Default > core::ops::Deref
      for Single0< T1, T2 >
      {
        type Target = T1;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< T1 : PartialEq + Copy, T2 : Default > From< T1 >
      for Single0< T1, T2 >
      {
        fn from( src : T1 ) -> Self
        {
          Single0::< T1, T2 >( src, T2::default() )
        }
      }

    }

    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::Single0< T1 : PartialEq + std::marker::Copy, T2 : Default >;

    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32, f64 > = ( mod1::Single0::from( 13.0 ) ).into();
    let instance2 = Single::< f32, f64 >::from( mod1::Single0::from( 13.0 ) );
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance2.0.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32, f64 > = ( Single::from( mod1::Single0::from( 13.0 ) ) ).into();
    let instance2 = Single::< f32, f64 >::from( Single::from( mod1::Single0::from( 13.0 ) ) );
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance2.0.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Single / into f32" ) */
    let instance1 : Single< f32, f64 > = ( mod1::Single0::from( 13.0 ) ).into();
    let got : mod1::Single0< f32, f64 > = instance1.into();
    assert_eq!( got.0, 13.0 );
    let instance1 : Single< f32, f64 > = ( mod1::Single0::from( 13.0 ) ).into();
    let got = mod1::Single0::< f32, f64 >::from( instance1 );
    assert_eq!( got.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32, f64 > = ( mod1::Single0::from( 13.0 ) ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, mod1::Single0::from( 13.0 ) );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32, f64 > = ( mod1::Single0::from( 13.5 ) ).into();
    assert_eq!( got.round(), 14.0 );

  }

  //

  fn parameter_test()
  {

    types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq, Default ) ]
      single Single : < T >;

    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = Single::< f32 >::from( 13.0 );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( 13.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( 13.5 ).into();
    assert_eq!( got.round(), 14.0 );

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
      single Single : < T : core::cmp::PartialEq + core::clone::Clone >;

    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = Single::< f32 >::from( 13.0 );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( 13.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( 13.5 ).into();
    assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn multiple_test()
  {
    use core::fmt::Debug;

    types!
    {

      single Single1 : f32;

      #[ derive( Debug ) ]
      #[ derive( PartialEq, Clone ) ]
      single Single2 : f32;

    }

    /* test.case( "from f32 / into Single2" ) */
    let instance1 : Single1 = ( 13.0 ).into();
    let instance2 = Single1::from( 13.0 );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert!( !implements!( instance1 => PartialEq ) );
    assert!( !implements!( instance1 => Clone ) );
    assert!( !implements!( instance1 => Debug ) );

    /* test.case( "from f32 / into Single2" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let instance2 = Single2::from( 13.0 );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from f32 / into Single2" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let instance2 = Single2::from( 13.0 );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single2 = ( Single2::from( 13.0 ) ).into();
    let instance2 = Single2::from( Single2::from( 13.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from Single2 / into f32" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let got : f32 = instance1.into();
    assert_eq!( got, 13.0 );
    let instance1 : Single2 = ( 13.0 ).into();
    let got = f32::from( instance1 );
    assert_eq!( got, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single2 = ( 13.5 ).into();
    assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn struct_basic_test()
  {

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = Single::< f32 >::from( 13.0 );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "from itself / into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( 13.0 ) );
    assert_eq!( instance1.0, 13.0 );
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    assert_eq!( instance2.0, 13.0 );
    assert_eq!( instance1, instance2 );

    /* test.case( "default" ) */
    let instance1 : Single< f32 > = Default::default();
    assert_eq!( instance1.0, 0.0 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( 13.5 ).into();
    assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn struct_deaf_test()
  {

    struct Single0< T >( pub T );

    impl< T > Single0< T >
    {
      pub fn new( src : T ) -> Self
      { Self( src ) }
    }

    /* test.case( "from f32 / into Single" ) */
    let instance1 : Single< Single0< f32 > > = ( Single0( 13.0 ) ).into();
    let instance2 = Single::< Single0< f32 > >::from( Single0( 13.0 ) );
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance2.0.0, 13.0 );

    /* test.case( "from itself / into itself" ) */
    let val = Single0::< f32 >::new( 13.0 );
    let instance1 : Single< Single0< f32 > > = ( Single::from( val ) ).into();
    let instance2 = Single::< Single0< f32 > >::from( Single::from( Single0( 13.0 ) ) );
    assert_eq!( instance1.0.0, 13.0 );
    assert_eq!( instance2.0.0, 13.0 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( 13.5 ).into();
    assert_eq!( got.round(), 14.0 );

  }

  //

  #[ test ]
  fn samples_test()
  {

    /* test.case( "single-line" ) */
    {
      types!( single MySingle : i32 );
      let x = MySingle( 13 );
      println!( "x : {}", x.0 );
    }

    /* test.case( "derives and attributes" ) */
    {
      types!
      {
        /// This is also attribute and macro understands it.
        #[ derive( Debug ) ]
        single MySingle : i32;
      }
      let x = MySingle( 13 );
      dbg!( x );
    }

    /* test.case( "struct instead of macro" ) */
    {
      let x = Single::< i32 >( 13 );
      dbg!( x );
    }

    /* test.case( "parametrized element" ) */
    {
      types!
      {
        #[ derive( Debug ) ]
        single MySingle : std::sync::Arc< T : Copy >;
      }
      let x = MySingle( std::sync::Arc::new( 13 ) );
      dbg!( x );
    }

    /* test.case( "parametrized tuple" ) */
    {
      types!
      {
        #[ derive( Debug ) ]
        single MySingle : < T : Copy >;
      }
      let x = MySingle( 13 );
      dbg!( x );
    }

  }

}

//

tests_index!
{

  basic_test,
  empty_parameter_test,
  parametrized_test,
  parametrized_complex_test,
  parametrized_multiple_test,
  parameter_test,
  parameter_complex_test,
  multiple_test,
  struct_basic_test,
  struct_deaf_test,
  samples_test,

}
