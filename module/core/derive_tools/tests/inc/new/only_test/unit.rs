#[ test ]
fn from_named()
{
  let got : UnitStruct = UnitStruct::new();
  let exp = UnitStruct;
  a_id!( got, exp );
}
