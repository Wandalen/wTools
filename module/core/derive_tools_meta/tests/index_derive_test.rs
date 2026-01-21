//! Comprehensive tests for `Index` derive macro.
//!
//! Tests all corner cases for the `#[derive(Index)]` macro including:
//! - Single-field structs (auto Index)
//! - Multi-field structs (requires `#[index]` marker)
//! - Generic types
//!
//! ## Test Matrix
//!
//! | Test Case | Struct Type | Fields | Marker | Expected | Status |
//! |-----------|-------------|--------|--------|----------|--------|
//! | `test_single_field_tuple` | Tuple | 1 | None | Success | ✅ |
//! | `test_single_field_named` | Named | 1 | None | Success | ✅ |
//! | `test_multi_field_with_marker` | Named | 2+ | `#[index]` | Success | ✅ |
//! | `test_generic_single_field` | Tuple | 1 generic | None | Success | ✅ |
//!
//! **Semantics:**
//! The `Index` derive provides syntactic sugar for accessing the wrapper's field.
//! `wrapper[idx]` returns `&wrapper.field` - the index parameter is ignored.
//! This is NOT indexing INTO the field, but accessing the field itself.

use derive_tools_meta::*;

/// Test 1: Single-field tuple struct
///
/// Should automatically implement `Index` to access the inner field.
#[ test ]
fn test_single_field_tuple()
{
  #[ derive( Index ) ]
  struct Wrapper( String );

  let w = Wrapper( "hello".to_string() );

  // Index provides syntactic sugar to access the field
  // Note: index parameter is ignored, always returns &self.0
  assert_eq!( &w[ 0 ], "hello" );
  assert_eq!( &w[ 999 ], "hello" ); // Same result for any index
}

/// Test 2: Single-field named struct
///
/// Should automatically implement `Index` to access the single field.
#[ test ]
fn test_single_field_named()
{
  #[ derive( Index ) ]
  struct Container
  {
    value: i32
  }

  let c = Container { value: 42 };

  assert_eq!( c[ 0 ], 42 );
}

/// Test 3: Multi-field struct with `#[index]` marker
///
/// Should implement `Index` to access the field marked with `#[index]` attribute.
#[ test ]
fn test_multi_field_with_marker()
{
  #[ derive( Index ) ]
  struct MultiField
  {
    #[ index ]
    primary: String,
    secondary: i32,
  }

  let m = MultiField
  {
    primary: "test".to_string(),
    secondary: 99,
  };

  // Index targets the marked field
  assert_eq!( &m[ 0 ], "test" );

  // Other fields remain accessible normally
  assert_eq!( m.secondary, 99 );
}

/// Test 4: Generic single-field struct
///
/// Should work with generic type parameters.
#[ test ]
fn test_generic_single_field()
{
  #[ derive( Index ) ]
  struct GenericWrapper<T>( T );

  let w: GenericWrapper<String> = GenericWrapper( "generic".to_string() );
  assert_eq!( &w[ 0 ], "generic" );

  let w2: GenericWrapper<Vec<i32>> = GenericWrapper( vec![ 1, 2, 3 ] );
  assert_eq!( w2[ 0 ].len(), 3 );
}
