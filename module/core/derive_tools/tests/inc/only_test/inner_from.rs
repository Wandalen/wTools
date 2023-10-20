
#[ test ]
fn from_outer_test()
{
  let got : bool = IsTransparent( true ).into();
  let exp = true;
  a_id!( got, exp );
  let got : bool = IsTransparent( false ).into();
  let exp = false;
  a_id!( got, exp );

  let got: u32 = Age{ age: 18 }.into();
  let exp: u32 = 18;
  a_id!( got, exp );
}
