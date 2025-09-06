use super::derives::a_id;

#[ test ]
fn basic_test()
{

  let _got = IsTransparent::default();
  let _exp = IsTransparent( true );
  a_id!( _got, _exp );

  // From

  let _got = IsTransparent::from( true );
  let _exp = IsTransparent( true );
  a_id!( _got, _exp );
  let _got = IsTransparent::from( false );
  let _exp = IsTransparent( false );
  a_id!( _got, _exp );

  // InnerFrom - commented out since InnerFrom derive is not available

  // let got : bool = IsTransparent::from( true ).into();
  // let exp = true;
  // a_id!( got, exp );
  // let got : bool = IsTransparent::from( false ).into();
  // let exp = false;
  // a_id!( got, exp );

  // Deref

  let _got = IsTransparent( true );
  let _exp = true;
  a_id!( *_got, _exp );

  // DerefMut

  let mut _got = IsTransparent( true );
  *_got = false;
  let _exp = false;
  a_id!( *_got, _exp );

  // AsRef

  let _got = IsTransparent( true );
  let _exp = true;
  a_id!( _got.as_ref(), &_exp );

  // AsMut

  let mut _got = IsTransparent( true );
  *_got.as_mut() = false;
  let _exp = false;
  a_id!( _got.0, _exp );

}
