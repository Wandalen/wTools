#[ test ]
fn as_mut()
{
  let mut a = GenericsTypesDefault( 2 );
  *a.as_mut() = -2;
  let got = &-2;
  let exp = &a.0;
  assert_eq!(got, exp);
}
