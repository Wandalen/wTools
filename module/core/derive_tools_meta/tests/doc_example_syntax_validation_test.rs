//! Bug Reproducer: Doc examples use incorrect attribute syntax
//!
//! ## Root Cause
//!
//! The inline doc examples in `src/lib.rs` for `AsMut`, `AsRef`, `Deref`, and `DerefMut`
//! use attribute syntax `#[attribute(original)]` with an `original` parameter.
//! However, the actual implementation does not accept any parameters - the correct
//! syntax is just `#[attribute]` without parameters.
//!
//! This was discovered during manual testing when attempting to verify doc examples
//! compile and execute correctly. The examples are marked as `text` blocks (not
//! compiled), but they still mislead users about the correct syntax.
//!
//! ## Why Not Caught
//!
//! The doc examples use `'''text` code blocks instead of `'''rust` or just `'''`,
//! which means rustdoc doesn't compile-check them. This allows syntax errors to
//! persist undetected.
//!
//! ## Fix Applied
//!
//! Changed all four doc examples to use correct attribute syntax:
//! - `#[as_mut(original)]` → `#[as_mut]`
//! - `#[as_ref(original)]` → `#[as_ref]`
//! - `#[deref(original)]` → `#[deref]`
//! - `#[deref_mut(original)]` → `#[deref_mut]`
//!
//! ## Prevention
//!
//! 1. Use `'''rust` or `'''` instead of `'''text` for doc examples so rustdoc
//!    compile-checks them
//! 2. Add integration tests that verify doc example syntax matches test suite
//! 3. Include doc example validation in CI pipeline
//!
//! ## Pitfall
//!
//! Doc examples marked as `text` blocks bypass all compile-time validation,
//! making it easy for syntax errors to persist. Only use `text` blocks for
//! pseudo-code or examples from external crates that aren't dependencies.

use derive_tools_meta::*;

/// Test 1: `AsMut` with correct syntax (no parameter) - should succeed
#[ test ]
fn doc_example_as_mut_correct_syntax()
{
  #[ derive( AsMut ) ]
  struct MyStruct
  {
    #[ as_mut ] // Correct syntax - no parameter
    a: i32,
  }

  let mut my_struct = MyStruct { a: 1 };
  let a_ref: &mut i32 = my_struct.as_mut();
  *a_ref += 1;
  assert_eq!( my_struct.a, 2 );
}

/// Test 2: `AsRef` with correct syntax (no parameter) - should succeed
#[ test ]
fn doc_example_as_ref_correct_syntax()
{
  #[ derive( AsRef ) ]
  struct MyStruct
  {
    #[ as_ref ] // Correct syntax - no parameter
    a: i32,
  }

  let my_struct = MyStruct { a: 1 };
  let a_ref: &i32 = my_struct.as_ref();
  assert_eq!( *a_ref, 1 );
}

/// Test 3: `Deref` with correct syntax (no parameter) - should succeed
#[ test ]
fn doc_example_deref_correct_syntax()
{
  #[ derive( Deref ) ]
  struct MyStruct
  {
    #[ deref ] // Correct syntax - no parameter
    a: i32,
  }

  let my_struct = MyStruct { a: 1 };
  assert_eq!( *my_struct, 1 );
}

/// Test 4: `DerefMut` with correct syntax (no parameter) - should succeed
#[ test ]
fn doc_example_deref_mut_correct_syntax()
{
  #[ derive( Deref, DerefMut ) ] // DerefMut requires Deref trait as well
  struct MyStruct
  {
    #[ deref ] // Required for Deref
    #[ deref_mut ] // Correct syntax - no parameter
    a: i32,
  }

  let mut my_struct = MyStruct { a: 1 };
  *my_struct += 1;
  assert_eq!( my_struct.a, 2 );
}
