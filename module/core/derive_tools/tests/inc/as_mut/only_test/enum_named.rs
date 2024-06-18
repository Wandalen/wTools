#[ test ]
fn as_mut()
{
  let mut a = EnumNamed::A { a : "boo".into(), b : 3 };
  *a.as_mut() = "foo".into();
  let exp = "foo";
  let got = match &a {
    EnumNamed::A{ a, .. } => a,
    _ => panic!(),
  };
  assert_eq!(got, exp);
}
