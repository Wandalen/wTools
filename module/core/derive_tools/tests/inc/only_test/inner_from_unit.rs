#[ test ]
fn inner_from_named() 
{
  let s = UnitStruct;
  let got : () = s.into();
  let exp = ();
  a_id!( got, exp );
}
