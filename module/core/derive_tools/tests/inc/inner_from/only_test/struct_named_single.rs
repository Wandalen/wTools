#[ test ]
fn inner_from()
{
  let a = StructNamedSingle{ a : 3 };
  let exp = 3;
  let got : i32 = a.into();
  assert_eq!(got, exp);
}
