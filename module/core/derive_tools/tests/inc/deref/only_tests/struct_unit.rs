#[ test ]
fn deref()
{
  let a = StructUnit;
  let exp = &();
  let got = a.deref();
  assert_eq!(got, exp);
}
