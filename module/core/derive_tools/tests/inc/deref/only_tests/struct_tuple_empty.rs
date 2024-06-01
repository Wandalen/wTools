#[ test ]
fn deref()
{
  let a = StructTupleEmpty();
  let exp = &();
  let got = a.deref();
  assert_eq!(got, exp);
}
