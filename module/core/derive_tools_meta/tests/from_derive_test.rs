//! Comprehensive tests for From derive macro.
//!
//! Tests all corner cases for the `#[derive(From)]` macro including:
//! - Single-field structs (auto From conversion)
//! - Multi-field **tuple** structs (requires `#[from]` marker, uses Default for others)
//! - Generic types
//! - Named and tuple structs
//!
//! **Implementation Limitations:**
//! - Multi-field **named** structs NOT supported (implementation bug - only initializes marked field)
//! - Multi-field tuple structs require other fields to implement Default
//!
//! ## Test Matrix
//!
//! | Test Case | Struct Type | Fields | Marker | Expected | Status |
//! |-----------|-------------|--------|--------|----------|--------|
//! | `test_single_field_tuple` | Tuple | 1 | None | Success | ✅ |
//! | `test_single_field_named` | Named | 1 | None | Success | ✅ |
//! | `test_multi_field_tuple` | Tuple | 2+ | `#[from]` | Success | ✅ |
//! | `test_generic_single_field` | Tuple | 1 generic | None | Success | ✅ |

use derive_tools_meta::*;

/// Test 1: Single-field tuple struct (most common newtype pattern)
///
/// Should automatically implement `From<T>` for single-field wrapper.
#[ test ]
fn test_single_field_tuple()
{
  #[ derive( From, Debug, PartialEq ) ]
  struct Wrapper( String );

  let w = Wrapper::from( "hello".to_string() );
  assert_eq!( w.0, "hello" );
}

/// Test 2: Single-field named struct
///
/// Should automatically implement `From<T>` for single named field.
#[ test ]
fn test_single_field_named()
{
  #[ derive( From ) ]
  struct Container
  {
    value: i32
  }

  let c = Container::from( 42 );
  assert_eq!( c.value, 42 );
}

/// Test 3: Multi-field tuple struct with `#[from]` marker
///
/// Should implement From for the marked field in tuple struct.
/// Other fields are initialized with `Default::default()`.
#[ test ]
fn test_multi_field_tuple()
{
  #[ derive( From ) ]
  struct TupleMulti( #[ from ] String, i32 );

  let t = TupleMulti::from( "test".to_string() );
  assert_eq!( t.0, "test" );
  assert_eq!( t.1, 0 ); // Default value for i32
}

/// Test 4: Generic single-field struct
///
/// Should work with generic type parameters.
#[ test ]
fn test_generic_single_field()
{
  #[ derive( From, Debug, PartialEq ) ]
  struct GenericWrapper<T>( T );

  let w: GenericWrapper<String> = GenericWrapper::from( "generic".to_string() );
  assert_eq!( w.0, "generic" );

  let w2: GenericWrapper<Vec<i32>> = GenericWrapper::from( vec![ 1, 2, 3 ] );
  assert_eq!( w2.0.len(), 3 );
}
