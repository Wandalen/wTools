#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{

  //

  fn basic()
  {
    use core::fmt;

    mod mod1
    {
      pub use f32;
    }

    // trace_macros!( true );
    the_module::types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::f32;

    }
    // trace_macros!( false );


    /* test.case( "from f32 into Single" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = Single::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from itself into itself" ) */
    let instance1 : Single = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::from( Single::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single into f32" ) */
    let instance1 : Single = ( 13.0 ).into();
    let got : f32 = instance1.into();
    a_id!( got, 13.0 );
    let instance1 : Single = ( 13.0 ).into();
    let got = f32::from( instance1 );
    a_id!( got, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    use core::ops::AddAssign;
    let mut got : Single = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );
    got.add_assign( 1.0 );
    a_id!( got.0, 14.5 );

  }

  //


  fn vis()
  {

    mod mod1
    {
      use super::*;
      the_module::types!
      {
        #[ derive( Debug, Clone ) ]
        pub single Public1 : f32;
        #[ derive( Debug, Clone ) ]
        single Private1 : f32;
      }
    }

    let instance1 : mod1::Public1 = ( 13.0 ).into();
    a_id!( instance1.0, 13.0 );
    // let instance1 : mod1::Private1 = ( 13.0 ).into();
    // a_id!( instance1.0, 13.0 );
    // qqq : add negative tests

  }

  //


  fn empty_parameter()
  {

    mod mod1
    {
      pub use f32;
    }

    the_module::types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::f32<>;
    }

    /* test.case( "from f32 into Single" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = Single::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Single = ( Single::from( 13.0 ) ).into();
    let instance2 = Single::from( Single::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single into f32" ) */
    let instance1 : Single = ( 13.0 ).into();
    let got : f32 = instance1.into();
    a_id!( got, 13.0 );
    let instance1 : Single = ( 13.0 ).into();
    let got = f32::from( instance1 );
    a_id!( got, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );

  }

  //


  fn no_parameter_no_derive()
  {

    mod mod1
    {
      #[ derive( Clone ) ]
      pub struct Float
      (
        pub f32,
      );
    }

    // trace_macros!( true );
    the_module::types!
    {
      single Single : mod1::Float;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Single( mod1::Float( 13.0 ) );

  }

  //


  fn parametrized()
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
      pub struct Floats< T >
      (
        pub T,
      );

      impl< T > core::ops::Deref
      for Floats< T >
      {
        type Target = T;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< T > From< T > for Floats< T >
      {
        fn from( src : T ) -> Self
        {
          Self( src )
        }
      }

    }

    the_module::types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::Floats< T >;
    }

    /* test.case( "from f32 into Single" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( mk!( 13.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( mk!( 13.0 ) ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( mk!( 13.0 ) ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single into f32" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let got : mod1::Floats< f32 > = instance1.into();
    a_id!( got.0, 13.0 );
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let got = mod1::Floats::< f32 >::from( instance1 );
    a_id!( got.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( mk!( 13.5 ) ).into();
    a_id!( got.round(), 14.0 );

  }

  //


  fn parametrized_complex()
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
      pub struct Floats< T : PartialEq + Copy >
      (
        pub T,
      );

      impl< T : PartialEq + Copy > core::ops::Deref
      for Floats< T >
      {
        type Target = T;
        fn deref( &self ) -> &Self::Target
        {
          &self.0
        }
      }

      impl< T : PartialEq + Copy > From< T > for Floats< T >
      {
        fn from( src : T ) -> Self
        {
          Self( src )
        }
      }

    }

    the_module::types!
    {

      ///
      /// Attribute which is inner.
      ///

      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      single Single : mod1::Floats< T : PartialEq + std::marker::Copy >;

    }

    /* test.case( "from f32 into Single" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let instance2 = Single::< f32 >::from( mk!( 13.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Single< f32 > = ( Single::from( mk!( 13.0 ) ) ).into();
    let instance2 = Single::< f32 >::from( Single::from( mk!( 13.0 ) ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single into f32" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let got : mod1::Floats< f32 > = instance1.into();
    a_id!( got.0, 13.0 );
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let got = mod1::Floats::< f32 >::from( instance1 );
    a_id!( got.0, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32 > = ( mk!( 13.0 ) ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32 > = ( mk!( 13.5 ) ).into();
    a_id!( got.round(), 14.0 );

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
      single Single : mod1::Floats< T1, T2 >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Single::< f32, f64 >( mod1::Floats( 13.0, 31.0 ) );

  }

  //

  fn multiple()
  {
    use core::fmt;

    the_module::types!
    {

      single Single1 : f32;

      #[ derive( Debug ) ]
      #[ derive( PartialEq, Clone ) ]
      single Single2 : f32;

    }

    /* test.case( "from f32 into Single2" ) */
    let instance1 : Single1 = ( 13.0 ).into();
    let instance2 = Single1::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    assert!( !implements!( instance1 => PartialEq ) );
    assert!( !implements!( instance1 => Clone ) );
    assert!( !implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from f32 into Single2" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let instance2 = Single2::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from f32 into Single2" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let instance2 = Single2::from( 13.0 );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Single2 = ( Single2::from( 13.0 ) ).into();
    let instance2 = Single2::from( Single2::from( 13.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single2 into f32" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let got : f32 = instance1.into();
    a_id!( got, 13.0 );
    let instance1 : Single2 = ( 13.0 ).into();
    let got = f32::from( instance1 );
    a_id!( got, 13.0 );

    /* test.case( "clone / eq" ) */
    let instance1 : Single2 = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single2 = ( 13.5 ).into();
    a_id!( got.round(), 14.0 );

  }

  //

  fn samples()
  {

    /* test.case( "multiple" ) */
    {
      the_module::types!
      {

        single MySingle : f32;
        single SingleWithParametrized : std::sync::Arc< T : Copy >;
        single SingleWithParameter : < T >;

        pair MyPair : f32;
        pair PairWithParametrized : std::sync::Arc< T1 : Copy >, std::sync::Arc< T2 : Copy >;
        pair PairWithParameter : < T1, T2 >;

        pair MyHomoPair : f32;
        pair HomoPairWithParametrized : std::sync::Arc< T : Copy >;
        pair HomoPairWithParameter : < T >;

        // #[ cfg
        // (
        //   all
        //   (
        //     feature = "many",
        //     any( not( feature = "no_std" ), feature = "use_alloc" ),
        //   )
        // ) ]
        // many MyMany : f32;
        // #[ cfg
        // (
        //   all
        //   (
        //     feature = "many",
        //     any( not( feature = "no_std" ), feature = "use_alloc" ),
        //   )
        // ) ]
        // many ManyWithParametrized : std::sync::Arc< T : Copy >;
        // #[ cfg
        // (
        //   all
        //   (
        //     feature = "many",
        //     any( not( feature = "no_std" ), feature = "use_alloc" ),
        //   )
        // ) ]
        // many ManyWithParameter : < T >;
      }
    }

    /* test.case( "no macro" ) */
    {
      let i32_in_tuple = the_module::Single::< i32 >::from( 13 );
      dbg!( i32_in_tuple );
      // i32_in_tuple = Single( 13 )
      let i32_and_f32_in_tuple = the_module::Pair::< i32, f32 >::from( the_module::Pair( 13, 13.0 ) );
      dbg!( i32_and_f32_in_tuple );
      // vec_of_i32_in_tuple = Pair( 13, 13.0 )
      let two_i32_in_tuple = the_module::HomoPair::< i32 >::from( the_module::HomoPair( 13, 31 ) );
      dbg!( two_i32_in_tuple );
      // vec_of_i32_in_tuple = HomoPair( 13, 31 )
      #[ cfg
      (
        all
        (
          feature = "many",
          any( not( feature = "no_std" ), feature = "use_alloc" ),
        )
      ) ]
      {
        let vec_of_i32_in_tuple = the_module::Many::< i32 >::from([ 1, 2, 3 ]);
        dbg!( vec_of_i32_in_tuple );
        // vec_of_i32_in_tuple = Many([ 1, 2, 3 ])
      }
    }

    /* test.case( "single-line" ) */
    {
      the_module::types!( single MySingle : i32 );
      let x = MySingle( 13 );
      println!( "x : {}", x.0 );
    }

    /* test.case( "derives and attributes" ) */
    {
      the_module::types!
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
      let x = the_module::Single::< i32 >( 13 );
      dbg!( x );
    }

    /* test.case( "parametrized element" ) */
    {
      the_module::types!
      {
        #[ derive( Debug ) ]
        single MySingle : std::sync::Arc< T : Copy >;
      }
      let x = MySingle( std::sync::Arc::new( 13 ) );
      dbg!( x );
    }

    /* test.case( "parametrized tuple" ) */
    {
      the_module::types!
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

  basic,
  vis,
  empty_parameter,
  no_parameter_no_derive,
  parametrized,
  parametrized_complex,
  // parametrized_multiple,
  parametrized_no_derives,
  multiple,
  samples,

}
