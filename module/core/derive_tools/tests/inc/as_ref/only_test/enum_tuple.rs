#[ test ]
fn as_ref()
{
  let a = EnumTuple::A( "boo".into(), 3 );
  let exp = "boo";
  let got = a.as_ref();
  assert_eq!(got, exp);
}
