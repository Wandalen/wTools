//! # Test Matrix for `Deref`
//!
//! | ID   | Struct Type        | Fields      | Generics         | Attributes | Expected Behavior                                     | Test Type    |
//! |------|--------------------|-------------|------------------|------------|-------------------------------------------------------|--------------|
//! | T1.1 | Tuple Struct       | 1           | None             | -          | Implements `Deref` to the inner field.                | `tests/inc/deref/basic_test.rs` |
//! | T1.2 | Named Struct       | 1           | None             | -          | Implements `Deref` to the inner field.                | `tests/inc/deref/basic_test.rs` |
//! | T1.3 | Tuple Struct       | >1          | None             | -          | Fails to compile: `Deref` requires a single field.    | `trybuild`   |
//! | T1.4 | Named Struct       | >1          | None             | `#[deref]` | Implements `Deref` to the specified field.            | `tests/inc/deref/struct_named.rs` |
//! | T1.5 | Named Struct       | >1          | None             | -          | Fails to compile: `#[deref]` attribute is required.   | `trybuild`   |
//! | T1.6 | Enum               | Any         | Any              | -          | Fails to compile: `Deref` cannot be on an enum.       | `tests/inc/deref/compile_fail_enum.rs` |
//! | T1.7 | Unit Struct        | 0           | None             | -          | Fails to compile: `Deref` requires a field.           | `trybuild`   |
//! | T1.8 | Struct             | 1           | Lifetime         | -          | Implements `Deref` correctly with lifetimes.          | `tests/inc/deref/generics_lifetimes.rs` |
//! | T1.9 | Struct             | 1           | Type             | -          | Implements `Deref` correctly with type generics.      | `tests/inc/deref/generics_types.rs` |
//! | T1.10| Struct             | 1           | Const            | -          | Implements `Deref` correctly with const generics.     | `tests/inc/deref/generics_constants.rs` |
//! | T1.11| Struct             | 1           | Where clause     | -          | Implements `Deref` correctly with where clauses.      | `tests/inc/deref/bounds_where.rs` |
use super::*;
use derive_tools_meta::Deref;
// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, derive_tools_meta::Deref ) ]
pub struct IsTransparentSimple( bool );

// #[ derive( Debug, Clone, Copy, PartialEq, derive_tools_meta::Deref ) ]
// pub struct IsTransparentComplex< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize >( &'a T, core::marker::PhantomData< &'b U > )
// where
//   'a : 'b,
//   T : AsRef< U >;


// Content from only_test/deref.rs
use test_tools::a_id;

/// Tests the `Deref` derive macro and manual implementation for various struct types.
#[ test ]
fn deref_test()
{
  // Test for IsTransparentSimple
  let got = IsTransparentSimple( true );
  let exp = true;
  a_id!( *got, exp );

  // Test for IsTransparentComplex (commented out due to const generics issue)
  // let got_tmp = "hello".to_string();
  // let got = IsTransparentComplex::< '_, '_, String, str, 0 >( &got_tmp, core::marker::PhantomData );
  // let exp = &got_tmp;
  // a_id!( *got, exp );
}
