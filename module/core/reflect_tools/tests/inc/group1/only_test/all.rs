
#[ test ]
fn basic_test()
{

  let got = IsTransparent::default();
  let exp = IsTransparent( true );
  a_id!( got, exp );

  // From

  let got = IsTransparent::from( true );
  let exp = IsTransparent( true );
  a_id!( got, exp );
  let got = IsTransparent::from( false );
  let exp = IsTransparent( false );
  a_id!( got, exp );

  // InnerFrom

  let got : bool = IsTransparent::from( true ).into();
  let exp = true;
  a_id!( got, exp );
  let got : bool = IsTransparent::from( false ).into();
  let exp = false;
  a_id!( got, exp );

  // Deref

  let got = IsTransparent( true );
  let exp = true;
  a_id!( *got, exp );

  // DerefMut

  let mut got = IsTransparent( true );
  *got = false;
  let exp = false;
  a_id!( *got, exp );

  // AsRef

  let got = IsTransparent( true );
  let exp = true;
  a_id!( got.as_ref(), &exp );

  // AsMut

  let mut got = IsTransparent( true );
  *got.as_mut() = false;
  let exp = false;
  a_id!( got.0, exp );

}
