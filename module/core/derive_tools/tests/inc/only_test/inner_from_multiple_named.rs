#[ test ]
fn from_inner_named() 
{
  let got : ( i32, bool ) = StructNamedFields{ a: 10, b: true }.into();
  let exp = ( 10 , true );
  a_id!( got, exp );
}
