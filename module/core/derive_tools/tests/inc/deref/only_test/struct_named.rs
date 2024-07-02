#[ test ]
fn deref()
{
  let a = StructNamed{ a : "boo".into(), b : 3 };
  let exp = "boo";
  let got = a.deref();
  assert_eq!(got, exp);
}
