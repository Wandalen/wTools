#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{

  //

  #[ test ]
  fn parameter_complex()
  {

    TheModule::types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      many Many : < T : core::cmp::PartialEq + core::clone::Clone >;
    }

    /* test.case( "from f32 into Many" ) */
    let instance1 : Many< f32 > = ( 13.0 ).into();
    let instance2 = Many::< f32 >::from( 13.0 );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
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
    TheModule::types!
    {
      many Many : < T >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Many( vec![ mod1::Floats( 13.0, 31.0 ) ] );

  }

  //

  // xxx : extend
  #[ test ]
  fn struct_basic()
  {

    /* test.case( "from f32 into Many" ) */
    let instance1 : TheModule::Many< f32 > = ( 13.0 ).into();
    let instance2 = TheModule::Many::< f32 >::from( 13.0 );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : TheModule::Many< f32 > = ( TheModule::Many::from( 13.0 ) ).into();
    let instance2 = TheModule::Many::< f32 >::from( TheModule::Many::from( 13.0 ) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : TheModule::Many< f32 > = ( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "default" ) */
    let instance1 : TheModule::Many< f32 > = Default::default();
    a_id!( instance1.0, std::vec::Vec::< f32 >::new() );

    /* test.case( "deref" ) */
    let mut got : TheModule::Many< f32 > = ( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );

    /* test.case( "iterate" ) */
    // let mut got : TheModule::Many< f32 > = [ 1.0, 2.0, 3.0 ].into();
    // a_id!( got.len(), 3 );
    // for e in got
    // {
    //   dbg!( e );
    // }
    // a_id!( got.len(), 3 );

    // xxx

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

    /* test.case( "from f32 into Many" ) */
    let instance1 : TheModule::Many< mod1::Floats< f32 > > = ( mod1::Floats( 13.0 ) ).into();
    let instance2 = TheModule::Many::< mod1::Floats< f32 > >::from( mod1::Floats( 13.0 ) );
    a_id!( instance1.0[ 0 ].0, 13.0 );
    a_id!( instance1.len(), 1 );
    a_id!( instance2.0[ 0 ].0, 13.0 );
    a_id!( instance2.len(), 1 );

    /* test.case( "deref" ) */
    let mut got : TheModule::Many< f32 > = ( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );

  }

  // xxx

//   #[ test ]
//   fn problem1()
//   {
//
//     // #[ derive( Clone ) ]
//     pub struct Struct
//     {
//     }
//
//     // trace_macros!( true );
//     // TheModule::types!
//     // {
//     //   pub many Structs : Struct;
//     // }
//     // trace_macros!( false );
//
//     pub struct Structs (pub TheModule :: _Vec < Struct >) ;
//
//     impl core :: ops :: Deref for Structs
//     {
//       type Target = TheModule :: _Vec < Struct > ; #[inline] fn deref(& self) -> &
//       Self :: Target { & self.0 }
//     }
//
//     impl core :: ops :: DerefMut for Structs
//     {
//       #[inline] fn deref_mut(& mut self) -> & mut Self :: Target
//       { & mut self.0 }
//     }
//
//     impl From < Struct > for Structs
//     { #[inline] fn from(src : Struct) -> Self { Self(TheModule :: _vec! [src]) } }
//
//     impl < __FromRef > From < & __FromRef > for Structs where __FromRef : Clone,
//     Self : From < __FromRef >,
//     {
//       #[inline] fn from(src : & __FromRef) -> Self
//       { From :: from((* src).clone()) }
//     }
//
//     impl From < (Struct,) > for Structs
//     {
//       #[inline] fn from(src : (Struct,)) -> Self
//       { Self(TheModule :: _vec! [src.0]) }
//     }
//
//     impl < const N : usize > From < [Struct ; N] >
//     for Structs
//     // where Struct : Clone,
//     {
//       #[inline] fn from(src : [Struct ; N]) -> Self
//       { Self(TheModule :: _Vec :: from(src)) }
//     }
//
//     impl From < & [Struct] > for Structs
//     where Struct : Clone,
//     {
//       // #[inline]
//       fn from(src : & [Struct]) -> Self
//       { Self(TheModule :: _Vec :: from(src)) }
//     }
//
//     impl TheModule :: AsSlice < Struct > for Structs
//     // where Struct : Clone,
//     { #[inline] fn as_slice(& self) -> & [Struct] { & self [..] } }
//
//     impl TheModule :: Make0 for Structs
//     {
//       #[inline] fn make_0() -> Self
//       { Self(TheModule :: _Vec :: < Struct > :: new()) }
//     }
//
//     impl TheModule :: Make1 < Struct > for Structs
//     {
//       #[inline] fn make_1(_0 : Struct,) -> Self
//       { Self(TheModule :: _vec! [_0]) }
//     }
//
//     impl TheModule :: Make2 < Struct, Struct, > for Structs
//     {
//       #[inline] fn make_2(_0 : Struct, _1 : Struct,) -> Self
//       { Self(TheModule :: _vec! [_0, _1]) }
//     }
//
//     impl TheModule :: Make3 < Struct, Struct, Struct, > for Structs
//     {
//       #[inline] fn make_3(_0 : Struct, _1 : Struct, _2 : Struct,) -> Self
//       { Self(TheModule :: _vec! [_0, _1, _2]) }
//     }
//
//   }

}

//

tests_index!
{
  // parameter,
  parameter_complex,
  parameter_no_derives,
  struct_basic,
  struct_no_derives,
  // problem1,
}
