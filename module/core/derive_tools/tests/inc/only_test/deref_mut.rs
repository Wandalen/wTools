
#[ cfg( feature = "derive_deref_mut" ) ]
#[ test ]
fn deref_mut_test()
{

  // Deref

  let got = IsTransparent( true );
  let exp = true;
  a_id!( *got, exp );

  // DerefMut

  let mut got = IsTransparent( true );
  *got = false;
  let exp = false;
  a_id!( *got, exp );

}
