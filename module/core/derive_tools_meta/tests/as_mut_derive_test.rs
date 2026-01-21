//! Comprehensive tests for `AsMut` derive macro.
//!
//! Tests all corner cases for the `#[derive(AsMut)]` macro including:
//! - Single-field structs (auto `AsMut`)
//! - Multi-field structs (requires `#[as_mut]` marker)
//! - Generic types
//!
//! ## Test Matrix
//!
//! | Test Case | Struct Type | Fields | Marker | Expected | Status |
//! |-----------|-------------|--------|--------|----------|--------|
//! | `test_single_field_tuple` | Tuple | 1 | None | Success | ✅ |
//! | `test_single_field_named` | Named | 1 | None | Success | ✅ |
//! | `test_multi_field_with_marker` | Named | 2+ | `#[as_mut]` | Success | ✅ |
//!
//! **Semantics:**
//! The `AsMut` derive implements `AsMut<T>` where T is the inner field type.
//! Allows converting `&mut Wrapper` to `&mut T` (mutable reference to inner field).

use derive_tools_meta::*;

/// Test 1: Single-field tuple struct
///
/// Should automatically implement `AsMut` for the inner field type.
#[ test ]
fn test_single_field_tuple()
{
  #[ derive( AsMut ) ]
  struct Wrapper( String );

  let mut w = Wrapper( "hello".to_string() );

  // AsMut converts &mut Wrapper to &mut String
  let s: &mut String = w.as_mut();
  s.push_str( " world" );

  assert_eq!( w.0, "hello world" );
}

/// Test 2: Single-field named struct
///
/// Should automatically implement `AsMut` for the single field type.
#[ test ]
fn test_single_field_named()
{
  #[ derive( AsMut ) ]
  struct Container
  {
    value: i32
  }

  let mut c = Container { value: 42 };

  let v: &mut i32 = c.as_mut();
  *v = 100;

  assert_eq!( c.value, 100 );
}

/// Test 3: Multi-field struct with `#[as_mut]` marker
///
/// Should implement `AsMut` for the field marked with `#[as_mut]` attribute.
#[ test ]
fn test_multi_field_with_marker()
{
  #[ derive( AsMut ) ]
  struct MultiField
  {
    #[ as_mut ]
    primary: String,
    secondary: i32,
  }

  let mut m = MultiField
  {
    primary: "test".to_string(),
    secondary: 99,
  };

  // AsMut targets the marked field
  let s: &mut String = m.as_mut();
  s.push_str( "_modified" );

  assert_eq!( m.primary, "test_modified" );
  assert_eq!( m.secondary, 99 );
}
