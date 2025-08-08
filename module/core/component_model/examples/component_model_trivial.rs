//! # Component Model - Quick Start Example
//! 
//! This is the simplest possible example showing component model in action.
//! Run this with: `cargo run --example component_model_trivial`

use component_model::Assign;

#[ derive( Default, Debug, PartialEq, Assign ) ]
struct Person
{
  name : String,
  age : i32,
}

fn main()
{
  println!( "🚀 Component Model Quick Start" );
  
  // Create and configure using type-driven assignment
  let person = Person::default()
    .impute( "Alice" )    // Sets String field (name)
    .impute( 25 );        // Sets i32 field (age)
  
  println!( "Created person: {person:?}" );
  assert_eq!( person, Person { name : "Alice".to_string(), age : 25 } );
  
  println!( "✅ Component model working perfectly!" );
}
