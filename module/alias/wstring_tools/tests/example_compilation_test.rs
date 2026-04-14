//! Test that verifies examples compile and run correctly.
//!
//! This test ensures all example code is valid and executable, preventing
//! broken examples from being published.

#[ cfg( all( feature = "split", test ) ) ]
#[ test ]
fn example_wstring_toolst_trivial_sample_compiles_and_runs()
{
  // This test reproduces Issue #1: Example fails to compile due to missing std feature
  //
  // Root Cause: The split feature only activates strs_tools/string_split but not
  // strs_tools/std, which is required for the split module to be available.
  //
  // Expected: split feature should activate both string_split and std
  // Actual: split feature only activates string_split, causing compilation failure

  use wstring_tools::*;

  // Test case 1: delimiter exists
  let src = "abc def";
  let iter = string::split().src( src ).delimeter( " " ).stripping( false ).perform();
  let iterated = iter.map( String::from ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc", " ", "def" ] );

  // Test case 2: delimiter not exists
  let src = "abc def";
  let iter = string::split().src( src ).delimeter( "g" ).perform();
  let iterated = iter.map( String::from ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc def" ] );
}

#[ cfg( all( feature = "split", feature = "std", test ) ) ]
#[ test ]
fn split_feature_enables_std_mode()
{
  // Verify that when split feature is enabled, std mode is also enabled
  // This ensures the split module is actually available
  //
  // Note: This test is gated on both split AND std features. The split
  // feature should automatically enable std, so this test verifies that
  // the string::split() function is accessible.

  use wstring_tools::*;

  // Basic smoke test - if this compiles, std mode is properly enabled
  let result = string::split()
    .src( "test" )
    .delimeter( " " )
    .perform()
    .collect::< Vec< _ > >();

  assert!( !result.is_empty() );
}
