
/// Tests the `AsRef` derive for a tuple struct with one field.
/// Test Matrix Row: T3.1
#[ test ]
fn as_ref_test()
{

  // AsRef

  let got = IsTransparent( true );
  let exp = true;
  a_id!( got.as_ref(), &exp );

}
