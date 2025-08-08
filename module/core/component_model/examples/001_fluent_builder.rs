//! # 001 - Fluent Builder Pattern
//! 
//! Demonstrates the `impute()` method for fluent, chainable component assignment.
//! Perfect for building configuration objects and immutable-style APIs.

use component_model::Assign;

#[ derive( Default, Debug, PartialEq, Assign ) ]
struct ServerConfig
{
  host : String,
  port : i32,  // Use i32 to avoid conflicts with other numeric types
}

fn main()
{
  println!( "=== Fluent Builder Pattern ===" );
  
  // Traditional mutable approach
  let mut config1 = ServerConfig::default();
  config1.assign( "localhost" );
  config1.assign( 8080 );
  
  println!( "Mutable style: {:?}", config1 );
  
  // Fluent builder style with impute()
  let config2 = ServerConfig::default()
    .impute( "api.example.com" )    // Returns Self for chaining
    .impute( 443 );                 // Chainable
  
  println!( "Fluent style: {:?}", config2 );
  
  // You can mix and match approaches
  let config3 = ServerConfig::default()
    .impute( "staging.example.com" )
    .impute( 8443 );
  
  println!( "Mixed style: {:?}", config3 );
  
  // Verify all configs are different
  assert_ne!( config1, config2 );
  assert_ne!( config2, config3 );
  
  println!( "✅ Fluent builder complete!" );
}