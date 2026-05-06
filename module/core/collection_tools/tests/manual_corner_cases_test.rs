//! Comprehensive corner case tests for collection macros
//!
//! Permanent regression tests verifying edge cases and boundary conditions:
//! - Empty collections
//! - Single element collections
//! - Trailing comma support
//! - Capacity pre-allocation
//! - Duplicate key behavior
//! - Large collections
//! - Type inference scenarios
//! - `BinaryHeap` max-heap ordering
//! - Into macros with heterogeneous types
//! - Complex nested types
//! - Non-Copy types (move semantics)
//!
//! # Test Kind
//!
//! Permanent corner case tests

#[ allow( unused_imports ) ]
use collection_tools as the_module;

// ============================================================================
// Empty Collection Tests
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_empty()
{
  let got: the_module::Vec< i32 > = the_module::vec![];
  assert_eq!( got.len(), 0 );
  assert!( got.is_empty() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_empty()
{
  let got: the_module::HashMap< i32, i32 > = the_module::hmap!{};
  assert_eq!( got.len(), 0 );
  assert!( got.is_empty() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hset_empty()
{
  let got: the_module::HashSet< i32 > = the_module::hset!{};
  assert_eq!( got.len(), 0 );
  assert!( got.is_empty() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn bmap_empty()
{
  let got: the_module::BTreeMap< i32, i32 > = the_module::bmap!{};
  assert_eq!( got.len(), 0 );
  assert!( got.is_empty() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn bset_empty()
{
  let got: the_module::BTreeSet< i32 > = the_module::bset!{};
  assert_eq!( got.len(), 0 );
  assert!( got.is_empty() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn llist_empty()
{
  let got: the_module::LinkedList< i32 > = the_module::llist!{};
  assert_eq!( got.len(), 0 );
  assert!( got.is_empty() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn deque_empty()
{
  let got: the_module::VecDeque< i32 > = the_module::deque!{};
  assert_eq!( got.len(), 0 );
  assert!( got.is_empty() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn heap_empty()
{
  let got: the_module::BinaryHeap< i32 > = the_module::heap!{};
  assert_eq!( got.len(), 0 );
  assert!( got.is_empty() );
}

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

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn bmap_trailing_comma()
{
  let got = the_module::bmap! { 1 => 10, 2 => 20, };
  let exp = the_module::bmap! { 1 => 10, 2 => 20 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn bset_trailing_comma()
{
  let got = the_module::bset! { 1, 2, 3, };
  let exp = the_module::bset! { 1, 2, 3 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn llist_trailing_comma()
{
  let got = the_module::llist! { 1, 2, 3, };
  let exp = the_module::llist! { 1, 2, 3 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn deque_trailing_comma()
{
  let got = the_module::deque! { 1, 2, 3, };
  let exp = the_module::deque! { 1, 2, 3 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn heap_trailing_comma()
{
  let got = the_module::heap! { 3, 1, 2, };
  let exp = the_module::heap! { 3, 1, 2 };
  assert_eq!( got.into_sorted_vec(), exp.into_sorted_vec() );
}

// ============================================================================
// Capacity Pre-allocation Tests
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_capacity_preallocated()
{
  let vec = the_module::vec![ 1, 2, 3, 4, 5 ];
  // Vec::with_capacity(N) gives exactly N — not just >= N.
  assert_eq!( vec.capacity(), 5, "Expected exact capacity 5, got {}", vec.capacity() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_capacity_preallocated()
{
  let map = the_module::hmap! { 1 => 10, 2 => 20, 3 => 30, 4 => 40, 5 => 50 };
  // HashMap::with_capacity(N) rounds up due to load factor — only >= N is guaranteed.
  assert!( map.capacity() >= 5, "Expected capacity >= 5, got {}", map.capacity() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hset_capacity_preallocated()
{
  let set = the_module::hset! { 1, 2, 3, 4, 5 };
  assert!( set.capacity() >= 5, "Expected capacity >= 5, got {}", set.capacity() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn deque_capacity_preallocated()
{
  let deque = the_module::deque! { 1, 2, 3, 4, 5 };
  // VecDeque::with_capacity(N) gives exactly N.
  assert_eq!( deque.capacity(), 5, "Expected exact capacity 5, got {}", deque.capacity() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn heap_capacity_preallocated()
{
  let heap = the_module::heap! { 1, 2, 3, 4, 5 };
  // BinaryHeap::with_capacity(N) gives exactly N.
  assert_eq!( heap.capacity(), 5, "Expected exact capacity 5, got {}", heap.capacity() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_empty_capacity_zero()
{
  let vec : the_module::Vec< i32 > = the_module::vec![];
  assert_eq!( vec.capacity(), 0, "Expected capacity 0 for empty vec, got {}", vec.capacity() );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_empty_capacity_zero()
{
  let map : the_module::HashMap< i32, i32 > = the_module::hmap! {};
  assert_eq!( map.capacity(), 0, "Expected capacity 0 for empty hmap, got {}", map.capacity() );
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

// ============================================================================
// BinaryHeap Max-Heap Ordering Tests
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn heap_max_heap_ordering()
{
  let mut got = the_module::heap!{ 3, 1, 4, 1, 5, 9, 2, 6 };

  // BinaryHeap is a max-heap, so pop should return largest first
  assert_eq!( got.pop(), Some( 9 ) );
  assert_eq!( got.pop(), Some( 6 ) );
  assert_eq!( got.pop(), Some( 5 ) );
  assert_eq!( got.pop(), Some( 4 ) );
  assert_eq!( got.pop(), Some( 3 ) );
  assert_eq!( got.pop(), Some( 2 ) );
  assert_eq!( got.pop(), Some( 1 ) );
  assert_eq!( got.pop(), Some( 1 ) );
  assert_eq!( got.pop(), None );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn heap_peek_returns_max()
{
  let got = the_module::heap!{ 5, 2, 8, 1 };
  assert_eq!( got.peek(), Some( &8 ) );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_heap_max_heap_ordering()
{
  let mut got: the_module::BinaryHeap< i32 > = the_module::into_heap!{ 3, 1, 4 };
  assert_eq!( got.pop(), Some( 4 ) );
  assert_eq!( got.pop(), Some( 3 ) );
  assert_eq!( got.pop(), Some( 1 ) );
}

// ============================================================================
// Into Macros with Heterogeneous Types
// ============================================================================

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_vec_heterogeneous_strings()
{
  // Mix &str and String via Into<String>
  let got: the_module::Vec< String > = the_module::into_vec![ "static", String::from( "owned" ), "another" ];
  assert_eq!( got.len(), 3 );
  assert_eq!( got[ 0 ], "static" );
  assert_eq!( got[ 1 ], "owned" );
  assert_eq!( got[ 2 ], "another" );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_hmap_heterogeneous_keys_and_values()
{
  // Mix &str and String for both keys and values
  let got: the_module::HashMap< String, String > = the_module::into_hmap! {
    "key1" => "val1",
    String::from( "key2" ) => String::from( "val2" ),
    "key3" => String::from( "val3" )
  };
  assert_eq!( got.len(), 3 );
  assert_eq!( got.get( "key1" ), Some( &String::from( "val1" ) ) );
  assert_eq!( got.get( "key2" ), Some( &String::from( "val2" ) ) );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_hset_heterogeneous_strings()
{
  let got: the_module::HashSet< String > = the_module::into_hset! {
    "static1",
    String::from( "owned" ),
    "static2"
  };
  assert_eq!( got.len(), 3 );
  assert!( got.contains( "static1" ) );
  assert!( got.contains( "owned" ) );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_bmap_heterogeneous()
{
  let got: the_module::BTreeMap< String, i64 > = the_module::into_bmap! {
    "key1" => 10,
    String::from( "key2" ) => 20i64
  };
  assert_eq!( got.len(), 2 );
  assert_eq!( got.get( "key1" ), Some( &10i64 ) );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_bset_heterogeneous()
{
  let got: the_module::BTreeSet< String > = the_module::into_bset! {
    "one",
    String::from( "two" )
  };
  assert_eq!( got.len(), 2 );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_llist_heterogeneous()
{
  let got: the_module::LinkedList< String > = the_module::into_llist! {
    "first",
    String::from( "second" )
  };
  assert_eq!( got.len(), 2 );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_vecd_heterogeneous()
{
  let got: the_module::VecDeque< String > = the_module::into_vecd! {
    "start",
    String::from( "middle" ),
    "end"
  };
  assert_eq!( got.len(), 3 );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_heap_heterogeneous()
{
  let got: the_module::BinaryHeap< String > = the_module::into_heap! {
    "alpha",
    String::from( "beta" )
  };
  assert_eq!( got.len(), 2 );
}

// ============================================================================
// Complex Nested Types
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_of_vecs()
{
  let got = the_module::vec![
    the_module::vec![ 1, 2 ],
    the_module::vec![ 3, 4, 5 ],
    the_module::vec![ 6 ]
  ];
  assert_eq!( got.len(), 3 );
  assert_eq!( got[ 0 ].len(), 2 );
  assert_eq!( got[ 1 ].len(), 3 );
  assert_eq!( got[ 2 ].len(), 1 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_with_vec_values()
{
  let got = the_module::hmap! {
    "nums" => the_module::vec![ 1, 2, 3 ],
    "more" => the_module::vec![ 4, 5 ]
  };
  assert_eq!( got.len(), 2 );
  assert_eq!( got.get( "nums" ).unwrap().len(), 3 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_of_tuples()
{
  let got = the_module::vec![ ( 1, "one" ), ( 2, "two" ), ( 3, "three" ) ];
  assert_eq!( got.len(), 3 );
  assert_eq!( got[ 0 ], ( 1, "one" ) );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_with_tuple_keys()
{
  let got = the_module::hmap! {
    ( 1, 2 ) => "pair1",
    ( 3, 4 ) => "pair2"
  };
  assert_eq!( got.len(), 2 );
  assert_eq!( got.get( &( 1, 2 ) ), Some( &"pair1" ) );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn bmap_with_hashmap_values()
{
  let got = the_module::bmap! {
    1 => the_module::hmap! { "a" => 10 },
    2 => the_module::hmap! { "b" => 20, "c" => 30 }
  };
  assert_eq!( got.len(), 2 );
  assert_eq!( got.get( &1 ).unwrap().len(), 1 );
  assert_eq!( got.get( &2 ).unwrap().len(), 2 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_of_options()
{
  let got = the_module::vec![ Some( 1 ), None, Some( 3 ) ];
  assert_eq!( got.len(), 3 );
  assert_eq!( got[ 0 ], Some( 1 ) );
  assert_eq!( got[ 1 ], None );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_of_results()
{
  let got = the_module::vec![ Ok( 1 ), Err( "error" ), Ok( 2 ) ];
  assert_eq!( got.len(), 3 );
  assert_eq!( got[ 0 ], Ok( 1 ) );
  assert_eq!( got[ 1 ], Err( "error" ) );
}

// ============================================================================
// Non-Copy Types (Move Semantics)
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_with_strings_moved()
{
  let s1 = String::from( "first" );
  let s2 = String::from( "second" );
  let got = the_module::vec![ s1, s2 ];
  assert_eq!( got.len(), 2 );
  assert_eq!( got[ 0 ], "first" );
  // s1 and s2 are moved, cannot use them here
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_with_string_keys_and_values_moved()
{
  let k1 = String::from( "key1" );
  let v1 = String::from( "val1" );
  let got = the_module::hmap! { k1 => v1 };
  assert_eq!( got.len(), 1 );
  assert_eq!( got.get( "key1" ), Some( &String::from( "val1" ) ) );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_with_boxed_values()
{
  let got = the_module::vec![ Box::new( 1 ), Box::new( 2 ), Box::new( 3 ) ];
  assert_eq!( got.len(), 3 );
  assert_eq!( *got[ 0 ], 1 );
  assert_eq!( *got[ 1 ], 2 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hset_with_strings_moved()
{
  let s1 = String::from( "alpha" );
  let s2 = String::from( "beta" );
  let got = the_module::hset! { s1, s2 };
  assert_eq!( got.len(), 2 );
  assert!( got.contains( "alpha" ) );
  assert!( got.contains( "beta" ) );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn llist_with_vecs_moved()
{
  let v1 = the_module::vec![ 1, 2 ];
  let v2 = the_module::vec![ 3, 4 ];
  let got = the_module::llist! { v1, v2 };
  assert_eq!( got.len(), 2 );
}

// Test with custom non-Copy struct
#[ derive( Debug, PartialEq ) ]
struct NonCopyStruct
{
  data: String,
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn vec_with_custom_non_copy_struct()
{
  let obj1 = NonCopyStruct { data: String::from( "obj1" ) };
  let obj2 = NonCopyStruct { data: String::from( "obj2" ) };
  let got = the_module::vec![ obj1, obj2 ];
  assert_eq!( got.len(), 2 );
  assert_eq!( got[ 0 ].data, "obj1" );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn hmap_with_custom_non_copy_values()
{
  let obj1 = NonCopyStruct { data: String::from( "value1" ) };
  let obj2 = NonCopyStruct { data: String::from( "value2" ) };
  let got = the_module::hmap! {
    1 => obj1,
    2 => obj2
  };
  assert_eq!( got.len(), 2 );
  assert_eq!( got.get( &1 ).unwrap().data, "value1" );
}

// ============================================================================
// Into Constructor Capacity Tests
// ============================================================================

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_vec_capacity_preallocated()
{
  let got : the_module::Vec< i32 > = the_module::into_vec![ 1, 2, 3, 4, 5 ];
  // into_vec! calls Vec::with_capacity(N) same as vec! — exact capacity expected.
  assert_eq!( got.capacity(), 5, "Expected exact capacity 5, got {}", got.capacity() );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_hmap_capacity_preallocated()
{
  let got : the_module::HashMap< i32, i32 > = the_module::into_hmap! { 1 => 10, 2 => 20, 3 => 30 };
  assert!( got.capacity() >= 3, "Expected capacity >= 3, got {}", got.capacity() );
}

// ============================================================================
// Alias Equality Tests — dlist! = vec!, into_dlist! = into_vec!
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn dlist_alias_equals_vec()
{
  // dlist! is a pub use alias for vec!; both must produce identical Vec values.
  let vec_result   = the_module ::vec![ 1, 2, 3 ];
  let dlist_result = the_module ::dlist![ 1, 2, 3 ];
  assert_eq!( vec_result, dlist_result );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_dlist_alias_equals_into_vec()
{
  // into_dlist! is a pub use alias for into_vec!; results must be equal.
  let a : the_module ::Vec< String > = the_module ::into_vec![ "a", "b" ];
  let b : the_module ::Vec< String > = the_module ::into_dlist![ "a", "b" ];
  assert_eq!( a, b );
}

// ============================================================================
// dlist! Specific Tests
// ============================================================================

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn dlist_empty()
{
  let got : the_module ::Vec< i32 > = the_module ::dlist![];
  assert!( got.is_empty() );
  assert_eq!( got.len(), 0 );
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn dlist_trailing_comma()
{
  let got = the_module ::dlist![ 1, 2, 3, ];
  let exp = the_module ::dlist![ 1, 2, 3 ];
  assert_eq!( got, exp );
}

// ============================================================================
// Into-macro Empty Construction — all 9 into-macros
// ============================================================================

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_macros_empty_all_nine()
{
  // Each macro tested in its own block to avoid similar-names lint across different collection types.
  { let col : the_module ::Vec< i32 >           = the_module ::into_vec![]; assert!( col.is_empty() ); }
  { let col : the_module ::Vec< i32 >           = the_module ::into_dlist![]; assert!( col.is_empty() ); }
  { let col : the_module ::HashMap< i32, i32 >  = the_module ::into_hmap!{}; assert!( col.is_empty() ); }
  { let col : the_module ::HashSet< i32 >       = the_module ::into_hset!{}; assert!( col.is_empty() ); }
  { let col : the_module ::BTreeMap< i32, i32 > = the_module ::into_bmap!{}; assert!( col.is_empty() ); }
  { let col : the_module ::BTreeSet< i32 >      = the_module ::into_bset!{}; assert!( col.is_empty() ); }
  { let col : the_module ::LinkedList< i32 >    = the_module ::into_llist!{}; assert!( col.is_empty() ); }
  { let col : the_module ::VecDeque< i32 >      = the_module ::into_vecd!{}; assert!( col.is_empty() ); }
  { let col : the_module ::BinaryHeap< i32 >    = the_module ::into_heap!{}; assert!( col.is_empty() ); }
}

// ============================================================================
// Into-macro Trailing Comma — all 9 into-macros
// ============================================================================

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_vec_trailing_comma()
{
  let got : the_module ::Vec< i32 > = the_module ::into_vec![ 1, 2, 3, ];
  let exp : the_module ::Vec< i32 > = the_module ::into_vec![ 1, 2, 3 ];
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_dlist_trailing_comma()
{
  let got : the_module ::Vec< i32 > = the_module ::into_dlist![ 1, 2, 3, ];
  let exp : the_module ::Vec< i32 > = the_module ::into_dlist![ 1, 2, 3 ];
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_hmap_trailing_comma()
{
  let got : the_module ::HashMap< i32, i32 > = the_module ::into_hmap!{ 1 => 10, 2 => 20, };
  let exp : the_module ::HashMap< i32, i32 > = the_module ::into_hmap!{ 1 => 10, 2 => 20 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_hset_trailing_comma()
{
  let got : the_module ::HashSet< i32 > = the_module ::into_hset!{ 1, 2, 3, };
  let exp : the_module ::HashSet< i32 > = the_module ::into_hset!{ 1, 2, 3 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_bmap_trailing_comma()
{
  let got : the_module ::BTreeMap< i32, i32 > = the_module ::into_bmap!{ 1 => 10, 2 => 20, };
  let exp : the_module ::BTreeMap< i32, i32 > = the_module ::into_bmap!{ 1 => 10, 2 => 20 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_bset_trailing_comma()
{
  let got : the_module ::BTreeSet< i32 > = the_module ::into_bset!{ 1, 2, 3, };
  let exp : the_module ::BTreeSet< i32 > = the_module ::into_bset!{ 1, 2, 3 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_llist_trailing_comma()
{
  let got : the_module ::LinkedList< i32 > = the_module ::into_llist!{ 1, 2, 3, };
  let exp : the_module ::LinkedList< i32 > = the_module ::into_llist!{ 1, 2, 3 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_vecd_trailing_comma()
{
  let got : the_module ::VecDeque< i32 > = the_module ::into_vecd!{ 1, 2, 3, };
  let exp : the_module ::VecDeque< i32 > = the_module ::into_vecd!{ 1, 2, 3 };
  assert_eq!( got, exp );
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_heap_trailing_comma()
{
  let got : the_module ::BinaryHeap< i32 > = the_module ::into_heap!{ 3, 1, 2, };
  let exp : the_module ::BinaryHeap< i32 > = the_module ::into_heap!{ 3, 1, 2 };
  assert_eq!( got.into_sorted_vec(), exp.into_sorted_vec() );
}

// ============================================================================
// Type Identity — IN-01: std config HashMap = std::collections::HashMap
// ============================================================================

// Under default features (std mode, no use_alloc), collection_tools::HashMap
// is a re-export of std::collections::HashMap. The direct assignment below
// compiles only when both sides name the same concrete type.
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn std_hashmap_is_std_collections()
{
  let map : std::collections::HashMap< u32, u32 > = the_module ::HashMap ::new();
  assert!( map.is_empty() );
}

// ============================================================================
// Full HashMap API Surface — AP-08
// ============================================================================

#[ test ]
fn hashmap_full_api_surface()
{
  let mut m = the_module ::HashMap ::new();
  m.insert( 1u32, "one" );
  m.insert( 2u32, "two" );
  m.insert( 3u32, "three" );
  // len and is_empty
  assert_eq!( m.len(), 3 );
  assert!( !m.is_empty() );
  // contains_key
  assert!( m.contains_key( &1u32 ) );
  assert!( !m.contains_key( &99u32 ) );
  // get
  assert_eq!( m.get( &1u32 ), Some( &"one" ) );
  // insert — returns previous value for duplicate key
  let prev = m.insert( 1u32, "ONE" );
  assert_eq!( prev, Some( "one" ) );
  assert_eq!( m.get( &1u32 ), Some( &"ONE" ) );
  // remove
  let removed = m.remove( &2u32 );
  assert_eq!( removed, Some( "two" ) );
  assert_eq!( m.len(), 2 );
  // iter — count key-value pairs remaining
  let pair_count = m.iter().count();
  assert_eq!( pair_count, 2 );
}
