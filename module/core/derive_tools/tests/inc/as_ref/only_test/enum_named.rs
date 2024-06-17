#[ test ]
fn as_ref()
{
  let a = EnumNamed::A { a : "boo".into(), b : 3 };
  let exp = "boo";
  let got = a.as_ref();
  assert_eq!(got, exp);
}
