#[ test ]
fn inner_from()
{
  let a = GenericsTypes::< &str >( "boo" );
  let exp = "boo";
  let got : &str = a.into();
  assert_eq!(got, exp);
}
