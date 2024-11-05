#[ test ]
fn from_named()
{
  use mod1::Struct1;

  let got : Struct1 = Struct1::new();
  let exp = Struct1;
  a_id!( got, exp );
}
