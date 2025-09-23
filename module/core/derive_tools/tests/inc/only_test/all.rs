use super ::derives ::a_id;

#[ test ]
fn basic_test()
{

  a_id!( IsTransparent ::default(), IsTransparent( true ) );

  // From

  a_id!( IsTransparent ::from( true ), IsTransparent( true ) );
  a_id!( IsTransparent ::from( false ), IsTransparent( false ) );

  // InnerFrom - commented out since InnerFrom derive is not available

  // let got: bool = IsTransparent ::from( true ).into();
  // let exp = true;
  // a_id!( got, exp );
  // let got: bool = IsTransparent ::from( false ).into();
  // let exp = false;
  // a_id!( got, exp );

  // Deref

  a_id!( *IsTransparent( true ), true );

  // DerefMut

  {
  let mut got = IsTransparent( true );
  *got = false;
  a_id!( *got, false );
 }

  // AsRef

  a_id!( IsTransparent( true ).as_ref(), &true );

  // AsMut

  {
  let mut got = IsTransparent( true );
  *got.as_mut() = false;
  a_id!( got.0, false );
 }

}
