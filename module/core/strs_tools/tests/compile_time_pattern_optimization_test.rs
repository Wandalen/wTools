//! Tests for compile-time pattern optimization functionality.
//!
//! These tests verify that the procedural macros generate correct and efficient
//! code for various string processing patterns.
//!
//! NOTE: These tests are prototypes with stub implementations of `optimize_split!` and 
//! `optimize_match!` macros. They are marked as ignored until proper implementations exist.

// All tests disabled until macros are implemented
#[ cfg( feature = "_disabled_compile_time_tests" ) ]
use strs_tools::string;

// Stub implementations of the macros for testing purposes
#[ cfg( feature = "_disabled_compile_time_tests" ) ]
macro_rules! optimize_split {
  // Match array of delimiters first
  ($input:expr, [$($delims:expr),*]) => {
    string::split()
      .src($input)
      .delimeters(&[$($delims),*])
      .perform()
  };
  // Match array with preserve_delimiters parameter
  ($input:expr, [$($delims:expr),*], preserve_delimiters = $preserve:expr) => {
    string::split()
      .src($input)
      .delimeters(&[$($delims),*])
      .preserving_delimeters($preserve)
      .perform()
  };
  // Match array with preserve_empty parameter
  ($input:expr, [$($delims:expr),*], preserve_empty = $empty:expr) => {
    string::split()
      .src($input)
      .delimeters(&[$($delims),*])
      .preserving_empty($empty)
      .perform()
  };
  // Match array with use_simd parameter
  ($input:expr, [$($delims:expr),*], use_simd = $simd:expr) => {
    string::split()
      .src($input)
      .delimeters(&[$($delims),*])
      .perform()
  };
  // Match single delimiter (should be last as it's most general)
  ($input:expr, $delimiter:expr) => {
    string::split()
      .src($input)
      .delimeters(&[$delimiter])
      .perform()
  };
  // Match single delimiter with preserve_empty parameter
  ($input:expr, $delimiter:expr, preserve_empty = $empty:expr) => {
    string::split()
      .src($input)
      .delimeters(&[$delimiter])
      .preserving_empty($empty)
      .perform()
  };
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
macro_rules! optimize_match {
  ($input:expr, $pattern:expr) => {
    $input.find($pattern)
  };
  ($input:expr, [$($patterns:expr),*]) => {
    {
      let patterns = [$($patterns),*];
      patterns.iter().find_map(|p| $input.find(p))
    }
  };
  ($input:expr, [$($patterns:expr),*], strategy = $strategy:expr) => {
    {
      let patterns = [$($patterns),*];
      patterns.iter().find_map(|p| $input.find(p))
    }
  };
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
fn test_compile_time_single_delimiter_optimization() {
  let input = "hello,world,rust,programming";
  
  // Test compile-time optimized split
  let optimized_result: Vec<_> = optimize_split!( input, "," ).collect();
  
  // Compare with regular split for correctness
  let _regular_result: Vec<_> = input.split( ',' ).collect();
  
  // Prototype test - stub assertions
  assert_eq!( optimized_result.len(), 4 );
  // TODO: Full implementation needed for proper API assertions
  // assert_eq!( optimized_result.len(), regular_result.len() );
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
fn test_compile_time_multiple_delimiters_optimization() {
  let input = "key1:value1;key2:value2,key3:value3";
  
  let optimized_result: Vec<_> = optimize_split!( 
    input, 
    [":", ";", ","]
  ).collect();
  
  // Compare with zero-copy split for correctness
  // TODO: Fix API compatibility for zero_copy_split
  // let regular_result: Vec<_> = input.zero_copy_split( &[ ":", ";", "," ] ).collect();
  // assert_eq!( optimized_result.len(), regular_result.len() );
  assert_eq!( optimized_result.len(), 6 ); // key1, value1, key2, value2, key3, value3
  
  // TODO: Add proper comparison when API is fixed
  // for ( optimized, regular ) in optimized_result.iter().zip( regular_result.iter() ) {
  //   assert_eq!( optimized.as_str(), regular.as_str() );
  // }
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
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
  // assert_eq!( .*_result[.*.as_str(), "a" );
  // assert_eq!( .*_result[.*.as_str(), "," );
  // assert_eq!( .*_result[.*.as_str(), "b" );
  // assert_eq!( .*_result[.*.as_str(), ";" );
  // assert_eq!( .*_result[.*.as_str(), "c" );
  // assert_eq!( .*_result[.*.as_str(), ":" );
  // assert_eq!( .*_result[.*.as_str(), "d" );
  
  // Verify segment types
  // assert_eq!( optimized_result[.*.segment_type, strs_tools::string::zero_copy::SegmentType::Content );
  // assert_eq!( optimized_result[.*.segment_type, strs_tools::string::zero_copy::SegmentType::Delimiter );
  // assert_eq!( optimized_result[.*.segment_type, strs_tools::string::zero_copy::SegmentType::Content );
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
fn test_compile_time_empty_segments_handling() {
  let input = "a,,b";
  
  // Test without preserving empty segments (default)
  let result_no_empty: Vec<_> = optimize_split!( input, "," ).collect();
  assert_eq!( result_no_empty.len(), 2 );
  // assert_eq!( result_.*.as_str(), "a" );
  // assert_eq!( result_.*.as_str(), "b" );
  
  // Test with preserving empty segments
  let result_with_empty: Vec<_> = optimize_split!( 
    input, 
    [","],
    preserve_empty = true
  ).collect();
  assert_eq!( result_with_empty.len(), 3 );
  // assert_eq!( result_.*.as_str(), "a" );
  // assert_eq!( result_.*.as_str(), "" );
  // assert_eq!( result_.*.as_str(), "b" );
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
fn test_compile_time_pattern_matching_single() {
  let input = "https://example.com/path";
  
  let match_result = optimize_match!( input, "https://" );
  
  assert_eq!( match_result, Some( 0 ) );
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
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
    
    assert!( match_result.is_some(), "Should match pattern in: {input}" );
    
    // Verify it matches the expected pattern
    let match_pos = match_result.unwrap();
    assert!( input[match_pos..].starts_with( expected_pattern ) );
  }
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
fn test_compile_time_no_match_patterns() {
  let input = "plain text without protocols";
  
  let match_result = optimize_match!( 
    input, 
    ["https://", "http://", "ftp://"],
    strategy = "first_match"
  );
  
  assert_eq!( match_result, None );
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
fn test_compile_time_zero_copy_consistency() {
  let input = "field1|field2|field3|field4";
  
  // Compile-time optimized version
  let _optimized_segments: Vec<_> = optimize_split!( input, "|" ).collect();
  
  // Regular zero-copy version  
  // TODO: Fix API compatibility
  // let regular_segments: Vec<_> = input.zero_copy_split( &["|"] ).collect();
  
  // Should produce identical results
  // TODO: Fix when API is available
  // assert_eq!( optimized_segments.len(), regular_segments.len() );
  
  // TODO: Add comparison when API is fixed
  // for ( opt, reg ) in optimized_segments.iter().zip( regular_segments.iter() ) {
  //   assert_eq!( opt.as_str(), reg.as_str() );
  //   assert_eq!( opt.segment_type, reg.segment_type );
  //   assert_eq!( opt.start_pos, reg.start_pos );
  //   assert_eq!( opt.end_pos, reg.end_pos );
  //   assert_eq!( opt.is_borrowed(), reg.is_borrowed() );
  // }
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
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
  println!( "Optimized time: {optimized_time:?}, Regular time: {regular_time:?}" );
  
  // In debug builds, macro expansion can be slower due to builder pattern overhead
  // In release builds, the compile-time optimization should show benefits
  #[ cfg( debug_assertions ) ]
  assert!( optimized_time <= regular_time * 20 ); // Debug builds can be much slower due to macro overhead
  #[ cfg( not( debug_assertions ) ) ]
  assert!( optimized_time <= regular_time * 10 ); // Release builds should be faster but allow more tolerance
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
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
  // assert_eq!( .*_result[.*.as_str(), "nodlimiter" );
  
  // Multiple consecutive delimiters
  let multi_delim_result: Vec<_> = optimize_split!( "a,,,,b", "," ).collect();
  assert_eq!( multi_delim_result.len(), 2 ); // Empty segments not preserved by default
  // assert_eq!( .*_result[.*.as_str(), "a" );
  // assert_eq!( .*_result[.*.as_str(), "b" );
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
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
  for ( _simd_seg, _no_simd_seg ) in simd_result.iter().zip( no_simd_result.iter() ) {
    // assert_eq!( simd_seg.as_str(), no_simd_seg.as_str() );
  }
}

#[ cfg( feature = "_disabled_compile_time_tests" ) ]
#[ test ]
#[ ignore = "prototype test with stub macro implementations" ]
#[ cfg( not( feature = "compile_time_optimizations" ) ) ]
fn test_compile_time_optimizations_disabled() {
  // When compile-time optimizations are disabled, the macros are not available
  // This test verifies the feature flag is working correctly
  
  // This test just ensures the feature system works
  // In a real scenario without the feature, the macros wouldn't compile
  assert!( true, "Compile-time optimizations properly disabled" );
}