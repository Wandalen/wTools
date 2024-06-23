#[ test ]
fn as_mut()
{
  let mut a = EnumTuple::A( "boo".into(), 3 );
  *a.as_mut() = "foo".into();
  let exp = "foo";
  let got = match &a {
    EnumTuple::A( a, _ ) => a,
    _ => panic!(),
  };
  assert_eq!(got, exp);
}
