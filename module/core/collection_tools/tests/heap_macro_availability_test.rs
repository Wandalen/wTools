//! Bug reproducer for heap macro accessibility (issue-1)
//!
//! Verifies that `heap!` and `into_heap!` macros are accessible from the public API
//! through both the crate root and the `exposed` re-export module.
//!
//! # Root Cause
//!
//! `heap!` and `into_heap!` macros were defined in `src/collection/binary_heap.rs` but
//! not re-exported through `src/lib.rs` at the crate root or through the `exposed` module.
//! All other constructor macros were re-exported, making this an omission in the macro
//! export list rather than a macro definition bug.
//!
//! # Why Not Caught
//!
//! No existing test exercised `heap!` through a fully-qualified `collection_tools::heap!`
//! path. The macro existed and compiled internally but was invisible from outside the crate,
//! so callers using `use collection_tools::*` silently fell back to no `heap!` in scope.
//!
//! # Fix Applied
//!
//! Added `pub use crate::collection::binary_heap::{ heap, into_heap }` to the re-export
//! block in `src/lib.rs` and to the `exposed` module, matching the pattern used by all
//! other constructor macros.
//!
//! # Prevention
//!
//! When adding a new collection macro, verify both the crate root re-export and the
//! `exposed` module re-export in the same commit. This test file acts as a permanent
//! compile-time guard against future re-export omissions for `heap!` and `into_heap!`.
//!
//! # Pitfall
//!
//! Macro visibility is not verified by the compiler for consumers outside the defining
//! crate — omitting a `pub use` silently prevents access without any diagnostic at the
//! definition site.
//!
//! # Test Kind
//!
//! bug_reproducer(issue-1)

use collection_tools as the_module;

// test_kind: bug_reproducer(issue-1)
#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn heap_macro_accessible_from_root()
{
  let _heap = the_module::heap!{ 1, 2, 3 };
}

// test_kind: bug_reproducer(issue-1)
#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn heap_macro_accessible_from_exposed()
{
  let _heap = the_module::exposed::heap!{ 1, 2, 3 };
}

// test_kind: bug_reproducer(issue-1)
#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_heap_macro_accessible_from_root()
{
  let _heap: the_module::BinaryHeap< i32 > = the_module::into_heap!{ 1, 2, 3 };
}

// test_kind: bug_reproducer(issue-1)
#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_heap_macro_accessible_from_exposed()
{
  let _heap: the_module::BinaryHeap< i32 > = the_module::exposed::into_heap!{ 1, 2, 3 };
}
