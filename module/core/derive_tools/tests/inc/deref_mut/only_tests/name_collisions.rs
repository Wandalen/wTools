#[ test ]
fn deref_mut()
{
  let mut a = NameCollisions { a : 5, b : "boo".into() };
  *a = -5;
  let exp = &-5;
  let got = a.deref();
  assert_eq!(got, exp);
}
