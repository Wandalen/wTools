#[ test ]
fn inner_from_named() 
{
  let got : () = UnitStruct.into();
  let exp = ();
  a_id!( got, exp );
}
