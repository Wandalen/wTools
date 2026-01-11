//! Comprehensive tests for `DerefMut` derive macro.
//!
//! Tests all corner cases for the `#[derive(DerefMut)]` macro including:
//! - Single-field structs (auto `DerefMut`)
//! - Multi-field structs (requires `#[deref_mut]` marker)
//! - Generic types
//! - Lifetime parameters
//!
//! ## Test Matrix
//!
//! | Test Case | Struct Type | Fields | Marker | Expected | Status |
//! |-----------|-------------|--------|--------|----------|--------|
//! | `test_single_field_tuple` | Tuple | 1 | None | Success | ✅ |
//! | `test_single_field_named` | Named | 1 | None | Success | ✅ |
//! | `test_multi_field_with_marker` | Named | 2+ | `#[deref_mut]` | Success | ✅ |
//! | `test_generic_single_field` | Tuple | 1 generic | None | Success | ✅ |
//!
//! **Issue #5 - FIXED:**
//! `DerefMut` derive previously failed on generic types with "expected one of..." compilation error.
//! Fixed by switching from `generic_params::decompose()` to `split_for_impl()` in implementation.
//! See `src/derive/deref_mut.rs:9-11` for details.

use derive_tools_meta::*;

/// Test 1: Single-field tuple struct
///
/// Should automatically implement `DerefMut` to the inner field without requiring marker.
#[ test ]
fn test_single_field_tuple()
{
  #[ derive( Deref, DerefMut ) ]
  struct Wrapper( String );

  let mut w = Wrapper( "hello".to_string() );

  // DerefMut allows mutation through dereference
  *w = "world".to_string();
  assert_eq!( w.0, "world" );
}

/// Test 2: Single-field named struct
///
/// Should automatically implement `DerefMut` to the single field.
#[ test ]
fn test_single_field_named()
{
  #[ derive( Deref, DerefMut ) ]
  struct Container
  {
    value: i32
  }

  let mut c = Container { value: 42 };

  *c = 100;
  assert_eq!( c.value, 100 );
}

/// Test 3: Multi-field struct with `#[deref_mut]` marker
///
/// Should implement `DerefMut` to the field marked with `#[deref_mut]` attribute.
#[ test ]
fn test_multi_field_with_marker()
{
  #[ derive( Deref, DerefMut ) ]
  struct MultiField
  {
    #[ deref_mut ]
    #[ deref ]
    primary: String,
    secondary: i32,
  }

  let mut m = MultiField
  {
    primary: "test".to_string(),
    secondary: 99,
  };

  // DerefMut targets the marked field
  *m = "modified".to_string();
  assert_eq!( m.primary, "modified" );

  // Other fields remain accessible and unchanged
  assert_eq!( m.secondary, 99 );
}

/// Test 4: Generic single-field struct
///
/// Should work with generic type parameters.
/// This test validates the fix for Issue #5.
#[ test ]
fn test_generic_single_field()
{
  #[ derive( Deref, DerefMut ) ]
  struct GenericWrapper< T >( T );

  let mut w: GenericWrapper< String > = GenericWrapper( "generic".to_string() );

  // DerefMut should allow mutation
  *w = "modified".to_string();
  assert_eq!( &*w, "modified" );

  let mut w2: GenericWrapper< Vec< i32 >> = GenericWrapper( vec![ 1, 2, 3 ] );
  *w2 = vec![ 4, 5, 6 ];
  assert_eq!( w2.len(), 3 );
}
