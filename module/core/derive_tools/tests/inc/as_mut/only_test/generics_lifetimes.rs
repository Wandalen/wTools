#[ test ]
fn as_mut()
{
  let mut a = GenericsLifetimes( &3 );
  *a.as_mut() = &-3;
  let exp = &&-3;
  let got = &a.0;
  assert_eq!(got, exp);
}
