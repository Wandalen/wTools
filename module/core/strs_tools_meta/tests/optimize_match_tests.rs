//! Integration tests for optimize_match macro
//!
//! # Test Matrix for optimize_match
//!
//! | Test ID | Scenario | Pattern Type | Strategy | Expected Behavior |
//! |---------|----------|--------------|----------|-------------------|
//! | TC1 | Single pattern | "prefix" | default | Single pattern optimization |
//! | TC2 | Multiple small patterns | ["http://", "https://"] | "first_match" | Trie-based optimization |
//! | TC3 | Multiple large patterns | Many long patterns | "first_match" | Sequential matching |
//! | TC4 | Strategy: longest_match | ["a", "ab", "abc"] | "longest_match" | Longest match strategy |
//! | TC5 | Strategy: all_matches | ["a", "b"] | "all_matches" | All matches strategy |
//! | TC6 | Debug mode | "test" | default, debug | Debug output generated |
//!

use strs_tools_meta::optimize_match;

// TC1: Single pattern - should use SinglePattern optimization
#[ test ]
fn tc1_single_pattern()
{
  let result = optimize_match!( "prefix_test_suffix", "test" );
  
  // Should find the pattern
  assert_eq!( result, Some( 7 ) );
}

// TC2: Multiple small patterns - should use TrieBasedMatch optimization
#[ test ]
fn tc2_multiple_small_patterns()
{
  let result = optimize_match!( "https://example.com", [ "http://", "https://" ] );
  
  // Should find https:// at position 0
  assert_eq!( result, Some( 0 ) );
}

// TC3: First match strategy explicit
#[ test ]
fn tc3_first_match_strategy()
{
  let result = optimize_match!( "test http:// and https://", [ "http://", "https://" ], strategy = "first_match" );
  
  // Should find http:// first at position 5
  assert_eq!( result, Some( 5 ) );
}

// TC4: Longest match strategy
#[ test ]
fn tc4_longest_match_strategy()
{
  let result = optimize_match!( "abcdef", [ "a", "ab", "abc" ], strategy = "longest_match" );
  
  // Should find the longest match
  assert_eq!( result, Some( 0 ) );
}

// TC5: All matches strategy
#[ test ]
fn tc5_all_matches_strategy()
{
  let result = optimize_match!( "a test b", [ "a", "b" ], strategy = "all_matches" );
  
  // Should find first match
  assert_eq!( result, Some( 0 ) );
}

// TC6: Debug mode test
// Note: Debug output goes to stderr and can be observed during manual testing
#[ test ]
fn tc6_debug_mode()
{
  let result = optimize_match!( "test_string", "test", debug );
  
  assert_eq!( result, Some( 0 ) );
}

// Test for explicit parameter values to avoid fragile tests
#[ test ]
fn tc7_explicit_parameters()
{
  let result = optimize_match!( "test_string", "test", strategy = "first_match" );
  
  assert_eq!( result, Some( 0 ) );
}

// Test default value equivalence - dedicated test for parameter defaults
#[ test ]
fn tc8_default_value_equivalence()
{
  let result_explicit = optimize_match!( "test_string", "test", strategy = "first_match" );
  let result_default = optimize_match!( "test_string", "test" );
  
  // Results should be equivalent
  assert_eq!( result_explicit, result_default );
}

// Test no match case
#[ test ]
fn tc9_no_match()
{
  let result = optimize_match!( "hello world", "xyz" );
  
  assert_eq!( result, None );
}

// Test empty input
#[ test ]
fn tc10_empty_input()
{
  let result = optimize_match!( "", "test" );
  
  assert_eq!( result, None );
}