//!
//! Tests for `zip` re-export functionality.
//!
//! ## Root Cause
//!
//! Issue found during manual testing: `zip()` was unavailable when `no_std` feature was enabled
//! because the cfg guard was `#[ cfg(not(feature = "no_std")) ]`, which disabled `zip` when
//! `no_std` was active. However, `core::iter::zip` is available in both std and `no_std`
//! environments, so this guard was incorrect.
//!
//! ## Why Not Caught
//!
//! - Default features don't include `no_std`, so normal testing passed
//! - Running with `--all-features` enabled `no_std`, exposing the bug
//! - No existing tests verified `zip` availability with all feature combinations
//!
//! ## Fix Applied
//!
//! Removed the `#[ cfg(not(feature = "no_std")) ]` guard from `zip` re-export in `src/iter.rs:267`,
//! making `zip` unconditionally available since `core::iter::zip` exists in all configurations.
//!
//! ## Prevention
//!
//! Added comprehensive tests that verify `zip` works with:
//! - Default features (`no_std` disabled)
//! - All features (`no_std` enabled)
//! - Various iterator configurations (empty, equal length, different lengths)
//!
//! ## Pitfall
//!
//! When re-exporting from `core::` namespace, don't add cfg guards based on `no_std` unless the
//! functionality genuinely requires std. `core::iter::zip` is available everywhere, so it should
//! be unconditionally re-exported.
//!

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use the_module::*;

//

/// Test zip with equal length iterators (normal case from `feature/001_itertools_reexports.md`)
#[ test ]
#[ cfg( feature = "enabled" ) ]
fn zip_equal_length()
{
  let vec = vec![ 5, 1, -2 ];
  let added = vec![ "a", "b", "c" ];
  let mut result = vec![];
  let zipped = zip( &vec, &added );
  for ( left, right ) in zipped
  {
    result.push( ( *left, *right ) );
  }
  assert_eq!( result, vec![ ( 5, "a" ), ( 1, "b" ), ( -2, "c" ) ] );
}

/// Test zip with empty iterators (corner case)
#[ test ]
#[ cfg( feature = "enabled" ) ]
fn zip_empty()
{
  let a: Vec< i32 > = vec![];
  let b: Vec< &str > = vec![];
  let zipped: Vec< _ > = zip( &a, &b ).collect();
  assert_eq!( zipped.len(), 0 );
}

/// Test zip with different length iterators - first longer (corner case)
#[ test ]
#[ cfg( feature = "enabled" ) ]
fn zip_first_longer()
{
  let a = vec![ 1, 2, 3, 4 ];
  let b = vec![ "a", "b" ];
  let zipped: Vec< _ > = zip( &a, &b ).collect();
  // zip stops at shortest iterator
  assert_eq!( zipped.len(), 2 );
  assert_eq!( zipped, vec![ ( &1, &"a" ), ( &2, &"b" ) ] );
}

/// Test zip with different length iterators - second longer (corner case)
#[ test ]
#[ cfg( feature = "enabled" ) ]
fn zip_second_longer()
{
  let a = vec![ 1, 2 ];
  let b = vec![ "a", "b", "c", "d" ];
  let zipped: Vec< _ > = zip( &a, &b ).collect();
  // zip stops at shortest iterator
  assert_eq!( zipped.len(), 2 );
  assert_eq!( zipped, vec![ ( &1, &"a" ), ( &2, &"b" ) ] );
}

/// Test that zip is available even when `no_std` feature is enabled.
/// This test exists specifically to prevent regression of the bug where zip
/// was unavailable with `no_std` feature.
#[ test ]
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "no_std" ) ]
fn zip_available_with_no_std()
{
  let a = vec![ 1, 2, 3 ];
  let b = vec![ "x", "y", "z" ];
  let result: Vec< _ > = zip( &a, &b ).collect();
  assert_eq!( result, vec![ ( &1, &"x" ), ( &2, &"y" ), ( &3, &"z" ) ] );
}
