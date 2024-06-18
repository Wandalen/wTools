#[ test ]
fn as_mut()
{
  let mut a = NameCollisions { a : 5, b : "boo".into() };
  *a.as_mut() = -5;
  let exp = &-5;
  let got = &a.a;
  assert_eq!(got, exp);
}
