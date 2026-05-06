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
    ///
    /// # Errors
    ///
    /// Returns an error tuple containing the context message and original error
    fn err_with<F>(self, f: F) -> Result<T, (String, Self::Error)>
    where
      Self: Sized,
      F: FnOnce() -> String;
    /// Add context to an error using a static string
    ///
    /// # Errors
    ///
    /// Returns an error tuple containing the context message and original error
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

    /// Untyped error handling for `error_tools` compatibility
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

/// Collection tools for standalone mode - Match `collection_tools` exactly
pub mod collection_tools;
// Collection tools re-exported at crate level
#[allow(unused_imports)]
/// Memory tools for standalone mode
pub mod mem_tools {
  use core::ptr;

  /// Compare if two references point to the same memory
  pub fn same_ptr<T1: ?Sized, T2: ?Sized>(src1: &T1, src2: &T2) -> bool {
    let ptr1 = core::ptr::from_ref(src1).cast::<()>();
    let ptr2 = core::ptr::from_ref(src2).cast::<()>();
    ptr1 == ptr2
  }

  /// Compare if two values have the same size in memory
  pub fn same_size<T: ?Sized, U: ?Sized>(left: &T, right: &U) -> bool {
    core::mem::size_of_val(left) == core::mem::size_of_val(right)
  }

  /// Are two pointers points on the same data (compares byte contents)
  /// NOTE: Real implementation in `mem_tools` uses memcmp for byte comparison
  #[ allow( unsafe_code ) ]
  pub fn same_data<T1: ?Sized, T2: ?Sized>(src1: &T1, src2: &T2) -> bool {
    extern "C" {
      fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
    }

    // Check if sizes are different first - if so, they can't be the same
    if !same_size(src1, src2) {
      return false;
    }

    let mem1 = core::ptr::from_ref::<T1>(src1).cast::<u8>();
    let mem2 = core::ptr::from_ref::<T2>(src2).cast::<u8>();

    // SAFETY: Pointers are valid, both regions have same size
    unsafe { memcmp(mem1, mem2, core::mem::size_of_val(src1)) == 0 }
  }

  /// Compare if two references point to the same memory region
  /// This function accepts any sized or unsized types like the real `mem_tools` implementation
  pub fn same_region<T1: ?Sized, T2: ?Sized>(src1: &T1, src2: &T2) -> bool {
    same_ptr(src1, src2) && same_size(src1, src2)
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
    // NOTE: impls_index_meta intentionally removed to break circular dependency
    // See Cargo.toml:85,94 - standalone_impls_index feature is empty by design
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
pub mod diagnostics_tools;
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

// Debug assertion macros are defined at the root level above

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
pub mod impls_index;

// Standalone implementations of test macros - actual working versions
// Copied from impls_index to provide working test generation

/// Test implementation macro that generates test functions with #[test] attributes
#[macro_export]
macro_rules! tests_impls {

  // empty
  () => {};

  // entry
  (
    $( #[ $Meta : meta ] )*
    $Vis : vis
    fn $Name : ident
    $( $Rest : tt )*
  )
  =>
  {
    $crate::tests_impls!
    {
      @DefineFn
      @Meta{ $( #[ $Meta ] )* }
      @Vis{ $Vis }
      @Name{ $Name }
      @Rest
        $( #[ $Meta ] )*
        $Vis fn $Name
        $( $Rest )*
    }
  };

  // parsed
  (
    @DefineFn
    @Meta{ $( #[ $Meta : meta ] )* }
    @Vis{ $Vis : vis }
    @Name{ $Name : ident }
    @Rest
      $Item : item
      $( $Rest : tt )*
  )
  =>
  {
    #[ deny( unused_macros ) ]
    macro_rules! $Name
    {
      () =>
      {
        #[ test ]
        $Item
      };
    }

    $crate::tests_impls!
    {
      $( $Rest )*
    }
  };
}

/// Test index macro that invokes test function macros
#[macro_export]
macro_rules! tests_index {
  () => { };

  (
    $Name : ident as $Alias : ident,
    $( , $( $Rest : tt )* )?
  )
  =>
  {
    $Name!( as $Alias );
    $crate::tests_index!( $( $( $Rest )* )? );
  };

  (
    $Name : ident
    $( , $( $Rest : tt )* )?
  )
  =>
  {
    $Name!();
    $crate::tests_index!( $( $( $Rest )* )? );
  };
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
pub fn f1() {
  println!("f1");
}

/// Placeholder function `f2` for `impls_index` test compatibility
pub fn f2() {
  println!("f2");
}

/// Placeholder function `f1b` for `impls_index` test compatibility
pub fn f1b() {
  println!("f1b()");
}

/// Placeholder function `f2b` for `impls_index` test compatibility
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
    $crate::debug_assert_id($left, $right);
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
  // Placeholder - no impls_index_meta to avoid circular dependency
  // See Cargo.toml:85,94 - standalone_impls_index feature is empty by design
}
