//! Bug reproducer for heap macro accessibility (issue-1)
//!
//! Verifies that `heap!` and `into_heap!` macros are accessible from the public API.
//!
//! # Issue
//!
//! Issue #1: `BinaryHeap` macros not accessible from public API
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
