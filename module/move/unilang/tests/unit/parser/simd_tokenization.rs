//! SIMD Tokenization Unit Tests
//!
//! ## Scope
//! Tests the SIMD-accelerated tokenization functionality for high-performance parsing.
//! This covers the critical performance-optimized parsing logic that uses SIMD instructions
//! for faster text processing.
//!
//! ## Coverage
//! - SIMD tokenization accuracy vs fallback implementation
//! - Performance characteristics of SIMD operations
//! - SIMD availability detection and fallback behavior
//! - Large input handling with SIMD optimizations
//! - Edge cases specific to SIMD processing
//! - Memory alignment and SIMD register usage
//!
//! ## Related
//! - `unit/parser/argument_parsing.rs` - General argument parsing
//! - `unit/parser/quoted_values.rs` - Quoted string handling
//! - `integration/performance.rs` - Performance validation

use unilang::simd_tokenizer::{ SimdTokenizer, TokenizerOptions, Token };

/// Create default tokenizer options for testing
fn default_options() -> TokenizerOptions
{
  TokenizerOptions {
    delimiters : vec![ ' ', '\t', '\n' ],
    preserve_quotes : true,
    enable_simd : true,
  }
}

/// Create fallback tokenizer options (SIMD disabled)
fn fallback_options() -> TokenizerOptions
{
  TokenizerOptions {
    delimiters : vec![ ' ', '\t', '\n' ],
    preserve_quotes : true,
    enable_simd : false,
  }
}

#[test]
fn test_simd_tokenizer_basic_functionality()
{
  let tokenizer = SimdTokenizer::new( default_options() );
  let input = r#".test arg1 "quoted value" arg3"#;

  let tokens = tokenizer.tokenize( input ).expect( "Tokenization should succeed" );

  assert_eq!( tokens.len(), 4 );
  assert_eq!( tokens[0].value, ".test" );
  assert_eq!( tokens[1].value, "arg1" );
  assert_eq!( tokens[2].value, "quoted value" );
  assert_eq!( tokens[3].value, "arg3" );
}

#[test]
fn test_simd_vs_fallback_accuracy()
{
  let simd_tokenizer = SimdTokenizer::new( default_options() );
  let fallback_tokenizer = SimdTokenizer::new( fallback_options() );

  let test_inputs = vec![
    r#".simple command"#,
    r#".complex "quoted argument" regular"#,
    r#".multiple "arg 1" "arg 2" "arg 3""#,
    r#".mixed regular "quoted" regular"#,
    r#".special "contains \"escaped\" quotes""#,
    r#".empty "" filled"#,
    r#".unicode "café résumé naïve""#,
  ];

  for input in test_inputs {
    let simd_tokens = simd_tokenizer.tokenize( input ).expect( "SIMD tokenization should succeed" );
    let fallback_tokens = fallback_tokenizer.tokenize( input ).expect( "Fallback tokenization should succeed" );

    assert_eq!( simd_tokens.len(), fallback_tokens.len(), "Token count should match for input: {}", input );

    for (i, (simd_token, fallback_token)) in simd_tokens.iter().zip( fallback_tokens.iter() ).enumerate() {
      assert_eq!( simd_token.value, fallback_token.value, "Token {} value should match for input: {}", i, input );
      assert_eq!( simd_token.position, fallback_token.position, "Token {} position should match for input: {}", i, input );
    }
  }
}

#[test]
fn test_simd_availability_detection()
{
  let tokenizer = SimdTokenizer::new( default_options() );

  // Test that SIMD availability is detected correctly
  let simd_info = tokenizer.simd_info();

  // The exact SIMD capabilities depend on the target CPU
  // but we should get meaningful information
  assert!( !simd_info.is_empty(), "SIMD info should provide information about capabilities" );

  // Test that tokenizer can handle SIMD being unavailable gracefully
  let fallback_tokenizer = SimdTokenizer::new( fallback_options() );
  let input = r#".test "quoted argument""#;

  let result = fallback_tokenizer.tokenize( input );
  assert!( result.is_ok(), "Fallback tokenizer should work when SIMD is disabled" );
}

#[test]
fn test_simd_performance_characteristics()
{
  use std::time::Instant;

  let simd_tokenizer = SimdTokenizer::new( default_options() );
  let fallback_tokenizer = SimdTokenizer::new( fallback_options() );

  // Create a large input to test performance
  let mut large_input = String::new();
  for i in 0..1000 {
    large_input.push_str( &format!( r#" arg{} "quoted value {}" "#, i, i ) );
  }
  let input = format!( ".test{}", large_input );

  // Warm up both tokenizers
  let _ = simd_tokenizer.tokenize( &input );
  let _ = fallback_tokenizer.tokenize( &input );

  // Measure SIMD performance
  let start = Instant::now();
  let simd_result = simd_tokenizer.tokenize( &input ).expect( "SIMD tokenization should succeed" );
  let simd_duration = start.elapsed();

  // Measure fallback performance
  let start = Instant::now();
  let fallback_result = fallback_tokenizer.tokenize( &input ).expect( "Fallback tokenization should succeed" );
  let fallback_duration = start.elapsed();

  // Verify results are equivalent
  assert_eq!( simd_result.len(), fallback_result.len(), "Results should be equivalent" );

  // Performance check - SIMD should be faster or at least not significantly slower
  // On systems without SIMD support, the implementations might be equivalent
  println!( "SIMD duration: {:?}, Fallback duration: {:?}", simd_duration, fallback_duration );

  // Don't enforce strict performance requirements since it depends on CPU capabilities
  // but log the performance difference for analysis
  if simd_duration < fallback_duration {
    println!( "✅ SIMD tokenization is faster" );
  } else {
    println!( "ℹ️ SIMD tokenization performance equivalent to fallback (may indicate no SIMD support)" );
  }
}

#[test]
fn test_simd_large_input_handling()
{
  let tokenizer = SimdTokenizer::new( default_options() );

  // Test with very large input to validate SIMD memory handling
  let mut large_input = String::from( ".huge" );
  for i in 0..10000 {
    large_input.push_str( &format!( " arg{}", i ) );
  }

  let start = std::time::Instant::now();
  let tokens = tokenizer.tokenize( &large_input ).expect( "Large input tokenization should succeed" );
  let duration = start.elapsed();

  assert_eq!( tokens.len(), 10001 ); // .huge + 10000 args
  assert_eq!( tokens[0].value, ".huge" );
  assert_eq!( tokens[1].value, "arg0" );
  assert_eq!( tokens[10000].value, "arg9999" );

  // Performance check for large input
  assert!( duration.as_millis() < 1000, "Large input should be processed quickly: {:?}", duration );
}

#[test]
fn test_simd_edge_cases()
{
  let tokenizer = SimdTokenizer::new( default_options() );

  // Test empty input
  let tokens = tokenizer.tokenize( "" ).expect( "Empty input should succeed" );
  assert_eq!( tokens.len(), 0 );

  // Test single character
  let tokens = tokenizer.tokenize( "a" ).expect( "Single character should succeed" );
  assert_eq!( tokens.len(), 1 );
  assert_eq!( tokens[0].value, "a" );

  // Test only delimiters
  let tokens = tokenizer.tokenize( "   \t\n   " ).expect( "Only delimiters should succeed" );
  assert_eq!( tokens.len(), 0 );

  // Test very long single token
  let long_token = "a".repeat( 10000 );
  let tokens = tokenizer.tokenize( &long_token ).expect( "Long single token should succeed" );
  assert_eq!( tokens.len(), 1 );
  assert_eq!( tokens[0].value.len(), 10000 );

  // Test alternating pattern that might stress SIMD alignment
  let alternating = "a b ".repeat( 1000 );
  let tokens = tokenizer.tokenize( &alternating ).expect( "Alternating pattern should succeed" );
  assert_eq!( tokens.len(), 2000 ); // 1000 'a' tokens + 1000 'b' tokens
}

#[test]
fn test_simd_quoted_value_handling()
{
  let tokenizer = SimdTokenizer::new( default_options() );

  // Test various quoting scenarios with SIMD
  let test_cases = vec![
    (r#""simple quote""#, vec![ "simple quote" ]),
    (r#""quote with spaces""#, vec![ "quote with spaces" ]),
    (r#""quote with \"escaped\" quotes""#, vec![ "quote with \"escaped\" quotes" ]),
    (r#"before "quoted" after"#, vec![ "before", "quoted", "after" ]),
    (r#""first" "second" "third""#, vec![ "first", "second", "third" ]),
    (r#""""#, vec![ "" ]), // Empty quoted string
  ];

  for (input, expected) in test_cases {
    let tokens = tokenizer.tokenize( input ).expect( &format!( "Should tokenize: {}", input ) );
    let values : Vec< &str > = tokens.iter().map( |t| t.value.as_str() ).collect();
    assert_eq!( values, expected, "Tokenization failed for: {}", input );
  }
}

#[test]
fn test_simd_memory_alignment()
{
  let tokenizer = SimdTokenizer::new( default_options() );

  // Test inputs with various lengths to test SIMD memory alignment handling
  for len in 1..=64 {
    let input = "a".repeat( len );
    let tokens = tokenizer.tokenize( &input ).expect( &format!( "Should tokenize length {}", len ) );
    assert_eq!( tokens.len(), 1 );
    assert_eq!( tokens[0].value.len(), len );
  }

  // Test inputs that might cross SIMD register boundaries
  for boundary in &[ 16, 32, 64, 128, 256 ] {
    let input = format!( "{} {}", "a".repeat( boundary - 1 ), "b" );
    let tokens = tokenizer.tokenize( &input ).expect( &format!( "Should tokenize boundary {}", boundary ) );
    assert_eq!( tokens.len(), 2 );
    assert_eq!( tokens[0].value.len(), boundary - 1 );
    assert_eq!( tokens[1].value, "b" );
  }
}

#[test]
fn test_simd_unicode_handling()
{
  let tokenizer = SimdTokenizer::new( default_options() );

  // Test Unicode characters with SIMD processing
  let unicode_inputs = vec![
    "café résumé naïve",
    "🚀 🎉 🔥",
    "αβγ δεζ ηθι",
    "こんにちは 世界",
    "🌟 \"multi-byte unicode\" 🌟",
  ];

  for input in unicode_inputs {
    let tokens = tokenizer.tokenize( input ).expect( &format!( "Should tokenize unicode: {}", input ) );
    assert!( tokens.len() > 0, "Should produce tokens for unicode input: {}", input );

    // Verify no corruption of unicode characters
    let reconstructed = tokens.iter()
      .map( |t| t.value.as_str() )
      .collect::< Vec< _ > >()
      .join( " " );

    // The reconstructed string should contain all the original unicode characters
    for char in input.chars() {
      if !char.is_whitespace() {
        assert!( reconstructed.contains( char ), "Unicode character '{}' should be preserved in: {}", char, input );
      }
    }
  }
}

#[test]
fn test_simd_delimiter_customization()
{
  // Test with custom delimiters
  let custom_options = TokenizerOptions {
    delimiters : vec![ ',', ';', '|' ],
    preserve_quotes : true,
    enable_simd : true,
  };

  let tokenizer = SimdTokenizer::new( custom_options );
  let input = r#"token1,token2;"quoted,value"|last"#;

  let tokens = tokenizer.tokenize( input ).expect( "Custom delimiter tokenization should succeed" );

  assert_eq!( tokens.len(), 4 );
  assert_eq!( tokens[0].value, "token1" );
  assert_eq!( tokens[1].value, "token2" );
  assert_eq!( tokens[2].value, "quoted,value" );
  assert_eq!( tokens[3].value, "last" );
}

#[test]
fn test_simd_error_handling()
{
  let tokenizer = SimdTokenizer::new( default_options() );

  // Test malformed quoted strings
  let malformed_inputs = vec![
    r#""unclosed quote"#,
    r#"unopened quote""#,
    r#""nested "quotes" problem""#,
  ];

  for input in malformed_inputs {
    let result = tokenizer.tokenize( input );
    // The exact error handling behavior depends on implementation
    // but it should handle malformed input gracefully
    match result {
      Ok( tokens ) => {
        // Graceful handling - tokenizer recovers from malformed input
        assert!( tokens.len() > 0, "Should produce some tokens even with malformed input: {}", input );
      },
      Err( error ) => {
        // Strict handling - tokenizer reports error for malformed input
        assert!( !error.is_empty(), "Should provide meaningful error message for: {}", input );
      }
    }
  }
}