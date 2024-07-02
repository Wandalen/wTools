#[ test ]
fn deref()
{
  let a = GenericsLifetimes( &3 );
  let exp = &&3;
  let got = a.deref();
  assert_eq!(got, exp);
}
