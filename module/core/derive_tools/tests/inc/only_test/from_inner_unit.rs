#[ test ]
fn from_inner_named() 
{
  let got : UnitStruct = UnitStruct::from( () );
  let exp = UnitStruct;
  a_id!( got, exp );
}
