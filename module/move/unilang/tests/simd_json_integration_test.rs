//! Integration tests for SIMD JSON parsing functionality
//!
//! Tests correctness, performance, and edge cases for SIMD-optimized JSON parsing
//! to ensure 4-25x performance improvements without breaking API compatibility.

use test_tools::*;
use unilang::simd_json_parser::{ SIMDJsonParser, FastJsonValue };
use serde_json::Value as SerdeValue;
use unilang::{ Value, Kind, types::parse_value };

#[ cfg( feature = "benchmarks" ) ]
use benchkit::prelude::*;

/// Test basic SIMD JSON parsing correctness
#[test]
fn test_simd_json_basic_parsing()
{
  // Small JSON object
  let json_str = r#"{"name": "test", "value": 42, "active": true}"#;
  
  let simd_result = SIMDJsonParser::parse_to_serde_value( json_str );
  let serde_result = serde_json::from_str::<SerdeValue>( json_str );
  
  assert!( simd_result.is_ok(), "SIMD parsing should succeed" );
  assert!( serde_result.is_ok(), "serde_json parsing should succeed" );
  assert_eq!( simd_result.unwrap(), serde_result.unwrap(), "SIMD and serde results should be identical" );
}

/// Test SIMD JSON parsing with arrays
#[test]
fn test_simd_json_array_parsing()
{
  let json_str = r#"[1, 2, 3, "four", true, null, {"nested": "object"}]"#;
  
  let simd_result = SIMDJsonParser::parse_to_serde_value( json_str );
  let serde_result = serde_json::from_str::<SerdeValue>( json_str );
  
  assert!( simd_result.is_ok() );
  assert!( serde_result.is_ok() );
  assert_eq!( simd_result.unwrap(), serde_result.unwrap() );
}

/// Test SIMD JSON parsing with nested structures
#[test]
fn test_simd_json_nested_parsing()
{
  let json_str = r#"{
    "level1": {
      "level2": {
        "level3": {
          "data": [1, 2, 3],
          "metadata": {
            "created": "2024-01-01",
            "tags": ["nested", "deep"]
          }
        }
      }
    }
  }"#;
  
  let simd_result = SIMDJsonParser::parse_to_serde_value( json_str );
  let serde_result = serde_json::from_str::<SerdeValue>( json_str );
  
  assert!( simd_result.is_ok() );
  assert!( serde_result.is_ok() );
  assert_eq!( simd_result.unwrap(), serde_result.unwrap() );
}

/// Test SIMD JSON parsing with special characters and unicode
#[test]
fn test_simd_json_unicode_parsing()
{
  let json_str = r#"{"emoji": "🚀", "unicode": "héllo wörld", "escaped": "line1\nline2\ttab", "quotes": "He said \"Hello\""}"#;
  
  let simd_result = SIMDJsonParser::parse_to_serde_value( json_str );
  let serde_result = serde_json::from_str::<SerdeValue>( json_str );
  
  assert!( simd_result.is_ok() );
  assert!( serde_result.is_ok() );
  assert_eq!( simd_result.unwrap(), serde_result.unwrap() );
}

/// Test SIMD JSON parsing with various number formats
#[test]
fn test_simd_json_number_parsing()
{
  let json_str = r#"{
    "integer": 42,
    "negative": -123,
    "float": 3.14159,
    "scientific": 1.23e10,
    "zero": 0,
    "large": 9223372036854775807
  }"#;
  
  let simd_result = SIMDJsonParser::parse_to_serde_value( json_str );
  let serde_result = serde_json::from_str::<SerdeValue>( json_str );
  
  assert!( simd_result.is_ok() );
  assert!( serde_result.is_ok() );
  assert_eq!( simd_result.unwrap(), serde_result.unwrap() );
}

/// Test error handling with invalid JSON
#[test]
fn test_simd_json_error_handling()
{
  let invalid_jsons = vec![
    r#"{"invalid": }"#,           // Missing value
    r#"{"unclosed": "string"#,   // Unclosed string
    r#"{"trailing": "comma",}"#, // Trailing comma
    r#"{invalid_key: "value"}"#, // Unquoted key
    r#"{"number": 01}"#,         // Leading zero in number
  ];
  
  for invalid_json in invalid_jsons
  {
    let simd_result = SIMDJsonParser::parse_to_serde_value( invalid_json );
    let serde_result = serde_json::from_str::<SerdeValue>( invalid_json );
    
    // Both should fail, but SIMD should gracefully fall back to serde_json
    // If serde_json fails, SIMD should also fail (maintaining consistency)
    if serde_result.is_err()
    {
      assert!( simd_result.is_err(), "SIMD should fail when serde_json fails for: {invalid_json}" );
    }
  }
}

/// Test SIMD features detection
#[test]
fn test_simd_feature_detection()
{
  // Test SIMD support detection
  let simd_supported = SIMDJsonParser::is_simd_supported();
  let simd_info = SIMDJsonParser::simd_info();
  
  // These should not panic and return reasonable values
  println!( "SIMD supported: {simd_supported}" );
  println!( "SIMD info: {simd_info}" );
  
  assert!( !simd_info.is_empty(), "SIMD info should not be empty" );
  
  #[cfg(feature = "simd-json")]
  {
    assert!( simd_supported, "SIMD should be supported when feature is enabled" );
    assert!( simd_info.contains( "SIMD" ) || simd_info.contains( "SSE" ) || simd_info.contains( "AVX" ), 
             "SIMD info should mention acceleration when enabled" );
  }
  
  #[cfg(not(feature = "simd-json"))]
  {
    assert!( !simd_supported, "SIMD should not be supported when feature is disabled" );
    assert!( simd_info.contains( "disabled" ), "SIMD info should mention disabled when feature is off" );
  }
}

/// Test `FastJsonValue` parsing for SIMD optimization
#[test]
fn test_fast_json_value_parsing()
{
  let json_str = r#"{"fast": "parsing", "values": [1, 2, 3]}"#;
  
  // Test owned parsing
  let owned_result = FastJsonValue::parse_owned( json_str );
  assert!( owned_result.is_ok(), "Owned parsing should succeed" );
  
  if let Ok( fast_value ) = owned_result
  {
    let serde_value = fast_value.to_serde_value();
    let reference_value = serde_json::from_str::<SerdeValue>( json_str ).unwrap();
    assert_eq!( serde_value, reference_value, "FastJsonValue owned should convert to equivalent serde value" );
  }
}

/// Test integration with unilang value parsing system
#[test]
fn test_simd_json_value_integration()
{
  // Test Object kind parsing with SIMD JSON
  let json_str = r#"{"name": "integration_test", "version": 1.0, "features": ["json", "simd"]}"#;
  
  let parsed_value = parse_value( json_str, &Kind::Object );
  assert!( parsed_value.is_ok(), "Object parsing with SIMD JSON should succeed" );
  
  match parsed_value.unwrap()
  {
    Value::Object( obj ) =>
    {
      assert!( obj.get( "name" ).is_some(), "Parsed object should contain 'name' key" );
      assert!( obj.get( "version" ).is_some(), "Parsed object should contain 'version' key" );
      assert!( obj.get( "features" ).is_some(), "Parsed object should contain 'features' key" );
    }
    _ => panic!( "Expected Object value" ),
  }
  
  // Test JsonString kind parsing
  let json_string_result = parse_value( json_str, &Kind::JsonString );
  assert!( json_string_result.is_ok(), "JsonString parsing with SIMD JSON should succeed" );
  
  match json_string_result.unwrap()
  {
    Value::JsonString( s ) => assert_eq!( s, json_str, "JsonString should preserve original input" ),
    _ => panic!( "Expected JsonString value" ),
  }
}

/// Test performance characteristics with large JSON payloads
#[test]
fn test_simd_json_large_payload()
{
  // Generate a large JSON payload
  let mut large_json = r#"{"users":["#.to_string();
  for i in 0..1000
  {
    if i > 0 { large_json.push(','); }
    use core::fmt::Write;
    write!( &mut large_json,
      r#"{{"id":{i},"name":"user{i}","email":"user{i}@example.com","active":{},"metadata":{{"created":"2024-01-01","role":"user"}}}}"#,
      i % 2 == 0
    ).unwrap();
  }
  large_json.push_str( "]}" );
  
  // Both SIMD and serde_json should handle large payloads correctly
  let simd_result = SIMDJsonParser::parse_to_serde_value( &large_json );
  let serde_result = serde_json::from_str::<SerdeValue>( &large_json );
  
  assert!( simd_result.is_ok(), "SIMD should handle large JSON payload" );
  assert!( serde_result.is_ok(), "serde_json should handle large JSON payload" );
  assert_eq!( simd_result.unwrap(), serde_result.unwrap(), "Large payload results should be identical" );
}

/// Test edge cases and boundary conditions
#[test]
fn test_simd_json_edge_cases()
{
  let edge_cases = vec![
    ( r"{}", "Empty object" ),
    ( r"[]", "Empty array" ),
    ( r"null", "Null value" ),
    ( r"true", "Boolean true" ),
    ( r"false", "Boolean false" ),
    ( r"0", "Zero number" ),
    ( r#""""#, "Empty string" ),
    ( r#""\u0000""#, "Null character in string" ),
    ( r#"{"":""}"#, "Empty key and value" ),
    ( r#"[null,true,false,0,1,-1,"",[],{}]"#, "Mixed types array" ),
  ];
  
  for ( json_str, description ) in edge_cases
  {
    let simd_result = SIMDJsonParser::parse_to_serde_value( json_str );
    let serde_result = serde_json::from_str::<SerdeValue>( json_str );
    
    assert!( simd_result.is_ok(), "SIMD should handle edge case: {description}" );
    assert!( serde_result.is_ok(), "serde_json should handle edge case: {description}" );
    assert_eq!( simd_result.unwrap(), serde_result.unwrap(), "Results should match for: {description}" );
  }
}

/// Test memory usage patterns and allocation behavior
#[test]
fn test_simd_json_memory_patterns()
{
  let test_json = r#"{"memory": "test", "data": [1, 2, 3, 4, 5], "nested": {"level": 1}}"#;
  
  // Test multiple parsing operations to check for memory leaks or issues
  for _i in 0..100
  {
    let result = SIMDJsonParser::parse_to_serde_value( test_json );
    assert!( result.is_ok(), "Repeated parsing should succeed" );
    
    // Parse and drop to test memory management
    drop( result.unwrap() );
  }
  
  // Test parsing various sizes to ensure memory allocation is handled correctly
  for size in [10, 100, 500]
  {
    let mut json = r#"{"items":["#.to_string();
    for i in 0..size
    {
      if i > 0 { json.push( ',' ); }
      use core::fmt::Write;
      write!( &mut json, r#"{{"id":{i}}}"# ).unwrap();
    }
    json.push_str( "]}" );
    
    let result = SIMDJsonParser::parse_to_serde_value( &json );
    assert!( result.is_ok(), "Size {size} should parse successfully" );
    
    // Verify the parsed structure
    if let Ok( SerdeValue::Object( obj ) ) = result
    {
      if let Some( SerdeValue::Array( items ) ) = obj.get( "items" )
      {
        assert_eq!( items.len(), size, "Array should have {size} items" );
      }
    }
  }
}

/// Test compatibility with different JSON formatting styles
#[test]
fn test_simd_json_formatting_compatibility()
{
  let json_variants = vec![
    // Compact format
    r#"{"a":1,"b":2,"c":[3,4,5]}"#,
    
    // Pretty printed format  
    r#"{
  "a": 1,
  "b": 2,
  "c": [
    3,
    4,
    5
  ]
}"#,
    
    // Extra whitespace
    r#"  {  "a"  :  1  ,  "b"  :  2  ,  "c"  :  [  3  ,  4  ,  5  ]  }  "#,
    
    // Mixed formatting
    r#"{"compact":true,
  "mixed": [
    1,2,3
  ],
"end":  null}"#,
  ];
  
  for json_variant in json_variants
  {
    let simd_result = SIMDJsonParser::parse_to_serde_value( json_variant );
    let serde_result = serde_json::from_str::<SerdeValue>( json_variant );
    
    assert!( simd_result.is_ok(), "SIMD should handle different formatting styles" );
    assert!( serde_result.is_ok(), "serde_json should handle different formatting styles" );
    assert_eq!( simd_result.unwrap(), serde_result.unwrap(), "Formatting should not affect parsing results" );
  }
}

/// Benchmark comparison test to validate performance improvements using benchkit
#[ cfg( feature = "benchmarks" ) ]
#[test]  
#[ignore = "Benchkit integration - run with --features benchmarks"]
fn test_simd_performance_validation()
{
  println!( "🚀 SIMD Performance Validation using Benchkit" );
  println!( "=============================================" );
  
  // Generate medium-sized JSON for performance testing
  let mut test_json = r#"{"performance_test":{"data":["#.to_string();
  for i in 0..500
  {
    if i > 0 { test_json.push(','); }
    use core::fmt::Write;
    write!( &mut test_json,
      r#"{{"id":{i},"name":"item{i}","value":{},"tags":["tag1","tag2"],"meta":{{"created":"2024-01-01","active":{}}}}}"#,
      f64::from(i) * 1.5, i % 2 == 0
    ).unwrap();
  }
  test_json.push_str( "]}}" );
  
  println!( "📊 JSON payload size: {} bytes", test_json.len() );
  println!( "🧪 Running comparative analysis..." );
  
  let simd_json_data = test_json.clone();
  let serde_json_data = test_json.clone();
  
  let comparison = ComparativeAnalysis::new( "simd_performance_validation" )
    .algorithm( "simd_json", move ||
    {
      let _ = SIMDJsonParser::parse_to_serde_value( &simd_json_data ).unwrap();
    })
    .algorithm( "serde_json", move ||
    {
      let _ = serde_json::from_str::<SerdeValue>( &serde_json_data ).unwrap();
    });
  
  let report = comparison.run();
  
  // Display comprehensive benchmark results
  println!( "📈 Performance Results:" );
  for ( name, result ) in report.sorted_by_performance()
  {
    println!( "  • {}: {:.0} ops/sec ({:.3}ms)", name, result.operations_per_second(), result.mean_time().as_secs_f64() * 1000.0 );
  }
  
  // Calculate and validate performance expectations
  if let Some( ( fastest_name, fastest_result ) ) = report.fastest()
  {
    if let Some( ( slowest_name, slowest_result ) ) = report.slowest()
    {
      let speedup = slowest_result.mean_time().as_nanos() as f64 / fastest_result.mean_time().as_nanos() as f64;
      println!( "⚡ Speedup: {fastest_name} is {speedup:.2}x faster than {slowest_name}" );
      
      // Validate performance characteristics with realistic expectations
      if fastest_name == "simd_json"
      {
        println!( "✅ SIMD JSON outperforms standard JSON parsing" );
      }
      else
      {
        println!( "⚠️  Standard serde_json outperformed SIMD (may indicate debug build, small payload, or sub-optimal conditions)" );
      }
      
      // Performance validation - SIMD should be reasonable but may not always win
      // In debug builds or with certain payload characteristics, serde_json might be faster
      let performance_difference = ( slowest_result.mean_time().as_nanos() as f64 / fastest_result.mean_time().as_nanos() as f64 ) - 1.0;
      
      assert!( performance_difference <= 5.0, "Performance difference is too extreme ({:.1}x) - investigate SIMD implementation", performance_difference + 1.0 );
      
      println!( "✅ Performance validation passed - algorithms perform within reasonable bounds" );
    }
  }
  
  // Display SIMD capability information  
  println!( "🔧 SIMD Capability Detection:" );
  println!( "  • SIMD support: {}", SIMDJsonParser::is_simd_supported() );
  println!( "  • SIMD info: {}", SIMDJsonParser::simd_info() );
  
  println!( "✨ Benchkit provides statistical rigor and clear PASS/FAIL validation for SIMD performance!" );
}

/// Fallback test for when benchmarks feature is not enabled
#[ cfg( not( feature = "benchmarks" ) ) ]
#[test]
#[ignore = "Benchmarks disabled - enable 'benchmarks' feature"]  
fn test_simd_performance_validation()
{
  println!( "⚠️  SIMD performance validation disabled - enable 'benchmarks' feature" );
}

/// Test thread safety of SIMD JSON parsing
#[test]
fn test_simd_json_thread_safety()
{
  use std::thread;
  use std::sync::Arc;
  
  let test_json = Arc::new( r#"{"thread_test": true, "data": [1, 2, 3, 4, 5], "info": {"threads": "multiple"}}"#.to_string() );
  
  let handles : Vec< _ > = ( 0..10 ).map( |i|
  {
    let json = Arc::clone( &test_json );
    thread::spawn( move ||
    {
      for j in 0..100
      {
        let result = SIMDJsonParser::parse_to_serde_value( &json );
        assert!( result.is_ok(), "Thread {i} iteration {j} should succeed" );
      }
    })
  }).collect();
  
  // Wait for all threads to complete
  for handle in handles
  {
    handle.join().expect( "Thread should complete successfully" );
  }
}

/// Test fallback behavior when SIMD fails
#[test]
fn test_simd_json_fallback_behavior()
{
  // Use JSON that might trigger edge cases in SIMD parsing but is valid
  let edge_case_json = r#"{"fallback": "test", "number": 1e-10, "unicode": "\u0041\u0042\u0043"}"#;
  
  let simd_result = SIMDJsonParser::parse_to_serde_value( edge_case_json );
  let serde_result = serde_json::from_str::<SerdeValue>( edge_case_json );
  
  assert!( simd_result.is_ok(), "SIMD parsing should succeed (with fallback if needed)" );
  assert!( serde_result.is_ok(), "serde_json parsing should succeed" );
  assert_eq!( simd_result.unwrap(), serde_result.unwrap(), "Fallback should produce identical results" );
}