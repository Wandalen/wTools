#[ test ]
fn deref()
{
  let a = StructTuple( "boo".into(), 3 );
  let exp = "boo";
  let got = a.deref();
  assert_eq!(got, exp);
}
