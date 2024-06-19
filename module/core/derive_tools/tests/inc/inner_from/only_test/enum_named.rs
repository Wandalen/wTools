#[ test ]
fn inner_from()
{
  let a = EnumNamed::A { a : "boo".into(), b : 3 };
  let exp = ( "boo".to_string(), 3 );
  let got : ( String, i32 ) = a.into();
  assert_eq!(got, exp);
}
