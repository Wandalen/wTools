
#[ test ]
fn as_ref_test()
{

  // AsRef

  let got = IsTransparent( true );
  let exp = true;
  a_id!( got.as_ref(), &exp );

}
