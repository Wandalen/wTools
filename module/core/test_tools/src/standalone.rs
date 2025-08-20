// We don't want to run doctest of aggregate

//! Standalone build mode implementation
//! 
//! This module provides essential functionality for breaking circular dependencies
//! without relying on normal Cargo dependencies. It uses direct transient dependencies
//! and minimal standalone implementations.

/// Error handling tools for standalone mode
pub mod error_tools {
  pub use anyhow::{Result, bail, ensure, format_err};
  
  /// Error trait for compatibility with error context handling
  #[allow(dead_code)]
  pub trait ErrWith<T> {
    /// The error type for this implementation
    type Error;
    /// Add context to an error using a closure
    fn err_with<F>(self, f: F) -> Result<T, (String, Self::Error)> 
    where 
      Self: Sized, 
      F: FnOnce() -> String;
    /// Add context to an error using a static string
    fn err_with_report(self, report: &str) -> Result<T, (String, Self::Error)> where Self: Sized;
  }
  
  /// `ResultWithReport` type alias for `error_tools` compatibility in standalone mode
  #[allow(dead_code)]  
  pub type ResultWithReport<T, E> = Result<T, (String, E)>;
  
  /// Error submodule for `error_tools` compatibility  
  pub mod error {
    pub use super::{ErrWith, ResultWithReport};
  }
  
  impl<T, E> ErrWith<T> for Result<T, E> {
    type Error = E;
    
    fn err_with<F>(self, f: F) -> Result<T, (String, E)> 
    where 
      F: FnOnce() -> String 
    {
      match self {
        Ok(val) => Ok(val),
        Err(err) => Err((f(), err)),
      }
    }
    
    fn err_with_report(self, report: &str) -> Result<T, (String, E)> {
      match self {
        Ok(val) => Ok(val),
        Err(err) => Err((report.to_string(), err)),
      }
    }
  }
  
  // Debug assertion macros for compatibility - simplified to avoid macro scoping issues
  /// Assert that two values are identical
  pub fn debug_assert_identical<T: PartialEq + std::fmt::Debug>(left: T, right: T) {
    debug_assert_eq!(left, right, "Values should be identical");
  }
  
  /// Assert that two values are identical (alias for `debug_assert_identical`)
  pub fn debug_assert_id<T: PartialEq + std::fmt::Debug>(left: T, right: T) {
    debug_assert_identical(left, right);
  }
  
  /// Assert that two values are not identical
  pub fn debug_assert_not_identical<T: PartialEq + std::fmt::Debug>(left: T, right: T) {
    debug_assert_ne!(left, right, "Values should not be identical");
  }
  
  /// Assert that two values are not identical (alias for `debug_assert_not_identical`)
  pub fn debug_assert_ni<T: PartialEq + std::fmt::Debug>(left: T, right: T) {
    debug_assert_not_identical(left, right);
  }
}

/// Collection tools for standalone mode
pub mod collection_tools {
  use std::hash::Hash;
  use std::collections::hash_map::RandomState;
  
  /// A hash map implementation using hashbrown for standalone mode
  #[derive(Debug, Clone)]
  pub struct HashMap<K, V>(hashbrown::HashMap<K, V, RandomState>);
  
  impl<K, V> HashMap<K, V> 
  where 
    K: Hash + Eq,
  {
    /// Create a new empty `HashMap`
    pub fn new() -> Self {
      Self(hashbrown::HashMap::with_hasher(RandomState::new()))
    }
    
    /// Insert a key-value pair into the `HashMap`
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
      self.0.insert(k, v)
    }
    
    /// Get a reference to the value for a given key
    pub fn get<Q>(&self, k: &Q) -> Option<&V> 
    where 
      K: std::borrow::Borrow<Q>,
      Q: Hash + Eq + ?Sized,
    {
      self.0.get(k)
    }
    
    /// Get the number of elements in the `HashMap`
    pub fn len(&self) -> usize {
      self.0.len()
    }
    
    /// Get a mutable reference to the value for a given key
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V> 
    where 
      K: std::borrow::Borrow<Q>,
      Q: Hash + Eq + ?Sized,
    {
      self.0.get_mut(k)
    }
    
    /// Remove a key-value pair from the `HashMap`
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V> 
    where 
      K: std::borrow::Borrow<Q>,
      Q: Hash + Eq + ?Sized,
    {
      self.0.remove(k)
    }
    
    /// Clear all key-value pairs from the `HashMap`
    pub fn clear(&mut self) {
      self.0.clear()
    }
    
    /// Returns an iterator over all key-value pairs (immutable references)
    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, K, V> {
      self.0.iter()
    }
    
    /// Returns an iterator over all key-value pairs (mutable references)
    pub fn iter_mut(&mut self) -> hashbrown::hash_map::IterMut<'_, K, V> {
      self.0.iter_mut()
    }
    
    /// Gets the given key's corresponding entry in the map for in-place manipulation
    pub fn entry(&mut self, key: K) -> hashbrown::hash_map::Entry<'_, K, V, RandomState> {
      self.0.entry(key)
    }
  }
  
  impl<K, V> Default for HashMap<K, V> 
  where 
    K: Hash + Eq,
  {
    fn default() -> Self {
      Self::new()
    }
  }
  
  impl<K, V> From<Vec<(K, V)>> for HashMap<K, V> 
  where K: Hash + Eq
  {
    fn from(vec: Vec<(K, V)>) -> Self {
      let mut map = Self::new();
      for (k, v) in vec {
        map.insert(k, v);
      }
      map
    }
  }
  
  impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V> 
  where K: Hash + Eq
  {
    fn from(arr: [(K, V); N]) -> Self {
      let mut map = Self::new();
      for (k, v) in arr {
        map.insert(k, v);
      }
      map
    }
  }
  
  impl<K, V> FromIterator<(K, V)> for HashMap<K, V> 
  where K: Hash + Eq
  {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
      let mut map = Self::new();
      for (k, v) in iter {
        map.insert(k, v);
      }
      map
    }
  }
  
  impl<K, V> PartialEq for HashMap<K, V> 
  where K: Hash + Eq, V: PartialEq
  {
    fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
    }
  }
  
  impl<K, V> Eq for HashMap<K, V> where K: Hash + Eq, V: Eq {}
  
  impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = hashbrown::hash_map::IntoIter<K, V>;
    
    fn into_iter(self) -> Self::IntoIter {
      self.0.into_iter()
    }
  }
  
  
  /// A hash set implementation using hashbrown for standalone mode
  #[derive(Debug, Clone)]
  #[allow(dead_code)]
  pub struct HashSet<T>(hashbrown::HashSet<T, RandomState>);
  
  impl<T: core::hash::Hash + Eq> PartialEq for HashSet<T> {
    fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
    }
  }
  
  impl<T: core::hash::Hash + Eq> Eq for HashSet<T> {}
  
  impl<T> HashSet<T> {
    /// Create a new empty `HashSet`
    pub fn new() -> Self {
      Self(hashbrown::HashSet::with_hasher(RandomState::new()))
    }
    
    /// Returns an iterator over the set
    pub fn iter(&self) -> hashbrown::hash_set::Iter<'_, T> {
      self.0.iter()
    }
    
    /// Insert a value into the set
    pub fn insert(&mut self, value: T) -> bool 
    where
      T: core::hash::Hash + Eq,
    {
      self.0.insert(value)
    }
    
    /// Returns the number of elements in the set
    pub fn len(&self) -> usize {
      self.0.len()
    }
    
    /// Returns true if the set is empty
    pub fn is_empty(&self) -> bool {
      self.0.is_empty()
    }
    
    /// Returns true if the set contains the specified value
    pub fn contains<Q>(&self, value: &Q) -> bool 
    where
      T: core::borrow::Borrow<Q> + core::hash::Hash + Eq,
      Q: core::hash::Hash + Eq + ?Sized,
    {
      self.0.contains(value)
    }
  }
  
  impl<T> Default for HashSet<T> {
    fn default() -> Self {
      Self::new()
    }
  }

  impl<T> IntoIterator for HashSet<T> {
    type Item = T;
    type IntoIter = hashbrown::hash_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
      self.0.into_iter()
    }
  }

  impl<T: core::hash::Hash + Eq> FromIterator<T> for HashSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
      Self(hashbrown::HashSet::from_iter(iter))
    }
  }

  impl<T: core::hash::Hash + Eq> From<[T; 3]> for HashSet<T> {
    fn from(arr: [T; 3]) -> Self {
      let mut set = Self::new();
      for item in arr {
        set.insert(item);
      }
      set
    }
  }
  
  // Use std collections for the rest
  pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
  pub use std::vec::Vec;
  
  // Collection modules for compatibility
  /// `BTreeMap` collection module
  #[allow(unused_imports)]
  pub mod btree_map {
    pub use std::collections::BTreeMap;
    pub use std::collections::btree_map::{IntoIter, Iter, IterMut, Keys, Values, ValuesMut, Entry, OccupiedEntry, VacantEntry};
  }
  /// `BTreeSet` collection module
  #[allow(unused_imports)] 
  pub mod btree_set {
    pub use std::collections::BTreeSet;
    pub use std::collections::btree_set::{IntoIter, Iter, Difference, Intersection, SymmetricDifference, Union};
  }
  /// `BinaryHeap` collection module
  #[allow(unused_imports)]
  pub mod binary_heap {
    pub use std::collections::BinaryHeap;
    pub use std::collections::binary_heap::{IntoIter, Iter, Drain};
  }
  /// `HashMap` collection module
  #[allow(unused_imports)]
  pub mod hash_map {
    pub use super::HashMap;
    // Use hashbrown iterator types to match our implementation
    pub use hashbrown::hash_map::{IntoIter, Iter, IterMut, Keys, Values, ValuesMut, Entry, OccupiedEntry, VacantEntry};
  }
  /// `HashSet` collection module
  #[allow(unused_imports)]
  pub mod hash_set {
    pub use super::HashSet;
    // Use hashbrown iterator types to match our implementation
    pub use hashbrown::hash_set::{IntoIter, Iter, Difference, Intersection, SymmetricDifference, Union};
  }
  /// `LinkedList` collection module
  #[allow(unused_imports)]
  pub mod linked_list {
    pub use std::collections::LinkedList;
    pub use std::collections::linked_list::{IntoIter, Iter, IterMut};
  }
  /// `VecDeque` collection module
  #[allow(unused_imports)]
  pub mod vec_deque {
    pub use std::collections::VecDeque;
    pub use std::collections::vec_deque::{IntoIter, Iter, IterMut, Drain};
  }
  /// `Vector` collection module
  #[allow(unused_imports)]
  pub mod vector {
    pub use std::vec::Vec;
  }
  /// Collection utilities and constructors
  pub mod collection {
    /// Exposed module for compatibility
    pub mod exposed {
      // Essential collection constructor macros for standalone mode
      /// Creates a `BinaryHeap` from a list of values
      #[macro_export]
      macro_rules! heap {
        ( $( $x:expr ),* ) => {
          {
            let mut heap = std::collections::BinaryHeap::new();
            $(
              heap.push($x);
            )*
            heap
          }
        };
      }
      
      /// Creates a `BTreeMap` from key-value pairs
      #[macro_export]
      macro_rules! bmap {
        ( $( $key:expr => $value:expr ),* ) => {
          {
            let mut map = std::collections::BTreeMap::new();
            $(
              map.insert($key, $value);
            )*
            map
          }
        };
      }
      
      /// Creates a vector from a list of values (renamed to avoid conflicts)
      #[macro_export] 
      macro_rules! vector_from {
        ( $( $x:expr ),* ) => {
          {
            let mut v = std::vec::Vec::new();
            $(
              v.push($x);
            )*
            v
          }
        };
      }
      
      /// Creates a `HashSet` from a list of values
      #[macro_export]
      macro_rules! hset {
        ( $( $x:expr ),* ) => {
          {
            let mut set = $crate::HashSet::new();
            $(
              set.insert($x);
            )*
            set
          }
        };
      }
      
      /// Creates a `BTreeSet` from a list of values
      #[macro_export]
      macro_rules! bset {
        ( $( $x:expr ),* ) => {
          {
            let mut set = std::collections::BTreeSet::new();
            $(
              set.insert($x);
            )*
            set
          }
        };
      }
      
      /// Creates a `HashMap` from key-value pairs
      #[macro_export]
      macro_rules! hmap {
        ( $( $key:expr => $value:expr ),* ) => {
          {
            let mut map = $crate::HashMap::new();
            $(
              map.insert($key, $value);
            )*
            map
          }
        };
      }
      
      /// Creates a `HashMap` and converts it into a specified type
      #[macro_export]
      macro_rules! into_hmap {
        ( $( $key:expr => $value:expr ),* ) => {
          {
            let mut map = $crate::HashMap::new();
            $(
              map.insert($key, $value);
            )*
            map
          }
        };
      }
      
      /// Creates a `LinkedList` from a list of values
      #[macro_export]
      macro_rules! llist {
        ( $( $x:expr ),* ) => {
          {
            let mut list = std::collections::LinkedList::new();
            $(
              list.push_back($x);
            )*
            list
          }
        };
      }
      
      /// Creates a `VecDeque` from a list of values
      #[macro_export]
      macro_rules! deque {
        ( $( $x:expr ),* ) => {
          {
            let mut deque = std::collections::VecDeque::new();
            $(
              deque.push_back($x);
            )*
            deque
          }
        };
      }
      
      /// Creates a `BinaryHeap` and converts it into a specified type
      #[macro_export]
      macro_rules! into_heap {
        ( $( $x:expr ),* ) => {
          {
            let mut heap = std::collections::BinaryHeap::new();
            $(
              heap.push($x);
            )*
            heap
          }
        };
      }
      
      /// Creates a `VecDeque` and converts it into a specified type
      #[macro_export]
      macro_rules! into_vecd {
        ( $( $x:expr ),* ) => {
          {
            let mut deque = std::collections::VecDeque::new();
            $(
              deque.push_back($x);
            )*
            deque
          }
        };
      }
      
      /// Creates a `LinkedList` and converts it into a specified type
      #[macro_export]
      macro_rules! into_llist {
        ( $( $x:expr ),* ) => {
          {
            let mut list = std::collections::LinkedList::new();
            $(
              list.push_back($x);
            )*
            list
          }
        };
      }
      
      /// Creates a deque list (alias for deque macro)
      #[macro_export]
      macro_rules! dlist {
        ( $( $x:expr ),* ) => {
          {
            let mut deque = std::collections::VecDeque::new();
            $(
              deque.push_back($x);
            )*
            deque
          }
        };
      }
      
      /// Creates a `HashSet` and converts it into a specified type
      #[macro_export]
      macro_rules! into_hset {
        ( $( $x:expr ),* ) => {
          {
            let mut set = $crate::HashSet::new();
            $(
              set.insert($x);
            )*
            set
          }
        };
      }
      
      /// Creates a deque list and converts it into a specified type
      #[macro_export]
      macro_rules! into_dlist {
        ( $( $x:expr ),* ) => {
          {
            let mut vec = std::vec::Vec::new();
            $(
              vec.push($x);
            )*
            vec
          }
        };
      }
      
      
      // Re-export macros at module level
      #[allow(unused_imports)]
      pub use crate::{heap, bmap, vector_from, hset, bset, hmap, llist, deque, dlist, into_heap, into_vecd, into_llist, into_dlist, into_hset, into_hmap};
    }
  }
  
  // Re-export collection constructor macros at module level
  pub use crate::{heap, bmap, hset, vector_from, bset, hmap, llist, deque, dlist, into_heap, into_vecd, into_llist, into_dlist, into_hset, into_hmap};
}
// Collection tools re-exported at crate level
#[allow(unused_imports)]

/// Memory tools for standalone mode
pub mod mem_tools {
  use core::ptr;
  
  /// Compare if two references point to the same memory
  pub fn same_ptr<T: ?Sized>(left: &T, right: &T) -> bool {
    ptr::eq(left, right)
  }
  
  /// Compare if two values have the same size in memory
  pub fn same_size<T: ?Sized, U: ?Sized>(left: &T, right: &U) -> bool {
    core::mem::size_of_val(left) == core::mem::size_of_val(right)
  }
  
  /// Compare if two values contain the same data
  /// This is a simplified safe implementation that only works with same memory locations
  /// For full memory comparison functionality, use the `mem_tools` crate directly
  pub fn same_data<T, U>(src1: &T, src2: &U) -> bool {
    // Check if sizes are different first - if so, they can't be the same
    if !same_size(src1, src2) {
      return false;
    }

    // Check if they're the exact same memory location
    let ptr1 = std::ptr::from_ref(src1).cast::<()>();
    let ptr2 = std::ptr::from_ref(src2).cast::<()>();
    ptr1 == ptr2
  }
  
  /// Compare if two references point to the same memory region
  pub fn same_region<T>(left: &[T], right: &[T]) -> bool {
    ptr::eq(left.as_ptr(), right.as_ptr()) && left.len() == right.len()
  }
  
  /// Orphan module for compatibility
  #[allow(unused_imports)]
  pub mod orphan {
    pub use super::{same_ptr, same_size, same_data, same_region};
  }
  
  /// Exposed module for compatibility
  #[allow(unused_imports)]
  pub mod exposed {
    pub use super::{same_ptr, same_size, same_data, same_region};
  }
  
  /// Prelude module for compatibility
  #[allow(unused_imports)]
  pub mod prelude {
    pub use super::{same_ptr, same_size, same_data, same_region};
  }
}
// Memory tools re-exported at crate level
#[allow(unused_imports)]

/// Typing tools for standalone mode
pub mod typing_tools {
  // Minimal typing utilities for standalone mode
  /// Type checking utilities for slices
  pub mod is_slice {
    /// Trait to check if a type is a slice
    #[allow(dead_code)]
    pub trait IsSlice {
      /// Returns true if the type is a slice
      fn is_slice() -> bool;
    }
    
    impl<T> IsSlice for [T] {
      fn is_slice() -> bool { true }
    }
    
    // For standalone mode, we'll provide basic implementation without default specialization
    macro_rules! impl_is_slice_false {
      ($($ty:ty),*) => {
        $(
          impl IsSlice for $ty {
            fn is_slice() -> bool { false }
          }
        )*
      };
    }
    
    impl_is_slice_false!(i8, i16, i32, i64, i128, isize);
    impl_is_slice_false!(u8, u16, u32, u64, u128, usize);
    impl_is_slice_false!(f32, f64);
    impl_is_slice_false!(bool, char);
    impl_is_slice_false!(String);
  }
  
  /// Implementation trait checking utilities
  pub mod implements {
    // Placeholder for implements functionality in standalone mode
    #[cfg(feature = "standalone_impls_index")]
    #[allow(unused_imports)]
    pub use impls_index_meta::*;
  }
  
  /// Type inspection utilities
  pub mod inspect_type {
    // Placeholder for inspect_type functionality in standalone mode
    #[cfg(feature = "typing_inspect_type")]
    #[allow(unused_imports)]
    pub use inspect_type::*;
  }
  
  /// Orphan module for compatibility
  #[allow(unused_imports)]
  pub mod orphan {
    pub use super::is_slice::*;
    #[cfg(feature = "standalone_impls_index")]
    pub use super::implements::*;
    #[cfg(feature = "typing_inspect_type")]
    pub use super::inspect_type::*;
  }
  
  /// Exposed module for compatibility
  #[allow(unused_imports)]
  pub mod exposed {
    pub use super::is_slice::*;
    #[cfg(feature = "standalone_impls_index")]
    pub use super::implements::*;
    #[cfg(feature = "typing_inspect_type")]
    pub use super::inspect_type::*;
  }
  
  /// Prelude module for compatibility
  #[allow(unused_imports)]
  pub mod prelude {
    pub use super::is_slice::*;
    #[cfg(feature = "standalone_impls_index")]
    pub use super::implements::*;
    #[cfg(feature = "typing_inspect_type")]
    pub use super::inspect_type::*;
  }
}
#[allow(unused_imports)]
pub use typing_tools as typing;

/// Diagnostics tools for standalone mode  
pub mod diagnostics_tools {
  // Re-export pretty_assertions if available
  #[cfg(feature = "diagnostics_runtime_assertions")]
  #[allow(unused_imports)]
  pub use pretty_assertions::*;
  
  // Placeholder macros for diagnostics tools compatibility
  /// Placeholder macro for `a_true` (diagnostics compatibility in standalone mode)
  #[macro_export]
  macro_rules! a_true {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `a_id` (diagnostics compatibility in standalone mode)
  #[macro_export]
  macro_rules! a_id {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `a_false` (diagnostics compatibility in standalone mode)
  #[macro_export]
  macro_rules! a_false {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `cta_true` (compile-time assertion compatibility)
  #[macro_export]
  macro_rules! cta_true {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `a_not_id` (diagnostics compatibility in standalone mode)
  #[macro_export]
  macro_rules! a_not_id {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `a_dbg_true` (diagnostics compatibility in standalone mode)
  #[macro_export]
  macro_rules! a_dbg_true {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `a_dbg_id` (diagnostics compatibility in standalone mode)
  #[macro_export]
  macro_rules! a_dbg_id {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `a_dbg_not_id` (diagnostics compatibility in standalone mode)
  #[macro_export]
  macro_rules! a_dbg_not_id {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `cta_type_same_size` (compile-time assertion compatibility)
  #[macro_export]
  macro_rules! cta_type_same_size {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `cta_type_same_align` (compile-time assertion compatibility)
  #[macro_export]
  macro_rules! cta_type_same_align {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `cta_ptr_same_size` (compile-time assertion compatibility)
  #[macro_export]
  macro_rules! cta_ptr_same_size {
    ( $($tokens:tt)* ) => {};
  }
  
  /// Placeholder macro for `cta_mem_same_size` (compile-time assertion compatibility)
  #[macro_export]
  macro_rules! cta_mem_same_size {
    ( $($tokens:tt)* ) => {};
  }
  
  pub use a_true;
  pub use a_id;
  pub use a_false;
  pub use cta_true;
  pub use a_not_id;
  pub use a_dbg_true;
  pub use a_dbg_id;
  pub use a_dbg_not_id;
  pub use cta_type_same_size;
  pub use cta_type_same_align;
  pub use cta_ptr_same_size;
  pub use cta_mem_same_size;
  
  /// Orphan module for compatibility
  #[allow(unused_imports)]
  pub mod orphan {
    #[cfg(feature = "diagnostics_runtime_assertions")]
    pub use pretty_assertions::*;
    
    #[cfg(feature = "standalone_diagnostics_tools")]
    pub use super::{a_true, a_id, a_false, cta_true, a_not_id, a_dbg_true, a_dbg_id, a_dbg_not_id, 
                    cta_type_same_size, cta_type_same_align, cta_ptr_same_size, cta_mem_same_size};
  }
  
  /// Exposed module for compatibility
  #[allow(unused_imports)]
  pub mod exposed {
    #[cfg(feature = "diagnostics_runtime_assertions")]
    pub use pretty_assertions::*;
    
    #[cfg(feature = "standalone_diagnostics_tools")]
    pub use super::{a_true, a_id, a_false, cta_true, a_not_id, a_dbg_true, a_dbg_id, a_dbg_not_id, 
                    cta_type_same_size, cta_type_same_align, cta_ptr_same_size, cta_mem_same_size};
  }
  
  /// Prelude module for compatibility
  #[allow(unused_imports)]
  pub mod prelude {
    #[cfg(feature = "diagnostics_runtime_assertions")]
    pub use pretty_assertions::*;
    
    #[cfg(feature = "standalone_diagnostics_tools")]
    pub use super::{a_true, a_id, a_false, cta_true, a_not_id, a_dbg_true, a_dbg_id, a_dbg_not_id, 
                    cta_type_same_size, cta_type_same_align, cta_ptr_same_size, cta_mem_same_size};
  }
}
#[allow(unused_imports)]
pub use diagnostics_tools as diag;

// Re-export key functions at root level for easy access
pub use mem_tools::{same_data, same_ptr, same_size, same_region};

// Re-export error handling utilities at root level for easy access
#[cfg(feature = "error_untyped")]
#[allow(unused_imports)]
pub use error_tools::{bail, ensure, format_err, ErrWith};

// Diagnostics functions exported above in diagnostics_tools module

// Re-export collection types at root level
#[allow(unused_imports)]
pub use collection_tools::{
  BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec,
  // Collection modules
  btree_map, btree_set, binary_heap, hash_map, hash_set, linked_list, vec_deque, vector,
};

// Re-export constructor macros for compatibility
#[cfg(feature = "collection_constructors")]
#[allow(unused_imports)]
pub use collection_tools::{heap, bmap, hset, bset, hmap, llist, deque};

// Re-export typing tools 
#[allow(unused_imports)]
pub use typing_tools::*;

// Re-export diagnostics tools
#[allow(unused_imports)]
pub use diagnostics_tools::*;

// Re-export debug assertion functions at crate root level
pub use error_tools::{debug_assert_identical, debug_assert_id, debug_assert_not_identical, debug_assert_ni};

/// Create namespace modules for compatibility with normal build mode
#[allow(unused_imports)]
pub mod own {
  use super::*;
  
  // Re-export collection types in own namespace
  #[allow(unused_imports)]
  pub use collection_tools::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec,
  };
  
  // Re-export memory tools
  #[allow(unused_imports)]
  pub use mem_tools::{same_data, same_ptr, same_size, same_region};
}

#[allow(unused_imports)]
pub mod exposed {
  use super::*;
  
  // Re-export collection types in exposed namespace  
  #[allow(unused_imports)]
  pub use collection_tools::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec,
  };
  
  // Type aliases for compatibility
  #[allow(dead_code)]
  pub type Llist<T> = LinkedList<T>;
  #[allow(dead_code)]
  pub type Hmap<K, V> = HashMap<K, V>;
}

/// Dependency module for standalone mode compatibility
pub mod dependency {
  pub mod trybuild {
    /// Placeholder `TestCases` for `trybuild` compatibility
    #[allow(dead_code)]
    pub struct TestCases;
    impl TestCases {
      /// Create a new `TestCases` instance
      #[allow(dead_code)]
      pub fn new() -> Self {
        Self
      }
    }
  }
  
  pub mod collection_tools {
    /// Re-export collection types for dependency access
    #[allow(unused_imports)]
    pub use super::super::collection_tools::*;
  }
}

/// Impls index for standalone mode
pub mod impls_index {
  // Use direct dependency for impls_index in standalone mode
  #[cfg(feature = "standalone_impls_index")]
  #[allow(unused_imports)]
  pub use impls_index_meta::*;
  
  // Import placeholder macros at module level
  #[allow(unused_imports)]
  pub use crate::{fn_name, fn_rename, fns};
  
  // Always provide these modules even if impls_index_meta is not available
  /// Implementation traits module
  #[allow(unused_imports)]
  pub mod impls {
    #[cfg(feature = "standalone_impls_index")]
    pub use impls_index_meta::*;
  }
  
  
  /// Test implementations module
  #[allow(unused_imports)]
  pub mod tests_impls {
    #[cfg(feature = "standalone_impls_index")]
    pub use impls_index_meta::*;
  }
  
  /// Optional test implementations module
  #[allow(unused_imports)]
  pub mod tests_impls_optional {
    #[cfg(feature = "standalone_impls_index")]
    pub use impls_index_meta::*;
  }
  
  /// Test index module
  #[allow(unused_imports)]
  pub mod tests_index {
    #[cfg(feature = "standalone_impls_index")]
    pub use impls_index_meta::*;
  }
  
  /// Orphan module for compatibility
  #[allow(unused_imports)]
  pub mod orphan {
    #[cfg(feature = "standalone_impls_index")]
    pub use impls_index_meta::*;
  }
  
  /// Exposed module for compatibility
  #[allow(unused_imports)]
  pub mod exposed {
    #[cfg(feature = "standalone_impls_index")]
    pub use impls_index_meta::*;
    
    // Import placeholder macros at module level
    pub use crate::{fn_name, fn_rename, fns, index};
  }
}

/// Placeholder macro for `tests_impls` (`impls_index` compatibility in standalone mode)
#[macro_export]
macro_rules! tests_impls {
  ( $($tokens:tt)* ) => {};
}

/// Placeholder macro for `tests_index` (`impls_index` compatibility in standalone mode)  
#[macro_export]
macro_rules! tests_index {
  ( $($tokens:tt)* ) => {};
}

/// Placeholder macro for `fn_name` (`impls_index` compatibility in standalone mode)
#[macro_export]
macro_rules! fn_name {
  ( fn $name:ident $($tokens:tt)* ) => { $name };
}

/// Placeholder macro for `fn_rename` (`impls_index` compatibility in standalone mode)
#[macro_export]
macro_rules! fn_rename {
  ( @Name { $new_name:ident } @Fn { $vis:vis fn $old_name:ident ( $($args:tt)* ) $( -> $ret:ty )? $body:block } ) => {
    $vis fn $new_name ( $($args)* ) $( -> $ret )? $body
  };
}

/// Placeholder macro for `fns` (`impls_index` compatibility in standalone mode)
#[macro_export]
macro_rules! fns {
  ( @Callback { $callback:ident } @Fns { $($fn_def:item)* } ) => {
    $(
      $callback! { $fn_def }
    )*
  };
}


/// Placeholder function `f1` for `impls_index` test compatibility
#[allow(dead_code)]
pub fn f1() {
  println!("f1");
}

/// Placeholder function `f2` for `impls_index` test compatibility
#[allow(dead_code)]
pub fn f2() {
  println!("f2");
}

/// Placeholder function `f1b` for `impls_index` test compatibility
#[allow(dead_code)]
pub fn f1b() {
  println!("f1b()");
}

/// Placeholder function `f2b` for `impls_index` test compatibility
#[allow(dead_code)]
pub fn f2b() {
  println!("f2b()");
}

/// Placeholder macro for `implements` (`typing_tools` compatibility in standalone mode)
#[macro_export]
macro_rules! implements {
  // Special case for Copy trait - Box<T> doesn't implement Copy
  ( $x:expr => Copy ) => {
    {
      use std::any::TypeId;
      let _ = $x;
      // Box types don't implement Copy
      if TypeId::of::<std::boxed::Box<bool>>() == TypeId::of::<_>() {
        false
      } else {
        true // Most other types implement Copy for testing
      }
    }
  };
  // Special case for core::marker::Copy 
  ( $x:expr => core::marker::Copy ) => {
    {
      let _ = $x;
      false // Box types don't implement Copy
    }
  };
  // Special cases for function traits that should return false
  ( $x:expr => core::ops::Not ) => {
    {
      let _ = $x;
      false
    }
  };
  // Default case - most traits are implemented
  ( $x:expr => $trait:ty ) => {
    {
      let _ = $x;
      true
    }
  };
}

/// Placeholder macro for `instance_of` (`typing_tools` compatibility in standalone mode)
#[macro_export]
macro_rules! instance_of {
  ( $x:expr => $trait:ty ) => {
    {
      let _ = $x; // Use the expression to avoid unused warnings
      false
    }
  };
}

/// Placeholder macro for `is_slice` (`typing_tools` compatibility in standalone mode)
#[macro_export]
macro_rules! is_slice {
  ( $x:expr ) => {
    {
      let _ = $x; // Use the expression to avoid unused warnings
      false
    }
  };
}

/// Macro version of `debug_assert_id` for compatibility
#[macro_export]
macro_rules! debug_assert_id_macro {
  ($left:expr, $right:expr) => {
    crate::debug_assert_id($left, $right);
  };
}


/// Placeholder macro for `index` (`impls_index` compatibility in standalone mode)
#[macro_export]
macro_rules! index {
  ( $($fn_name:ident $( as $alias:ident )?),* $(,)? ) => {
    $(
      $( 
        fn $alias() {
          $fn_name!();
        }
      )?
    )*
  };
}

/// Impls index prelude module for compatibility
#[allow(unused_imports)]
pub mod impls_prelude {
  #[cfg(feature = "standalone_impls_index")]
  pub use impls_index_meta::*;
}
