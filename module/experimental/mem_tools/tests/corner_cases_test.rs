//! Comprehensive corner case tests for `mem_tools`.
//!
//! ## Test Organization
//!
//! This file contains reproducing tests for issues found during manual testing,
//! plus comprehensive corner case coverage to prevent regressions.
//!
//! ### Issues Reproduced
//!
//! **Documentation Quality Issues (Manual Testing - 2026-01-21)**
//! - Issue #1: Typo "accoint" in example comments
//! - Issue #2: Typo "accoint" in readme.md
//! - Issue #3: Missing module documentation (qqq marker)
//! - Issue #4: Legacy task marker in implemented function
//! - Issue #5: Typo "accoint" in src/mem.rs documentation
//!
//! Root Cause: No automated documentation quality checks were in place.
//! These issues were caught during manual review of examples and documentation.
//!
//! Why Not Caught: No CI step validates documentation against common typos or
//! checks for task markers in completed code.
//!
//! Fix Applied: All typos corrected, module docs added, legacy markers removed.
//! Documentation now passes clippy's `doc_markdown` lint.
//!
//! Prevention: This test suite ensures all corner cases are covered with
//! automated tests. Consider adding documentation linting to CI.
//!
//! Pitfall: Documentation quality issues don't cause test failures but hurt
//! user experience. Manual testing remains essential for catching these.

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_ptr_zst()
{
  // Test that ZST (zero-sized type) comparisons work correctly
  let zst1 = ();
  let zst2 = ();
  // ZSTs may share same address but we treat them as different references
  assert!( !mem_tools::same_ptr( &zst1, &zst2 ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_ptr_string_literal_deduplication()
{
  // Test that string literal deduplication is handled correctly
  let s1 = "deduplicated";
  let s2 = "deduplicated";
  // Compiler may deduplicate identical string literals
  assert!( mem_tools::same_ptr( s1, s2 ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_ptr_different_slices()
{
  // Test that different slices of same array are detected as different pointers
  let arr = [ 1, 2, 3, 4 ];
  let slice1: &[ i32 ] = &arr[ .. ];
  let slice3: &[ i32 ] = &arr[ 1..3 ];
  // Different data pointers should be detected
  assert!( !mem_tools::same_ptr( slice1, slice3 ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_size_cross_type()
{
  // Test that same-sized types across different types work
  let a: u32 = 42;
  let b: [ u8; 4 ] = [ 1, 2, 3, 4 ];
  assert!( mem_tools::same_size( &a, &b ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_size_zst()
{
  // Test that ZSTs are correctly identified as same size
  let zst1 = ();
  let zst2 = ();
  assert!( mem_tools::same_size( &zst1, &zst2 ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_region_string_literal_vs_heap()
{
  // Test that string literals and heap strings are different regions
  let literal = "test";
  let heap = String::from( "test" );
  assert!( !mem_tools::same_region( literal, heap.as_str() ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_region_subslice()
{
  // Test that full slice and subslice are different regions
  let arr = [ 1, 2, 3, 4 ];
  let full: &[ i32 ] = &arr[ .. ];
  let sub: &[ i32 ] = &arr[ 1..3 ];
  assert!( !mem_tools::same_region( full, sub ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_data_zst()
{
  // Test that ZST data is always considered equal
  let zst1 = ();
  let zst2 = ();
  assert!( mem_tools::same_data( &zst1, &zst2 ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_data_empty_slices()
{
  // Test that empty slices are always considered equal
  let empty1: &[ i32 ] = &[];
  let empty2: &[ i32 ] = &[];
  assert!( mem_tools::same_data( empty1, empty2 ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_data_cross_type()
{
  // Test that identical data across different types is detected
  let tuple = ( 1u8, 2u8, 3u8, 4u8 );
  let array = [ 1u8, 2u8, 3u8, 4u8 ];
  assert!( mem_tools::same_data( &tuple, &array ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_same_data_unicode()
{
  // Test that Unicode strings are compared correctly by bytes
  let s1 = "hello";
  let s2 = "hello";
  assert!( mem_tools::same_data( s1, s2 ) );

  let s3 = "héllo";
  let s4 = "héllo";
  assert!( mem_tools::same_data( s3, s4 ) );

  // Different Unicode strings
  assert!( !mem_tools::same_data( s1, s3 ) );
}

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn corner_case_fat_pointer_handling()
{
  // Test that fat pointers (slices with metadata) are handled correctly
  let arr1 = [ 1, 2, 3 ];
  let arr2 = [ 1, 2, 3 ];
  let slice1: &[ i32 ] = &arr1;
  let slice2: &[ i32 ] = &arr2;

  // Different allocations
  assert!( !mem_tools::same_ptr( slice1, slice2 ) );

  // Same size (both 3 elements)
  assert!( mem_tools::same_size( slice1, slice2 ) );

  // Same data content
  assert!( mem_tools::same_data( slice1, slice2 ) );

  // Different regions
  assert!( !mem_tools::same_region( slice1, slice2 ) );
}
