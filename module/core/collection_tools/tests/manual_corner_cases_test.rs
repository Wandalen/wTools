//! Comprehensive corner case tests for collection macros
//!
//! Permanent regression tests verifying edge cases and boundary conditions:
//! - Single element collections
//! - Trailing comma support
//! - Capacity pre-allocation
//! - Duplicate key behavior
//! - Large collections
//! - Type inference scenarios
//!
//! # Test Kind
//!
//! Permanent corner case tests

#[ allow( unused_imports ) ]
use collection_tools as the_module;

// ============================================================================
// Single Element Tests
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_single_element()
{
  let got = the_module::vec![ 42 ];
  let exp = vec![ 42 ];
  assert_eq!( got, exp );
  assert_eq!( got.len(), 1 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_single_element()
{
  let got = the_module::hmap! { "key" => "value" };
  let mut exp = the_module::HashMap::new();
  exp.insert( "key", "value" );
  assert_eq!( got, exp );
  assert_eq!( got.len(), 1 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hset_single_element()
{
  let got = the_module::hset! { 42 };
  let mut exp = the_module::HashSet::new();
  exp.insert( 42 );
  assert_eq!( got, exp );
  assert_eq!( got.len(), 1 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn bmap_single_element()
{
  let got = the_module::bmap! { 1 => 100 };
  let mut exp = the_module::BTreeMap::new();
  exp.insert( 1, 100 );
  assert_eq!( got, exp );
  assert_eq!( got.len(), 1 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn bset_single_element()
{
  let got = the_module::bset! { 42 };
  let mut exp = the_module::BTreeSet::new();
  exp.insert( 42 );
  assert_eq!( got, exp );
  assert_eq!( got.len(), 1 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn llist_single_element()
{
  let got = the_module::llist! { 42 };
  let mut exp = the_module::LinkedList::new();
  exp.push_back( 42 );
  assert_eq!( got, exp );
  assert_eq!( got.len(), 1 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn deque_single_element()
{
  let got = the_module::deque! { 42 };
  let mut exp = the_module::VecDeque::new();
  exp.push_back( 42 );
  assert_eq!( got, exp );
  assert_eq!( got.len(), 1 );
}

// ============================================================================
// Trailing Comma Tests
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_trailing_comma()
{
  let got = the_module::vec![ 1, 2, 3, ];
  let exp = the_module::vec![ 1, 2, 3 ];
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_trailing_comma()
{
  let got = the_module::hmap! { 1 => 10, 2 => 20, };
  let exp = the_module::hmap! { 1 => 10, 2 => 20 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hset_trailing_comma()
{
  let got = the_module::hset! { 1, 2, 3, };
  let exp = the_module::hset! { 1, 2, 3 };
  assert_eq!( got, exp );
}

// ============================================================================
// Capacity Pre-allocation Tests
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_capacity_preallocated()
{
  let vec = the_module::vec![ 1, 2, 3, 4, 5 ];
  // Capacity should be at least the number of elements (might be more due to allocator)
  assert!( vec.capacity() >= 5, "Expected capacity >= 5, got {}", vec.capacity() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_capacity_preallocated()
{
  let map = the_module::hmap! { 1 => 10, 2 => 20, 3 => 30, 4 => 40, 5 => 50 };
  // HashMap capacity is at least the number of elements
  assert!( map.capacity() >= 5, "Expected capacity >= 5, got {}", map.capacity() );
}

// ============================================================================
// Duplicate Key Behavior Tests (HashMap should keep last value)
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_duplicate_keys_keeps_last()
{
  let got = the_module::hmap! { "key" => "first", "key" => "last" };
  assert_eq!( got.get( "key" ), Some( &"last" ) );
  assert_eq!( got.len(), 1 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn bmap_duplicate_keys_keeps_last()
{
  let got = the_module::bmap! { 1 => 100, 1 => 200 };
  assert_eq!( got.get( &1 ), Some( &200 ) );
  assert_eq!( got.len(), 1 );
}

// ============================================================================
// Large Collection Tests
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_large_collection()
{
  let got = the_module::vec![
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20
  ];
  assert_eq!( got.len(), 20 );
  assert_eq!( got[ 0 ], 1 );
  assert_eq!( got[ 19 ], 20 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_large_collection()
{
  let got = the_module::hmap! {
    1 => 10, 2 => 20, 3 => 30, 4 => 40, 5 => 50,
    6 => 60, 7 => 70, 8 => 80, 9 => 90, 10 => 100
  };
  assert_eq!( got.len(), 10 );
  assert_eq!( got.get( &1 ), Some( &10 ) );
  assert_eq!( got.get( &10 ), Some( &100 ) );
}

// ============================================================================
// Type Inference Tests
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_type_inference_works()
{
  // No type annotation needed - inferred from usage
  let vec = the_module::vec![ 1, 2, 3 ];
  let sum: i32 = vec.iter().sum();
  assert_eq!( sum, 6 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_type_inference_works()
{
  // No type annotation needed - inferred from usage
  let map = the_module::hmap! { "a" => 1, "b" => 2 };
  let value: i32 = *map.get( "a" ).unwrap();
  assert_eq!( value, 1 );
}

// ============================================================================
// Into Constructor Tests (require type annotations)
// ============================================================================

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_vec_with_type_annotation()
{
  let got: the_module::Vec< i32 > = the_module::into_vec![ 1, 2, 3 ];
  assert_eq!( got.len(), 3 );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_hmap_with_type_annotation()
{
  let got: the_module::HashMap< i32, i32 > = the_module::into_hmap! { 1 => 10, 2 => 20 };
  assert_eq!( got.len(), 2 );
}
