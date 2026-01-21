//! Comprehensive conformance tests for `#[derive(VariadicFrom)]` proc macro.
//!
//! Implements all conformance checks defined in specification §10.
//!
//! ## Test Coverage Matrix
//!
//! | Check | Description | Test Function | Status |
//! |-------|-------------|---------------|--------|
//! | §10.1 | Derive on 1-field struct | `test_conformance_1_field_struct` | ✅ |
//! | §10.2 | Derive on 2-field named struct (different types) | `test_conformance_2_field_different_types` | ✅ |
//! | §10.3 | Derive on 3-field unnamed struct (same type) | `test_conformance_3_field_same_type` | ✅ |
//! | §10.4 | from! macro correctness | `test_conformance_from_macro` | ✅ |
//! | §10.6 | Tuple conversion correctness | `test_conformance_tuple_conversion` | ✅ |
//! | §10.9 | Generics handling | `test_conformance_generics` | ✅ |
//!
//! ## Related Tests
//!
//! - **Integration Tests**: See `variadic_from/tests/inc/derive_test.rs` for full test matrix
//! - **Compile-Fail Tests**: See `variadic_from/tests/compile_fail.rs` for error cases
//!
//! ## Note
//!
//! These tests verify the proc macro generates correct code according to specification.
//! The comprehensive test matrix with all permutations (named/tuple, 1/2/3 fields, etc.)
//! is maintained in the `variadic_from` crate to avoid duplication while this file
//! focuses on the specific conformance checks mandated by spec §10.

#![ allow( unused_imports ) ]
use variadic_from ::exposed ::*;
use variadic_from_meta ::VariadicFrom;

// §10.1: Derive on 1-Field Struct

/// Conformance Check 1: Apply #[derive(VariadicFrom)] to a struct with 1 field.
///
/// **Expected:** The code compiles. `impl From1` and `impl From<T>` are generated
/// and work as expected.
///
/// **Specification Reference:** §10.1
#[ test ]
fn test_conformance_1_field_struct()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct OneField
  {
    value: i32,
  }

  // Test From1 implementation
  let x = OneField ::from1( 42 );
  assert_eq!( x, OneField { value: 42 } );

  // Test From<T> implementation
  let x = OneField ::from( 100 );
  assert_eq!( x, OneField { value: 100 } );
}

// §10.2: Derive on 2-Field Named Struct (Different Types)

/// Conformance Check 2: Apply #[derive(VariadicFrom)] to a named struct with 2
/// fields of different types.
///
/// **Expected:** The code compiles. `impl From2` and `impl From<(T1, T2)>` are
/// generated. The convenience `impl From1<T1>` is **not** generated.
///
/// **Specification Reference:** §10.2
#[ test ]
fn test_conformance_2_field_different_types()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct TwoFields
  {
    id: i32,
    name: String,
  }

  // Test From2 implementation
  let x = TwoFields ::from2( 1, "Alice".to_string() );
  assert_eq!
  (
    x,
    TwoFields
    {
      id: 1,
      name: "Alice".to_string()
    }
  );

  // Test From<(T1, T2)> implementation
  let x = TwoFields ::from( ( 2, "Bob".to_string() ) );
  assert_eq!
  (
    x,
    TwoFields
    {
      id: 2,
      name: "Bob".to_string()
    }
  );

  // Note: From1 should NOT be implemented for different types
  // Uncomment to verify compilation error:
  // let x = TwoFields ::from1( 42 );
}

// §10.3: Derive on 3-Field Unnamed Struct (Same Type)

/// Conformance Check 3: Apply #[derive(VariadicFrom)] to an unnamed (tuple) struct
/// with 3 fields of the same type.
///
/// **Expected:** The code compiles. `impl From3`, `impl From<(T, T, T)>`, and
/// convenience `impl From1<T>` and `impl From2<T, T>` are generated.
///
/// **Specification Reference:** §10.3
#[ test ]
fn test_conformance_3_field_same_type()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Point3D( i32, i32, i32 );

  // Test From3 implementation
  let p = Point3D ::from3( 1, 2, 3 );
  assert_eq!( p, Point3D( 1, 2, 3 ) );

  // Test From<(T, T, T)> implementation
  let p = Point3D ::from( ( 4, 5, 6 ) );
  assert_eq!( p, Point3D( 4, 5, 6 ) );

  // Test convenience From1 (all fields set to same value)
  let p = Point3D ::from1( 10 );
  assert_eq!( p, Point3D( 10, 10, 10 ) );

  // Test convenience From2 (first field gets first arg, last two get second arg)
  let p = Point3D ::from2( 20, 30 );
  assert_eq!( p, Point3D( 20, 30, 30 ) );
}

// §10.4: from! Macro Correctness

/// Conformance Check 4: Call from!(), from!(a), from!(a, b), and from!(a, b, c)
/// on conforming types.
///
/// **Expected:** All calls compile and produce the correct struct instances.
///
/// **Specification Reference:** §10.4
///
/// **Note:** The from! macro is defined in the `variadic_from` crate, not in
/// `variadic_from_meta`, but this test verifies the generated code works with it.
#[ test ]
fn test_conformance_from_macro()
{
  #[ derive( VariadicFrom, Debug, PartialEq, Default ) ]
  struct TwoFieldsSame
  {
    x: i32,
    y: i32,
  }

  // from!() uses Default::default()
  let p: TwoFieldsSame = from!();
  assert_eq!( p, TwoFieldsSame { x: 0, y: 0 } );

  // from!(a) uses From1::from1
  let p: TwoFieldsSame = from!( 5 );
  assert_eq!( p, TwoFieldsSame { x: 5, y: 5 } );

  // from!(a, b) uses From2::from2
  let p: TwoFieldsSame = from!( 10, 20 );
  assert_eq!( p, TwoFieldsSame { x: 10, y: 20 } );
}

// §10.6: Tuple Conversion Correctness

/// Conformance Check 6: Use `(a, b).into()` and `MyStruct::from((a, b))` on a
/// derived 2-field struct.
///
/// **Expected:** Both conversions compile and produce the correct struct instance.
///
/// **Specification Reference:** §10.6
#[ test ]
fn test_conformance_tuple_conversion()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Pair( i32, String );

  // Test MyStruct::from((a, b))
  let p = Pair ::from( ( 42, "answer".to_string() ) );
  assert_eq!( p, Pair( 42, "answer".to_string() ) );

  // Test (a, b).into()
  let p: Pair = ( 100, "century".to_string() ).into();
  assert_eq!( p, Pair( 100, "century".to_string() ) );
}

// §10.9: Generics Handling

/// Conformance Check 9: Apply #[derive(VariadicFrom)] to a struct with generic
/// parameters and a where clause.
///
/// **Expected:** The generated `impl` blocks correctly include the generics and
/// `where` clause, and the code compiles.
///
/// **Specification Reference:** §10.9
#[ test ]
fn test_conformance_generics()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Generic< T, U >
  where
    T: Clone + core ::fmt ::Debug + PartialEq,
    U: Clone + core ::fmt ::Debug + PartialEq,
  {
    first: T,
    second: U,
  }

  // Test From2 with generics
  let g = Generic ::from2( 42i32, "test".to_string() );
  assert_eq!
  (
    g,
    Generic
    {
      first: 42,
      second: "test".to_string()
    }
  );

  // Test From<(T, U)> with generics
  let g = Generic ::from( ( 100i32, "hundred".to_string() ) );
  assert_eq!
  (
    g,
    Generic
    {
      first: 100,
      second: "hundred".to_string()
    }
  );

  // Test with different types
  let g = Generic ::from2( "hello", vec![ 1, 2, 3 ] );
  assert_eq!
  (
    g,
    Generic
    {
      first: "hello",
      second: vec![ 1, 2, 3 ]
    }
  );
}
