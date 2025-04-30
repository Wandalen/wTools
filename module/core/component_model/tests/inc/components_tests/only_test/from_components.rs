
#[ test ]
fn from_components()
{

  // o2 : Options2 = o1.into()

  let o1 = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.01 };
  let o2 : Options2 = Into::< Options2 >::into( &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );
  let o2 : Options2 = (&o1).into();
  assert_eq!( o2, exp );

}
