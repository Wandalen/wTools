
#[ test ]
fn from_test()
{
  use mod1::Struct1;

  let got = Struct1::new( true );
  let exp = Struct1( true );
  a_id!( got, exp );
  let got = Struct1::new( false );
  let exp = Struct1( false );
  a_id!( got, exp );

}
