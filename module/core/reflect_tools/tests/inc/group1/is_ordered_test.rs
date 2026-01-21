//!
//! Tests for `is_ordered()` method correctness across different collection types.
//!
//! This test file reproduces and validates the fix for issue-manual-test-001:
//! `HashMap` and `HashSet` Entity implementations incorrectly returned `is_ordered() = true`
//! due to relying on trait default, when they should return `false` (hash collections
//! do not guarantee element order).
//!

use super :: *;
pub use the_module ::reflect;
use test_tools ::a_id;

//
// Bug Reproducer: issue-manual-test-001
//

/// Reproduces bug where `HashMap.is_ordered()` incorrectly returned true.
/// HashMap does not guarantee element order, so `is_ordered()` must return false.
// test_kind: bug_reproducer(issue-manual-test-001)
#[ test ]
fn hashmap_is_ordered_returns_false()
{
  use reflect :: { Entity, reflect };
  use std ::collections ::HashMap;

  // Empty HashMap
  let map : HashMap< String, i32 > = HashMap ::new();
  let reflected = reflect( &map );
  a_id!( reflected.is_ordered(), false, "Empty HashMap must report is_ordered() = false" );

  // HashMap with single entry
  let mut map = HashMap ::new();
  map.insert( "key".to_string(), 42 );
  let reflected = reflect( &map );
  a_id!( reflected.is_ordered(), false, "Single-entry HashMap must report is_ordered() = false" );

  // HashMap with multiple entries
  let mut map = HashMap ::new();
  map.insert( "alpha".to_string(), 1 );
  map.insert( "beta".to_string(), 2 );
  map.insert( "gamma".to_string(), 3 );
  let reflected = reflect( &map );
  a_id!( reflected.is_ordered(), false, "Multi-entry HashMap must report is_ordered() = false" );
}

/// Reproduces bug where `HashSet.is_ordered()` incorrectly returned true.
/// HashSet does not guarantee element order, so `is_ordered()` must return false.
// test_kind: bug_reproducer(issue-manual-test-001)
#[ test ]
fn hashset_is_ordered_returns_false()
{
  use reflect :: { Entity, reflect };
  use std ::collections ::HashSet;

  // Empty HashSet
  let set : HashSet< i32 > = HashSet ::new();
  let reflected = reflect( &set );
  a_id!( reflected.is_ordered(), false, "Empty HashSet must report is_ordered() = false" );

  // HashSet with single element
  let mut set = HashSet ::new();
  set.insert( 42 );
  let reflected = reflect( &set );
  a_id!( reflected.is_ordered(), false, "Single-element HashSet must report is_ordered() = false" );

  // HashSet with multiple elements
  let mut set = HashSet ::new();
  set.insert( 10 );
  set.insert( 20 );
  set.insert( 30 );
  let reflected = reflect( &set );
  a_id!( reflected.is_ordered(), false, "Multi-element HashSet must report is_ordered() = false" );
}

//
// Correctness Verification: Ordered Collections
//

/// Verifies Vec correctly reports `is_ordered() = true` (ordered collection).
#[ test ]
fn vec_is_ordered_returns_true()
{
  use reflect :: { Entity, reflect };

  // Empty Vec
  let vec : Vec< i32 > = Vec ::new();
  let reflected = reflect( &vec );
  a_id!( reflected.is_ordered(), true, "Empty Vec must report is_ordered() = true" );

  // Vec with elements
  let vec = vec![ 1, 2, 3, 4, 5 ];
  let reflected = reflect( &vec );
  a_id!( reflected.is_ordered(), true, "Vec with elements must report is_ordered() = true" );
}

/// Verifies array correctly reports `is_ordered() = true` (ordered collection).
#[ test ]
fn array_is_ordered_returns_true()
{
  use reflect :: { Entity, reflect };

  let array = [ 10, 20, 30 ];
  let reflected = reflect( &array );
  a_id!( reflected.is_ordered(), true, "Array must report is_ordered() = true" );
}

/// Verifies slice correctly reports `is_ordered() = true` (ordered collection).
#[ test ]
fn slice_is_ordered_returns_true()
{
  use reflect :: { Entity, reflect };

  // Use static slice to satisfy 'static lifetime requirement
  let slice : &'static [ i32 ] = &[ 100, 200, 300 ];
  let reflected = reflect( &slice );
  a_id!( reflected.is_ordered(), true, "Slice must report is_ordered() = true" );
}

//
// Semantic Validation
//

/// Validates `is_ordered()` correctly distinguishes ordered from unordered collections.
#[ test ]
fn is_ordered_semantic_correctness()
{
  use reflect :: { Entity, reflect };
  use std ::collections ::{ HashMap, HashSet };

  // Ordered collections (insertion/index order preserved)
  let vec = vec![ 1, 2, 3 ];
  assert!( reflect( &vec ).is_ordered(), "Vec guarantees order" );

  let array = [ 1, 2, 3 ];
  assert!( reflect( &array ).is_ordered(), "Array guarantees order" );

  // Unordered collections (hash-based, no order guarantee)
  let mut map = HashMap ::new();
  map.insert( "a", 1 );
  assert!( !reflect( &map ).is_ordered(), "HashMap does NOT guarantee order" );

  let mut set = HashSet ::new();
  set.insert( 42 );
  assert!( !reflect( &set ).is_ordered(), "HashSet does NOT guarantee order" );
}
