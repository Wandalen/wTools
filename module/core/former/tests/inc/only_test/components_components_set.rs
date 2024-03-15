

#[ test ]
fn component_set()
{

  let mut o2 = Options2::default();
  o2.set( 42 );
  o2.set( "Hello, world!" );
  println!( "field1: {}, field2: {}", o2.field1, o2.field2 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

}

#[ test ]
fn components_set()
{

  // o1.components_set( &o2 )

  let o1 = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.1 };
  let mut o2 = Options2::default();
  o2.components_set( &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

}
