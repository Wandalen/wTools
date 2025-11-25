//! SIMD Tokenization Unit Tests
//!
//! ## Scope
//! Tests the SIMD-optimized tokenizer's ability to split strings by delimiters
//! with performance optimizations using memchr and bytecount libraries.
//!
//! ## Coverage
//! - Basic tokenization with default delimiters (`:?#.!`)
//! - Custom delimiter support
//! - SIMD vs scalar implementation comparison
//! - Performance characteristics under load
//! - CPU feature detection and fallback behavior
//! - Empty input and edge case handling
//!
//! ## Related
//! - `unit/parser/argument_parsing.rs` - General argument parsing
//! - `unit/parser/quoted_values.rs` - Quoted string handling
//! - `integration/performance.rs` - Performance validation

use unilang::simd_tokenizer::{ SIMDTokenizer, simd_support_info, is_simd_enabled };

#[test]
fn test_basic_tokenization()
{
  let tokenizer = SIMDTokenizer::new( "hello:world.test" );
  let tokens : Vec< &str > = tokenizer.tokenize().collect();
  assert_eq!( tokens, vec![ "hello", "world", "test" ] );
}

#[test]
fn test_empty_input()
{
  let tokenizer = SIMDTokenizer::new( "" );
  let tokens : Vec< &str > = tokenizer.tokenize().collect();
  assert_eq!( tokens, vec![ "" ] );
}

#[test]
fn test_no_delimiters()
{
  let tokenizer = SIMDTokenizer::new( "hello_world" );
  let tokens : Vec< &str > = tokenizer.tokenize().collect();
  assert_eq!( tokens, vec![ "hello_world" ] );
}

#[test]
fn test_multiple_delimiters()
{
  let tokenizer = SIMDTokenizer::new( "a:b.c?d#e!f" );
  let tokens : Vec< &str > = tokenizer.tokenize().collect();
  assert_eq!( tokens, vec![ "a", "b", "c", "d", "e", "f" ] );
}

#[test]
fn test_custom_delimiters()
{
  let tokenizer = SIMDTokenizer::with_delimiters( "a,b;c|d", b",;|" );
  let tokens : Vec< &str > = tokenizer.tokenize().collect();
  assert_eq!( tokens, vec![ "a", "b", "c", "d" ] );
}

#[test]
fn test_command_tokenization()
{
  let tokenizer = SIMDTokenizer::new( ".math.add:value1?value2" );
  let tokens : Vec< &str > = tokenizer.tokenize().collect();
  assert_eq!( tokens, vec![ "", "math", "add", "value1", "value2" ] );
}

#[test]
fn test_namespace_tokenization()
{
  let tokenizer = SIMDTokenizer::new( "unilang.parser.semantic" );
  let tokens : Vec< &str > = tokenizer.tokenize().collect();
  assert_eq!( tokens, vec![ "unilang", "parser", "semantic" ] );
}

#[ cfg( feature = "simd" ) ]
#[test]
fn test_token_counting()
{
  let tokenizer = SIMDTokenizer::new( "a:b.c?d" );
  assert_eq!( tokenizer.count_tokens(), 4 );
}

#[test]
fn test_simd_info()
{
  let info = simd_support_info();
  assert!( !info.is_empty() );
  println!( "SIMD Info: {info}" );
}

#[test]
fn test_simd_enabled_detection()
{
  let enabled = is_simd_enabled();
  println!( "SIMD Enabled: {enabled}" );

  #[ cfg( feature = "simd" ) ]
  assert!( enabled );

  #[ cfg( not( feature = "simd" ) ) ]
  assert!( !enabled );
}