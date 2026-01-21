//! Comprehensive tests for `IndexMut` derive macro.
//!
//! Tests all corner cases for the `#[derive(IndexMut)]` macro including:
//! - Single-field structs (auto `IndexMut`)
//! - Multi-field structs (requires `#[index_mut]` marker)
//! - Generic types
//!
//! ## Test Matrix
//!
//! | Test Case | Struct Type | Fields | Marker | Expected | Status |
//! |-----------|-------------|--------|--------|----------|--------|
//! | `test_single_field_tuple` | Tuple | 1 | None | Success | ✅ |
//! | `test_single_field_named` | Named | 1 | None | Success | ✅ |
//! | `test_multi_field_with_marker` | Named | 2+ | `#[index_mut]` | Success | ✅ |
//!
//! **Semantics:**
//! The `IndexMut` derive generates BOTH `Index` and `IndexMut` implementations.
//! `wrapper[idx]` returns `&wrapper.field` (immutable).
//! `wrapper[idx] = value` uses mutable indexing to modify the field.
//! Index parameter is ignored - provides syntactic sugar for field access.
//!
//! **Known Issues:**
//! - `IndexMut` with generics not yet tested (may have issues like `DerefMut`)

use derive_tools_meta::*;

/// Test 1: Single-field tuple struct
///
/// Should automatically implement `IndexMut` (and `Index`) for the inner field.
#[ test ]
fn test_single_field_tuple()
{
  #[ derive( IndexMut ) ]
  struct Wrapper( String );

  let mut w = Wrapper( "hello".to_string() );

  // Immutable indexing (from auto-generated Index impl)
  assert_eq!( &w[ 0 ], "hello" );

  // Mutable indexing allows modification
  w[ 0 ] = "world".to_string();
  assert_eq!( w.0, "world" );
}

/// Test 2: Single-field named struct
///
/// Should automatically implement `IndexMut` for the single field.
#[ test ]
fn test_single_field_named()
{
  #[ derive( IndexMut ) ]
  struct Container
  {
    value: i32
  }

  let mut c = Container { value: 42 };

  assert_eq!( c[ 0 ], 42 );

  c[ 0 ] = 100;
  assert_eq!( c.value, 100 );
}

/// Test 3: Multi-field struct with `#[index_mut]` marker
///
/// Should implement `IndexMut` for the field marked with `#[index_mut]` attribute.
#[ test ]
fn test_multi_field_with_marker()
{
  #[ derive( IndexMut ) ]
  struct MultiField
  {
    #[ index_mut ]
    primary: String,
    secondary: i32,
  }

  let mut m = MultiField
  {
    primary: "test".to_string(),
    secondary: 99,
  };

  // Mutable indexing targets the marked field
  m[ 0 ] = "modified".to_string();
  assert_eq!( m.primary, "modified" );

  // Other fields remain accessible and unchanged
  assert_eq!( m.secondary, 99 );
}
