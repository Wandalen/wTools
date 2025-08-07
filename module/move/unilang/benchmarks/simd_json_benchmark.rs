//! SIMD JSON Parsing Performance Benchmarks
//!
//! Comprehensive benchmarking of SIMD-optimized JSON parsing vs serde_json
//! across different payload sizes and structures to validate 4-25x performance improvements.

#![ allow( missing_docs ) ]

use criterion::{ black_box, criterion_group, criterion_main, Criterion, BenchmarkId };
use serde_json::Value as SerdeValue;
use unilang::simd_json_parser::SIMDJsonParser;

/// Generate test JSON data of different sizes and complexities
struct JsonTestData;

impl JsonTestData
{
  /// Small JSON payload (< 1KB) - Expected: 4x improvement
  fn small_json() -> String
  {
    r#"{"name":"test","id":42,"active":true,"tags":["rust","json","simd"],"metadata":{"version":"1.0","author":"benchmark"}}"#.to_string()
  }
  
  /// Medium JSON payload (1-10KB) - Expected: 8x improvement  
  fn medium_json() -> String
  {
    let mut json = r#"{"users":["#.to_string();
    for i in 0..100
    {
      if i > 0 { json.push_str( "," ); }
      json.push_str( &format!(
        r#"{{"id":{},"name":"user{}","email":"user{}@example.com","active":{},"roles":["admin","user"],"created":"2024-01-01T00:00:00Z","profile":{{"age":{},"country":"US","preferences":{{"theme":"dark","lang":"en"}}}}}}"#,
        i, i, i, i % 2 == 0, 20 + ( i % 50 )
      ));
    }
    json.push_str( "]}" );
    json
  }
  
  /// Large JSON payload (> 10KB) - Expected: 15-25x improvement
  fn large_json() -> String
  {
    let mut json = r#"{"data":{"items":["#.to_string();
    for i in 0..1000
    {
      if i > 0 { json.push_str( "," ); }
      json.push_str( &format!(
        r#"{{"id":{},"title":"Item {}","description":"This is a detailed description for item {} with various properties and nested data structures","price":{},"category":"category_{}","tags":["tag1","tag2","tag3"],"attributes":{{"color":"red","size":"large","weight":{},"dimensions":{{"width":10,"height":20,"depth":5}}}},"reviews":[{{"rating":5,"comment":"Excellent product","reviewer":"user1"}},{{"rating":4,"comment":"Good value","reviewer":"user2"}}],"inventory":{{"stock":{},"reserved":{},"available":{}}},"timestamps":{{"created":"2024-01-01T00:00:00Z","updated":"2024-01-02T12:00:00Z","expires":"2024-12-31T23:59:59Z"}}}}"#,
        i, i, i, 10.99 + ( i as f64 * 0.1 ), i % 10, 1.5 + ( i as f64 * 0.01 ), 100 + i, i % 10, 90 + i
      ));
    }
    json.push_str( "]," );
    json.push_str( r#""metadata":{"total":1000,"page":1,"pageSize":50,"hasMore":true,"filters":{"active":true,"category":"all"},"aggregations":{"totalValue":10999.99,"avgRating":4.5}}}}"# );
    json.push_str( "}" );
    json
  }
  
  /// Very large JSON payload (> 100KB) - Expected: 25x improvement
  fn very_large_json() -> String
  {
    let mut json = r#"{"massiveDataset":{"records":["#.to_string();
    for i in 0..5000
    {
      if i > 0 { json.push_str( "," ); }
      json.push_str( &format!(
        r#"{{"id":{},"title":"Record {}","data":{{"value1":"{}","value2":{},"value3":{},"tags":["tag1","tag2"],"metadata":{{"active":{},"score":{},"created":"2024-01-01T00:00:00Z"}}}},"stats":{{"views":{},"likes":{}}},"content":{{"body":"Large content body for record {}","wordCount":{}}},"relations":{{"refs":[{},{},{}]}}}}"#,
        i, i, format!( "item_{}", i ), i * 2, i * 3, 
        i % 2 == 0, ( i % 100 ) as f64 / 10.0,
        i * 10, i * 5,
        i, 150 + i,
        i + 10, i + 20, i + 30
      ));
    }
    json.push_str( r#"],"summary":{"totalRecords":5000,"processingTime":"145ms","memoryUsage":"256MB","version":"1.2.3"}}"# );
    json.push_str( "}" );
    json
  }
  
  /// Nested object structure for testing deep parsing
  fn nested_json() -> String
  {
    r#"{
      "level1": {
        "level2": {
          "level3": {
            "level4": {
              "level5": {
                "data": [1, 2, 3, 4, 5],
                "metadata": {
                  "created": "2024-01-01",
                  "tags": ["deep", "nested", "structure"]
                }
              }
            }
          }
        }
      },
      "arrays": [
        [1, 2, [3, 4, [5, 6, [7, 8, [9, 10]]]]],
        [
          {"id": 1, "values": [1, 2, 3]},
          {"id": 2, "values": [4, 5, 6]},
          {"id": 3, "values": [7, 8, 9]}
        ]
      ],
      "mixed": {
        "strings": ["a", "b", "c"],
        "numbers": [1, 2.5, 3.14159],
        "booleans": [true, false, true],
        "nulls": [null, null, null]
      }
    }"#.to_string()
  }
  
  /// Array-heavy structure for testing array parsing performance
  fn array_heavy_json() -> String
  {
    let mut json = r#"{"arrays":{"integers":["#.to_string();
    for i in 0..1000 { if i > 0 { json.push( ',' ); } json.push_str( &i.to_string() ); }
    json.push_str( r#"],"floats":[1.1"# );
    for i in 1..500 { json.push_str( &format!( ",{}.{}", i, i % 10 ) ); }
    json.push_str( r#"],"strings":["str0""# );
    for i in 1..300 { json.push_str( &format!( r#","str{}""#, i ) ); }
    json.push_str( r#"],"booleans":["# );
    for i in 0..200 { if i > 0 { json.push( ',' ); } json.push_str( if i % 2 == 0 { "true" } else { "false" } ); }
    json.push_str( r#"],"mixed":[1,"two",3.0,true,null,{"nested":true},[1,2,3]]"# );
    json.push_str( "}}" );
    json
  }
}

/// Benchmark serde_json parsing performance across different payload sizes
fn bench_serde_json_parsing( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "JSON Parsing - serde_json" );
  
  let small_json = JsonTestData::small_json();
  let medium_json = JsonTestData::medium_json();
  let large_json = JsonTestData::large_json();
  let very_large_json = JsonTestData::very_large_json();
  let nested_json = JsonTestData::nested_json();
  let array_json = JsonTestData::array_heavy_json();
  
  group.bench_with_input(
    BenchmarkId::new( "serde_json", "small_<1KB" ),
    &small_json,
    |b, json| b.iter( || serde_json::from_str::<SerdeValue>( black_box( json ) ).unwrap() )
  );
  
  group.bench_with_input(
    BenchmarkId::new( "serde_json", "medium_1-10KB" ),
    &medium_json,
    |b, json| b.iter( || serde_json::from_str::<SerdeValue>( black_box( json ) ).unwrap() )
  );
  
  group.bench_with_input(
    BenchmarkId::new( "serde_json", "large_>10KB" ),
    &large_json,
    |b, json| b.iter( || serde_json::from_str::<SerdeValue>( black_box( json ) ).unwrap() )
  );
  
  group.bench_with_input(
    BenchmarkId::new( "serde_json", "very_large_>100KB" ),
    &very_large_json,
    |b, json| b.iter( || serde_json::from_str::<SerdeValue>( black_box( json ) ).unwrap() )
  );
  
  group.bench_with_input(
    BenchmarkId::new( "serde_json", "nested_objects" ),
    &nested_json,
    |b, json| b.iter( || serde_json::from_str::<SerdeValue>( black_box( json ) ).unwrap() )
  );
  
  group.bench_with_input(
    BenchmarkId::new( "serde_json", "array_heavy" ),
    &array_json,
    |b, json| b.iter( || serde_json::from_str::<SerdeValue>( black_box( json ) ).unwrap() )
  );
  
  group.finish();
}

/// Benchmark SIMD JSON parsing performance across different payload sizes
fn bench_simd_json_parsing( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "JSON Parsing - SIMD" );
  
  let small_json = JsonTestData::small_json();
  let medium_json = JsonTestData::medium_json();
  let large_json = JsonTestData::large_json();
  let very_large_json = JsonTestData::very_large_json();
  let nested_json = JsonTestData::nested_json();
  let array_json = JsonTestData::array_heavy_json();
  
  group.bench_with_input(
    BenchmarkId::new( "simd_json", "small_<1KB" ),
    &small_json,
    |b, json| b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( json ) ).unwrap() )
  );
  
  group.bench_with_input(
    BenchmarkId::new( "simd_json", "medium_1-10KB" ),
    &medium_json,
    |b, json| b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( json ) ).unwrap() )
  );
  
  group.bench_with_input(
    BenchmarkId::new( "simd_json", "large_>10KB" ),
    &large_json,
    |b, json| b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( json ) ).unwrap() )
  );
  
  group.bench_with_input(
    BenchmarkId::new( "simd_json", "very_large_>100KB" ),
    &very_large_json,
    |b, json| b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( json ) ).unwrap() )
  );
  
  group.bench_with_input(
    BenchmarkId::new( "simd_json", "nested_objects" ),
    &nested_json,
    |b, json| b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( json ) ).unwrap() )
  );
  
  group.bench_with_input(
    BenchmarkId::new( "simd_json", "array_heavy" ),
    &array_json,
    |b, json| b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( json ) ).unwrap() )
  );
  
  group.finish();
}

/// Direct performance comparison between serde_json and SIMD JSON
fn bench_json_comparison( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "JSON Comparison - serde vs SIMD" );
  
  // Use medium-sized JSON for direct comparison
  let test_json = JsonTestData::medium_json();
  
  group.bench_function( "serde_json_baseline", |b|
  {
    b.iter( || serde_json::from_str::<SerdeValue>( black_box( &test_json ) ).unwrap() )
  });
  
  group.bench_function( "simd_json_optimized", |b|
  {
    b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( &test_json ) ).unwrap() )
  });
  
  group.finish();
}

/// Benchmark memory allocation patterns
fn bench_json_allocation( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "JSON Memory Allocation" );
  group.measurement_time( std::time::Duration::from_secs( 10 ) );
  
  let large_json = JsonTestData::large_json();
  
  group.bench_function( "serde_json_allocations", |b|
  {
    b.iter( ||
    {
      // Parse and immediately drop to measure allocation overhead
      let _value = serde_json::from_str::<SerdeValue>( black_box( &large_json ) ).unwrap();
    })
  });
  
  group.bench_function( "simd_json_allocations", |b|
  {
    b.iter( ||
    {
      // Parse and immediately drop to measure allocation overhead
      let _value = SIMDJsonParser::parse_to_serde_value( black_box( &large_json ) ).unwrap();
    })
  });
  
  group.finish();
}

/// Benchmark parsing different JSON structures to test SIMD effectiveness
fn bench_json_structures( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "JSON Structure Types - SIMD vs serde" );
  
  // Generate different structure types
  let flat_object = r#"{"a":1,"b":2,"c":3,"d":4,"e":5,"f":6,"g":7,"h":8,"i":9,"j":10}"#;
  let number_array = format!( "[{}]", ( 0..100 ).map( |i| i.to_string() ).collect::<Vec<_>>().join( "," ) );
  let string_array = format!( r#"[{}]"#, ( 0..50 ).map( |i| format!( r#""str{}""#, i ) ).collect::<Vec<_>>().join( "," ) );
  let mixed_array = r#"[1,"two",3.14,true,null,{"nested":true},[1,2,3]]"#;
  
  // Flat object parsing
  group.bench_function( "flat_object_serde", |b|
    b.iter( || serde_json::from_str::<SerdeValue>( black_box( flat_object ) ).unwrap() )
  );
  group.bench_function( "flat_object_simd", |b|
    b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( flat_object ) ).unwrap() )
  );
  
  // Number array parsing
  group.bench_function( "number_array_serde", |b|
    b.iter( || serde_json::from_str::<SerdeValue>( black_box( &number_array ) ).unwrap() )
  );
  group.bench_function( "number_array_simd", |b|
    b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( &number_array ) ).unwrap() )
  );
  
  // String array parsing  
  group.bench_function( "string_array_serde", |b|
    b.iter( || serde_json::from_str::<SerdeValue>( black_box( &string_array ) ).unwrap() )
  );
  group.bench_function( "string_array_simd", |b|
    b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( &string_array ) ).unwrap() )
  );
  
  // Mixed type parsing
  group.bench_function( "mixed_types_serde", |b|
    b.iter( || serde_json::from_str::<SerdeValue>( black_box( mixed_array ) ).unwrap() )
  );
  group.bench_function( "mixed_types_simd", |b|
    b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( mixed_array ) ).unwrap() )
  );
  
  group.finish();
}

/// Performance analysis across increasing payload sizes
fn bench_json_scaling( c : &mut Criterion )
{
  let mut group = c.benchmark_group( "JSON Scaling Analysis" );
  
  // Generate JSON payloads of increasing sizes
  let sizes = vec![ 10, 50, 100, 500, 1000, 2000 ];
  
  for size in sizes
  {
    let mut json = r#"{"items":["#.to_string();
    for i in 0..size
    {
      if i > 0 { json.push( ',' ); }
      json.push_str( &format!(
        r#"{{"id":{},"name":"item{}","value":{}}}"#,
        i, i, i as f64 * 1.5
      ));
    }
    json.push_str( "]}" );
    
    group.bench_with_input(
      BenchmarkId::new( "serde_scaling", size ),
      &json,
      |b, json| b.iter( || serde_json::from_str::<SerdeValue>( black_box( json ) ).unwrap() )
    );
    
    group.bench_with_input(
      BenchmarkId::new( "simd_scaling", size ),
      &json,
      |b, json| b.iter( || SIMDJsonParser::parse_to_serde_value( black_box( json ) ).unwrap() )
    );
  }
  
  group.finish();
}

criterion_group!(
  json_parsing_benches,
  bench_serde_json_parsing,
  bench_simd_json_parsing,
  bench_json_comparison,
  bench_json_allocation,
  bench_json_structures,
  bench_json_scaling
);
criterion_main!( json_parsing_benches );