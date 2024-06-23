#[ test ]
fn inner_from()
{
  let a = NameCollisions { a : 5, b : "boo".into() };
  let exp = ( 5, "boo".to_string() );
  let got : ( i32, String ) = a.into();
  assert_eq!(got, exp);
}
