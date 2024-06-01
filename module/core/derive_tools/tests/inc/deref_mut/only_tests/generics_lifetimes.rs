#[ test ]
fn deref_mut()
{
  let mut a = GenericsLifetimes( &3 );
  *a = &-3;
  let exp = &&-3;
  let got = a.deref();
  assert_eq!(got, exp);
}
