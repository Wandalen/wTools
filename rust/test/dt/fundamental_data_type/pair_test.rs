#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
// use TheModule::*;

//

fn basic_test()
{

  mod mod1
  {
    pub use f32;
    pub use f64;
  }

//   pair!
//   {
//
//     ///
//     /// Attribute which is inner.
//     ///
//
//     #[ derive( Debug, Clone ) ]
//     #[ derive( PartialEq ) ]
//     Pair : ( mod1::f32, mod1::f64 );
//
//   }
//
//   /* test.case( "from f32 / into Pair" ) */
//   let instance1 : Pair = ( 13.0, 31.0 ).into();
//   let instance2 = Pair::from( ( 13.0, 31.0 ) );
//   assert_eq!( instance1.0, 13.0 );
//   assert_eq!( instance1.1, 31.0 );
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance2.1, 31.0 );
//   assert_eq!( instance1, instance2 );

//   /* test.case( "from itself / into itself" ) */
//   let instance1 : Pair = ( Pair::from( 13.0 ) ).into();
//   let instance2 = Pair::from( Pair::from( 13.0 ) );
//   assert_eq!( instance1.0, 13.0 );
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "from Pair / into f32" ) */
//   let instance1 : Pair = ( 13.0 ).into();
//   let got : f32 = instance1.into();
//   assert_eq!( got, 13.0 );
//   let instance1 : Pair = ( 13.0 ).into();
//   let got = f32::from( instance1 );
//   assert_eq!( got, 13.0 );
//
//   /* test.case( "clone / eq" ) */
//   let instance1 : Pair = ( 13.0 ).into();
//   let instance2 = instance1.clone();
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "deref" ) */
//   let got : Pair = ( 13.5 ).into();
//   assert_eq!( got.round(), 14.0 );

}

//

// fn empty_parameter_test()
// {
//
//   mod mod1
//   {
//     pub use f32;
//   }
//
//   pair!
//   {
//
//     ///
//     /// Attribute which is inner.
//     ///
//
//     #[ derive( Debug, Clone ) ]
//     #[ derive( PartialEq ) ]
//     Pair : mod1::f32<>;
//
//   }
//
//   /* test.case( "from f32 / into Pair" ) */
//   let instance1 : Pair = ( 13.0 ).into();
//   let instance2 = Pair::from( 13.0 );
//   assert_eq!( instance1.0, 13.0 );
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "from itself / into itself" ) */
//   let instance1 : Pair = ( Pair::from( 13.0 ) ).into();
//   let instance2 = Pair::from( Pair::from( 13.0 ) );
//   assert_eq!( instance1.0, 13.0 );
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "from Pair / into f32" ) */
//   let instance1 : Pair = ( 13.0 ).into();
//   let got : f32 = instance1.into();
//   assert_eq!( got, 13.0 );
//   let instance1 : Pair = ( 13.0 ).into();
//   let got = f32::from( instance1 );
//   assert_eq!( got, 13.0 );
//
//   /* test.case( "clone / eq" ) */
//   let instance1 : Pair = ( 13.0 ).into();
//   let instance2 = instance1.clone();
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "deref" ) */
//   let got : Pair = ( 13.5 ).into();
//   assert_eq!( got.round(), 14.0 );
//
// }
//
// //
//
// fn parametrized_test()
// {
//
//   mod mod1
//   {
//
//     #[ derive( Debug, Clone, PartialEq ) ]
//     pub struct Wrap0< T >
//     (
//       pub T,
//     );
//
//     impl< T > core::ops::Deref
//     for Wrap0< T >
//     {
//       type Target = T;
//       fn deref( &self ) -> &Self::Target
//       {
//         &self.0
//       }
//     }
//
//     impl< T > From< T > for Wrap0< T >
//     {
//       fn from( src : T ) -> Self
//       {
//         Self( src )
//       }
//     }
//
//   }
//
//   pair!
//   {
//
//     ///
//     /// Attribute which is inner.
//     ///
//
//     #[ derive( Debug, Clone ) ]
//     #[ derive( PartialEq ) ]
//     Pair : mod1::Wrap0< T >;
//
//   }
//
//   /* test.case( "from f32 / into Pair" ) */
//   let instance1 : Pair< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
//   let instance2 = Pair::< f32 >::from( mod1::Wrap0::from( 13.0 ) );
//   assert_eq!( instance1.0.0, 13.0 );
//   assert_eq!( instance2.0.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "from itself / into itself" ) */
//   let instance1 : Pair< f32 > = ( Pair::from( mod1::Wrap0::from( 13.0 ) ) ).into();
//   let instance2 = Pair::< f32 >::from( Pair::from( mod1::Wrap0::from( 13.0 ) ) );
//   assert_eq!( instance1.0.0, 13.0 );
//   assert_eq!( instance2.0.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "from Pair / into f32" ) */
//   let instance1 : Pair< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
//   let got : mod1::Wrap0< f32 > = instance1.into();
//   assert_eq!( got.0, 13.0 );
//   let instance1 : Pair< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
//   let got = mod1::Wrap0::< f32 >::from( instance1 );
//   assert_eq!( got.0, 13.0 );
//
//   /* test.case( "clone / eq" ) */
//   let instance1 : Pair< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
//   let instance2 = instance1.clone();
//   assert_eq!( instance2.0, mod1::Wrap0::from( 13.0 ) );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "deref" ) */
//   let got : Pair< f32 > = ( mod1::Wrap0::from( 13.5 ) ).into();
//   assert_eq!( got.round(), 14.0 );
//
// }
//
// //
//
// fn parametrized_complex_test()
// {
//
//   mod mod1
//   {
//
//     #[ derive( Debug, Clone, PartialEq ) ]
//     pub struct Wrap0< T : PartialEq + Copy >
//     (
//       pub T,
//     );
//
//     impl< T : PartialEq + Copy > core::ops::Deref
//     for Wrap0< T >
//     {
//       type Target = T;
//       fn deref( &self ) -> &Self::Target
//       {
//         &self.0
//       }
//     }
//
//     impl< T : PartialEq + Copy > From< T > for Wrap0< T >
//     {
//       fn from( src : T ) -> Self
//       {
//         Self( src )
//       }
//     }
//
//   }
//
//   pair!
//   {
//
//     ///
//     /// Attribute which is inner.
//     ///
//
//     #[ derive( Debug, Clone ) ]
//     #[ derive( PartialEq ) ]
//     Pair : mod1::Wrap0< T : PartialEq + std::marker::Copy >;
//
//   }
//
//   /* test.case( "from f32 / into Pair" ) */
//   let instance1 : Pair< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
//   let instance2 = Pair::< f32 >::from( mod1::Wrap0::from( 13.0 ) );
//   assert_eq!( instance1.0.0, 13.0 );
//   assert_eq!( instance2.0.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "from itself / into itself" ) */
//   let instance1 : Pair< f32 > = ( Pair::from( mod1::Wrap0::from( 13.0 ) ) ).into();
//   let instance2 = Pair::< f32 >::from( Pair::from( mod1::Wrap0::from( 13.0 ) ) );
//   assert_eq!( instance1.0.0, 13.0 );
//   assert_eq!( instance2.0.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "from Pair / into f32" ) */
//   let instance1 : Pair< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
//   let got : mod1::Wrap0< f32 > = instance1.into();
//   assert_eq!( got.0, 13.0 );
//   let instance1 : Pair< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
//   let got = mod1::Wrap0::< f32 >::from( instance1 );
//   assert_eq!( got.0, 13.0 );
//
//   /* test.case( "clone / eq" ) */
//   let instance1 : Pair< f32 > = ( mod1::Wrap0::from( 13.0 ) ).into();
//   let instance2 = instance1.clone();
//   assert_eq!( instance2.0, mod1::Wrap0::from( 13.0 ) );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "deref" ) */
//   let got : Pair< f32 > = ( mod1::Wrap0::from( 13.5 ) ).into();
//   assert_eq!( got.round(), 14.0 );
//
// }
//
// //
//
// fn parameter_test()
// {
//
//   pair!
//   {
//
//     ///
//     /// Attribute which is inner.
//     ///
//
//     #[ derive( Debug, Clone ) ]
//     #[ derive( PartialEq ) ]
//     Pair : < T >;
//
//   }
//
//   /* test.case( "from f32 / into Pair" ) */
//   let instance1 : Pair< f32 > = ( 13.0 ).into();
//   let instance2 = Pair::< f32 >::from( 13.0 );
//   assert_eq!( instance1.0, 13.0 );
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "from itself / into itself" ) */
//   let instance1 : Pair< f32 > = ( Pair::from( 13.0 ) ).into();
//   let instance2 = Pair::< f32 >::from( Pair::from( 13.0 ) );
//   assert_eq!( instance1.0, 13.0 );
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   // /* test.case( "from Pair / into f32" ) */
//   // let instance1 : Pair< f32 > = ( 13.0 ).into();
//   // let got : f32 = instance1.into();
//   // assert_eq!( got, 13.0 );
//   // let instance1 : Pair< f32 > = ( 13.0 ).into();
//   // let got = f32::from( instance1 );
//   // assert_eq!( got, 13.0 );
//
//   /* test.case( "clone / eq" ) */
//   let instance1 : Pair< f32 > = ( 13.0 ).into();
//   let instance2 = instance1.clone();
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "deref" ) */
//   let got : Pair< f32 > = ( 13.5 ).into();
//   assert_eq!( got.round(), 14.0 );
//
// }
//
// //
//
// fn parameter_complex_test()
// {
//
//   pair!
//   {
//
//     ///
//     /// Attribute which is inner.
//     ///
//
//     #[ derive( Debug, Clone ) ]
//     #[ derive( PartialEq ) ]
//     Pair : < T : core::cmp::PartialEq + core::clone::Clone >;
//
//   }
//
//   /* test.case( "from f32 / into Pair" ) */
//   let instance1 : Pair< f32 > = ( 13.0 ).into();
//   let instance2 = Pair::< f32 >::from( 13.0 );
//   assert_eq!( instance1.0, 13.0 );
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "from itself / into itself" ) */
//   let instance1 : Pair< f32 > = ( Pair::from( 13.0 ) ).into();
//   let instance2 = Pair::< f32 >::from( Pair::from( 13.0 ) );
//   assert_eq!( instance1.0, 13.0 );
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   // /* test.case( "from Pair / into f32" ) */
//   // let instance1 : Pair< f32 > = ( 13.0 ).into();
//   // let got : f32 = instance1.into();
//   // assert_eq!( got, 13.0 );
//   // let instance1 : Pair< f32 > = ( 13.0 ).into();
//   // let got = f32::from( instance1 );
//   // assert_eq!( got, 13.0 );
//
//   /* test.case( "clone / eq" ) */
//   let instance1 : Pair< f32 > = ( 13.0 ).into();
//   let instance2 = instance1.clone();
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "deref" ) */
//   let got : Pair< f32 > = ( 13.5 ).into();
//   assert_eq!( got.round(), 14.0 );
//
// }
//
// //
//
// fn struct_basic_test()
// {
//
//   /* test.case( "from f32 / into Pair" ) */
//   let instance1 : Pair< f32 > = ( 13.0 ).into();
//   let instance2 = Pair::< f32 >::from( 13.0 );
//   assert_eq!( instance1.0, 13.0 );
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "from itself / into itself" ) */
//   let instance1 : Pair< f32 > = ( Pair::from( 13.0 ) ).into();
//   let instance2 = Pair::< f32 >::from( Pair::from( 13.0 ) );
//   assert_eq!( instance1.0, 13.0 );
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   // /* test.case( "from Pair / into f32" ) */
//   // let instance1 : Pair< f32 > = ( 13.0 ).into();
//   // let got : f32 = instance1.into();
//   // assert_eq!( got, 13.0 );
//   // let instance1 : Pair< f32 > = ( 13.0 ).into();
//   // let got = f32::from( instance1 );
//   // assert_eq!( got, 13.0 );
//
//   /* test.case( "clone / eq" ) */
//   let instance1 : Pair< f32 > = ( 13.0 ).into();
//   let instance2 = instance1.clone();
//   assert_eq!( instance2.0, 13.0 );
//   assert_eq!( instance1, instance2 );
//
//   /* test.case( "deref" ) */
//   let got : Pair< f32 > = ( 13.5 ).into();
//   assert_eq!( got.round(), 14.0 );
//
// }
//
// //
//
// fn struct_deaf_test()
// {
//
//   struct Pair0< T >( pub T );
//
//   impl< T > Pair0< T >
//   {
//     pub fn new( src : T ) -> Self
//     { Self( src ) }
//   }
//
//   /* test.case( "from f32 / into Pair" ) */
//   let instance1 : Pair< Pair0< f32 > > = ( Pair0( 13.0 ) ).into();
//   let instance2 = Pair::< Pair0< f32 > >::from( Pair0( 13.0 ) );
//   assert_eq!( instance1.0.0, 13.0 );
//   assert_eq!( instance2.0.0, 13.0 );
//
//   /* test.case( "from itself / into itself" ) */
//   let val = Pair0::< f32 >::new( 13.0 );
//   let instance1 : Pair< Pair0< f32 > > = ( Pair::from( val ) ).into();
//   let instance2 = Pair::< Pair0< f32 > >::from( Pair::from( Pair0( 13.0 ) ) );
//   assert_eq!( instance1.0.0, 13.0 );
//   assert_eq!( instance2.0.0, 13.0 );
//
//   /* test.case( "deref" ) */
//   let got : Pair< f32 > = ( 13.5 ).into();
//   assert_eq!( got.round(), 14.0 );
//
// }

//

test_suite!
{

  basic,
//   empty_parameter,
//   parametrized,
//   parametrized_complex,
//   parameter,
//   parameter_complex,
//
//   struct_basic,
//   struct_deaf,

}
