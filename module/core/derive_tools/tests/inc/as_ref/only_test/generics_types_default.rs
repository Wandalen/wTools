#[ test ]
fn as_ref()
{
  let a = GenericsTypesDefault( 2 );
  let got = &2;
  let exp = a.as_ref();
  assert_eq!(got, exp);
}
