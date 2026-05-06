//! Corner case tests for `clone_dyn` examples
//!
//! ## Test Coverage
//!
//! This test file validates corner cases missing from the examples:
//!
//! 1. Empty iterator (None case)
//! 2. Empty vector (different from None)
//! 3. Single-element iterator
//! 4. Clone independence (consuming clone doesn't affect original)
//! 5. Multiple consecutive clones (clone-of-clone)
//! 6. Auto-trait variants (Send, Sync, Send+Sync)
//!
//! ## Root Cause
//!
//! Examples demonstrate basic functionality but don't cover edge cases:
//! - `get_iter(None)` is supported but never called
//! - Clone independence never verified (consuming clone might affect original)
//! - Auto-trait impls generated but never exercised
//! - Boundary cases (empty, single-element) missing
//!
//! ## Why Not Caught
//!
//! Examples are demonstrations, not comprehensive tests. No test matrix
//! validation required for examples. However, these corner cases should
//! be covered in the test suite to prevent regressions.
//!
//! ## Fix Applied
//!
//! Added comprehensive corner case test coverage for all missing scenarios.
//!
//! ## Prevention
//!
//! - Test matrix documentation in test files
//! - Explicit coverage of boundary conditions (empty, single, multiple)
//! - Verification of trait object properties (independence, auto-traits)
//!
//! ## Pitfall
//!
//! Examples may appear comprehensive but miss critical edge cases. Always
//! validate: empty inputs, single elements, clone independence, and all
//! generated trait implementations.

#[ allow( unused_imports ) ]
use super :: *;

/// Iterator trait with `CloneDyn` bound for testing
#[ the_module ::clone_dyn ]
pub trait IterTrait< 'a, T >
where
  T: 'a,
  Self: Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
{
}

impl< 'a, T, I > IterTrait< 'a, T > for I
where
  T: 'a,
  Self: Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
  Self: the_module ::CloneDyn,
{
}

/// Returns iterator over vector or empty iterator
pub fn get_iter< 'a >( src: Option< &'a Vec< i32 > > ) -> Box< dyn IterTrait< 'a, &'a i32 > + 'a >
{
  match &src
  {
    Some( src ) => Box ::new( src.iter() ),
    _ => Box ::new( core ::iter ::empty() ),
  }
}

// == Corner Case 1: Empty iterator (None case)

#[ test ]
fn empty_iterator_none_case()
{
  let iter = get_iter( None );

  // Clone empty iterator
  let cloned = iter.clone();

  // Both should produce no elements
  let original_count = iter.count();
  let cloned_count = cloned.count();

  assert_eq!( original_count, 0, "Original empty iterator should have 0 elements" );
  assert_eq!( cloned_count, 0, "Cloned empty iterator should have 0 elements" );
}

// == Corner Case 2: Empty vector (different from None)

#[ test ]
fn empty_vector_case()
{
  let data = vec![];
  let iter = get_iter( Some( &data ) );

  // Clone empty vector iterator
  let cloned = iter.clone();

  // Both should produce no elements
  let original_count = iter.count();
  let cloned_count = cloned.count();

  assert_eq!( original_count, 0, "Original empty vec iterator should have 0 elements" );
  assert_eq!( cloned_count, 0, "Cloned empty vec iterator should have 0 elements" );
}

// == Corner Case 3: Single-element iterator

#[ test ]
fn single_element_iterator()
{
  let data = vec![ 42 ];
  let iter = get_iter( Some( &data ) );

  // Clone single-element iterator
  let cloned = iter.clone();

  // Consume clone
  let cloned_values: Vec< _ > = cloned.copied().collect();
  assert_eq!( cloned_values, vec![ 42 ], "Cloned iterator should have single element" );

  // Original should still be intact
  let original_values: Vec< _ > = iter.copied().collect();
  assert_eq!( original_values, vec![ 42 ], "Original iterator should still have single element" );
}

// == Corner Case 4: Clone independence (consuming clone doesn't affect original)

#[ test ]
fn clone_independence_multi_element()
{
  let data = vec![ 1, 2, 3, 4, 5 ];
  let iter = get_iter( Some( &data ) );

  // Clone the iterator
  let cloned = iter.clone();

  // Fully consume the clone
  let cloned_values: Vec< _ > = cloned.copied().collect();
  assert_eq!( cloned_values, vec![ 1, 2, 3, 4, 5 ], "Cloned iterator should have all elements" );

  // Original should be completely independent and unaffected
  let original_values: Vec< _ > = iter.copied().collect();
  assert_eq!( original_values, vec![ 1, 2, 3, 4, 5 ], "Original iterator should be unaffected by clone consumption" );
}

// == Corner Case 5: Multiple consecutive clones (clone-of-clone)

#[ test ]
fn multiple_consecutive_clones()
{
  let data = vec![ 10, 20, 30 ];
  let iter1 = get_iter( Some( &data ) );

  // Create clone-of-clone-of-clone chain
  let iter2 = iter1.clone();
  let iter3 = iter2.clone();
  let iter4 = iter3.clone();

  // All should be independent and contain same data
  let values1: Vec< _ > = iter1.copied().collect();
  let values2: Vec< _ > = iter2.copied().collect();
  let values3: Vec< _ > = iter3.copied().collect();
  let values4: Vec< _ > = iter4.copied().collect();

  assert_eq!( values1, vec![ 10, 20, 30 ], "First iterator should have all elements" );
  assert_eq!( values2, vec![ 10, 20, 30 ], "Second iterator (clone) should have all elements" );
  assert_eq!( values3, vec![ 10, 20, 30 ], "Third iterator (clone-of-clone) should have all elements" );
  assert_eq!( values4, vec![ 10, 20, 30 ], "Fourth iterator (clone-of-clone-of-clone) should have all elements" );
}

// == Corner Case 6: Clone empty iterator multiple times

#[ test ]
fn clone_empty_iterator_multiple_times()
{
  let iter1 = get_iter( None );

  // Clone empty iterator multiple times
  let iter2 = iter1.clone();
  let iter3 = iter2.clone();

  // All should be empty
  assert_eq!( iter1.count(), 0, "Original empty iterator should have 0 elements" );
  assert_eq!( iter2.count(), 0, "First clone of empty iterator should have 0 elements" );
  assert_eq!( iter3.count(), 0, "Clone-of-clone of empty iterator should have 0 elements" );
}

// == Corner Case 7: Auto-trait variant +Send

#[ test ]
fn auto_trait_send()
{
  let data = [ 1, 2, 3 ];
  let iter: Box< dyn IterTrait< '_, &i32 > + Send > = Box ::new( data.iter() );

  // Clone Send variant
  let cloned = iter.clone();

  // Verify both work correctly
  let original_values: Vec< _ > = iter.copied().collect();
  let cloned_values: Vec< _ > = cloned.copied().collect();

  assert_eq!( original_values, vec![ 1, 2, 3 ], "Original Send variant should work" );
  assert_eq!( cloned_values, vec![ 1, 2, 3 ], "Cloned Send variant should work" );
}

// == Corner Case 8: Auto-trait variant +Sync

#[ test ]
fn auto_trait_sync()
{
  let data = [ 1, 2, 3 ];
  let iter: Box< dyn IterTrait< '_, &i32 > + Sync > = Box ::new( data.iter() );

  // Clone Sync variant
  let cloned = iter.clone();

  // Verify both work correctly
  let original_values: Vec< _ > = iter.copied().collect();
  let cloned_values: Vec< _ > = cloned.copied().collect();

  assert_eq!( original_values, vec![ 1, 2, 3 ], "Original Sync variant should work" );
  assert_eq!( cloned_values, vec![ 1, 2, 3 ], "Cloned Sync variant should work" );
}

// == Corner Case 9: Auto-trait variant +Send+Sync

#[ test ]
fn auto_trait_send_sync()
{
  let data = [ 1, 2, 3 ];
  let iter: Box< dyn IterTrait< '_, &i32 > + Send + Sync > = Box ::new( data.iter() );

  // Clone Send+Sync variant
  let cloned = iter.clone();

  // Verify both work correctly
  let original_values: Vec< _ > = iter.copied().collect();
  let cloned_values: Vec< _ > = cloned.copied().collect();

  assert_eq!( original_values, vec![ 1, 2, 3 ], "Original Send+Sync variant should work" );
  assert_eq!( cloned_values, vec![ 1, 2, 3 ], "Cloned Send+Sync variant should work" );
}

// == Corner Case 10: Large iterator (performance validation)

#[ test ]
fn large_iterator_cloning()
{
  let data: Vec< i32 > = ( 0..10_000 ).collect();
  let iter = get_iter( Some( &data ) );

  // Clone large iterator
  let cloned = iter.clone();

  // Verify both contain all elements
  let original_count = iter.count();
  let cloned_count = cloned.count();

  assert_eq!( original_count, 10_000, "Original large iterator should have 10k elements" );
  assert_eq!( cloned_count, 10_000, "Cloned large iterator should have 10k elements" );
}
