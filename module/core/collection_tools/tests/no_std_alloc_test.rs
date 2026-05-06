//! Tests for `invariant/001_no_std_alloc` — verifies `HashMap` and `HashSet` resolve to
//! hashbrown when `use_alloc` is active (IN-02, IN-03, IN-04).
//!
//! This file is entirely cfg-gated on `feature = "use_alloc"`. Under the default
//! feature set the file compiles to zero test functions and contributes nothing
//! to the binary. Run with `--features use_alloc` to exercise these cases.
//!
//! # Test Kind
//!
//! invariant

#![ cfg( feature = "use_alloc" ) ]

use collection_tools as the_module;

// Shorthand for the hashbrown crate re-exported via collection_tools::dependency.
use the_module ::dependency ::hashbrown;

/// IN-02: `use_alloc` config `HashMap` resolves to `hashbrown::HashMap`.
///
/// This test is a compile-time type-identity check. The direct assignment
/// `hashbrown::HashMap<u32, u32> = collection_tools::HashMap::new()` only
/// compiles when both sides are the same concrete type.
#[ test ]
fn hashmap_resolves_to_hashbrown()
{
  let map : hashbrown ::HashMap< u32, u32 > = the_module ::HashMap ::new();
  assert!( map.is_empty() );
}

/// IN-03: `use_alloc` config `HashSet` resolves to `hashbrown::HashSet`.
#[ test ]
fn hashset_resolves_to_hashbrown()
{
  let set : hashbrown ::HashSet< u32 > = the_module ::HashSet ::new();
  assert!( set.is_empty() );
}

/// IN-04: `use_alloc` config — other collection types come from alloc and work correctly.
#[ test ]
fn other_types_come_from_alloc()
{
  let vec : the_module ::Vec< u32 > = the_module ::Vec ::from( [ 1 ] );
  assert_eq!( vec.len(), 1 );

  let mut bmap : the_module ::BTreeMap< u32, u32 > = the_module ::BTreeMap ::new();
  bmap.insert( 1, 10 );
  assert_eq!( bmap.get( &1 ), Some( &10 ) );

  let mut bset : the_module ::BTreeSet< u32 > = the_module ::BTreeSet ::new();
  bset.insert( 42 );
  assert!( bset.contains( &42 ) );

  let mut llist : the_module ::LinkedList< u32 > = the_module ::LinkedList ::new();
  llist.push_back( 7 );
  assert_eq!( llist.front(), Some( &7 ) );

  let mut deque : the_module ::VecDeque< u32 > = the_module ::VecDeque ::new();
  deque.push_back( 3 );
  assert_eq!( deque.front(), Some( &3 ) );

  let mut heap : the_module ::BinaryHeap< u32 > = the_module ::BinaryHeap ::new();
  heap.push( 5 );
  assert_eq!( heap.peek(), Some( &5 ) );
}
