// We don't want to run doctest of aggregate

//! Standalone build mode implementation
//! 
//! This module provides essential functionality for breaking circular dependencies
//! without relying on normal Cargo dependencies. It uses direct transient dependencies
//! and minimal standalone implementations.

// Debug assertion macros for compatibility with error_tools
/// Macro asserts that two expressions are identical to each other. Unlike `std::assert_eq` it is removed from a release build.
#[ macro_export ]
macro_rules! debug_assert_id
{
  ( $( $arg : tt )+ ) =>
  {
    #[ cfg( debug_assertions ) ]
    std::assert_eq!( $( $arg )+ );
  };
}

/// Macro asserts that two expressions are identical to each other. Unlike `std::assert_eq` it is removed from a release build. Alias of `debug_assert_id`.
#[ macro_export ]
macro_rules! debug_assert_identical
{
  ( $( $arg : tt )+ ) =>
  {
    #[ cfg( debug_assertions ) ]
    $crate::debug_assert_id!( $( $arg )+ );
  };
}

/// Macro asserts that two expressions are not identical to each other. Unlike `std::assert_ne` it is removed from a release build.
#[ macro_export ]
macro_rules! debug_assert_ni
{
  ( $( $arg : tt )+ ) =>
  {
    #[ cfg( debug_assertions ) ]
    std::assert_ne!( $( $arg )+ );
  };
}

/// Macro asserts that two expressions are not identical to each other. Unlike `std::assert_ne` it is removed from a release build. Alias of `debug_assert_ni`.
#[ macro_export ]
macro_rules! debug_assert_not_identical
{
  ( $( $arg : tt )+ ) =>
  {
    #[ cfg( debug_assertions ) ]
    $crate::debug_assert_ni!( $( $arg )+ );
  };
}

// Macros are exported at crate root via #[macro_export] - no additional pub use needed

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
    
    /// Assert submodule for error tools compatibility
    pub mod assert {
      pub use crate::{debug_assert_id, debug_assert_identical, debug_assert_ni, debug_assert_not_identical};
    }
    
    /// Untyped error handling for error_tools compatibility
    #[cfg(feature = "standalone_error_tools")]
    pub mod untyped {
      // Re-export anyhow functionality for untyped error tests
      #[cfg(feature = "error_untyped")]
      pub use anyhow::{Error, format_err};
      
      #[cfg(not(feature = "error_untyped"))]
      pub struct Error;
      
      #[cfg(not(feature = "error_untyped"))]
      pub fn format_err(_msg: &str) -> Error {
        Error
      }
    }
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
}

/// Collection tools for standalone mode - Direct re-exports for type compatibility
pub mod collection_tools {
  // Use direct re-exports to match collection_tools exactly
  // This ensures type identity between native and standalone modes
  pub use hashbrown::{HashMap, HashSet};
  
  // Use std collections for the rest
  pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
  pub use std::vec::Vec;
  
  // Collection modules for compatibility  
  pub mod btree_map {
    pub use std::collections::BTreeMap;
    pub use std::collections::btree_map::{IntoIter, Iter, IterMut, Keys, Values, ValuesMut, Entry, OccupiedEntry, VacantEntry};
  }
  
  pub mod btree_set {
    pub use std::collections::BTreeSet;
    pub use std::collections::btree_set::{IntoIter, Iter, Difference, Intersection, SymmetricDifference, Union};
  }
  
  pub mod binary_heap {
    pub use std::collections::BinaryHeap;
    pub use std::collections::binary_heap::{IntoIter, Iter, Drain};
  }
  
  pub mod hash_map {
    pub use super::HashMap;
    pub use hashbrown::hash_map::{IntoIter, Iter, IterMut, Keys, Values, ValuesMut, Entry, OccupiedEntry, VacantEntry};
  }
  
  pub mod hash_set {
    pub use super::HashSet;
    pub use hashbrown::hash_set::{IntoIter, Iter, Difference, Intersection, SymmetricDifference, Union};
  }
  
  pub mod linked_list {
    pub use std::collections::LinkedList;
    pub use std::collections::linked_list::{IntoIter, Iter, IterMut};
  }
  
  pub mod vec_deque {
    pub use std::collections::VecDeque;
    pub use std::collections::vec_deque::{IntoIter, Iter, IterMut, Drain};
  }
  
  pub mod vector {
    pub use std::vec::Vec;
    pub use std::vec::{IntoIter};
  }
  
  // Type aliases for compatibility
  pub type Hmap<K, V> = HashMap<K, V>;
  pub type Hset<T> = HashSet<T>;
  
  // Prelude module
  pub mod prelude {
    pub use super::{
      HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque, Vec,
      Hmap, Hset
    };
    
    pub mod exposed {
      pub use crate::{heap, bmap, vector_from, hset, bset, hmap, llist, deque, dlist, into_heap, into_vecd, into_llist, into_dlist, into_hset, into_hmap};
    }
  }
  
  // Re-export collection constructor macros at module level
  pub use crate::{heap, bmap, hset, vector_from, bset, hmap, llist, deque, dlist, into_heap, into_vecd, into_llist, into_dlist, into_hset, into_hmap};
}

// Collection tools re-exported at crate level
#[allow(unused_imports)]
pub use collection_tools::{HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque, Vec, Hmap, Hset};

// TODO: Add other standalone modules as needed (mem_tools, diagnostics_tools, etc.)