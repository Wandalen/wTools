

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

  // o1.options_2_set( &o2 )

  let o1 = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.1 };
  let mut o2 = Options2::default();
  o2.options_2_set( &o1 );
  Options2ComponentsSet::options_2_set( &mut o2, &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );


  // o1.options_2_set( &o2 )

  let o2 = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  let mut o1 = Options1::default();
  o1.options_2_set( &o2 );
  Options2ComponentsSet::options_2_set( &mut o1, &o2 );
  let exp = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 0.0 };
  assert_eq!( o1, exp );


}

#[ test ]
fn components_set_self()
{

  // o1.options_1_set( &o2 )

  let o1 = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.1 };
  let mut o2 = Options1::default();
  o2.options_1_set( &o1 );
  Options1ComponentsSet::options_1_set( &mut o2, &o1 );
  let exp = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.1 };
  assert_eq!( o2, exp );

  // o1.options_2_set( &o2 )

  let o1 = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  let mut o2 = Options2::default();
  o2.options_2_set( &o1 );
  Options2ComponentsSet::options_2_set( &mut o2, &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

}
