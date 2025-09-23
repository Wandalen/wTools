//! Minimal test case to demonstrate boolean assignment error

use component_model ::ComponentModel;
use component_model_types ::Assign;

#[ derive( Default, ComponentModel ) ]
struct MinimalConfig 
{
  host: String,
  enabled: bool,
}

#[ test ]
fn test_string_assignment_works() 
{
  let mut config = MinimalConfig ::default();
  config.assign( "localhost".to_string() );  // This works
  assert_eq!( config.host, "localhost" );
}

#[ test ]
fn test_explicit_bool_assignment_works() 
{
  let mut config = MinimalConfig ::default();
  // This works with explicit type annotation :
  Assign :: < bool, bool > ::assign( &mut config, true );
  assert!( config.enabled );
}

// Note: Previously there was a commented-out test here that demonstrated the
// boolean assignment type ambiguity error. This test has been removed as the
// issue has been resolved with field-specific methods (config.enabled_set(true)).