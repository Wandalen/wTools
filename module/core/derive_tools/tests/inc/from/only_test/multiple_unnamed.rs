#[ test ]
fn from_named()
{
  let got : StructWithManyFields = StructWithManyFields::from((10, true));
  let exp = StructWithManyFields( 10 , true );
  a_id!( got, exp );
}
