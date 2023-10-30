use derive_tools_meta::InnerFrom;

#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named_fields()
{
  #[ derive( Debug, PartialEq, InnerFrom ) ]
  struct StructNamedFields
  {
    a : i32,
    b : bool, 
  }

  let got : ( i32, bool ) = ( 10, true );
  let exp : ( i32, bool ) = StructNamedFields { a : 10 , b : true }.into();
  a_id!( got, exp );
}