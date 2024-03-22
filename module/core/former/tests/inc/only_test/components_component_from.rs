

#[ test ]
fn component_assign()
{

  let o1 = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.01 };

  let field1 : i32 = ( &o1 ).into();
  assert_eq!( field1, 42 );

  let field2 : String = ( &o1 ).into();
  assert_eq!( field2, "Hello, world!".to_string() );

  let field3 : f32 = ( &o1 ).into();
  assert_eq!( field3, 13.01 );

}
