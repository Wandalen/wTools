

#[ test ]
fn component_assign()
{

  let mut o2 = Options2::default();
  o2.assign( 42 );
  o2.assign( "Hello, world!" );
  println!( "field1 : {}, field2 : {}", o2.field1, o2.field2 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

}

#[ test ]
fn components_assign()
{

  // o1.options_2_assign( &o2 )

  let o1 = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.1 };
  let mut o2 = Options2::default();
  o2.options_2_assign( &o1 );
  Options2ComponentsAssign::options_2_assign( &mut o2, &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );


  // o1.options_2_assign( &o2 )

  let o2 = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  let mut o1 = Options1::default();
  o1.options_2_assign( &o2 );
  Options2ComponentsAssign::options_2_assign( &mut o1, &o2 );
  let exp = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 0.0 };
  assert_eq!( o1, exp );


}

#[ test ]
fn components_assign_self()
{

  // o1.options_1_assign( &o2 )

  let o1 = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.1 };
  let mut o2 = Options1::default();
  o2.options_1_assign( &o1 );
  Options1ComponentsAssign::options_1_assign( &mut o2, &o1 );
  let exp = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.1 };
  assert_eq!( o2, exp );

  // o1.options_2_assign( &o2 )

  let o1 = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  let mut o2 = Options2::default();
  o2.options_2_assign( &o1 );
  Options2ComponentsAssign::options_2_assign( &mut o2, &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

}
