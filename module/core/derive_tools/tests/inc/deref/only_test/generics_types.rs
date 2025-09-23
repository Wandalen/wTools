use super :: *;


#[ test ]
fn deref()
{
  let a = GenericsTypes :: < &str >( "boo" );
  let got = &"boo";
  let exp = &*a;
  assert_eq!(got, exp);
}
