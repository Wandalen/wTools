#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
use TheModule::*;

//

fn single_basic_test()
{

  mod mod1
  {
    pub use f32;
  }

  single!
  {

    ///
    /// Attribute which is inner.
    ///

    #[ derive( Debug, Clone ) ]
    #[ derive( PartialEq ) ]
    Single : mod1::f32;

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

fn single_empty_parameter_test()
{

  mod mod1
  {
    pub use f32;
  }

  single!
  {

    ///
    /// Attribute which is inner.
    ///

    #[ derive( Debug, Clone ) ]
    #[ derive( PartialEq ) ]
    Single : mod1::f32<>;

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

fn single_parametrized_test()
{

  mod mod1
  {

    #[ derive( Debug, Clone, PartialEq ) ]
    pub struct Wrap0< T >
    (
      pub T,
    );

    impl< T > core::ops::Deref
    for Wrap0< T >
    {
      type Target = T;
      fn deref( &self ) -> &Self::Target
      {
        &self.0
      }
    }

    impl< T > From< T > for Wrap0< T >
    {
      fn from( src : T ) -> Self
      {
        Self( src )
      }
    }

  }

  single!
  {

    ///
    /// Attribute which is inner.
    ///

    #[ derive( Debug, Clone ) ]
    #[ derive( PartialEq ) ]
    Single : mod1::Wrap0< T >;

  }

  /* test.case( "from f32 / into Single" ) */
  let instance1 : Single< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
  let instance2 = Single::< f32 >::from( mod1::Wrap0::from( 13.0 ) );
  assert_eq!( instance1.0.0, 13.0 );
  assert_eq!( instance2.0.0, 13.0 );
  assert_eq!( instance1, instance2 );

  /* test.case( "from itself / into itself" ) */
  let instance1 : Single< f32 > = ( Single::from( mod1::Wrap0::from( 13.0 ) ) ).into();
  let instance2 = Single::< f32 >::from( Single::from( mod1::Wrap0::from( 13.0 ) ) );
  assert_eq!( instance1.0.0, 13.0 );
  assert_eq!( instance2.0.0, 13.0 );
  assert_eq!( instance1, instance2 );

  /* test.case( "from Single / into f32" ) */
  let instance1 : Single< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
  let got : mod1::Wrap0< f32 > = instance1.into();
  assert_eq!( got.0, 13.0 );
  let instance1 : Single< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
  let got = mod1::Wrap0::< f32 >::from( instance1 );
  assert_eq!( got.0, 13.0 );

  /* test.case( "clone / eq" ) */
  let instance1 : Single< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
  let instance2 = instance1.clone();
  assert_eq!( instance2.0, mod1::Wrap0::from( 13.0 ) );
  assert_eq!( instance1, instance2 );

  /* test.case( "deref" ) */
  let got : Single< f32 > = ( mod1::Wrap0::from( 13.5 ) ).into();
  assert_eq!( got.round(), 14.0 );

}

//

fn single_parametrized_complex_test()
{

  mod mod1
  {

    #[ derive( Debug, Clone, PartialEq ) ]
    pub struct Wrap0< T : PartialEq + Copy >
    (
      pub T,
    );

    impl< T : PartialEq + Copy > core::ops::Deref
    for Wrap0< T >
    {
      type Target = T;
      fn deref( &self ) -> &Self::Target
      {
        &self.0
      }
    }

    impl< T : PartialEq + Copy > From< T > for Wrap0< T >
    {
      fn from( src : T ) -> Self
      {
        Self( src )
      }
    }

  }

  single!
  {

    ///
    /// Attribute which is inner.
    ///

    #[ derive( Debug, Clone ) ]
    #[ derive( PartialEq ) ]
    Single : mod1::Wrap0< T : PartialEq + std::marker::Copy >;

  }

  /* test.case( "from f32 / into Single" ) */
  let instance1 : Single< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
  let instance2 = Single::< f32 >::from( mod1::Wrap0::from( 13.0 ) );
  assert_eq!( instance1.0.0, 13.0 );
  assert_eq!( instance2.0.0, 13.0 );
  assert_eq!( instance1, instance2 );

  /* test.case( "from itself / into itself" ) */
  let instance1 : Single< f32 > = ( Single::from( mod1::Wrap0::from( 13.0 ) ) ).into();
  let instance2 = Single::< f32 >::from( Single::from( mod1::Wrap0::from( 13.0 ) ) );
  assert_eq!( instance1.0.0, 13.0 );
  assert_eq!( instance2.0.0, 13.0 );
  assert_eq!( instance1, instance2 );

  /* test.case( "from Single / into f32" ) */
  let instance1 : Single< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
  let got : mod1::Wrap0< f32 > = instance1.into();
  assert_eq!( got.0, 13.0 );
  let instance1 : Single< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
  let got = mod1::Wrap0::< f32 >::from( instance1 );
  assert_eq!( got.0, 13.0 );

  /* test.case( "clone / eq" ) */
  let instance1 : Single< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
  let instance2 = instance1.clone();
  assert_eq!( instance2.0, mod1::Wrap0::from( 13.0 ) );
  assert_eq!( instance1, instance2 );

  /* test.case( "deref" ) */
  let got : Single< f32 > = ( mod1::Wrap0::from( 13.5 ) ).into();
  assert_eq!( got.round(), 14.0 );

}

//

fn single_parameter_test()
{

  single!
  {

    ///
    /// Attribute which is inner.
    ///

    #[ derive( Debug, Clone ) ]
    #[ derive( PartialEq ) ]
    Single : < T >;

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

  // /* test.case( "from Single / into f32" ) */
  // let instance1 : Single< f32 > = ( 13.0 ).into();
  // let got : f32 = instance1.into();
  // assert_eq!( got, 13.0 );
  // let instance1 : Single< f32 > = ( 13.0 ).into();
  // let got = f32::from( instance1 );
  // assert_eq!( got, 13.0 );

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

fn single_parameter_complex_test()
{

  single!
  {

    ///
    /// Attribute which is inner.
    ///

    #[ derive( Debug, Clone ) ]
    #[ derive( PartialEq ) ]
    Single : < T : core::cmp::PartialEq + core::clone::Clone >;

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

  // /* test.case( "from Single / into f32" ) */
  // let instance1 : Single< f32 > = ( 13.0 ).into();
  // let got : f32 = instance1.into();
  // assert_eq!( got, 13.0 );
  // let instance1 : Single< f32 > = ( 13.0 ).into();
  // let got = f32::from( instance1 );
  // assert_eq!( got, 13.0 );

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

fn struct_single_basic_test()
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

  // /* test.case( "from Single / into f32" ) */
  // let instance1 : Single< f32 > = ( 13.0 ).into();
  // let got : f32 = instance1.into();
  // assert_eq!( got, 13.0 );
  // let instance1 : Single< f32 > = ( 13.0 ).into();
  // let got = f32::from( instance1 );
  // assert_eq!( got, 13.0 );

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

fn struct_single_deaf_test()
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

//   // /* test.case( "from Single / into f32" ) */
//   // let instance1 : Single< f32 > = ( 13.0 ).into();
//   // let got : f32 = instance1.into();
//   // assert_eq!( got, 13.0 );
//   // let instance1 : Single< f32 > = ( 13.0 ).into();
//   // let got = f32::from( instance1 );
//   // assert_eq!( got, 13.0 );
//
//   /* test.case( "clone / eq" ) */
//   let instance1 : Single< f32 > = ( 13.0 ).into();
//   let instance2 = instance1.clone();
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );

  /* test.case( "deref" ) */
  let got : Single< f32 > = ( 13.5 ).into();
  assert_eq!( got.round(), 14.0 );

}

//

test_suite!
{

  single_basic,
  single_empty_parameter,
  single_parametrized,
  single_parametrized_complex,
  single_parameter,
  single_parameter_complex,

  struct_single_basic,
  struct_single_deaf,

}
