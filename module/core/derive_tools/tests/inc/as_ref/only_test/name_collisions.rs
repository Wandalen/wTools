#[ test ]
fn as_ref()
{
  let a = NameCollisions { a : 5, b : "boo".into() };
  let exp = &5;
  let got = a.as_ref();
  assert_eq!(got, exp);
}
