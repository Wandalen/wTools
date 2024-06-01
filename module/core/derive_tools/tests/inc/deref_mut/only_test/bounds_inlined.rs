#[ test ]
fn deref_mut()
{
  let mut a = BoundsInlined::< String, i32 >( "boo".into(), 3 );
  *a = "foo".into();
  let exp = "foo";
  let got = a.deref();
  assert_eq!(got, exp);
}
