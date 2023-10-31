#[ test ]
fn from_inner_named() 
{
    let got : ( i32, bool ) = StructWithManyFields( 10, true ).into();
    let exp = ( 10 , true );
    a_id!( got, exp );
}
