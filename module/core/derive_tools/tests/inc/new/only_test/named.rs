#[ test ]
fn from_named()
{
  use mod1::Struct1;

  let got : Struct1 = Struct1::new( 13 );
  let exp = Struct1 { a : 13 };
  a_id!( got, exp );
}
