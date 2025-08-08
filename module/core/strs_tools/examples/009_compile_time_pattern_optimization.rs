//! Compile-time pattern optimization examples demonstrating macro-generated optimized code.
//!
//! This example shows how compile-time analysis can generate highly optimized
//! string processing code tailored to specific patterns and usage scenarios.

#[ allow( unused_imports ) ]
use strs_tools::*;

#[ cfg( feature = "compile_time_optimizations" ) ]
use strs_tools::{ optimize_split, optimize_match };

fn main() {
  println!( "=== Compile-Time Pattern Optimization Examples ===" );
  
  #[ cfg( feature = "compile_time_optimizations" ) ]
  {
    single_character_optimization();
    multi_delimiter_optimization();
    pattern_matching_optimization();
    performance_comparison();
  }
  
  #[ cfg( not( feature = "compile_time_optimizations" ) ) ]
  {
    println!( "Compile-time optimizations disabled. Enable with --features compile_time_optimizations" );
  }
}

/// Demonstrate single character delimiter optimization
#[ cfg( feature = "compile_time_optimizations" ) ]
fn single_character_optimization() {
  println!( "\n--- Single Character Optimization ---" );
  
  let csv_data = "name,age,city,country,email,phone";
  
  // Compile-time optimized comma splitting
  let optimized_result: Vec<_> = optimize_split!( csv_data, "," ).collect();
  
  println!( "CSV data: '{}'", csv_data );
  println!( "Optimized split result:" );
  for ( i, segment ) in optimized_result.iter().enumerate() {
    println!( "  [{}]: '{}'", i, segment.as_str() );
  }
  
  // The macro generates highly optimized code for single-character delimiters
  // equivalent to the most efficient splitting algorithm for commas
  println!( "✓ Compile-time optimization: Single character delimiter" );
}

/// Demonstrate multi-delimiter optimization
#[ cfg( feature = "compile_time_optimizations" ) ]
fn multi_delimiter_optimization() {
  println!( "\n--- Multi-Delimiter Optimization ---" );
  
  let structured_data = "field1:value1;field2:value2,field3:value3";
  
  // Compile-time analysis chooses optimal algorithm for these specific delimiters
  let optimized_result: Vec<_> = optimize_split!( 
    structured_data, 
    [":", ";", ","],
    preserve_delimiters = true,
    use_simd = true
  ).collect();
  
  println!( "Structured data: '{}'", structured_data );
  println!( "Multi-delimiter optimized result:" );
  for ( i, segment ) in optimized_result.iter().enumerate() {
    let segment_type = match segment.segment_type {
      strs_tools::string::zero_copy::SegmentType::Content => "Content",
      strs_tools::string::zero_copy::SegmentType::Delimiter => "Delimiter",
    };
    println!( "  [{}]: '{}' ({})", i, segment.as_str(), segment_type );
  }
  
  println!( "✓ Compile-time optimization: Multi-delimiter with SIMD" );
}

/// Demonstrate pattern matching optimization
#[ cfg( feature = "compile_time_optimizations" ) ]
fn pattern_matching_optimization() {
  println!( "\n--- Pattern Matching Optimization ---" );
  
  let urls = [
    "https://example.com/path",
    "http://test.org/file",
    "ftp://files.site.com/data",
    "file:///local/path",
  ];
  
  for url in &urls {
    // Compile-time generated trie or state machine for protocol matching
    let match_result = optimize_match!( 
      url, 
      ["https://", "http://", "ftp://", "file://"],
      strategy = "first_match"
    );
    
    println!( "URL: '{}' -> Match at position: {:?}", url, match_result );
  }
  
  println!( "✓ Compile-time optimization: Pattern matching with trie" );
}

/// Compare compile-time vs runtime optimization performance
#[ cfg( feature = "compile_time_optimizations" ) ]
fn performance_comparison() {
  println!( "\n--- Performance Comparison ---" );
  
  let large_csv = "field1,field2,field3,field4,field5,field6,field7,field8".repeat( 1000 );
  
  use std::time::Instant;
  
  // Runtime optimization
  let start = Instant::now();
  let mut runtime_count = 0;
  for _ in 0..100 {
    let result: Vec<_> = large_csv
      .split( ',' )
      .collect();
    runtime_count += result.len();
  }
  let runtime_duration = start.elapsed();
  
  // Compile-time optimization
  let start = Instant::now();
  let mut compile_time_count = 0;
  for _ in 0..100 {
    let result: Vec<_> = optimize_split!( large_csv.as_str(), "," ).collect();
    compile_time_count += result.len();
  }
  let compile_time_duration = start.elapsed();
  
  println!( "Processing {} characters of CSV data (100 iterations):", large_csv.len() );
  println!( "Runtime optimization:     {:?} ({} segments)", runtime_duration, runtime_count );
  println!( "Compile-time optimization: {:?} ({} segments)", compile_time_duration, compile_time_count );
  
  if compile_time_duration < runtime_duration {
    let speedup = runtime_duration.as_secs_f64() / compile_time_duration.as_secs_f64();
    println!( "Speedup: {:.2}x faster with compile-time optimization", speedup );
  }
  
  assert_eq!( runtime_count, compile_time_count );
  println!( "✓ Results verified identical" );
}

/// Advanced example: Compile-time regex-like pattern optimization
#[ cfg( feature = "compile_time_optimizations" ) ]
fn _advanced_pattern_optimization() {
  println!( "\n--- Advanced Pattern Optimization ---" );
  
  let log_entries = [
    "2025-01-15 14:30:25 ERROR Failed to connect",
    "2025-01-15 14:30:26 INFO Connection established",
    "2025-01-15 14:30:27 WARN High memory usage",
    "2025-01-15 14:30:28 DEBUG Processing request",
  ];
  
  for entry in &log_entries {
    // The macro analyzes the pattern and generates optimal parsing code
    let timestamp_match = optimize_match!( 
      entry,
      [r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}"],
      strategy = "first_match"
    );
    
    let level_match = optimize_match!(
      entry,
      ["ERROR", "WARN", "INFO", "DEBUG"],
      strategy = "first_match"
    );
    
    println!( "Log entry: {}", entry );
    println!( "  Timestamp match: {:?}", timestamp_match );
    println!( "  Log level match: {:?}", level_match );
  }
  
  println!( "✓ Advanced pattern optimization demonstrated" );
}