
/// Tests the `AsRef` derive for a tuple struct with one field.
/// Test Matrix Row: T3.1
#[ test ]
fn as_ref_test()
{

  // AsRef

  let _got = IsTransparent( true );
  let _exp = true;
  a_id!( _got.as_ref(), &_exp );

}
