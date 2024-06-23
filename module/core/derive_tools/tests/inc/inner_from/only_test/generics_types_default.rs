#[ test ]
fn inner_from()
{
  let a = GenericsTypesDefault( 2 );
  let exp = 2;
  let got : i32 = a.into();
  assert_eq!(got, exp);
}
