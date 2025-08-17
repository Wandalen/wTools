//! Test to verify the boolean assignment fix works correctly
//!
//! ## Test Matrix for Boolean Assignment Fix
//!
//! | ID   | Test Case                           | Expected Output                    |
//! |------|------------------------------------|------------------------------------|
//! | T1.1 | Field-specific setter methods      | Methods work without type ambiguity|
//! | T1.2 | Field-specific builder methods     | Fluent pattern works correctly     |
//! | T1.3 | Explicit Assign trait usage       | Original trait still functional    |
//! | T1.4 | Multiple bool fields handling      | Each field gets specific methods   |
//! | T1.5 | Multiple bool fields fluent        | Fluent pattern with all bool fields|

use component_model::ComponentModel;
use component_model_types::Assign;

#[ derive( Default, ComponentModel, PartialEq, Debug ) ]
struct TestConfig
{
  host : String,
  port : i32,
  enabled : bool,
}

/// Test that field-specific setter methods work correctly
/// Test Combination: T1.1
#[ test ]
fn test_field_specific_assignment_methods()
{
  let mut config = TestConfig::default();
  
  // Use field-specific setter methods to avoid type ambiguity
  config.host_set( "localhost".to_string() );
  config.port_set( 8080i32 );
  config.enabled_set( true );
  
  assert_eq!( config.host, "localhost" );
  assert_eq!( config.port, 8080 );
  assert!( config.enabled );
}

/// Test that field-specific builder methods work for fluent builder pattern
/// Test Combination: T1.2
#[ test ]
fn test_field_specific_impute_methods()
{
  let config = TestConfig::default()
    .host_with( "api.example.com".to_string() )
    .port_with( 3000i32 )
    .enabled_with( false );
    
  assert_eq!( config.host, "api.example.com" );
  assert_eq!( config.port, 3000 );
  assert!( !config.enabled );
}

/// Test that original Assign trait still works with explicit type annotations
/// Test Combination: T1.3
#[ test ]
fn test_explicit_assign_trait_still_works()
{
  let mut config = TestConfig::default();
  
  // Explicit type annotation still works
  Assign::<String, String>::assign( &mut config, "test".to_string() );
  Assign::<i32, i32>::assign( &mut config, 1234i32 );
  Assign::<bool, bool>::assign( &mut config, true );
  
  assert_eq!( config.host, "test" );
  assert_eq!( config.port, 1234 );
  assert!( config.enabled );
}

/// Test with multiple bool fields to ensure only one impl is generated
#[ derive( Default, ComponentModel, PartialEq, Debug ) ]
struct MultiBoolConfig
{
  enabled : bool,
  debug : bool,
  verbose : bool,
}

/// Test multiple bool fields each get their own specific setter methods
/// Test Combination: T1.4
#[ test ]
fn test_multiple_bool_fields_with_field_specific_methods()
{
  let mut config = MultiBoolConfig::default();
  
  // Each bool field gets its own specific method
  config.enabled_set( true );
  config.debug_set( false );  
  config.verbose_set( true );
  
  assert!( config.enabled );
  assert!( !config.debug );
  assert!( config.verbose );
}

/// Test fluent pattern works with multiple bool fields
/// Test Combination: T1.5
#[ test ]
fn test_multiple_bool_fields_fluent_pattern()
{
  let config = MultiBoolConfig::default()
    .enabled_with( true )
    .debug_with( false )
    .verbose_with( true );
    
  assert!( config.enabled );
  assert!( !config.debug );
  assert!( config.verbose );
}