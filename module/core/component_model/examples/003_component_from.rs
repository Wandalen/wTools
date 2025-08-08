//! # 003 - Advanced Assignment
//! 
//! Demonstrates advanced assignment patterns and shows how component model
//! provides type-safe assignment without field name conflicts.

use component_model::Assign;

#[ derive( Default, Debug, PartialEq, Assign ) ]
struct NetworkConfig
{
  host : String,
  port : i32,
}

#[ derive( Default, Debug, PartialEq, Assign ) ]
struct UserProfile
{
  username : String,
  user_id : i32,
}

fn main()
{
  println!( "=== Advanced Assignment Patterns ===" );
  
  // Network configuration
  let mut net_config = NetworkConfig::default();
  net_config.assign( "api.example.com" );
  net_config.assign( 443 );
  println!( "Network config: {:?}", net_config );
  
  // User profile with fluent style
  let user_profile = UserProfile::default()
    .impute( "alice_dev" )
    .impute( 1001 );
  println!( "User profile: {:?}", user_profile );
  
  // Demonstrate type safety - String goes to String field, i32 goes to i32 field
  let mut mixed_config = NetworkConfig::default();
  mixed_config.assign( 8080 );           // Goes to port (i32)
  mixed_config.assign( "localhost" );    // Goes to host (String)
  
  println!( "Mixed assignment: {:?}", mixed_config );
  
  // Show that order doesn't matter due to type-driven assignment
  let user1 = UserProfile::default()
    .impute( "bob_user" )      // String -> username
    .impute( 2002 );           // i32 -> user_id
  
  let user2 = UserProfile::default()
    .impute( 2002 )            // i32 -> user_id  
    .impute( "bob_user" );     // String -> username
  
  // Both should be identical despite different assignment order
  assert_eq!( user1, user2 );
  println!( "Order-independent assignment: {:?} == {:?}", user1, user2 );
  
  // Verify final state
  assert_eq!( mixed_config.host, "localhost" );
  assert_eq!( mixed_config.port, 8080 );
  assert_eq!( user_profile.username, "alice_dev" );
  assert_eq!( user_profile.user_id, 1001 );
  
  println!( "✅ Advanced assignment patterns complete!" );
}