//!
//! Demonstrates basic runtime type reflection using `reflect_tools`.
//!
//! This example shows how to:
//! - Reflect on primitive types and collections
//! - Inspect type names, IDs, and properties
//! - Iterate over collection elements
//! - Use the reflection system for type introspection
//!

use reflect_tools::reflect::{ reflect, Entity, Instance };
use std::collections::HashMap;

fn main()
{
  println!( "=== reflect_tools: Runtime Type Reflection ===" );
  println!();

  // Primitive type reflection
  demonstrate_primitive_reflection();
  println!();

  // Collection reflection
  demonstrate_collection_reflection();
  println!();

  // HashMap reflection
  demonstrate_hashmap_reflection();
  println!();

  // Practical use case: generic type inspector
  demonstrate_generic_inspector();
}

/// Demonstrates reflection on primitive types
fn demonstrate_primitive_reflection()
{
  println!( "--- Primitive Type Reflection ---" );

  let number : i32 = 42;
  let reflected = reflect( &number );

  println!( "Type name: {}", reflected.type_name() );
  println!( "Type ID: {:?}", reflected.type_id() );
  println!( "Is container: {}", reflected.is_container() );
  println!( "Length: {}", reflected.len() );
}

/// Demonstrates reflection on Vec collections
fn demonstrate_collection_reflection()
{
  println!( "--- Vec Collection Reflection ---" );

  let numbers = vec![ 10, 20, 30, 40, 50 ];
  let reflected = reflect( &numbers );

  println!( "Type name: {}", reflected.type_name() );
  println!( "Is container: {}", reflected.is_container() );
  println!( "Is ordered: {}", reflected.is_ordered() );
  println!( "Length: {}", reflected.len() );

  println!( "Elements:" );
  for ( idx, element ) in reflected.elements().enumerate()
  {
    println!( "  [{}] key={:?}, type={}", idx, element.key, element.val.type_name() );
  }
}

/// Demonstrates reflection on `HashMap` collections
fn demonstrate_hashmap_reflection()
{
  println!( "--- HashMap Collection Reflection ---" );

  let mut scores = HashMap::new();
  scores.insert( "Alice".to_string(), 95 );
  scores.insert( "Bob".to_string(), 87 );
  scores.insert( "Carol".to_string(), 92 );

  let reflected = reflect( &scores );

  println!( "Type name: {}", reflected.type_name() );
  println!( "Is container: {}", reflected.is_container() );
  println!( "Is ordered: {}", reflected.is_ordered() );
  println!( "Length: {}", reflected.len() );

  println!( "Elements (unordered):" );
  for ( idx, element ) in reflected.elements().enumerate()
  {
    println!( "  [{}] key={:?}, value_type={}", idx, element.key, element.val.type_name() );
  }
}

/// Demonstrates a generic type inspector using reflection
fn demonstrate_generic_inspector()
{
  println!( "--- Generic Type Inspector ---" );

  inspect_any( &42_i32 );
  inspect_any( &"Hello, reflection!" );
  inspect_any( &vec![ 1, 2, 3, 4, 5 ] );
  inspect_any( &Vec::< String >::new() );
}

/// Generic function that inspects any reflectable type
fn inspect_any< T >( value : &T )
where
  T : Instance,
{
  let reflected = reflect( value );
  println!(
    "Type: {} | Container: {} | Elements: {}",
    reflected.type_name(),
    reflected.is_container(),
    reflected.len()
  );
}
