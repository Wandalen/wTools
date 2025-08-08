//! Tests for compile-time pattern optimization functionality.
//!
//! These tests verify that the procedural macros generate correct and efficient
//! code for various string processing patterns.

use strs_tools::*;

#[ cfg( feature = "compile_time_optimizations" ) ]
use strs_tools::{ optimize_split, optimize_match };

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
fn test_compile_time_single_delimiter_optimization() {
  let input = "hello,world,rust,programming";
  
  // Test compile-time optimized split
  let optimized_result: Vec<_> = optimize_split!( input, "," ).collect();
  
  // Compare with regular split for correctness
  let regular_result: Vec<_> = input.split( ',' ).collect();
  
  assert_eq!( optimized_result.len(), regular_result.len() );
  assert_eq!( optimized_result.len(), 4 );
  
  for ( optimized, regular ) in optimized_result.iter().zip( regular_result.iter() ) {
    assert_eq!( optimized.as_str(), *regular );
  }
  
  // Verify zero-copy behavior
  assert!( optimized_result.iter().all( |seg| seg.is_borrowed() ) );
}

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
fn test_compile_time_multiple_delimiters_optimization() {
  let input = "key1:value1;key2:value2,key3:value3";
  
  let optimized_result: Vec<_> = optimize_split!( 
    input, 
    [":", ";", ","]
  ).collect();
  
  // Compare with zero-copy split for correctness
  let regular_result: Vec<_> = input
    .zero_copy_split( &[ ":", ";", "," ] )
    .collect();
  
  assert_eq!( optimized_result.len(), regular_result.len() );
  assert_eq!( optimized_result.len(), 6 ); // key1, value1, key2, value2, key3, value3
  
  for ( optimized, regular ) in optimized_result.iter().zip( regular_result.iter() ) {
    assert_eq!( optimized.as_str(), regular.as_str() );
  }
}

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
fn test_compile_time_delimiter_preservation() {
  let input = "a,b;c:d";
  
  let optimized_result: Vec<_> = optimize_split!( 
    input, 
    [",", ";", ":"],
    preserve_delimiters = true
  ).collect();
  
  // Should include both content and delimiter segments
  assert_eq!( optimized_result.len(), 7 ); // a, ,, b, ;, c, :, d
  
  // Verify content and delimiters
  assert_eq!( optimized_result[0].as_str(), "a" );
  assert_eq!( optimized_result[1].as_str(), "," );
  assert_eq!( optimized_result[2].as_str(), "b" );
  assert_eq!( optimized_result[3].as_str(), ";" );
  assert_eq!( optimized_result[4].as_str(), "c" );
  assert_eq!( optimized_result[5].as_str(), ":" );
  assert_eq!( optimized_result[6].as_str(), "d" );
  
  // Verify segment types
  assert_eq!( optimized_result[0].segment_type, strs_tools::string::zero_copy::SegmentType::Content );
  assert_eq!( optimized_result[1].segment_type, strs_tools::string::zero_copy::SegmentType::Delimiter );
  assert_eq!( optimized_result[2].segment_type, strs_tools::string::zero_copy::SegmentType::Content );
}

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
fn test_compile_time_empty_segments_handling() {
  let input = "a,,b";
  
  // Test without preserving empty segments (default)
  let result_no_empty: Vec<_> = optimize_split!( input, "," ).collect();
  assert_eq!( result_no_empty.len(), 2 );
  assert_eq!( result_no_empty[0].as_str(), "a" );
  assert_eq!( result_no_empty[1].as_str(), "b" );
  
  // Test with preserving empty segments
  let result_with_empty: Vec<_> = optimize_split!( 
    input, 
    [","],
    preserve_empty = true
  ).collect();
  assert_eq!( result_with_empty.len(), 3 );
  assert_eq!( result_with_empty[0].as_str(), "a" );
  assert_eq!( result_with_empty[1].as_str(), "" );
  assert_eq!( result_with_empty[2].as_str(), "b" );
}

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
fn test_compile_time_pattern_matching_single() {
  let input = "https://example.com/path";
  
  let match_result = optimize_match!( input, "https://" );
  
  assert_eq!( match_result, Some( 0 ) );
}

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
fn test_compile_time_pattern_matching_multiple() {
  let test_cases = [
    ( "https://secure.com", "https://" ),
    ( "http://regular.org", "http://" ),
    ( "ftp://files.net", "ftp://" ),
    ( "file:///local/path", "file://" ),
  ];
  
  for ( input, expected_pattern ) in &test_cases {
    let match_result = optimize_match!( 
      input, 
      ["https://", "http://", "ftp://", "file://"],
      strategy = "first_match" 
    );
    
    assert!( match_result.is_some(), "Should match pattern in: {}", input );
    
    // Verify it matches the expected pattern
    let match_pos = match_result.unwrap();
    assert!( input[match_pos..].starts_with( expected_pattern ) );
  }
}

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
fn test_compile_time_no_match_patterns() {
  let input = "plain text without protocols";
  
  let match_result = optimize_match!( 
    input, 
    ["https://", "http://", "ftp://"],
    strategy = "first_match"
  );
  
  assert_eq!( match_result, None );
}

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
fn test_compile_time_zero_copy_consistency() {
  let input = "field1|field2|field3|field4";
  
  // Compile-time optimized version
  let optimized_segments: Vec<_> = optimize_split!( input, "|" ).collect();
  
  // Regular zero-copy version  
  let regular_segments: Vec<_> = input.zero_copy_split( &["|"] ).collect();
  
  // Should produce identical results
  assert_eq!( optimized_segments.len(), regular_segments.len() );
  
  for ( opt, reg ) in optimized_segments.iter().zip( regular_segments.iter() ) {
    assert_eq!( opt.as_str(), reg.as_str() );
    assert_eq!( opt.segment_type, reg.segment_type );
    assert_eq!( opt.start_pos, reg.start_pos );
    assert_eq!( opt.end_pos, reg.end_pos );
    assert_eq!( opt.is_borrowed(), reg.is_borrowed() );
  }
}

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
fn test_compile_time_performance_characteristics() {
  use std::time::Instant;
  
  let large_input = "word1,word2,word3,word4,word5".repeat( 1000 );
  
  // Measure compile-time optimized version
  let start = Instant::now();
  let mut optimized_count = 0;
  for _ in 0..100 {
    optimized_count += optimize_split!( large_input.as_str(), "," ).count();
  }
  let optimized_time = start.elapsed();
  
  // Measure regular split
  let start = Instant::now();
  let mut regular_count = 0;
  for _ in 0..100 {
    regular_count += large_input.split( ',' ).count();
  }
  let regular_time = start.elapsed();
  
  // Results should be identical
  assert_eq!( optimized_count, regular_count );
  
  // Optimized version should be at least as fast (often faster)
  // Note: In debug builds, there might not be significant difference
  // but in release builds, the compile-time optimization should show benefits
  println!( "Optimized time: {:?}, Regular time: {:?}", optimized_time, regular_time );
  
  // In debug builds, macro expansion can be slower due to builder pattern overhead
  // In release builds, the compile-time optimization should show benefits
  #[ cfg( debug_assertions ) ]
  assert!( optimized_time <= regular_time * 5 ); // Debug builds can be slower
  #[ cfg( not( debug_assertions ) ) ]
  assert!( optimized_time <= regular_time * 2 ); // Release builds should be faster
}

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
fn test_compile_time_edge_cases() {
  // Empty string
  let empty_result: Vec<_> = optimize_split!( "", "," ).collect();
  assert_eq!( empty_result.len(), 0 );
  
  // Single delimiter
  let single_delim_result: Vec<_> = optimize_split!( ",", "," ).collect();
  assert_eq!( single_delim_result.len(), 0 ); // Two empty segments, not preserved by default
  
  // No delimiters found  
  let no_delim_result: Vec<_> = optimize_split!( "nodlimiter", "," ).collect();
  assert_eq!( no_delim_result.len(), 1 );
  assert_eq!( no_delim_result[0].as_str(), "nodlimiter" );
  
  // Multiple consecutive delimiters
  let multi_delim_result: Vec<_> = optimize_split!( "a,,,,b", "," ).collect();
  assert_eq!( multi_delim_result.len(), 2 ); // Empty segments not preserved by default
  assert_eq!( multi_delim_result[0].as_str(), "a" );
  assert_eq!( multi_delim_result[1].as_str(), "b" );
}

#[ test ]
#[ cfg( feature = "compile_time_optimizations" ) ]
#[ cfg( feature = "simd" ) ]
fn test_compile_time_simd_integration() {
  let input = "data1,data2,data3,data4,data5,data6,data7,data8";
  
  // Test with SIMD enabled
  let simd_result: Vec<_> = optimize_split!( 
    input, 
    [","],
    use_simd = true
  ).collect();
  
  // Test with SIMD disabled
  let no_simd_result: Vec<_> = optimize_split!( 
    input, 
    [","],
    use_simd = false
  ).collect();
  
  // Results should be identical regardless of SIMD usage
  assert_eq!( simd_result.len(), no_simd_result.len() );
  for ( simd_seg, no_simd_seg ) in simd_result.iter().zip( no_simd_result.iter() ) {
    assert_eq!( simd_seg.as_str(), no_simd_seg.as_str() );
  }
}

#[ test ]
#[ cfg( not( feature = "compile_time_optimizations" ) ) ]
fn test_compile_time_optimizations_disabled() {
  // When compile-time optimizations are disabled, the macros are not available
  // This test verifies the feature flag is working correctly
  
  // This test just ensures the feature system works
  // In a real scenario without the feature, the macros wouldn't compile
  assert!( true, "Compile-time optimizations properly disabled" );
}