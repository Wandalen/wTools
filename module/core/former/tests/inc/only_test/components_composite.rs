

#[ test ]
fn component_assign()
{

  let mut o1 = Options1::default();
  o1.assign( 42 );
  o1.assign( "Hello, world!" );
  o1.assign( 13.01 );
  println!( "field1: {}, field2: {}", o1.field1, o1.field2 );
  let exp = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.01 };
  assert_eq!( o1, exp );

}

#[ test ]
fn component_assign_with_composite()
{

  // assign( Into::< i32 >::into( &o1 ) )

  let mut o1 = Options1::default();
  o1.assign( 42 );
  o1.assign( "Hello, world!" );
  o1.assign( 13.01 );
  let mut o2 = Options2::default();
  o2.assign( Into::< i32 >::into( &o1 ) );
  o2.assign( Into::< String >::into( &o1 ) );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

  // assign_with_type

  let mut o1 = Options1::default();
  o1.assign( 42 );
  o1.assign( "Hello, world!" );
  o1.assign( 13.01 );
  let mut o2 = Options2::default();
  o2.assign_with_type::< i32, _ >( &o1 );
  o2.assign_with_type::< String, _ >( &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

}

#[ test ]
fn assign()
{

  // o2.assign( &o1 )

  let mut o1 = Options1::default();
  o1.assign( 42 );
  o1.assign( "Hello, world!" );
  o1.assign( 13.01 );
  let mut o2 = Options2::default();
  o2.options_2_assign( &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

  // o1.assign( &o2 )

  let mut o2 = Options2::default();
  o2.assign( 42 );
  o2.assign( "Hello, world!" );
  let mut o1 = Options1::default();
  o1.options_2_assign( &o2 );
  Options2ComponentsAssign::options_2_assign( &mut o1, &o2 );
  let exp = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 0.0 };
  assert_eq!( o1, exp );

}

#[ test ]
fn from_components()
{

  // o2 : Options2 = o1.into()

  let mut o1 = Options1::default();
  o1.assign( 42 );
  o1.assign( "Hello, world!" );
  o1.assign( 13.01 );
  let o2 : Options2 = Into::< Options2 >::into( &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );
  let o2 : Options2 = (&o1).into();
  assert_eq!( o2, exp );

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
