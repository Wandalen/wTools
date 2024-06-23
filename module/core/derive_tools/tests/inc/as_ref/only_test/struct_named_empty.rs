#[ test ]
fn as_ref()
{
  let a = StructNamedEmpty{};
  let exp = &();
  let got = a.as_ref();
  assert_eq!(got, exp);
}
