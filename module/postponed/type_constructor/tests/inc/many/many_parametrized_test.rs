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
      many Many : mod1::f32;

    }
    // trace_macros!( false );

    /* test.case( "from f32 into Many" ) */
    let instance1 : Many = [ 13.0 ].into();
    let instance2 = Many::from([ 13.0 ]);
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from itself into itself" ) */
    let instance1 : Many = ( Many::from([ 13.0 ]) ).into();
    let instance2 = Many::from( Many::from([ 13.0 ]) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Many = [ 13.0 ].into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many = [ 13.0 ].into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );

  }

  //

  fn empty_parameter()
  {

    mod mod1
    {
      pub use f32;
    }

    // trace_macros!( true );
    the_module::types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      many Many : mod1::f32<>;
    }
    // trace_macros!( false );

    /* test.case( "from f32 into Many" ) */
    let instance1 : Many = [ 13.0 ].into();
    let instance2 = Many::from([ 13.0 ]);
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

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
      many Many : mod1::Float;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Many( vec![ mod1::Float( 13.0 ) ] );

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
      many Many : mod1::Floats< T1, T2 >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Many::< f32, f64 >( vec![ mod1::Floats( 13.0, 31.0 ) ] );

  }

  // zzz

//   fn problem1()
//   {
//
//     // #[ derive( Clone ) ]
//     pub struct Struct
//     {
//     }
//
//     // trace_macros!( true );
//     // the_module::types!
//     // {
//     //   pub many Structs : Struct;
//     // }
//     // trace_macros!( false );
//
//     pub struct Structs (pub the_module :: _Vec < Struct >) ;
//
//     impl core :: ops :: Deref for Structs
//     {
//       type Target = the_module :: _Vec < Struct > ; #[ inline ] fn deref(& self) -> &
//       Self :: Target { & self.0 }
//     }
//
//     impl core :: ops :: DerefMut for Structs
//     {
//       #[ inline ] fn deref_mut(& mut self) -> & mut Self :: Target
//       { & mut self.0 }
//     }
//
//     impl From < Struct > for Structs
//     { #[ inline ] fn from(src : Struct) -> Self { Self(the_module :: _vec! [src]) } }
//
//     impl < __FromRef > From < & __FromRef > for Structs where __FromRef : Clone,
//     Self : From < __FromRef >,
//     {
//       #[ inline ] fn from(src : & __FromRef) -> Self
//       { From :: from((* src).clone()) }
//     }
//
//     impl From < (Struct,) > for Structs
//     {
//       #[ inline ] fn from(src : (Struct,)) -> Self
//       { Self(the_module :: _vec! [src.0]) }
//     }
//
//     impl < const N : usize > From < [Struct ; N] >
//     for Structs
//     // where Struct : Clone,
//     {
//       #[ inline ] fn from(src : [Struct ; N]) -> Self
//       { Self(the_module :: _Vec :: from(src)) }
//     }
//
//     impl From < & [Struct] > for Structs
//     where Struct : Clone,
//     {
//       // #[ inline ]
//       fn from(src : & [Struct]) -> Self
//       { Self(the_module :: _Vec :: from(src)) }
//     }
//
//     impl the_module :: AsSlice < Struct > for Structs
//     // where Struct : Clone,
//     { #[ inline ] fn as_slice(& self) -> & [Struct] { & self [..] } }
//
//     impl the_module :: From_0 for Structs
//     {
//       #[ inline ] fn from_0() -> Self
//       { Self(the_module :: _Vec :: < Struct > :: new()) }
//     }
//
//     impl the_module :: From_1 < Struct > for Structs
//     {
//       #[ inline ] fn from_1(_0 : Struct,) -> Self
//       { Self(the_module :: _vec! [_0]) }
//     }
//
//     impl the_module :: From_2 < Struct, Struct, > for Structs
//     {
//       #[ inline ] fn from_2(_0 : Struct, _1 : Struct,) -> Self
//       { Self(the_module :: _vec! [_0, _1]) }
//     }
//
//     impl the_module :: From_3 < Struct, Struct, Struct, > for Structs
//     {
//       #[ inline ] fn from_3(_0 : Struct, _1 : Struct, _2 : Struct,) -> Self
//       { Self(the_module :: _vec! [_0, _1, _2]) }
//     }
//
//   }

  //


  //

  fn multiple()
  {
    use core::fmt;

    the_module::types!
    {

      many Many1 : f32;

      #[ derive( Debug ) ]
      #[ derive( PartialEq, Clone ) ]
      many Many2 : f32;

    }

    /* test.case( "from f32 into Many2" ) */
    let instance1 : Many1 = [ 13.0 ].into();
    let instance2 = Many1::from( core::iter::once( 13.0 ) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    assert!( !implements!( instance1 => PartialEq ) );
    assert!( !implements!( instance1 => Clone ) );
    assert!( !implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from f32 into Many2" ) */
    let instance1 : Many2 = [ 13.0 ].into();
    let instance2 = Many2::from( core::iter::once( 13.0 ) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "clone / eq" ) */
    let instance1 : Many2 = [ 13.0 ].into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

  }

  //

  fn samples()
  {

    // let slice = &[ 1, 2, 3 ][ .. ];
    // for e in slice
    // {
    //   inspect_type::inspect_type_of!( e );
    //   // dbg!( e );
    // }

    /* test.case( "single-line" ) */
    {
      the_module::types!( many MyMany : i32 );
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
  no_parameter_no_derive,
  parametrized_no_derives,
  multiple,
  samples,
}
