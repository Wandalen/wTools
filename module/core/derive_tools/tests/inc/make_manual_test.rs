#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn make_named_fields()
{

  #[ derive( Debug, PartialEq ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  impl TheModule::wtools::Make0 for StructNamedFields
  {
    fn make_0() -> Self
    {
      let a = Default::default();
      let b = Default::default();
      let c = Default::default();
      let d = Default::default();
      Self{ a, b, c, d }
    }
  }

  impl TheModule::wtools::Make1< i32 > for StructNamedFields
  {
    fn make_1( a : i32 ) -> Self { Self{ a, b : a, c : a, d : a } }
  }

  impl TheModule::wtools::Make2< i32, i32 > for StructNamedFields
  {
    fn make_2( a : i32, b : i32 ) -> Self { Self{ a, b, c : b, d : b } }
  }

  impl TheModule::wtools::Make3< i32, i32, i32 > for StructNamedFields
  {
    fn make_3( a : i32, b : i32, c : i32 ) -> Self { Self{ a, b, c, d : c } }
  }

  include!( "./only_test/make_named_fields.rs" );
}

//

#[ test ]
fn make_tuple()
{

  #[ derive( Debug, PartialEq ) ]
  struct StructTuple( i32, i32, i32, i32 );

  impl TheModule::wtools::Make0 for StructTuple
  {
    fn make_0() -> Self
    {
      let a = Default::default();
      let b = Default::default();
      let c = Default::default();
      let d = Default::default();
      Self( a, b, c, d )
    }
  }

  impl TheModule::wtools::Make1< i32 > for StructTuple
  {
    fn make_1( a : i32 ) -> Self { Self( a, a, a, a ) }
  }

  impl TheModule::wtools::Make2< i32, i32 > for StructTuple
  {
    fn make_2( a : i32, b : i32 ) -> Self { Self( a, b, b, b ) }
  }

  impl TheModule::wtools::Make3< i32, i32, i32 > for StructTuple
  {
    fn make_3( a : i32, b : i32, c : i32 ) -> Self { Self( a, b, c, c ) }
  }

  include!( "./only_test/make_tuple.rs" );
}

//
