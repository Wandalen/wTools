
#[ test ]
fn from_outer_test()
{
  let age: u32 = Age{age: 18}.into();
  let got : bool = IsTransparent( true ).into();
  let exp = true;
  a_id!( got, exp );
  let got : bool = IsTransparent( false ).into();
  let exp = false;
  a_id!( got, exp );

}
