//! Comprehensive tests for `Not` derive macro.
//!
//! Tests all corner cases for the `#[derive(Not)]` macro including:
//! - Unit structs (identity Not)
//! - Single-field structs (forward Not to inner field)
//! - Generic types
//!
//! ## Test Matrix
//!
//! | Test Case | Struct Type | Fields | Inner Type | Expected | Status |
//! |-----------|-------------|--------|------------|----------|--------|
//! | `test_unit_struct` | Unit | 0 | N/A | Identity | ✅ |
//! | `test_single_field_tuple_bool` | Tuple | 1 | bool | `!inner` | ✅ |
//! | `test_single_field_named_bool` | Named | 1 | bool | `!inner` | ✅ |
//!
//! **Semantics:**
//! The `Not` derive implements `Not` for wrapper types.
//! - Unit struct: `!unit` returns `unit` (identity)
//! - Field struct: `!wrapper` returns `Wrapper(!wrapper.field)` (applies Not to inner, re-wraps)
//!
//! **Known Issues:**
//! - Generic types don't work without manual where clause (derive doesn't add `where T: Not`)

use derive_tools_meta::*;

/// Test 1: Unit struct
///
/// Should implement identity `Not` (returns self unchanged).
#[ test ]
fn test_unit_struct()
{
  #[ derive( Not, Debug, PartialEq ) ]
  struct Unit;

  let u = Unit;

  // Not on unit struct is identity
  let not_u = !u;
  assert_eq!( not_u, Unit );
}

/// Test 2: Single-field tuple struct with bool
///
/// Should apply `Not` to the inner bool and re-wrap.
#[ test ]
fn test_single_field_tuple_bool()
{
  #[ derive( Not, Debug, PartialEq ) ]
  struct Wrapper( bool );

  let w_true = Wrapper( true );
  let w_false = Wrapper( false );

  // Not applies to inner field and re-wraps
  assert_eq!( !w_true, Wrapper( false ) );
  assert_eq!( !w_false, Wrapper( true ) );
}

/// Test 3: Single-field named struct with bool
///
/// Should apply `Not` to the inner bool and re-wrap.
#[ test ]
fn test_single_field_named_bool()
{
  #[ derive( Not, Debug, PartialEq ) ]
  struct Container
  {
    value: bool
  }

  let c_true = Container { value: true };
  let c_false = Container { value: false };

  assert_eq!( !c_true, Container { value: false } );
  assert_eq!( !c_false, Container { value: true } );
}
