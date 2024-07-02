#[ test ]
fn deref()
{
  let a = EnumTupleEmpty::A();
  let exp = &();
  let got = a.deref();
  assert_eq!(got, exp);
}
