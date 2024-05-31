
#[ test ]
fn from_test()
{

  // let got = IsTransparent::default();
  // let exp = IsTransparent( true );
  // a_id!( got, exp );

  let got = IsTransparent::from( true );
  let exp = IsTransparent( true );
  a_id!( got, exp );
  let got = IsTransparent::from( false );
  let exp = IsTransparent( false );
  a_id!( got, exp );

  // let got : bool = IsTransparent::from( true ).into();
  // let exp = true;
  // a_id!( got, exp );
  // let got : bool = IsTransparent::from( false ).into();
  // let exp = false;
  // a_id!( got, exp );

//   let got = IsTransparent::default();
//   let exp = true;
//   a_id!( *got, exp );
//
//   let mut got = IsTransparent::default();
//   *got = false;
//   let exp = false;
//   a_id!( *got, exp );

}
