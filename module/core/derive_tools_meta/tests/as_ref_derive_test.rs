//! Comprehensive tests for `AsRef` derive macro.
//!
//! Tests all corner cases for the `#[derive(AsRef)]` macro including:
//! - Single-field structs (auto `AsRef`)
//! - Generic types
//! - Reference conversion behavior
//!
//! ## Test Matrix
//!
//! | Test Case | Struct Type | Fields | Marker | Expected | Status |
//! |-----------|-------------|--------|--------|----------|--------|
//! | `test_single_field_tuple` | Tuple | 1 | None | Success | ✅ |
//! | `test_single_field_named` | Named | 1 | None | Success | ✅ |
//! | `test_generic_single_field` | Tuple | 1 generic | None | Success | ✅ |
//!
//! **Semantics:**
//! The `AsRef` derive implements `AsRef<T>` where T is the inner field type.
//! Allows converting `&Wrapper` to `&T` (reference to inner field).

use derive_tools_meta::*;

/// Test 1: Single-field tuple struct
///
/// Should automatically implement `AsRef` for the inner field type.
#[ test ]
fn test_single_field_tuple()
{
  #[ derive( AsRef ) ]
  struct Wrapper( String );

  let w = Wrapper( "hello".to_string() );

  // AsRef converts &Wrapper to &String
  let s: &String = w.as_ref();
  assert_eq!( s, "hello" );

  // Can also use AsRef<str> thanks to String: AsRef<str>
  let str_ref: &str = w.as_ref().as_ref();
  assert_eq!( str_ref, "hello" );
}

/// Test 2: Single-field named struct
///
/// Should automatically implement `AsRef` for the single field type.
#[ test ]
fn test_single_field_named()
{
  #[ derive( AsRef ) ]
  struct Container
  {
    value: i32
  }

  let c = Container { value: 42 };

  let v: &i32 = c.as_ref();
  assert_eq!( *v, 42 );
}

/// Test 3: Generic single-field struct
///
/// Should work with generic type parameters.
#[ test ]
fn test_generic_single_field()
{
  #[ derive( AsRef ) ]
  struct GenericWrapper<T>( T );

  let w: GenericWrapper<String> = GenericWrapper( "generic".to_string() );
  let s: &String = w.as_ref();
  assert_eq!( s, "generic" );

  let w2: GenericWrapper<Vec<i32>> = GenericWrapper( vec![ 1, 2, 3 ] );
  let v: &Vec<i32> = w2.as_ref();
  assert_eq!( v.len(), 3 );
}
