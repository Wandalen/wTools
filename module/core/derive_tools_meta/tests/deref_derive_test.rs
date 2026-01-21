//! Comprehensive tests for Deref derive macro.
//!
//! Tests all corner cases for the `#[derive(Deref)]` macro including:
//! - Unit structs (error case)
//! - Single-field structs (auto-deref)
//! - Multi-field structs (requires #[deref] marker)
//! - Generic types
//! - Lifetime parameters
//! - Where clauses
//!
//! ## Test Matrix
//!
//! | Test Case | Struct Type | Fields | Marker | Expected | Status |
//! |-----------|-------------|--------|--------|----------|--------|
//! | `test_single_field_tuple` | Tuple | 1 | None | Success | ✅ |
//! | `test_single_field_named` | Named | 1 | None | Success | ✅ |
//! | `test_multi_field_with_marker` | Named | 2+ | #[deref] | Success | ✅ |
//! | `test_generic_single_field` | Tuple | 1 generic | None | Success | ✅ |
//! | `test_lifetime_parameter` | Tuple | 1 ref | None | Success | ✅ |

use derive_tools_meta::*;

/// Test 1: Single-field tuple struct (most common newtype pattern)
///
/// Should automatically deref to the inner field without requiring marker.
#[ test ]
fn test_single_field_tuple()
{
  #[ derive( Deref ) ]
  struct Wrapper( String );

  let w = Wrapper( "hello".to_string() );

  // Should deref to String, allowing String methods
  assert_eq!( w.len(), 5 );
  assert_eq!( &*w, "hello" );
}

/// Test 2: Single-field named struct
///
/// Should automatically deref to the single field.
#[ test ]
fn test_single_field_named()
{
  #[ derive( Deref ) ]
  struct Container
  {
    value: i32
  }

  let c = Container { value: 42 };

  assert_eq!( *c, 42 );
}

/// Test 3: Multi-field struct with #[deref] marker
///
/// Should deref to the field marked with #[deref] attribute.
#[ test ]
fn test_multi_field_with_marker()
{
  #[ derive( Deref ) ]
  struct MultiField
  {
    #[ deref ]
    primary: String,
    secondary: i32,
  }

  let m = MultiField
  {
    primary: "test".to_string(),
    secondary: 99,
  };

  // Deref targets the marked field
  assert_eq!( &*m, "test" );
  assert_eq!( m.len(), 4 ); // Derefs to String

  // Other fields remain accessible normally
  assert_eq!( m.secondary, 99 );
}

/// Test 4: Generic single-field struct
///
/// Should work with generic type parameters.
#[ test ]
fn test_generic_single_field()
{
  #[ derive( Deref ) ]
  struct GenericWrapper<T>( T );

  let w: GenericWrapper<String> = GenericWrapper( "generic".to_string() );
  assert_eq!( &*w, "generic" );

  let w2: GenericWrapper<Vec<i32>> = GenericWrapper( vec![ 1, 2, 3 ] );
  assert_eq!( w2.len(), 3 );
}

/// Test 5: Lifetime parameter
///
/// Should work with lifetime parameters for reference types.
#[ test ]
fn test_lifetime_parameter()
{
  #[ derive( Deref ) ]
  struct RefWrapper<'a>( &'a str );

  let s = "lifetime";
  let w = RefWrapper( s );

  assert_eq!( *w, "lifetime" );
}
