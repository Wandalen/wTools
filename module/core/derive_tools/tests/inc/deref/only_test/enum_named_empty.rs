#[ test ]
fn deref()
{
  let a = EnumNamedEmpty::A {};
  let exp = &();
  let got = a.deref();
  assert_eq!(got, exp);
}
