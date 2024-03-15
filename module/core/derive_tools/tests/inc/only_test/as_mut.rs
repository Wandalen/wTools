
#[ test ]
fn as_mut_test()
{

  // AsMut

  let mut got = IsTransparent( true );
  *got.as_mut() = false;
  let exp = false;
  a_id!( got.0, exp );

}
