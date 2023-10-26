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

    impl From< (i32, bool) > for StructNamedFields
    {
        fn from( src : (i32, bool ) ) -> Self { Self{ a: src.0, b: src.1 } }
    }

    let got : StructNamedFields = StructNamedFields::from((10, true));
    let exp = StructNamedFields{ a : 10 , b : true };
    a_id!( got, exp );
}