
#[ test ]
fn deref()
{
  let a = GenericsTypesDefault( 2 );
  let got = &2;
  let exp = &*a;
  assert_eq!(got, exp);
}
