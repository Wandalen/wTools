#[ test ]
fn deref_mut()
{
  let mut a = StructNamed{ a : "boo".into(), b : 3 };
  *a = "foo".into();
  let exp = "foo";
  let got = a.deref();
  assert_eq!(got, exp);
}
