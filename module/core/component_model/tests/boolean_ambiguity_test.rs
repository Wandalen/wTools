//! Comprehensive tests to prevent regression while fixing boolean assignment type ambiguity
//!
//! ## Test Matrix for Boolean Ambiguity Prevention
//!
//! | ID   | Test Case                           | Expected Output                      |
//! |------|-------------------------------------|--------------------------------------|
//! | T2.1 | Non-boolean assignments work        | String/i32 assignments successful   |
//! | T2.2 | Fluent builder non-boolean         | Fluent pattern with non-bool types   |
//! | T2.3 | Multiple bool single impl          | Only one bool impl generated        |
//! | T2.4 | Distinct types work normally       | Custom types assign without conflict |
//! | T2.5 | Single bool field explicit assign  | Explicit type annotations work       |
//! | T2.6 | Explicit type workaround           | Manual Assign trait usage works     |
//! | T2.7 | Fluent with explicit types         | Fluent builder with explicit types  |

use component_model::ComponentModel;
use component_model_types::Assign;

// Test struct with unique types - this currently has type ambiguity for bool
#[ derive( Default, ComponentModel, PartialEq, Debug ) ]
struct ConfigWithUniqueTypes
{
  host : String,
  port : i32,
  enabled : bool,
}

// Test struct with multiple bool fields - should only generate one bool impl
#[ derive( Default, ComponentModel, PartialEq, Debug ) ]
struct ConfigWithMultipleBools
{
  enabled : bool,
  debug : bool,
  verbose : bool,
}

// Custom type to avoid conversion conflicts
#[ derive( Default, PartialEq, Debug, Clone ) ]
struct CustomType( String );

impl From< &str > for CustomType {
  fn from( s : &str ) -> Self { CustomType( s.to_string() ) }
}

// Test struct with completely distinct types
#[ derive( Default, ComponentModel, PartialEq, Debug ) ]
struct ConfigWithDistinctTypes
{
  host : String,
  port : i32,
  custom : CustomType,
}

// Test struct with single bool field
#[ derive( Default, ComponentModel, PartialEq, Debug ) ]
struct ConfigSingleBool
{
  enabled : bool,
}

/// Test that non-boolean assignments work correctly (regression prevention)
/// Test Combination: T2.1
#[ test ]
fn test_non_boolean_assignment_still_works()
{
  let mut config = ConfigWithUniqueTypes::default();
  
  // String assignment should work
  config.assign( "localhost".to_string() );
  assert_eq!( config.host, "localhost" );
  
  // i32 assignment should work  
  config.assign( 8080i32 );
  assert_eq!( config.port, 8080 );
}

/// Test fluent builder pattern with non-booleans (regression prevention)
/// Test Combination: T2.2
#[ test ]
fn test_fluent_builder_non_boolean()
{
  let config = ConfigWithUniqueTypes::default()
    .impute( "api.example.com".to_string() )
    .impute( 3000i32 );
    
  assert_eq!( config.host, "api.example.com" );
  assert_eq!( config.port, 3000 );
}

/// Test that structs with multiple bool fields only generate one bool implementation
/// Test Combination: T2.3
#[ test ]
fn test_multiple_bool_fields_generate_single_impl()
{
  let mut config = ConfigWithMultipleBools::default();
  
  // Should work - only one Assign<bool, _> implementation exists
  config.assign( true );
  // We can't test which field got set without checking all, but it should compile
}

/// Test struct with distinct types works normally
/// Test Combination: T2.4
#[ test ]
fn test_struct_with_distinct_types()
{
  let mut config = ConfigWithDistinctTypes::default();
  
  config.assign( "localhost".to_string() );
  config.assign( 8080i32 );
  config.assign( CustomType::from( "test" ) );
  
  assert_eq!( config.host, "localhost" );
  assert_eq!( config.port, 8080 );
  assert_eq!( config.custom.0, "test" );
}

/// Test single bool field struct
/// Test Combination: T2.5
#[ test ]
fn test_single_bool_field()
{
  let mut config = ConfigSingleBool::default();
  
  // This should work with explicit type annotation
  Assign::<bool, bool>::assign( &mut config, true );
  assert!( config.enabled );
}

/// Test that explicit type annotations work as a workaround
/// Test Combination: T2.6
#[ test ]
fn test_explicit_type_annotation_workaround()
{
  let mut config = ConfigWithUniqueTypes::default();
  
  // Explicit assignment should work
  Assign::<String, String>::assign( &mut config, "test".to_string() );
  Assign::<i32, i32>::assign( &mut config, 1234i32 );
  Assign::<bool, bool>::assign( &mut config, true );
  
  assert_eq!( config.host, "test" );
  assert_eq!( config.port, 1234 );
  assert!( config.enabled );
}

/// Test fluent pattern with explicit types
/// Test Combination: T2.7
#[ test ]
fn test_fluent_with_explicit_types()
{
  let config = ConfigWithUniqueTypes::default()
    .impute( "test".to_string() )
    .impute( 9999i32 );
    // Note: Can't use .impute(bool) due to same ambiguity
  
  assert_eq!( config.host, "test" );
  assert_eq!( config.port, 9999 );
  
  // But we can assign bool afterwards with explicit type
  let mut config = config;
  Assign::<bool, bool>::assign( &mut config, true );
  assert!( config.enabled );
}

// Note: Previously there were commented-out tests here that demonstrated the
// boolean assignment type ambiguity errors. These tests have been removed as the
// issue has been resolved with field-specific methods (config.enabled_set(true)).