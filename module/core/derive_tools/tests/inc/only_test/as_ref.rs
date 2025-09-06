
/// Tests the `AsRef` derive for a tuple struct with one field.
/// Test Matrix Row: T3.1
#[ test ]
fn as_ref_test()
{

  // AsRef

  a_id!( IsTransparent( true ).as_ref(), &true );

}
