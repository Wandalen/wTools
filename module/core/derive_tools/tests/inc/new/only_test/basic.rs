
#[ test ]
fn from_test()
{

  let got = IsTransparent::new( true );
  let exp = IsTransparent( true );
  a_id!( got, exp );
  let got = IsTransparent::new( false );
  let exp = IsTransparent( false );
  a_id!( got, exp );

}
