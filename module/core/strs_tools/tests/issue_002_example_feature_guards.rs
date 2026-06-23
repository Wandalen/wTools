//! Bug Reproducer: ISSUE-002 - Incorrect Feature Guards in Examples
//!
//! ## Root Cause
//!
//! Examples use `#[cfg(all(feature = "string_split", not(feature = "no_std")))]`
//! guards, but the default feature set includes BOTH `string_split` AND `no_std`.
//! This causes all example code to be excluded from compilation even when the
//! features are available.
//!
//! ## Why Not Caught
//!
//! - Examples compile successfully (empty main functions are valid)
//! - No automated testing of example execution
//! - Feature guard logic is subtle and easy to misunderstand
//! - Default feature combinations not tested
//!
//! ## Fix Applied
//!
//! Updated feature guards to one of:
//! 1. `#[cfg(feature = "string_split")]` - Most common case
//! 2. `#[cfg(all(feature = "string_split", feature = "std"))]` - When std required
//! 3. `#[cfg(not(feature = "no_std"))]` - When checking for std environment
//!
//! ## Prevention
//!
//! - Add CI check that examples produce expected output
//! - Test examples with default features, not just `--all-features`
//! - Document feature guard patterns in contribution guidelines
//! - Use feature guard validation in pre-commit hooks
//!
//! ## Pitfall
//!
//! **Never use `not(feature = "no_std")` with other feature requirements in `all()`**
//! - Rationale: `no_std` is often included in default features for portability
//! - Pattern `all(feature = "X", not(feature = "no_std"))` excludes defaults
//! - Use `feature = "std"` explicitly if std environment required
//! - Use `feature = "X"` alone if feature works in both `no_std` and std

// test_kind: bug_reproducer(issue-002)

#[ cfg( feature = "string_split" ) ]
#[ test ]
fn test_default_features_enable_string_split()
{
  // This test verifies that string_split functionality is available
  // with default features, which was the root cause of ISSUE-002

  let src = "a,b,c";
  let iter = strs_tools::string::split()
    .src( src )
    .delimiter( "," )
    .perform();

  let result: Vec< String > = iter.map( String::from ).collect();
  assert_eq!( result.len(), 5 ); // ["a", ",", "b", ",", "c"]
}

#[ cfg( all( feature = "string_split", feature = "no_std" ) ) ]
#[ test ]
fn test_no_std_and_string_split_coexist()
{
  // This test verifies that no_std and string_split CAN coexist
  // (which was incorrectly excluded by the buggy feature guards)

  let src = "x:y";
  let iter = strs_tools::string::split()
    .src( src )
    .delimiter( ":" )
    .perform();

  let count = iter.count();
  assert_eq!( count, 3 ); // ["x", ":", "y"]
}
