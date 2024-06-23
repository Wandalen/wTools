#[ test ]
fn as_ref()
{
  let a = GenericsLifetimes( &3 );
  let exp = &&3;
  let got = a.as_ref();
  assert_eq!(got, exp);
}
