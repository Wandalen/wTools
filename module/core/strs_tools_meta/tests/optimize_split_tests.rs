//! Integration tests for `optimize_split` macro
//!
//! # Test Matrix for `optimize_split`
//!
//! | Test ID | Scenario | Delimiter Type | Options | Expected Behavior |
//! |---------|----------|----------------|---------|-------------------|
//! | TC1 | Single char delimiter | "," | default | Single char optimization |
//! | TC2 | Multiple char single delim | "->" | default | Multi-char delimiter optimization |
//! | TC3 | Multiple delimiters | `[",", ";"]` | default | Multi-delimiter optimization |
//! | TC4 | Complex delimiters | `[",", "->", "::"]` | default | Complex pattern fallback |
//! | TC5 | Preserve delimiters | "," | preserve_delimiters=true | Include delimiters in result |
//! | TC6 | Preserve empty | "," | preserve_empty=true | Include empty segments |
//! | TC7 | Multiple delimiters simple | `[",", ";"]` | default | Multi-delimiter optimization |
//! | TC8 | Debug mode | "," | debug | Debug output generated |
//!

#[ cfg( feature = "optimize_split" ) ]
use strs_tools_meta::optimize_split;

// TC1: Single character delimiter - should use SingleCharDelimiter optimization
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn tc1_single_char_delimiter()
{
  let result = optimize_split!( "a,b,c", "," );
  
  // Should generate optimized single character split
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], "a" );
  assert_eq!( result[ 1 ], "b" );
  assert_eq!( result[ 2 ], "c" );
}

// TC2: Multiple character single delimiter - should use MultipleCharDelimiters optimization
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn tc2_multi_char_single_delimiter()
{
  let result = optimize_split!( "a->b->c", "->" );
  
  // Should generate multi-char delimiter optimization
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], "a" );
  assert_eq!( result[ 1 ], "b" );
  assert_eq!( result[ 2 ], "c" );
}

// TC3: Multiple delimiters - should use MultipleCharDelimiters optimization
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn tc3_multiple_delimiters()
{
  let result = optimize_split!( "a,b;c", [ ",", ";" ] );
  
  // Should generate multi-delimiter optimization
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], "a" );
  assert_eq!( result[ 1 ], "b" );
  assert_eq!( result[ 2 ], "c" );
}

// TC4: Complex delimiters - should use ComplexPattern fallback
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn tc4_complex_delimiters()
{
  let result = optimize_split!( "a,b->c::d", [ ",", "->", "::" ] );
  
  // Should generate complex pattern fallback
  assert!( result.len() >= 3 );
  assert_eq!( result[ 0 ], "a" );
}

// TC5: Preserve delimiters option
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn tc5_preserve_delimiters()
{
  let result = optimize_split!( "a,b,c", ",", preserve_delimiters = true );
  
  // Should include delimiters in result
  assert!( result.len() >= 3 );
  assert_eq!( result[ 0 ], "a" );
}

// TC6: Preserve empty segments option
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn tc6_preserve_empty()
{
  let result = optimize_split!( "a,,c", ",", preserve_empty = true );
  
  // Should include empty segments
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], "a" );
  assert_eq!( result[ 1 ], "" );
  assert_eq!( result[ 2 ], "c" );
}

// TC7: Multiple delimiters (formerly SIMD disabled test - SIMD parameter removed)
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn tc7_multiple_delimiters_simple()
{
  let result = optimize_split!( "a,b;c", [ ",", ";" ] );
  
  // Should use optimized multi-delimiter split
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], "a" );
  assert_eq!( result[ 1 ], "b" );
  assert_eq!( result[ 2 ], "c" );
}

// TC8: Debug mode test
// Note: Debug output goes to stderr and can be observed during manual testing
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn tc8_debug_mode()
{
  let result = optimize_split!( "a,b,c", ",", debug );
  
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], "a" );
  assert_eq!( result[ 1 ], "b" );
  assert_eq!( result[ 2 ], "c" );
}

// Test for explicit parameter values to avoid fragile tests
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn tc9_explicit_parameters()
{
  let result = optimize_split!(
    "a,b,c",
    ",",
    preserve_delimiters = false,
    preserve_empty = false
  );
  
  assert_eq!( result.len(), 3 );
  assert_eq!( result[ 0 ], "a" );
  assert_eq!( result[ 1 ], "b" );
  assert_eq!( result[ 2 ], "c" );
}

// Test default value equivalence - dedicated test for parameter defaults
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn tc10_default_value_equivalence()
{
  let result_explicit = optimize_split!(
    "a,b,c",
    ",",
    preserve_delimiters = false,
    preserve_empty = false
  );
  
  let result_default = optimize_split!( "a,b,c", "," );
  
  // Results should be equivalent
  assert_eq!( result_explicit, result_default );
}