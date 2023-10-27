#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named_fields()
{
    #[ derive( Debug, PartialEq ) ]
    struct StructNamedFields( i32, bool); 

    impl From< StructNamedFields > for (i32, bool)
    {
        fn from( src : StructNamedFields ) -> Self { (src.0, src.1 ) }
    }

    let got : (i32, bool) = (10, true);
    let exp: (i32, bool) = StructNamedFields(10 , true ).into();
    a_id!( got, exp );
}