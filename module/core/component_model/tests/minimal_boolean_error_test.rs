//! Minimal test case to demonstrate boolean assignment error

use component_model::ComponentModel;
use component_model_types::Assign;

#[ derive( Default, ComponentModel ) ]
struct MinimalConfig 
{
  host : String,
  enabled : bool,
}

#[ test ]
fn test_string_assignment_works() 
{
  let mut config = MinimalConfig::default();
  config.assign( "localhost".to_string() );  // This works
  assert_eq!( config.host, "localhost" );
}

#[ test ]
fn test_explicit_bool_assignment_works() 
{
  let mut config = MinimalConfig::default();
  // This works with explicit type annotation:
  Assign::<bool, bool>::assign( &mut config, true );
  assert!( config.enabled );
}

// Uncomment this to see the actual error:
// #[ test ]
// fn test_boolean_assignment_fails() 
// {
//   let mut config = MinimalConfig::default();
//   config.assign( true );  // ERROR: E0283 type annotations needed
// }