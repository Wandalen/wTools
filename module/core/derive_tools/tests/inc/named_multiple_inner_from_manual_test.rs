#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named_fields()
{
  #[ derive( Debug, PartialEq ) ]
  struct StructNamedFields
  {
    a : i32,
    b : bool, 
  }

  impl From< StructNamedFields > for ( i32, bool )
  {
      fn from( src : StructNamedFields ) -> Self { ( src.a, src.b ) }
  }

  let got : ( i32, bool ) = ( 10, true );
  let exp : ( i32, bool ) = StructNamedFields { a : 10 , b : true }.into();
  a_id!( got, exp );
}