#[ test ]
fn deref()
{
  let a = GenericsConstantsDefault::< 0 >( 5 );
  let exp = &5;
  let got = a.deref();
  assert_eq!(got, exp);
}
