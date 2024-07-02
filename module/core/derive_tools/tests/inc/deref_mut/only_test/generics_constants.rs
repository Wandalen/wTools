#[ test ]
fn deref_mut()
{
  let mut a = GenericsConstants::< 0 >( 5 );
  *a = -5;
  let exp = &-5;
  let got = a.deref();
  assert_eq!(got, exp);
}
