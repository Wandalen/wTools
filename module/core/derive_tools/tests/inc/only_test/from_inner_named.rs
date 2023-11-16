#[ test ]
fn from_inner_named() 
{
  let got : MyStruct = MyStruct::from( 13 );
  let exp = MyStruct { a : 13 };
  a_id!( got, exp );
}
