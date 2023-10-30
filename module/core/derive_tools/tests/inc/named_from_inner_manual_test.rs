#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named_fields()
{
  #[ derive( Debug, PartialEq ) ]
  struct StructNamedFields
  {
    a : i32,
  }

  impl From< i32 > for StructNamedFields
  {
    fn from( a : i32 ) -> Self { Self { a } }
  }

  let got : StructNamedFields = StructNamedFields::from( 10 );
  let exp = StructNamedFields { a : 10 };
  a_id!( got, exp );
}