#[ test ]
fn deref()
{
  let a = NameCollisions { a : 5, b : "boo".into() };
  let exp = &5;
  let got = a.deref();
  assert_eq!(got, exp);
}
