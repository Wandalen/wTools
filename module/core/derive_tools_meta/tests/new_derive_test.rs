//! Comprehensive tests for New derive macro.
//!
//! Tests all corner cases for the `#[derive(New)]` macro including:
//! - Unit structs (`new()` → Self)
//! - Single-field named structs (`new(T)` → Self)
//! - Multi-field named structs (`new(T1, T2, ...)` → Self)
//! - Generic types
//!
//! **Implementation Limitations:**
//! - Only supports named structs (NOT tuple structs)
//! - Tuple structs cause panic: "Expected named field"
//!
//! ## Test Matrix
//!
//! | Test Case | Struct Type | Fields | Expected Signature | Status |
//! |-----------|-------------|--------|-------------------|--------|
//! | `test_unit_struct` | Unit | 0 | `new() → Self` | ✅ |
//! | `test_single_field_named` | Named | 1 | `new(T) → Self` | ✅ |
//! | `test_multi_field_named` | Named | 2+ | `new(T1, T2) → Self` | ✅ |
//! | `test_generic_type` | Named Generic | 1+ | `new<T>(T) → Self<T>` | ✅ |

use derive_tools_meta::*;

/// Test 1: Unit struct should generate `new() -> Self`
///
/// Zero-argument constructor for unit structs.
#[ test ]
fn test_unit_struct()
{
  #[ derive( New ) ]
  struct Empty;

  let _e = Empty::new();
  // Compilation success is the assertion
}

/// Test 2: Single-field named struct
///
/// Should generate `new(T) -> Self` with named field.
#[ test ]
fn test_single_field_named()
{
  #[ derive( New ) ]
  struct Container
  {
    value: i32
  }

  let c = Container::new( 42 );
  assert_eq!( c.value, 42 );
}

/// Test 3: Multi-field named struct
///
/// Should generate `new(T1, T2, T3) -> Self` matching field order.
#[ test ]
fn test_multi_field_named()
{
  #[ derive( New ) ]
  struct Point3D
  {
    x: i32,
    y: i32,
    z: i32,
  }

  let p = Point3D::new( 1, 2, 3 );
  assert_eq!( p.x, 1 );
  assert_eq!( p.y, 2 );
  assert_eq!( p.z, 3 );
}

/// Test 4: Generic type parameter
///
/// Should work with generic constructors.
#[ test ]
fn test_generic_type()
{
  #[ derive( New ) ]
  struct GenericWrapper<T>
  {
    inner: T
  }

  let w: GenericWrapper<String> = GenericWrapper::new( "generic".to_string() );
  assert_eq!( w.inner, "generic" );

  let w2: GenericWrapper<Vec<i32>> = GenericWrapper::new( vec![ 1, 2, 3 ] );
  assert_eq!( w2.inner.len(), 3 );
}
