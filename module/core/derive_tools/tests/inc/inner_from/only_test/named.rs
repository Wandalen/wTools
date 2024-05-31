#[ test ]
fn inner_from_named() 
{
  let got : i32 = MyStruct{ a: 10 }.into();
  let exp = 10;
  a_id!( got, exp );
}
