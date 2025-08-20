// We don't want to run doctest of aggregate

/// Error tools.
#[path = "../../../core/error_tools/src/error/mod.rs"]
pub mod error_tools;
pub use error_tools as error;

/// Collection tools.
#[path = "../../../core/collection_tools/src/collection/mod.rs"]
pub mod collection_tools;
pub use collection_tools as collection;

/// impl and index macros.
#[path = "../../../core/impls_index/src/implsindex/mod.rs"]
pub mod implsindex;

/// Memory tools.
#[path = "../../../core/mem_tools/src/mem.rs"]
pub mod mem_tools;
pub use mem_tools as mem;

/// Typing tools.
#[path = "../../../core/typing_tools/src/typing.rs"]
pub mod typing_tools;
pub use typing_tools as typing;

/// Dagnostics tools.
#[path = "../../../core/diagnostics_tools/src/diag/mod.rs"]
pub mod diagnostics_tools;
pub use diagnostics_tools as diag;

// Re-export key mem_tools functions at root level for easy access
pub use mem_tools::{same_data, same_ptr, same_size, same_region};

// Re-export error handling utilities at root level for easy access
// Note: error_tools included via #[path] may not have all the same exports as the crate
// We'll provide basic error functionality through what's available

// Re-export collection_tools types that are available
pub use collection_tools::{
  // Basic collection types from std that should be available
  BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec,
};

// Re-export typing tools functions
pub use typing_tools::*;

// Re-export diagnostics tools functions
pub use diagnostics_tools::*;

// Create namespace modules for standalone mode compatibility
pub mod own {
  use super::*;
  
  // Re-export collection types in own namespace
  pub use collection_tools::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec,
  };
  
  // Re-export memory tools
  pub use mem_tools::{same_data, same_ptr, same_size, same_region};
}

pub mod exposed {
  use super::*;
  
  // Re-export collection types in exposed namespace
  pub use collection_tools::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque, Vec,
  };
}

// Add dependency module for standalone mode (placeholder)
pub mod dependency {
  pub mod trybuild {
    pub struct TestCases;
    impl TestCases {
      pub fn new() -> Self {
        Self
      }
    }
  }
}

// Re-export impls_index for standalone mode
pub use implsindex as impls_index;
