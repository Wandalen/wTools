#[ test ]
fn as_mut()
{
  let mut a = GenericsTypes::< &str >( "boo" );
  *a.as_mut() = "foo";
  let got = &"foo";
  let exp = &a.0;
  assert_eq!(got, exp);
}
