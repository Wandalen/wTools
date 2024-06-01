#[ test ]
fn deref()
{
  let a = EnumNamed::A { a : "boo".into(), b : 3 };
  let exp = "boo";
  let got = a.deref();
  assert_eq!(got, exp);
}
