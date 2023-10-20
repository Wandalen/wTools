
#[ cfg( feature = "derive_deref" ) ]
#[ test ]
fn deref_test()
{

  // Deref

  let got = IsTransparent( true );
  let exp = true;
  a_id!( *got, exp );

}
