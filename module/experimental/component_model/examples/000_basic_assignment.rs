//! # 000 - Basic Component Assignment
//! 
//! This example demonstrates the fundamental concept of component assignment -
//! setting struct fields by component type rather than field name.

use component_model ::Assign;

#[ derive( Default, Debug, PartialEq, Assign ) ]
struct Person
{
  age: i32,
  name: String,
}

fn main()
{
  println!( "=== Basic Component Assignment ===" );
  
  let mut person = Person ::default();
  println!( "Initial person: {person:?}" );
  
  // Assign components by type - no field names needed!
  person.assign( 25 );           // Sets age: i32
  person.assign( "Alice" );      // Sets name: String (via Into< String >)
  
  println!( "After assignment: {person:?}" );
  
  // Verify the assignment worked
  assert_eq!( person, Person { age: 25, name: "Alice".to_string() } );
  
  // You can assign again to update values
  person.assign( 30 );
  person.assign( "Bob".to_string() );
  
  println!( "After updates: {person:?}" );
  assert_eq!( person, Person { age: 30, name: "Bob".to_string() } );
  
  println!( "✅ Basic assignment complete!" );
}