#[ test ]
fn deref()
{
  let a = GenericsConstants::< 0 >( 5 );
  let exp = &5;
  let got = a.deref();
  assert_eq!(got, exp);
}
