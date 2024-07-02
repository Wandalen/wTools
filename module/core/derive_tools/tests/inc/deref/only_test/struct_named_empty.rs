#[ test ]
fn deref()
{
  let a = StructNamedEmpty{};
  let exp = &();
  let got = a.deref();
  assert_eq!(got, exp);
}
