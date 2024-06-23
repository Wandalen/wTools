#[ test ]
fn as_mut()
{
  let mut a = StructNamed{ a : "boo".into(), b : 3 };
  *a.as_mut() = "foo".into();
  let exp = "foo";
  let got = &a.a;
  assert_eq!(got, exp);
}
