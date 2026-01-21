//! Edge case tests for collection constructor macros
//!
//! These tests verify that re-exported collection constructors work correctly
//! in various edge cases.

use test_tools::*;

#[cfg(test)]
mod collection_constructor_edge_cases
{
  use super::*;

  /// Test empty heap! constructor
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_empty_heap()
  {
    let h = heap!();
    assert_eq!(h.len(), 0);
    assert!(h.is_empty());
  }

  /// Test single element heap!
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_single_element_heap()
  {
    let h = heap!(42);
    assert_eq!(h.len(), 1);
    assert_eq!(h.peek(), Some(&42));
  }

  /// Test multiple element heap!
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_multiple_element_heap()
  {
    let h = heap!(1, 2, 3, 4, 5);
    assert_eq!(h.len(), 5);
    // Heap is max-heap by default
    assert_eq!(h.peek(), Some(&5));
  }

  /// Test large heap!
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_large_heap()
  {
    let h = heap!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20);
    assert_eq!(h.len(), 20);
    assert_eq!(h.peek(), Some(&20));
  }

  /// Test heap! with String elements
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_heap_with_strings()
  {
    let h = heap!(
      "apple".to_string(),
      "banana".to_string(),
      "cherry".to_string()
    );
    assert_eq!(h.len(), 3);
  }

  /// Test hmap! (hash map) empty
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_empty_hmap()
  {
    let m = hmap!();
    assert_eq!(m.len(), 0);
    assert!(m.is_empty());
  }

  /// Test hmap! with single entry
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_single_entry_hmap()
  {
    let m = hmap!( 1 => "one" );
    assert_eq!(m.len(), 1);
    assert_eq!(m.get(&1), Some(&"one"));
  }

  /// Test hmap! with multiple entries
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_multiple_entry_hmap()
  {
    let m = hmap!
    (
      1 => "one",
      2 => "two",
      3 => "three"
    );
    assert_eq!(m.len(), 3);
    assert_eq!(m.get(&1), Some(&"one"));
    assert_eq!(m.get(&2), Some(&"two"));
    assert_eq!(m.get(&3), Some(&"three"));
  }

  /// Test hset! (hash set) empty
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_empty_hset()
  {
    let s = hset!();
    assert_eq!(s.len(), 0);
    assert!(s.is_empty());
  }

  /// Test hset! with elements
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_hset_with_elements()
  {
    let s = hset!(1, 2, 3, 2, 1); // Duplicates should be removed
    assert_eq!(s.len(), 3); // Only unique elements
    assert!(s.contains(&1));
    assert!(s.contains(&2));
    assert!(s.contains(&3));
  }

  /// Test dlist! (doubly-linked list) empty
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_empty_dlist()
  {
    let l = dlist!();
    assert_eq!(l.len(), 0);
    assert!(l.is_empty());
  }

  /// Test dlist! with elements
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_dlist_with_elements()
  {
    let l = dlist!(1, 2, 3, 4, 5);
    assert_eq!(l.len(), 5);
    assert_eq!(l.front(), Some(&1));
    assert_eq!(l.back(), Some(&5));
  }

  /// Test bmap! (BTreeMap) empty
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_empty_bmap()
  {
    let m = bmap!();
    assert_eq!(m.len(), 0);
    assert!(m.is_empty());
  }

  /// Test bmap! with entries
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_bmap_with_entries()
  {
    let m = bmap!
    (
      1 => "one",
      2 => "two",
      3 => "three"
    );
    assert_eq!(m.len(), 3);
    // BTreeMap maintains sorted order
    let keys: Vec<_> = m.keys().copied().collect();
    assert_eq!(keys, vec![1, 2, 3]);
  }

  /// Test bset! (BTreeSet) empty
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_empty_bset()
  {
    let s = bset!();
    assert_eq!(s.len(), 0);
    assert!(s.is_empty());
  }

  /// Test bset! with elements
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_bset_with_elements()
  {
    let s = bset!(3, 1, 4, 1, 5); // Duplicates removed, maintains order
    assert_eq!(s.len(), 4); // Only unique elements
    // BTreeSet maintains sorted order
    let values: Vec<_> = s.iter().copied().collect();
    assert_eq!(values, vec![1, 3, 4, 5]);
  }

  /// Test nested collections
  #[test]
  #[cfg(feature = "collection_constructors")]
  fn test_nested_collections()
  {
    let nested = hmap!
    (
      "numbers" => vec![1, 2, 3],
      "letters" => vec![4, 5, 6]
    );
    assert_eq!(nested.len(), 2);
    assert_eq!(nested.get("numbers"), Some(&vec![1, 2, 3]));
  }

  /// Test vec! macro (verify no ambiguity with std::vec!)
  #[test]
  fn test_vec_macro_basic()
  {
    // Should use std::vec! by default
    let v = vec![1, 2, 3];
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 2);
    assert_eq!(v[2], 3);
  }

  /// Test vec! macro disambiguation - test_tools provides vec! that works like std::vec!
  #[test]
  fn test_vec_macro_disambiguation()
  {
    // test_tools re-exports vec! that is compatible with std::vec!
    let v1 = vec![1, 2, 3];
    let v2 = std::vec![4, 5, 6];

    assert_eq!(v1.len(), 3);
    assert_eq!(v2.len(), 3);

    // Both should be std::vec::Vec
    assert_eq!(std::any::type_name_of_val(&v1), std::any::type_name_of_val(&v2));
  }
}
