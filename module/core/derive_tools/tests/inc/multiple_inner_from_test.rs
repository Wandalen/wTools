use derive_tools_meta::InnerFrom;

#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named_fields()
{
    #[ derive( Debug, PartialEq, InnerFrom) ]
    struct StructNamedFields(i32, bool);

    let got : (i32, bool) = (10, true);
    let exp: (i32, bool) = StructNamedFields( 10 , true ).into();
    a_id!( got, exp );
}