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

    impl From< StructNamedFields > for i32
    {
        fn from( a : StructNamedFields ) -> Self { a.a }
    }

    let got : i32 = StructNamedFields{ a: 10 }.into();
    let exp: i32 = 10;
    a_id!( got, exp );
}