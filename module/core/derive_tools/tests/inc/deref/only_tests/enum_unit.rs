#[ test ]
fn deref()
{
  let a = EnumUnit::A;
  let exp = &();
  let got = a.deref();
  assert_eq!(got, exp);
}
