// qqq2 : move top 2 lines into test.rs file for each file in the dir
#[ test ]
fn from_outer_test()
{

  let got : bool = IsTransparent( true ).into();
  let exp = true;
  a_id!( got, exp );
  let got : bool = IsTransparent( false ).into();
  let exp = false;
  a_id!( got, exp );

}
