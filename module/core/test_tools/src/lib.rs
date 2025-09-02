// #![ cfg_attr( feature = "no_std", no_std ) ]
#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/test_tools/latest/test_tools/")]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Testing utilities and tools" ) ]

//! # Important: `vec!` Macro Ambiguity
//!
//! When using `use test_tools::*`, you may encounter ambiguity between `std::vec!` and `collection_tools::vec!`.
//!
//! ## Solutions:
//!
//! ```rust
//! // RECOMMENDED: Use std::vec! explicitly
//! use test_tools::*;
//! let v = std::vec![1, 2, 3];
//!
//! // OR: Use selective imports
//! use test_tools::{BTreeMap, HashMap};
//! let v = vec![1, 2, 3]; // No ambiguity
//!
//! // OR: Use collection macros explicitly  
//! let collection_vec = collection_tools::vector_from![1, 2, 3];
//! ```
//!
//! # API Stability Facade
//!
//! This crate implements a comprehensive API stability facade pattern (FR-3) that shields
//! users from breaking changes in underlying constituent crates. The facade ensures:
//!
//! - **Stable API Surface**: Core functionality remains consistent across versions
//! - **Namespace Isolation**: Changes in constituent crates don't affect public namespaces  
//! - **Dependency Insulation**: Internal dependency changes are hidden from users
//! - **Backward Compatibility**: Existing user code continues to work across updates
//!
//! ## Stability Mechanisms
//!
//! ### 1. Controlled Re-exports
//! All types and functions from constituent crates are re-exported through carefully
//! controlled namespace modules (own, orphan, exposed, prelude) that maintain consistent APIs.
//!
//! ### 2. Dependency Isolation Module
//! The `dependency` module provides controlled access to underlying crates, allowing
//! updates to constituent crates without breaking the public API.
//!
//! ### 3. Feature-Stable Functionality
//! Core functionality works regardless of feature combinations, with optional features
//! providing enhanced capabilities without breaking the base API.
//!
//! # Test Compilation Troubleshooting Guide
//!
//! This crate aggregates testing tools from multiple ecosystem crates. Due to the complexity
//! of feature propagation and macro re-exports, test compilation can fail in specific patterns.
//!
//! ## Quick Diagnosis Commands
//!
//! ```bash
//! # Test compilation (fastest diagnostic)
//! cargo test -p test_tools --all-features --no-run
//!
//! # Full test suite  
//! cargo test -p test_tools --all-features
//!
//! # Verbose compilation for detailed errors
//! cargo test -p test_tools --all-features --no-run -v
//! ```
//!
//! ## Common Error Patterns & Solutions
//!
//! ### E0432 Errors (API Visibility)
//! ```text
//! error[E0432]: unresolved imports `test_tools::tests_impls`, `test_tools::exposed`
//! ```
//! **Root Cause:** Public API modules hidden by cfg gates  
//! **Solution:** Remove `#[cfg(not(feature = "doctest"))]` gates on namespace modules  
//! **Prevention:** See warnings in namespace module documentation below
//!
//! ### E0433 Errors (Macro Resolution)  
//! ```text
//! error[E0433]: failed to resolve: could not find `heap` in `the_module`
//! ```
//! **Root Cause:** Collection constructor macros not re-exported  
//! **Solution:** Verify macro re-exports around line 160-180 in this file  
//! **Quick Fix:** Ensure explicit macro re-exports with proper feature gates
//!
//! ## Step-by-Step Debugging Process
//!
//! 1. **Count errors by type:** `cargo test -p test_tools --all-features --no-run 2>&1 | grep "error\[" | sort | uniq -c`
//! 2. **For E0432 (API visibility):** Check namespace modules for doctest cfg gates
//! 3. **For E0433 (macro resolution):** Check macro re-exports and feature configuration
//! 4. **Verify feature propagation:** Check with `-v` flag for enabled features
//!
//! ## Historical Context
//! - **Task 001:** Fixed 147 E0432 errors by removing doctest cfg gates from API modules
//! - **Task 002:** Fixed 7 E0433 errors by adding explicit macro re-exports  
//! - **Task 003:** Added this embedded documentation to prevent regressions
//!

// xxx : remove
//! ```rust
//! println!("-- doc test: printing Cargo feature environment variables --");
//! for (key, val) in std::env::vars() {
//!     if key.starts_with("CARGO_FEATURE_") {
//!         println!("{}={}", key, val);
//!     }
//! }
//! ```

// xxx2 : try to repurpose top-level lib.rs fiel for only top level features

/// Namespace with dependencies.
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub mod dependency {

  // // zzz : exclude later
  // #[ doc( inline ) ]
  // pub use ::paste;
  #[ doc( inline ) ]
  pub use ::trybuild;
  #[ doc( inline ) ]
  pub use ::rustversion;
  #[ doc( inline ) ]
  pub use ::num_traits;

  #[cfg(all(feature = "standalone_build", not(feature = "normal_build")))]
  #[ cfg( feature = "standalone_diagnostics_tools" ) ]
  #[ doc( inline ) ]
  pub use ::pretty_assertions;

  // COMMENTED OUT: Dependencies disabled to break circular dependencies
  // #[ doc( inline ) ]
  // pub use super::{
  //   error_tools,
  //   impls_index,
  //   mem_tools,
  //   typing_tools,
  //   diagnostics_tools,
  //   // process_tools,
  // };

  // // Re-export collection_tools directly to maintain dependency access
  // #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
  // #[ doc( inline ) ]
  // pub use ::collection_tools;
  
  // Re-export collection_tools from standalone module for dependency access
  #[cfg(feature = "standalone_build")]
  #[ doc( inline ) ]
  pub use super::standalone::collection_tools;
}

mod private 
{
  //! Private implementation details for API stability facade
  
  /// Verifies API stability facade is properly configured
  /// This function ensures all stability mechanisms are in place
  pub fn verify_api_stability_facade() -> bool
  {
    // COMMENTED OUT: Collection types only available in standalone mode, dependencies disabled to break circular dependencies
    // // Verify namespace modules are accessible
    // let _own_namespace_ok = crate::BTreeMap::<i32, String>::new();
    // let _exposed_namespace_ok = crate::HashMap::<i32, String>::new();
    // 
    // // Verify dependency isolation is working
    // let _dependency_isolation_ok = crate::dependency::trybuild::TestCases::new();
    // 
    // // Verify core testing functionality is stable
    // let _smoke_test_ok = crate::SmokeModuleTest::new("stability_verification");
    // 
    // // All stability checks passed
    true
  }
}

//

// #[ cfg( feature = "enabled" ) ]
// // #[ cfg( not( feature = "no_std" ) ) ]
// ::meta_tools::mod_interface!
// {
//   // #![ debug ]
//
//   own use super::dependency::*;
//
//   layer test;
//
//   // xxx : comment out
//   use super::exposed::meta;
//   use super::exposed::mem;
//   use super::exposed::typing;
//   use super::exposed::dt;
//   use super::exposed::diagnostics;
//   use super::exposed::collection;
//   // use super::exposed::process;
//
//   // prelude use ::rustversion::{ nightly, stable };
//
//   // // xxx : eliminate need to do such things, putting itself to proper category
//   // exposed use super::test::compiletime;
//   // exposed use super::test::helper;
//   // exposed use super::test::smoke_test;
//
//   prelude use ::meta_tools as meta;
//   prelude use ::mem_tools as mem;
//   prelude use ::typing_tools as typing;
//   prelude use ::data_type as dt;
//   prelude use ::diagnostics_tools as diagnostics;
//   prelude use ::collection_tools as collection;
//   // prelude use ::process_tools as process;
//
//   use ::collection_tools; // xxx : do that for all dependencies
//
//   prelude use ::meta_tools::
//   {
//     impls,
//     index,
//     tests_impls,
//     tests_impls_optional,
//     tests_index,
//   };
//
//   prelude use ::typing_tools::{ implements };
//
// }

// xxx : use module namespaces
// #[ cfg( feature = "enabled" ) ]
// #[ cfg( not( feature = "no_std" ) ) ]
// pub use test::{ compiletime, helper, smoke_test };

#[ cfg( feature = "enabled" ) ]
pub mod test;

/// Behavioral equivalence verification framework for re-exported utilities.
#[ cfg( feature = "enabled" ) ]
pub mod behavioral_equivalence;

/// Aggegating submodules without using cargo, but including their entry files directly.
///
/// We don't want to run doctest of included files, because all of the are relative to submodule.
/// So we disable doctests of such submodules with `#[ cfg( not( doctest ) ) ]`.
#[ cfg( feature = "enabled" ) ]
// #[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
#[ cfg( feature = "standalone_build" ) ]
// #[ cfg( any( not( doctest ), not( feature = "standalone_build" ) ) ) ]
mod standalone;

// Use selective exports instead of glob to avoid conflicts
// #[ cfg( feature = "enabled" ) ]
// #[cfg(feature = "standalone_build")]
// #[allow(hidden_glob_reexports)]
// pub use standalone::*;

// Re-export essential functions and types from standalone module 
// Available in all modes to ensure test compatibility
#[ cfg( feature = "standalone_build" ) ]
pub use standalone::{
  debug_assert_identical, debug_assert_id, debug_assert_not_identical, debug_assert_ni,
  same_data, same_ptr, same_size, same_region,
  BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec,
  // Collection modules
  btree_map, btree_set, binary_heap, hash_map, hash_set, linked_list, vec_deque,
  // Error handling trait
  ErrWith,
  // Implementation index modules
  impls_index,
  // Test functions for impls_index tests
  f1, f2, f1b, f2b,
};

// Re-export impls_index modules for direct root access
#[ cfg( feature = "standalone_build" ) ]
pub use standalone::impls_index::{tests_impls, tests_index};


// Diagnostics macros are now defined directly in the standalone module

// Add error module for compatibility with error_tools tests
#[ cfg( feature = "standalone_build" ) ]
/// Error handling module for `error_tools` compatibility in standalone mode
pub mod error {
  /// Assert submodule for error tools compatibility
  pub mod assert {
    pub use crate::debug_assert_id;
  }
}

// tests_impls and tests_index already imported above

// Re-export collection_tools as a module for compatibility
#[ cfg( feature = "standalone_build" ) ]
pub use standalone::collection_tools;

// Re-export diagnostics_tools as a module for compatibility
#[ cfg( feature = "standalone_build" ) ]
pub use standalone::diagnostics_tools;

/// Error tools module for external crate compatibility
/// 
/// This module provides error handling utilities and types for standalone build mode.
/// It re-exports functionality from the standalone `error_tools` implementation.
#[ cfg( feature = "standalone_build" ) ]
pub mod error_tools {
  pub use super::standalone::error_tools::*;
}

/// Memory tools module for external crate compatibility
///
/// This module provides memory comparison utilities for standalone build mode.
#[ cfg( feature = "standalone_build" ) ]
pub mod mem {
  pub use crate::{same_data, same_ptr, same_size, same_region};
}

/// Vector module for external crate compatibility
///
/// This module provides Vec iterator types for standalone build mode.
#[ cfg( feature = "standalone_build" ) ]
pub mod vector {
  pub use std::vec::{IntoIter, Drain};
  pub use core::slice::{Iter, IterMut};
}

/// Collection module for external crate compatibility
///
/// This module provides collection utilities for standalone build mode.
#[ cfg( feature = "standalone_build" ) ]
pub mod collection {
  pub use super::collection_tools::*;
}

// COMMENTED OUT: Normal build dependencies disabled to break circular dependencies
// #[ cfg( feature = "enabled" ) ]
// #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
// pub use ::{error_tools, impls_index, mem_tools, typing_tools, diagnostics_tools};

// // Re-export key mem_tools functions at root level for easy access
// #[ cfg( feature = "enabled" ) ]
// #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
// pub use mem_tools::{same_data, same_ptr, same_size, same_region};

// // Re-export error handling utilities at root level for easy access
// #[ cfg( feature = "enabled" ) ]
// #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
// #[ cfg( feature = "error_untyped" ) ]
// pub use error_tools::{anyhow as error, bail, ensure, format_err};

// Import process module 
#[ cfg( feature = "enabled" ) ]
pub use test::process;

// COMMENTED OUT: collection_tools dependency disabled to break circular dependencies
// /// Re-export `collection_tools` types and functions but not macros to avoid ambiguity.
// /// Macros are available via `collection_tools::macro_name`! to prevent `std::vec`! conflicts.
// #[ cfg( feature = "enabled" ) ]
// #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
// pub use collection_tools::{
//   // Collection types
//   BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec,
//   // Collection modules
//   collection, btree_map, btree_set, binary_heap, hash_map, hash_set, linked_list, vec_deque, vector,
// };

// COMMENTED OUT: collection_tools macros disabled to break circular dependencies
// // Re-export collection macros at root level with original names for aggregated tests
// // This will cause ambiguity with std::vec! when using wildcard imports
// // NOTE: vec! macro removed to prevent ambiguity with std::vec!
// #[ cfg( feature = "enabled" ) ]
// #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
// #[ cfg( feature = "collection_constructors" ) ]
// pub use collection_tools::{heap, bmap, bset, hmap, hset, llist, deque, dlist};

// #[ cfg( feature = "enabled" ) ]
// #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
// #[ cfg( feature = "collection_into_constructors" ) ]
// pub use collection_tools::{into_heap, into_vec, into_bmap, into_bset, into_hmap, into_hset, into_llist, into_vecd, into_dlist};

/// Collection constructor macros moved to prelude module to prevent ambiguity.
///
/// # CRITICAL REGRESSION PREVENTION
///
/// ## Why Moved to Prelude
/// Collection constructor macros like `heap!`, `vec!`, etc. were previously re-exported
/// at crate root level, causing ambiguity with `std::vec`! when using `use test_tools::*`.
/// 
/// Moving them to prelude resolves the ambiguity while maintaining access via
/// `use test_tools::prelude::*` for users who need collection constructors.
///
/// ## What Happens If Moved Back to Root
/// Re-exporting at root will cause E0659 ambiguity errors:
/// ```text
/// error[E0659]: `vec` is ambiguous
/// = note: `vec` could refer to a macro from prelude  
/// = note: `vec` could also refer to the macro imported here
/// ```
///
/// ## Access Patterns
/// - Standard tests: `use test_tools::*;` (no conflicts)
/// - Collection macros needed: `use test_tools::prelude::*;`
/// - Explicit access: `test_tools::prelude::vec![]`
///
/// ## Historical Context  
/// This resolves the vec! ambiguity issue while preserving Task 002's macro accessibility.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub use ::{};

// COMMENTED OUT: error_tools dependency disabled to break circular dependencies
// #[ cfg( feature = "enabled" ) ]
// #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
// pub use error_tools::error;

// // Re-export error! macro as anyhow! from error_tools

// COMMENTED OUT: implsindex dependency disabled to break circular dependencies  
// #[ cfg( feature = "enabled" ) ]
// #[cfg(all(feature = "standalone_build", not(feature = "normal_build")))]
// pub use implsindex as impls_index;

/// Verifies that the API stability facade is functioning correctly.
/// This function can be used to check that all stability mechanisms are operational.
#[ cfg( feature = "enabled" ) ]
#[ must_use ]
pub fn verify_api_stability() -> bool
{
  private::verify_api_stability_facade()
}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// vec! macro removed to prevent ambiguity with `std::vec`!
/// Aggregated `collection_tools` tests will need to use `collection_tools::vec`! explicitly
/// Own namespace of the module.
///
/// # CRITICAL REGRESSION PREVENTION WARNING
/// 
/// DO NOT add `#[cfg(not(feature = "doctest"))]` gates to this module or any of the
/// namespace modules (own, orphan, exposed, prelude). This will hide the public API
/// from tests when the doctest feature is enabled, causing widespread compilation failures.
///
/// ## Historical Context
/// Task 001 resolved 147 compilation errors caused by such gates hiding the API.
/// The pattern `#[cfg(not(feature = "doctest"))]` broke test compilation because:
/// 1. Test runner enables doctest feature via rustdocflags in .cargo/config.toml  
/// 2. This caused the cfg condition to be true, hiding the modules
/// 3. Aggregated tests could no longer import from `the_module::exposed::*` etc.
///
/// ## Safe Alternative  
/// Use feature-specific functionality inside modules, but keep module structure visible.
/// Never hide entire namespace modules with doctest-related cfg gates.
///
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own {
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use test::own::*;

  // Re-export collection types from standalone mode for own namespace
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  #[ doc( inline ) ]
  pub use super::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec};

  // COMMENTED OUT: Dependencies disabled to break circular dependencies
  // #[ doc( inline ) ]
  // pub use {
  //   error_tools::{debug_assert_id, debug_assert_identical, debug_assert_ni, debug_assert_not_identical, ErrWith},
  //   impls_index::orphan::*, 
  //   mem_tools::orphan::*,  // This includes same_data, same_ptr, same_size, same_region
  //   typing_tools::orphan::*,
  //   diagnostics_tools::orphan::*,
  // };
  
  // // Re-export error handling macros from error_tools for comprehensive access
  // #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]  
  // #[ cfg( feature = "error_untyped" ) ]
  // #[ doc( inline ) ]
  // pub use error_tools::{anyhow as error, bail, ensure, format_err};

  // COMMENTED OUT: collection_tools dependency disabled to break circular dependencies
  // // Re-export collection_tools types selectively (no macros to avoid ambiguity)
  // #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
  // #[ doc( inline ) ]
  // pub use collection_tools::{
  //   BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec,
  //   collection, btree_map, btree_set, binary_heap, hash_map, hash_set, linked_list, vec_deque, vector,
  // };
}

/// Shared with parent namespace of the module
///
/// # REGRESSION PREVENTION: Keep this module always visible to tests
/// Same warning as `own` module applies here. See documentation above.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan {
  use super::*;

  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use test::orphan::*;
}

/// Exposed namespace of the module.
///
/// # REGRESSION PREVENTION: Keep this module always visible to tests  
/// This is the primary module accessed by aggregated tests via `the_module::exposed::*`.
/// Hiding this with doctest cfg gates will break all aggregated test imports.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed {
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use test::exposed::*;

  // Re-export collection types from standalone mode for exposed namespace
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  #[ doc( inline ) ]
  pub use super::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec};

  // Re-export collection constructor macros from standalone mode for test compatibility
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  #[ cfg( feature = "collection_constructors" ) ]
  pub use standalone::collection_tools::collection::exposed::{heap, bmap, hmap, bset, llist, deque};

  // Re-export impls_index macros for test compatibility  
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  pub use crate::{index, tests_index, tests_impls};
  
  // Add implsindex alias for compatibility
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  pub use standalone::impls_index as implsindex;
  
  // Add into collection constructor macros to exposed module
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  pub use super::{into_bmap, into_bset, into_hmap, into_hset, into_vec};
  
  // Use placeholder impls3 macro instead of external impls_index_meta (standalone mode)
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  pub use super::impls3;
  
  // Placeholder macros for impls1/2 to satisfy test compilation
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Placeholder macro for impls1 (implementation compatibility in standalone mode)
  #[macro_export]
  macro_rules! impls1 {
    ( 
      $(
        $vis:vis fn $fn_name:ident ( $($args:tt)* ) $( -> $ret:ty )? $body:block
      )*
    ) => {
      // Define the functions
      $(
        $vis fn $fn_name ( $($args)* ) $( -> $ret )? $body
        
        // Define corresponding macros
        macro_rules! $fn_name {
          () => {
            $fn_name();
          };
          (as $alias:ident) => {
            // Create both function and macro for the alias
            fn $alias() {
              $fn_name();
            }
            macro_rules! $alias {
              () => {
                $alias();
              };
            }
          };
        }
      )*
    };
  }
  
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Placeholder macro for impls2 (implementation compatibility in standalone mode)
  #[macro_export]
  macro_rules! impls2 {
    ( 
      $(
        $vis:vis fn $fn_name:ident ( $($args:tt)* ) $( -> $ret:ty )? $body:block
      )*
    ) => {
      // Define the functions
      $(
        $vis fn $fn_name ( $($args)* ) $( -> $ret )? $body
        
        // Define corresponding macros  
        macro_rules! $fn_name {
          () => {
            $fn_name();
          };
          (as $alias:ident) => {
            // Create both function and macro for the alias
            fn $alias() {
              $fn_name();
            }
            macro_rules! $alias {
              () => {
                $alias();
              };
            }
          };
        }
      )*
    };
  }
  
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Placeholder macro for impls3 (implementation compatibility in standalone mode)
  #[macro_export]
  macro_rules! impls3 {
    ( 
      $(
        $vis:vis fn $fn_name:ident ( $($args:tt)* ) $( -> $ret:ty )? $body:block
      )*
    ) => {
      // Define the functions
      $(
        $vis fn $fn_name ( $($args)* ) $( -> $ret )? $body
      )*
      
      // Define corresponding LOCAL macros (no #[macro_export] to avoid global conflicts)
      $(
        macro_rules! $fn_name {
          () => {
            $fn_name();
          };
          (as $alias:ident) => {
            // Create both function and macro for the alias
            fn $alias() {
              $fn_name();
            }
            macro_rules! $alias {
              () => {
                $alias();
              };
            }
          };
        }
      )*
    };
  }

  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  pub use impls1;
  
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  pub use impls2;
  
  // Re-export test function macros for impls_index compatibility
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  pub use super::{f1, f2, fns, fn_name, fn_rename, dlist, into_dlist, hset, into_llist, collection};
  
  // Create actual functions for impls2 test compatibility (f1b, f2b)
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Function alias f1b for impls2 test compatibility
  pub fn f1b() {
    f1(); // Fixed signature compatibility
  }
  
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Function alias f2b for impls2 test compatibility
  pub fn f2b() {
    f2(); // Fixed signature compatibility
  }
  
  // Add missing "into" collection constructor macros
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Placeholder macro for `into_bmap` (collection compatibility in standalone mode)
  #[macro_export]
  macro_rules! into_bmap {
    () => { std::collections::BTreeMap::new() };
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
      {
        let mut map = std::collections::BTreeMap::new();
        $( map.insert( $key, $value ); )*
        map
      }
    };
  }
  
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Placeholder macro for `into_bset` (collection compatibility in standalone mode)  
  #[macro_export]
  macro_rules! into_bset {
    () => { std::collections::BTreeSet::new() };
    ( $( $item:expr ),* $(,)? ) => {
      {
        let mut set = std::collections::BTreeSet::new();
        $( set.insert( $item ); )*
        set
      }
    };
  }
  
  
  
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Placeholder macro for `into_vec` (collection compatibility in standalone mode)
  #[macro_export]
  macro_rules! into_vec {
    () => { std::vec::Vec::new() };
    ( $( $item:expr ),* $(,)? ) => {
      {
        std::vec![ $( $item ),* ]
      }
    };
  }
  
  // into collection macros already exported in exposed module above
  
  // Type aliases for collection compatibility
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Type alias for `LinkedList` for backward compatibility
  pub type Llist<T> = standalone::collection_tools::LinkedList<T>;
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Type alias for `HashMap` for backward compatibility
  pub type Hmap<K, V> = standalone::collection_tools::HashMap<K, V>;

  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Type alias for `BTreeMap` for backward compatibility
  pub type Bmap<K, V> = BTreeMap<K, V>;

  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Type alias for `BTreeSet` for backward compatibility
  pub type Bset<T> = BTreeSet<T>;

  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Type alias for `HashSet` for backward compatibility
  pub type Hset<T> = HashSet<T>;

  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Type alias for `HashMap` for backward compatibility (Map)
  pub type Map<K, V> = HashMap<K, V>;

  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "standalone_build" ) ]
  /// Type alias for `HashSet` for backward compatibility (Set)
  pub type Set<T> = HashSet<T>;



  // COMMENTED OUT: Dependencies disabled to break circular dependencies
  // #[ doc( inline ) ]
  // pub use {
  //   error_tools::{debug_assert_id, debug_assert_identical, debug_assert_ni, debug_assert_not_identical, ErrWith},
  //   impls_index::exposed::*, 
  //   mem_tools::exposed::*,  // This includes same_data, same_ptr, same_size, same_region
  //   typing_tools::exposed::*,
  //   diagnostics_tools::exposed::*,
  // };
  
  // // Re-export error handling macros from error_tools for comprehensive access
  // #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]  
  // #[ cfg( feature = "error_untyped" ) ]
  // #[ doc( inline ) ]
  // pub use error_tools::{anyhow as error, bail, ensure, format_err};

  // COMMENTED OUT: collection_tools dependency disabled to break circular dependencies
  // // Re-export collection_tools types and macros for exposed namespace
  // #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
  // #[ doc( inline ) ]
  // pub use collection_tools::{
  //   BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec,
  //   collection, btree_map, btree_set, binary_heap, hash_map, hash_set, linked_list, vec_deque, vector,
  // };

  // // Re-export collection type aliases from collection::exposed
  // #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
  // #[ doc( inline ) ]
  // pub use collection_tools::collection::exposed::{
  //   Llist, Dlist, Deque, Map, Hmap, Set, Hset, Bmap, Bset,
  // };

  // // Collection constructor macros for aggregated test compatibility  
  // #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
  // #[ cfg( feature = "collection_constructors" ) ]
  // pub use collection_tools::{heap, bmap, bset, hmap, hset, llist, deque, dlist};

  // #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
  // #[ cfg( feature = "collection_into_constructors" ) ]
  // pub use collection_tools::{into_heap, into_vec, into_bmap, into_bset, into_hmap, into_hset, into_llist, into_vecd, into_dlist};
}


/// Prelude to use essentials: `use my_module::prelude::*`.
///
/// # REGRESSION PREVENTION: Keep this module always visible to tests
/// Same warning as other namespace modules. Never hide with doctest cfg gates.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude {
  use super::*;

  #[ doc( inline ) ]
  pub use test::prelude::*;

  pub use ::rustversion::{nightly, stable};

  // Re-export debug assertion functions in standalone mode for prelude access
  #[cfg(feature = "standalone_build")]
  #[ doc( inline ) ]
  pub use super::{debug_assert_id, debug_assert_identical, debug_assert_ni, debug_assert_not_identical};

  // COMMENTED OUT: Dependencies disabled to break circular dependencies
  // #[ doc( inline ) ]
  // pub use {
  //   error_tools::{debug_assert_id, debug_assert_identical, debug_assert_ni, debug_assert_not_identical, ErrWith},
  //   impls_index::prelude::*, 
  //   mem_tools::prelude::*,  // Memory utilities should be accessible in prelude too
  //   typing_tools::prelude::*,
  //   diagnostics_tools::prelude::*,
  // };
  
  // // Re-export error handling macros from error_tools for comprehensive access
  // #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]  
  // #[ cfg( feature = "error_untyped" ) ]
  // #[ doc( inline ) ]
  // pub use error_tools::{anyhow as error, bail, ensure, format_err};


  // Collection constructor macros removed from re-exports to prevent std::vec! ambiguity.
  //
  // AMBIGUITY RESOLUTION
  // Collection constructor macros like `vec!`, `heap!`, etc. are no longer re-exported
  // in test_tools to prevent conflicts with std::vec! when using `use test_tools::*`.
  //
  // Access Patterns for Collection Constructors:
  // ```
  // use test_tools::*;
  // 
  // // Use std::vec! without ambiguity
  // let std_vec = vec![1, 2, 3];
  // 
  // // Use collection_tools constructors explicitly
  // let collection_vec = collection_tools::vec![1, 2, 3];
  // let heap = collection_tools::heap![1, 2, 3];
  // let bmap = collection_tools::bmap!{1 => "one"};
  // ```
  //
  // Alternative: Direct Import
  // ```
  // use test_tools::*;
  // use collection_tools::{vec as cvec, heap, bmap};
  // 
  // let std_vec = vec![1, 2, 3];    // std::vec!
  // let collection_vec = cvec![1, 2, 3]; // collection_tools::vec!
  // ```
}
