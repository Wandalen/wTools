#[ test ]
fn deref()
{
  let a = GenericsTypes::< &str >( "boo" );
  let got = &"boo";
  let exp = a.deref();
  assert_eq!(got, exp);
}
