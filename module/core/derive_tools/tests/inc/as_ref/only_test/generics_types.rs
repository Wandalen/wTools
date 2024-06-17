#[ test ]
fn as_ref()
{
  let a = GenericsTypes::< &str >( "boo" );
  let got = &"boo";
  let exp = a.as_ref();
  assert_eq!(got, exp);
}
