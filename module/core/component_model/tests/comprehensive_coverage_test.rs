//! Comprehensive test coverage for ComponentModel derive macro
//!
//! ## Test Matrix for Complete Coverage
//!
//! | ID    | Test Case                              | Expected Output                        |
//! |-------|----------------------------------------|----------------------------------------|
//! | T3.1a | Basic structs without generics         | Field-specific methods work correctly  |
//! | T3.2  | Keyword field names (r#type, etc)     | Methods with clean names (assign_type)|
//! | T3.3  | Single field struct                    | Single field-specific method          |
//! | T3.4  | Complex field types (Vec, Option, etc)| Methods work with complex types        |
//! | T3.6  | Mixed field types comprehensive        | All supported field types work        |
//!
//! Note: Generic structs, lifetimes, and complex where clauses are not yet supported

use component_model::ComponentModel;
use std::collections::HashMap;

// Test simple structs without generics first
/// Test basic struct works correctly with field-specific methods
/// Test Combination: T3.1a
#[ derive( ComponentModel, Debug, PartialEq ) ]
struct BasicConfig
{
  value : i32,
  name : String,
}

#[ test ]
fn test_basic_struct_field_methods()
{
  let mut config = BasicConfig { value: 0, name: String::new() };
  
  // Field-specific methods should work
  config.value_set( 42i32 );
  config.name_set( "test".to_string() );
  
  assert_eq!( config.value, 42 );
  assert_eq!( config.name, "test" );
}

/// Test fluent pattern works
/// Test Combination: T3.1a
#[ test ]
fn test_basic_struct_fluent_pattern()
{
  let config = BasicConfig { value: 0, name: String::new() }
    .value_with( 100 )
    .name_with( "fluent".to_string() );
    
  assert_eq!( config.value, 100 );
  assert_eq!( config.name, "fluent" );
}

// Test keyword field names
/// Test keyword field names are handled correctly
/// Test Combination: T3.2
#[ derive( ComponentModel, Debug, PartialEq ) ]
struct KeywordFields
{
  r#type : String,
  r#match : i32,
  r#use : bool,
}

#[ test ] 
fn test_keyword_field_names()
{
  let mut config = KeywordFields { r#type: String::new(), r#match: 0, r#use: false };
  
  // Methods should have clean names without r# prefix
  config.type_set( "test_type".to_string() );
  config.match_set( 100i32 );
  config.use_set( true );
  
  assert_eq!( config.r#type, "test_type" );
  assert_eq!( config.r#match, 100 );
  assert_eq!( config.r#use, true );
}

/// Test keyword fields fluent pattern
/// Test Combination: T3.2
#[ test ]
fn test_keyword_fields_fluent()
{
  let config = KeywordFields { r#type: String::new(), r#match: 0, r#use: false }
    .type_with( "fluent_type".to_string() )
    .match_with( 200i32 )
    .use_with( true );
    
  assert_eq!( config.r#type, "fluent_type" );
  assert_eq!( config.r#match, 200 );
  assert_eq!( config.r#use, true );
}

// Test single field struct
/// Test single field struct generates correct methods
/// Test Combination: T3.3
#[ derive( ComponentModel, Debug, PartialEq ) ]
struct SingleField
{
  data : String,
}

#[ test ]
fn test_single_field_struct()
{
  let mut config = SingleField { data: String::new() };
  
  config.data_set( "single".to_string() );
  assert_eq!( config.data, "single" );
  
  let config2 = SingleField { data: String::new() }
    .data_with( "single_fluent".to_string() );
  assert_eq!( config2.data, "single_fluent" );
}

// Test complex field types
/// Test complex field types (Vec, Option, HashMap, etc.) work correctly
/// Test Combination: T3.4
#[ derive( ComponentModel, Debug, PartialEq ) ]
struct ComplexFields
{
  items : Vec< String >,
  maybe_value : Option< i32 >,
  mapping : HashMap< String, i32 >,
}

impl Default for ComplexFields {
  fn default() -> Self {
    Self {
      items: Vec::new(),
      maybe_value: None,
      mapping: HashMap::new(),
    }
  }
}

#[ test ]
fn test_complex_field_types()
{
  let mut config = ComplexFields::default();
  
  config.items_set( vec![ "a".to_string(), "b".to_string() ] );
  config.maybe_value_set( Some( 42 ) );
  config.mapping_set( {
    let mut map = HashMap::new();
    map.insert( "key".to_string(), 100 );
    map
  } );
  
  assert_eq!( config.items, vec![ "a".to_string(), "b".to_string() ] );
  assert_eq!( config.maybe_value, Some( 42 ) );
  assert_eq!( config.mapping.get( "key" ), Some( &100 ) );
}

/// Test complex types fluent pattern
/// Test Combination: T3.4
#[ test ]
fn test_complex_types_fluent()
{
  let config = ComplexFields::default()
    .items_with( vec![ "x".to_string() ] )
    .maybe_value_with( Some( 999 ) )
    .mapping_with( HashMap::new() );
    
  assert_eq!( config.items, vec![ "x".to_string() ] );
  assert_eq!( config.maybe_value, Some( 999 ) );
  assert_eq!( config.mapping.len(), 0 );
}

// Note: Lifetime parameters are not yet supported by ComponentModel derive
// This is a known limitation of the current implementation

// Test mixed comprehensive field types (without generics)
/// Test comprehensive mix of all field types
/// Test Combination: T3.6
#[ derive( ComponentModel, Debug ) ]
struct ComprehensiveMix
{
  float_field : f64,
  string_field : String, 
  int_field : i32,
  bool_field : bool,
  vec_field : Vec< i32 >,
  option_field : Option< String >,
  r#async : bool,
}

#[ test ]
fn test_comprehensive_field_mix()
{
  let mut config = ComprehensiveMix {
    float_field: 0.0f64,
    string_field: String::new(),
    int_field: 0,
    bool_field: false,
    vec_field: Vec::new(),
    option_field: None,
    r#async: false,
  };
  
  // Test all field-specific assignment methods
  config.float_field_set( 3.14f64 );
  config.string_field_set( "mixed".to_string() );
  config.int_field_set( 789i32 );
  config.bool_field_set( true );
  config.vec_field_set( vec![ 1, 2, 3 ] );
  config.option_field_set( Some( "option".to_string() ) );
  config.async_set( true );
  
  assert_eq!( config.float_field, 3.14f64 );
  assert_eq!( config.string_field, "mixed" );
  assert_eq!( config.int_field, 789 );
  assert_eq!( config.bool_field, true );
  assert_eq!( config.vec_field, vec![ 1, 2, 3 ] );
  assert_eq!( config.option_field, Some( "option".to_string() ) );
  assert_eq!( config.r#async, true );
}

// Note: Complex generic types with where clauses are not yet fully supported
// This is a known limitation that could be addressed in future versions