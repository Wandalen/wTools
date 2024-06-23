#[ test ]
fn as_mut()
{
  let mut a = GenericsConstantsDefault::< 0 >( 5 );
  *a.as_mut() = -5;
  let exp = &-5;
  let got = &a.0;
  assert_eq!(got, exp);
}
