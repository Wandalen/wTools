#[ test ]
fn deref_mut()
{
  let mut a = GenericsTypesDefault( 2 );
  *a = -2;
  let got = &-2;
  let exp = a.deref();
  assert_eq!(got, exp);
}
