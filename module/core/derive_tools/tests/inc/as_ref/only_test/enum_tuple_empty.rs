#[ test ]
fn as_ref()
{
  let a = EnumTupleEmpty::A();
  let exp = &();
  let got = a.as_ref();
  assert_eq!(got, exp);
}
