

#[ test ]
fn component_set()
{

  let mut o1 = Options1::default();
  o1.set( 42 );
  o1.set( "Hello, world!" );
  o1.set( 13.01 );
  println!( "field1: {}, field2: {}", o1.field1, o1.field2 );
  let exp = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.01 };
  assert_eq!( o1, exp );

}

#[ test ]
fn component_set_with_composite()
{

  // set( Into::< i32 >::into( &o1 ) )

  let mut o1 = Options1::default();
  o1.set( 42 );
  o1.set( "Hello, world!" );
  o1.set( 13.01 );
  let mut o2 = Options2::default();
  o2.set( Into::< i32 >::into( &o1 ) );
  o2.set( Into::< String >::into( &o1 ) );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

  // set_with_type

  let mut o1 = Options1::default();
  o1.set( 42 );
  o1.set( "Hello, world!" );
  o1.set( 13.01 );
  let mut o2 = Options2::default();
  o2.set_with_type::< i32, _ >( &o1 );
  o2.set_with_type::< String, _ >( &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

}

#[ test ]
fn set()
{

  // o2.set( &o1 )

  let mut o1 = Options1::default();
  o1.set( 42 );
  o1.set( "Hello, world!" );
  o1.set( 13.01 );
  let mut o2 = Options2::default();
  o2.options_2_set( &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

  // o1.set( &o2 )

  let mut o2 = Options2::default();
  o2.set( 42 );
  o2.set( "Hello, world!" );
  let mut o1 = Options1::default();
  o1.options_2_set( &o2 );
  Options2ComponentsSet::options_2_set( &mut o1, &o2 );
  let exp = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 0.0 };
  assert_eq!( o1, exp );

}

#[ test ]
fn from_components()
{

  // o2 : Options2 = o1.into()

  let mut o1 = Options1::default();
  o1.set( 42 );
  o1.set( "Hello, world!" );
  o1.set( 13.01 );
  let o2 : Options2 = Into::< Options2 >::into( &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );
  let o2 : Options2 = (&o1).into();
  assert_eq!( o2, exp );

}
