use super :: *;


#[ test ]
fn deref()
{
  let a = BoundsMixed :: < String, i32 >( "boo".into(), 3 );
  let exp = "boo";
  let got = &*a;
  assert_eq!(got, exp);
}
