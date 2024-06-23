#[ test ]
fn inner_from()
{
  let a = GenericsConstants::< 0 >( 5 );
  let exp = 5;
  let got : i32 = a.into();
  assert_eq!(got, exp);
}
