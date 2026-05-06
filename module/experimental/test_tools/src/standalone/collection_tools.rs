// Use API compatibility wrappers when standalone_collection_tools is enabled
#[cfg(feature = "standalone_collection_tools")]
/// HashMap/HashSet API compatibility wrappers for hashbrown collections
pub mod hashmap_compat {
  use std::collections::hash_map::RandomState;
  use core::hash::Hash;

  /// `HashMap` wrapper providing `std::collections::HashMap` API compatibility
  #[derive(Debug, Clone)]
  pub struct HashMap<K, V>(hashbrown::HashMap<K, V, RandomState>);

  impl<K, V> HashMap<K, V>
  where
    K: Hash + Eq,
  {
    /// Create new `HashMap` (std API compatibility)
    #[must_use]
    pub fn new() -> Self {
      Self(hashbrown::HashMap::with_hasher(RandomState::new()))
    }

    /// Insert key-value pair
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
      self.0.insert(k, v)
    }

    /// Get reference to value
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
      K: core::borrow::Borrow<Q>,
      Q: Hash + Eq + ?Sized,
    {
      self.0.get(k)
    }

    /// Get mutable reference to value
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
      K: core::borrow::Borrow<Q>,
      Q: Hash + Eq + ?Sized,
    {
      self.0.get_mut(k)
    }

    /// Remove key-value pair
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
      K: core::borrow::Borrow<Q>,
      Q: Hash + Eq + ?Sized,
    {
      self.0.remove(k)
    }

    /// Check if map is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
      self.0.is_empty()
    }

    /// Get number of elements
    #[must_use]
    pub fn len(&self) -> usize {
      self.0.len()
    }

    /// Clear all elements
    pub fn clear(&mut self) {
      self.0.clear();
    }

    /// Get iterator over key-value pairs
    #[must_use]
    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, K, V> {
      self.0.iter()
    }

    /// Get mutable iterator over key-value pairs
    pub fn iter_mut(&mut self) -> hashbrown::hash_map::IterMut<'_, K, V> {
      self.0.iter_mut()
    }

    /// Get iterator over keys
    #[must_use]
    pub fn keys(&self) -> hashbrown::hash_map::Keys<'_, K, V> {
      self.0.keys()
    }

    /// Get iterator over values
    #[must_use]
    pub fn values(&self) -> hashbrown::hash_map::Values<'_, K, V> {
      self.0.values()
    }
  }

  // Implement IntoIterator for &HashMap<K, V>
  impl<'a, K, V> IntoIterator for &'a HashMap<K, V>
  where
    K: Hash + Eq,
  {
    type Item = (&'a K, &'a V);
    type IntoIter = hashbrown::hash_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
      self.iter()
    }
  }

  // Implement IntoIterator for &mut HashMap<K, V>
  impl<'a, K, V> IntoIterator for &'a mut HashMap<K, V>
  where
    K: Hash + Eq,
  {
    type Item = (&'a K, &'a mut V);
    type IntoIter = hashbrown::hash_map::IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
      self.iter_mut()
    }
  }

  /// `HashSet` wrapper providing `std::collections::HashSet` API compatibility
  #[derive(Debug, Clone)]
  pub struct HashSet<T>(hashbrown::HashSet<T, RandomState>);

  impl<T> HashSet<T>
  where
    T: Hash + Eq,
  {
    /// Create new `HashSet` (std API compatibility)
    #[must_use]
    pub fn new() -> Self {
      Self(hashbrown::HashSet::with_hasher(RandomState::new()))
    }

    /// Insert value
    pub fn insert(&mut self, value: T) -> bool {
      self.0.insert(value)
    }

    /// Check if set contains value
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
      T: core::borrow::Borrow<Q>,
      Q: Hash + Eq + ?Sized,
    {
      self.0.contains(value)
    }

    /// Remove value
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
      T: core::borrow::Borrow<Q>,
      Q: Hash + Eq + ?Sized,
    {
      self.0.remove(value)
    }

    /// Check if set is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
      self.0.is_empty()
    }

    /// Get number of elements
    #[must_use]
    pub fn len(&self) -> usize {
      self.0.len()
    }

    /// Clear all elements
    pub fn clear(&mut self) {
      self.0.clear();
    }

    /// Get iterator over values
    #[must_use]
    pub fn iter(&self) -> hashbrown::hash_set::Iter<'_, T> {
      self.0.iter()
    }
  }

  // Implement IntoIterator for &HashSet<T>
  impl<'a, T> IntoIterator for &'a HashSet<T>
  where
    T: Hash + Eq,
  {
    type Item = &'a T;
    type IntoIter = hashbrown::hash_set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
      self.iter()
    }
  }

  // Implement required traits for HashMap
  impl<K, V> Default for HashMap<K, V>
  where
    K: Hash + Eq,
  {
    fn default() -> Self {
      Self::new()
    }
  }

  impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V>
  where
    K: Hash + Eq,
  {
    fn from(arr: [(K, V); N]) -> Self {
      let mut map = Self::new();
      for (k, v) in arr {
        map.insert(k, v);
      }
      map
    }
  }

  impl<K, V> core::iter::FromIterator<(K, V)> for HashMap<K, V>
  where
    K: Hash + Eq,
  {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
      let mut map = Self::new();
      for (k, v) in iter {
        map.insert(k, v);
      }
      map
    }
  }

  impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = hashbrown::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
      self.0.into_iter()
    }
  }

  impl<K, V> PartialEq for HashMap<K, V>
  where
    K: Eq + Hash,
    V: PartialEq,
  {
    fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
    }
  }

  impl<K, V> Eq for HashMap<K, V>
  where
    K: Eq + Hash,
    V: Eq,
  {}

  // Implement required traits for HashSet
  impl<T> Default for HashSet<T>
  where
    T: Hash + Eq,
  {
    fn default() -> Self {
      Self::new()
    }
  }

  impl<T, const N: usize> From<[T; N]> for HashSet<T>
  where
    T: Hash + Eq,
  {
    fn from(arr: [T; N]) -> Self {
      let mut set = Self::new();
      for item in arr {
        set.insert(item);
      }
      set
    }
  }

  impl<T> core::iter::FromIterator<T> for HashSet<T>
  where
    T: Hash + Eq,
  {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
      let mut set = Self::new();
      for item in iter {
        set.insert(item);
      }
      set
    }
  }

  impl<T> IntoIterator for HashSet<T> {
    type Item = T;
    type IntoIter = hashbrown::hash_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
      self.0.into_iter()
    }
  }

  impl<T> PartialEq for HashSet<T>
  where
    T: Eq + Hash,
  {
    fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
    }
  }

  impl<T> Eq for HashSet<T>
  where
    T: Eq + Hash,
  {}
}

#[cfg(feature = "standalone_collection_tools")]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use hashmap_compat::{HashMap, HashSet};

// Fallback to std when not in standalone mode
#[cfg(not(feature = "standalone_collection_tools"))]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ allow( clippy::pub_use ) ]
pub use std::collections::{HashMap, HashSet};

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
