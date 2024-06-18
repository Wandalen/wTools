#[ test ]
fn as_mut()
{
  let mut a = StructTuple( "boo".into(), 3 );
  *a.as_mut() = "foo".into();
  let exp = "foo";
  let got = &a.0;
  assert_eq!(got, exp);
}
