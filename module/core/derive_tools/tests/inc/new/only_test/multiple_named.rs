#[ test ]
fn from_named()
{
  use mod1::Struct1;

  let got : Struct1 = Struct1::new( 10, true );
  let exp = Struct1{ a : 10 , b : true };
  a_id!( got, exp );
}
