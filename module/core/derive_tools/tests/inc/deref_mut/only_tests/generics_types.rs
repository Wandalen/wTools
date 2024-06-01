#[ test ]
fn deref_mut()
{
  let mut a = GenericsTypes::< &str >( "boo" );
  *a = "foo";
  let got = &"foo";
  let exp = a.deref();
  assert_eq!(got, exp);
}
