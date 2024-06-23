#[ test ]
fn as_ref()
{
  let a = GenericsConstants::< 0 >( 5 );
  let exp = &5;
  let got = a.as_ref();
  assert_eq!(got, exp);
}
