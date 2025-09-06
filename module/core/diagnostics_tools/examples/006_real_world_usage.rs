//! # Example 006: Real-World Usage Scenarios
//!
//! This example demonstrates practical, real-world usage patterns for `diagnostics_tools`
//! in different contexts: testing, API validation, data processing, and more.
//!
//! ## What you'll learn:
//! - Testing with enhanced assertions
//! - API input validation
//! - Data processing pipelines
//! - Performance validation
//! - Integration patterns
//!
//! ## Run this example:
//! ```bash
//! cargo run --example 006_real_world_usage
//! ```

use diagnostics_tools::*;
use std::collections::HashMap;

// ========================================
// Scenario 1: Enhanced Testing
// ========================================

#[ derive( Debug, PartialEq ) ]
#[ allow( dead_code ) ]
struct ApiResponse
{
  status : u16,
  message : String,
  data : serde_json::Value,
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  
  // This test shows how diagnostics_tools makes test failures much clearer
  #[ test ]
  fn test_api_response_parsing()
  {
    let json_input = r#"{"status": 200, "message": "Success", "data": {"items": [1,2,3]}}"#;
    let response = parse_api_response( json_input ).unwrap();
    
    // Instead of assert_eq!, use a_id! for better diff output
    a_id!( response.status, 200 );
    a_id!( response.message, "Success" );
    
    // When comparing complex JSON, the diff output is invaluable
    let expected_data = serde_json::json!( { "items": [ 1, 2, 3 ] } );
    a_id!( response.data, expected_data );
  }
  
  #[ test ]
  fn test_user_creation_validation()
  {
    let user_data = UserData
    {
      name : "Alice Johnson".to_string(),
      email : "alice@example.com".to_string(),
      age : 28,
      preferences : vec![ "dark_mode".to_string(), "notifications".to_string() ],
    };
    
    let validation_result = validate_user_data( &user_data );
    
    // Better error messages for validation results
    a_true!( validation_result.is_ok(), "User data should be valid" );
    
    let user = validation_result.unwrap();
    a_id!( user.name, "Alice Johnson" );
    a_true!( user.email.contains( "@" ), "Email should contain @ symbol" );
    a_true!( user.age >= 18, "User should be adult" );
  }
}

// ========================================
// Scenario 2: API Input Validation
// ========================================

#[ derive( Debug, PartialEq ) ]
struct UserData
{
  name : String,
  email : String,
  age : u32,
  preferences : Vec< String >,
}

#[ derive( Debug, PartialEq ) ]
struct ValidatedUser
{
  name : String,
  email : String,
  age : u32,
  preferences : Vec< String >,
}

fn validate_user_data( data : &UserData ) -> Result< ValidatedUser, String >
{
  // Proper error handling instead of assertions
  if data.name.is_empty() {
    return Err( "Name cannot be empty".to_string() );
  }
  if data.name.len() > 100 {
    return Err( "Name too long".to_string() );
  }
  
  if !data.email.contains( '@' ) {
    return Err( "Email must contain @".to_string() );
  }
  if data.email.len() < 5 {
    return Err( "Email too short".to_string() );
  }
  
  if data.age < 13 {
    return Err( "Must be at least 13 years old".to_string() );
  }
  if data.age > 150 {
    return Err( "Age seems unrealistic".to_string() );
  }
  
  if data.preferences.len() > 10 {
    return Err( "Too many preferences".to_string() );
  }
  
  // Compile-time validation of assumptions
  cta_type_same_size!( u32, u32 ); // Sanity check
  
  Ok( ValidatedUser
  {
    name : data.name.clone(),
    email : data.email.clone(),
    age : data.age,
    preferences : data.preferences.clone(),
  } )
}

// ========================================
// Scenario 3: Data Processing Pipeline
// ========================================

#[ derive( Debug, PartialEq ) ]
struct DataBatch
{
  id : String,
  items : Vec< f64 >,
  metadata : HashMap< String, String >,
}

fn process_data_batch( batch : &DataBatch ) -> Result< ProcessedBatch, String >
{
  // Proper error handling instead of assertions
  if batch.id.is_empty() {
    return Err( "Batch ID cannot be empty".to_string() );
  }
  if batch.items.is_empty() {
    return Err( "Batch cannot be empty".to_string() );
  }
  if batch.items.len() > 10000 {
    return Err( "Batch too large for processing".to_string() );
  }
  
  // Validate data quality
  if !batch.items.iter().all( |x| x.is_finite() ) {
    return Err( "All items must be finite numbers".to_string() );
  }
  
  let mut processed_items = Vec::new();
  let mut validation_errors = 0;
  
  for &item in &batch.items
  {
    if item >= 0.0
    {
      processed_items.push( item * 1.1 ); // Apply 10% increase
    }
    else
    {
      validation_errors += 1;
    }
  }
  
  // Validate processing results
  if processed_items.is_empty() {
    return Err( "Processing should produce some results".to_string() );
  }
  if validation_errors >= batch.items.len() / 2 {
    return Err( "Too many validation errors".to_string() );
  }
  
  let success_rate = processed_items.len() as f64 / batch.items.len() as f64;
  if success_rate < 0.8 {
    return Err( "Success rate should be at least 80%".to_string() );
  }
  
  Ok( ProcessedBatch
  {
    original_id : batch.id.clone(),
    processed_items,
    success_rate,
    error_count : validation_errors,
  } )
}

#[ derive( Debug, PartialEq ) ]
struct ProcessedBatch
{
  original_id : String,
  processed_items : Vec< f64 >,
  success_rate : f64,
  error_count : usize,
}

// ========================================
// Scenario 4: Performance Validation
// ========================================

fn performance_critical_function( data : &[ i32 ] ) -> Vec< i32 >
{
  use std::time::Instant;
  
  // Compile-time validation of type assumptions
  cta_type_same_size!( i32, i32 );
  cta_type_same_size!( usize, *const i32 );
  
  // Runtime validation of input
  a_true!( !data.is_empty(), "Input data cannot be empty" );
  a_true!( data.len() <= 1_000_000, "Input data too large for this function" );
  
  let start = Instant::now();
  
  // Process data (simplified example)
  let result : Vec< i32 > = data.iter().map( |&x| x * 2 ).collect();
  
  let duration = start.elapsed();
  
  // Performance validation
  let items_per_second = data.len() as f64 / duration.as_secs_f64();
  a_true!( items_per_second > 1000.0, "Performance should be at least 1000 items/sec" );
  
  // Output validation
  a_id!( result.len(), data.len() );
  a_true!( result.iter().zip( data ).all( |(r, d)| r == &(d * 2) ), "All calculations should be correct" );
  
  result
}

// ========================================
// Main Example Runner
// ========================================

fn main()
{
  println!( "🌍 Real-World Usage Scenarios for diagnostics_tools\n" );

  // Scenario 1: Testing (run the actual tests to see)
  println!( "1. Enhanced Testing:" );
  println!( "   ✓ See the #[ cfg( test ) ] mod tests above" );
  println!( "   ✓ Run 'cargo test' to see enhanced assertion output" );
  println!( "   ✓ Better diffs for complex data structures in test failures\n" );

  // Scenario 2: API Validation
  println!( "2. API Input Validation:" );
  let user_data = UserData
  {
    name : "Bob Smith".to_string(),
    email : "bob@company.com".to_string(),
    age : 35,
    preferences : vec![ "email_notifications".to_string() ],
  };
  
  match validate_user_data( &user_data )
  {
    Ok( user ) => 
    {
      a_id!( user.name, "Bob Smith" );
      println!( "   ✓ User validation passed: {}", user.name );
    }
    Err( error ) => println!( "   ✗ Validation failed: {error}" ),
  }

  // Scenario 3: Data Processing
  println!( "\n3. Data Processing Pipeline:" );
  let batch = DataBatch
  {
    id : "batch_001".to_string(),
    items : vec![ 1.0, 2.5, 3.7, 4.2, 5.0, -0.5, 6.8 ],
    metadata : HashMap::new(),
  };
  
  match process_data_batch( &batch )
  {
    Ok( result ) => 
    {
      a_true!( result.success_rate > 0.7, "Processing success rate should be good" );
      a_dbg_id!( result.original_id, "batch_001", "Batch ID should be preserved" );
      println!( "   ✓ Batch processing completed with {:.1}% success rate", 
               result.success_rate * 100.0 );
    }
    Err( error ) => println!( "   ✗ Processing failed: {error}" ),
  }

  // Scenario 4: Performance Validation
  println!( "\n4. Performance Critical Operations:" );
  let test_data : Vec< i32 > = ( 1..=1000 ).collect();
  let result = performance_critical_function( &test_data );
  
  a_id!( result.len(), 1000 );
  a_id!( result[ 0 ], 2 );     // First item: 1 * 2 = 2
  a_id!( result[ 999 ], 2000 ); // Last item: 1000 * 2 = 2000
  println!( "   ✓ Performance function processed {} items successfully", result.len() );

  // Scenario 5: Integration with external libraries
  demonstrate_json_integration();
  
  // Scenario 6: Configuration validation  
  demonstrate_config_validation();

  println!( "\n🎉 All real-world scenarios completed successfully!" );
  println!( "\n💡 Key patterns for real-world usage:" );
  println!( "   • Use a_id!() in tests for better failure diagnostics" );
  println!( "   • Use a_true!() for business rule validation with clear messages" );
  println!( "   • Use cta_*!() macros to validate assumptions at compile-time" );
  println!( "   • Use a_dbg_*!() variants during development and debugging" );
  println!( "   • Combine runtime and compile-time checks for comprehensive validation" );
  println!( "\n🏆 You've completed all diagnostics_tools examples!" );
  println!( "    You're now ready to enhance your own projects with better assertions." );
}

// Additional helper functions for examples

#[ allow( dead_code ) ]
fn parse_api_response( json : &str ) -> Result< ApiResponse, Box< dyn core::error::Error > >
{
  let value : serde_json::Value = serde_json::from_str( json )?;
  
  // Safe casting with proper error handling
  let status_u64 = value[ "status" ].as_u64().unwrap();
  let status = u16::try_from( status_u64 )
    .map_err( |_| format!( "Status value {status_u64} is too large for u16" ) )?;

  Ok( ApiResponse
  {
    status,
    message : value[ "message" ].as_str().unwrap().to_string(),
    data : value[ "data" ].clone(),
  } )
}

fn demonstrate_json_integration()
{
  println!( "\n5. JSON/Serde Integration:" );
  
  let json_data = serde_json::json!( {
    "name": "Integration Test",
    "values": [ 1, 2, 3, 4, 5 ],
    "config": {
      "enabled": true,
      "threshold": 0.95
    }
  } );
  
  // Validate JSON structure with assertions
  a_true!( json_data[ "name" ].is_string(), "Name should be a string" );
  a_true!( json_data[ "values" ].is_array(), "Values should be an array" );
  a_id!( json_data[ "values" ].as_array().unwrap().len(), 5 );
  a_true!( json_data[ "config" ][ "enabled" ].as_bool().unwrap(), "Config should be enabled" );
  
  println!( "   ✓ JSON structure validation completed" );
}

fn demonstrate_config_validation()
{
  println!( "\n6. Configuration Validation:" );
  
  // Simulate loading configuration
  let config = AppConfig
  {
    max_retries : 3,
    timeout_seconds : 30,
    enable_logging : true,
    log_level : "INFO".to_string(),
  };
  
  // Validate configuration with clear error messages
  a_true!( config.max_retries > 0, "Max retries must be positive" );
  a_true!( config.max_retries <= 10, "Max retries should be reasonable" );
  a_true!( config.timeout_seconds >= 1, "Timeout must be at least 1 second" );
  a_true!( config.timeout_seconds <= 300, "Timeout should not exceed 5 minutes" );
  
  let valid_log_levels = [ "ERROR", "WARN", "INFO", "DEBUG", "TRACE" ];
  a_true!( valid_log_levels.contains( &config.log_level.as_str() ), 
          "Log level must be valid" );
  
  println!( "   ✓ Configuration validation completed" );
}

#[ derive( Debug ) ]
struct AppConfig
{
  max_retries : u32,
  timeout_seconds : u32,
  #[ allow( dead_code ) ]
  enable_logging : bool,
  log_level : String,
}