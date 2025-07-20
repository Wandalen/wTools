// tests/inc/derive_test.rs

//! ## Test Matrix for `VariadicFrom` Derive Macro
//!
//! This matrix outlines the test cases for the `#[derive(VariadicFrom)]` macro, covering various struct types, field counts, and type identity conditions.
//!
//! **Test Factors:**
//! - Struct Type: Named struct (`struct Named { a: i32, b: i32 }`) vs. Tuple struct (`struct Tuple(i32, i32)`).
//! - Field Count: 1, 2, or 3 fields.
//! - Field Type Identity: Whether all fields have identical types, or if a subset (e.g., last two) have identical types.
//! - Generics: Presence and handling of generic parameters.
//!
//! **Test Combinations:**
//!
//! | ID    | Struct Type | Field Count | Field Types                               | Expected `FromN` Impls | Expected `From<Tuple>` Impls | Expected Convenience Impls | Notes                                                              |
//! |-------|-------------|-------------|-------------------------------------------|------------------------|------------------------------|----------------------------|--------------------------------------------------------------------|
//! | T1.1  | Named       | 1           | `i32`                                     | `From1<i32>`           | `From<i32>`                  | N/A                        | Basic 1-field named struct.                                        |
//! | T1.2  | Tuple       | 1           | `i32`                                     | `From1<i32>`           | `From<i32>`                  | N/A                        | Basic 1-field tuple struct.                                        |
//! | T2.1  | Named       | 2           | `i32`, `i32`                              | `From2<i32, i32>`      | `From<(i32, i32)>`           | `From1<i32>`               | 2-field named struct with identical types.                         |
//! | T2.2  | Tuple       | 2           | `i32`, `i32`                              | `From2<i32, i32>`      | `From<(i32, i32)>`           | `From1<i32>`               | 2-field tuple struct with identical types.                         |
//! | T2.3  | Named       | 2           | `i32`, `String`                           | `From2<i32, String>`   | `From<(i32, String)>`        | N/A                        | 2-field named struct with different types.                         |
//! | T2.4  | Tuple       | 2           | `i32`, `String`                           | `From2<i32, String>`   | `From<(i32, String)>`        | N/A                        | 2-field tuple struct with different types.                         |
//! | T3.1  | Named       | 3           | `i32`, `i32`, `i32`                       | `From3<i32, i32, i32>` | `From<(i32, i32, i32)>`      | `From1<i32>`, `From2<i32, i32>` | 3-field named struct with all identical types.                     |
//! | T3.2  | Tuple       | 3           | `i32`, `i32`, `i32`                       | `From3<i32, i32, i32>` | `From<(i32, i32, i32)>`      | `From1<i32>`, `From2<i32, i32>` | 3-field tuple struct with all identical types.                     |
//! | T3.3  | Named       | 3           | `i32`, `i32`, `String`                    | `From3<i32, i32, String>` | `From<(i32, i32, String)>`   | N/A                        | 3-field named struct with last field different.                    |
//! | T3.4  | Tuple       | 3           | `i32`, `i32`, `String`                    | `From3<i32, i32, String>` | `From<(i32, i32, String)>`   | N/A                        | 3-field tuple struct with last field different.                    |
//! | T3.5  | Named       | 3           | `i32`, `String`, `String`                 | `From3<i32, String, String>` | `From<(i32, String, String)>` | `From2<i32, String>`       | 3-field named struct with last two fields identical.               |
//! | T3.6  | Tuple       | 3           | `i32`, `String`, `String`                 | `From3<i32, String, String>` | `From<(i32, String, String)>` | `From2<i32, String>`       | 3-field tuple struct with last two fields identical.               |
//! | T4.1  | Named       | 1           | `T` (generic)                             | `From1<T>`             | `From<T>`                    | N/A                        | 1-field named struct with generic type.                            |
//! | T4.2  | Tuple       | 2           | `T`, `U` (generic)                        | `From2<T, U>`          | `From<(T, U)>`               | N/A                        | 2-field tuple struct with generic types.                           |
//!
//! **Compile-Fail Test Combinations:**
//!
//! | ID    | Struct Type | Field Count | Expected Error                               | Notes                                                              |
//! |-------|-------------|-------------|----------------------------------------------|--------------------------------------------------------------------|
//! | C5.1  | Named       | 0           | "VariadicFrom can only be derived for structs with 1, 2, or 3 fields." | Struct with no fields should fail.                                 |
//! | C5.2  | Named       | 4           | "VariadicFrom can only be derived for structs with 1, 2, or 3 fields." | Struct with more than 3 fields should fail.                        |
//! | C5.3  | N/A         | N/A         | "Too many arguments"                         | `from!` macro invoked with too many arguments.                     |
//!

#![ allow( unused_imports ) ]
use super::*;
use variadic_from::exposed::*;
use variadic_from_meta::VariadicFrom;

// Phase 1: Foundation & Simplest Case (1-Field Structs)

/// Tests a named struct with 1 field.
/// Test Combination: T1.1
#[ test ]
fn test_named_struct_1_field()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test1
  {
    a : i32,
  }

  let x = Test1::from1( 10 );
  assert_eq!( x, Test1 { a : 10 } );

  let x = Test1::from( 20 );
  assert_eq!( x, Test1 { a : 20 } );
}

/// Tests a tuple struct with 1 field.
/// Test Combination: T1.2
#[ test ]
fn test_tuple_struct_1_field()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test2( i32 );

  let x = Test2::from1( 10 );
  assert_eq!( x, Test2( 10 ) );

  let x = Test2::from( 20 );
  assert_eq!( x, Test2( 20 ) );
}

// Phase 2: Two-Field Structs

/// Tests a named struct with 2 identical fields.
/// Test Combination: T2.1
#[ test ]
fn test_named_struct_2_identical_fields()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test3
  {
    a : i32,
    b : i32,
  }

  let x = Test3::from2( 10, 20 );
  assert_eq!( x, Test3 { a : 10, b : 20 } );

  let x = Test3::from( ( 30, 40 ) );
  assert_eq!( x, Test3 { a : 30, b : 40 } );

  // Test convenience From1
  let x = Test3::from1( 50 );
  assert_eq!( x, Test3 { a : 50, b : 50 } );
}

/// Tests a tuple struct with 2 identical fields.
/// Test Combination: T2.2
#[ test ]
fn test_tuple_struct_2_identical_fields()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test4( i32, i32 );

  let x = Test4::from2( 10, 20 );
  assert_eq!( x, Test4( 10, 20 ) );

  let x = Test4::from( ( 30, 40 ) );
  assert_eq!( x, Test4( 30, 40 ) );

  // Test convenience From1
  let x = Test4::from1( 50 );
  assert_eq!( x, Test4( 50, 50 ) );
}

/// Tests a named struct with 2 different fields.
/// Test Combination: T2.3
#[ test ]
fn test_named_struct_2_different_fields()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test5
  {
    a : i32,
    b : String,
  }

  let x = Test5::from2( 10, "hello".to_string() );
  assert_eq!( x, Test5 { a : 10, b : "hello".to_string() } );

  let x = Test5::from( ( 20, "world".to_string() ) );
  assert_eq!( x, Test5 { a : 20, b : "world".to_string() } );

  // No convenience From1 expected
  // let x = Test5::from1( 50 ); // Should not compile
}

/// Tests a tuple struct with 2 different fields.
/// Test Combination: T2.4
#[ test ]
fn test_tuple_struct_2_different_fields()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test6( i32, String );

  let x = Test6::from2( 10, "hello".to_string() );
  assert_eq!( x, Test6( 10, "hello".to_string() ) );

  let x = Test6::from( ( 20, "world".to_string() ) );
  assert_eq!( x, Test6( 20, "world".to_string() ) );

  // No convenience From1 expected
  // let x = Test6::from1( 50 ); // Should not compile
}

// Phase 3: Three-Field Structs

/// Tests a named struct with 3 identical fields.
/// Test Combination: T3.1
#[ test ]
fn test_named_struct_3_identical_fields()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test7
  {
    a : i32,
    b : i32,
    c : i32,
  }

  let x = Test7::from3( 10, 20, 30 );
  assert_eq!( x, Test7 { a : 10, b : 20, c : 30 } );

  let x = Test7::from( ( 40, 50, 60 ) );
  assert_eq!( x, Test7 { a : 40, b : 50, c : 60 } );

  // Test convenience From1
  let x = Test7::from1( 70 );
  assert_eq!( x, Test7 { a : 70, b : 70, c : 70 } );

  // Test convenience From2
  let x = Test7::from2( 80, 90 );
  assert_eq!( x, Test7 { a : 80, b : 90, c : 90 } );
}

/// Tests a tuple struct with 3 identical fields.
/// Test Combination: T3.2
#[ test ]
fn test_tuple_struct_3_identical_fields()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test8( i32, i32, i32 );

  let x = Test8::from3( 10, 20, 30 );
  assert_eq!( x, Test8( 10, 20, 30 ) );

  let x = Test8( 40, 50, 60 );
  assert_eq!( x, Test8( 40, 50, 60 ) );

  // Test convenience From1
  let x = Test8::from1( 70 );
  assert_eq!( x, Test8( 70, 70, 70 ) );

  // Test convenience From2
  let x = Test8::from2( 80, 90 );
  assert_eq!( x, Test8( 80, 90, 90 ) );
}

/// Tests a named struct with 3 fields, last one different.
/// Test Combination: T3.3
#[ test ]
fn test_named_struct_3_fields_last_different()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test9
  {
    a : i32,
    b : i32,
    c : String,
  }

  let x = Test9::from3( 10, 20, "hello".to_string().clone() );
  assert_eq!( x, Test9 { a : 10, b : 20, c : "hello".to_string() } );

  let x = Test9::from( ( 30, 40, "world".to_string().clone() ) );
  assert_eq!( x, Test9 { a : 30, b : 40, c : "world".to_string() } );

  // No convenience From1 or From2 expected
  // let x = Test9::from1( 50 ); // Should not compile
}

/// Tests a tuple struct with 3 fields, last one different.
/// Test Combination: T3.4
#[ test ]
fn test_tuple_struct_3_fields_last_different()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test10( i32, i32, String );

  let x = Test10::from3( 10, 20, "hello".to_string().clone() );
  assert_eq!( x, Test10( 10, 20, "hello".to_string() ) );

  let x = Test10::from( ( 30, 40, "world".to_string().clone() ) );
  assert_eq!( x, Test10( 30, 40, "world".to_string() ) );

  // No convenience From1 or From2 expected
  // let x = Test10::from1( 50 ); // Should not compile
}

/// Tests a named struct with 3 fields, last two identical.
/// Test Combination: T3.5
#[ test ]
fn test_named_struct_3_fields_last_two_identical()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test11
  {
    a : i32,
    b : String,
    c : String,
  }

  let x = Test11::from3( 10, "a".to_string().clone(), "b".to_string().clone() );
  assert_eq!( x, Test11 { a : 10, b : "a".to_string(), c : "b".to_string() } );

  let x = Test11::from( ( 20, "c".to_string().clone(), "d".to_string().clone() ) );
  assert_eq!( x, Test11 { a : 20, b : "c".to_string(), c : "d".to_string() } );

  // Test convenience From2
  let x = Test11::from2( 30, "e".to_string().clone() );
  assert_eq!( x, Test11 { a : 30, b : "e".to_string(), c : "e".to_string() } );

  // No convenience From1 expected
  // let x = Test11::from1( 50 ); // Should not compile
}

/// Tests a tuple struct with 3 fields, last two identical.
/// Test Combination: T3.6
#[ test ]
fn test_tuple_struct_3_fields_last_two_identical()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test12( i32, String, String );
  
  let x = Test12::from3( 10, "a".to_string().clone(), "b".to_string().clone() );
  assert_eq!( x, Test12( 10, "a".to_string(), "b".to_string() ) );

  let x = Test12::from( ( 20, "c".to_string().clone(), "d".to_string().clone() ) );
  assert_eq!( x, Test12( 20, "c".to_string(), "d".to_string() ) );

  // Test convenience From2
  let x = Test12::from2( 30, "e".to_string().clone() );
  assert_eq!( x, Test12( 30, "e".to_string(), "e".to_string() ) );

  // No convenience From1 expected
  // let x = Test12::from1( 50 ); // Should not compile
}

// Phase 4: Generic Structs

/// Tests a named struct with 1 generic field.
/// Test Combination: T4.1
#[ test ]
fn test_named_struct_1_generic_field()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test13< T >
  where
    T : Clone + core::fmt::Debug + PartialEq,
  {
    a : T,
  }

  let x = Test13::from1( 10 );
  assert_eq!( x, Test13 { a : 10 } );

  let x = Test13::from( 20 );
  assert_eq!( x, Test13 { a : 20 } );

  let x = Test13::from1( "hello".to_string() );
  assert_eq!( x, Test13 { a : "hello".to_string() } );
}

/// Tests a tuple struct with 2 generic fields.
/// Test Combination: T4.2
#[ test ]
fn test_tuple_struct_2_generic_fields()
{
  #[ derive( VariadicFrom, Debug, PartialEq ) ]
  struct Test14< T, U >
  where
    T : Clone + core::fmt::Debug + PartialEq,
    U : Clone + core::fmt::Debug + PartialEq,
  ( T, U ) : Into< ( T, U ) >,
  {
    a : T,
    b : U,
  }

  let x = Test14::from2( 10, "hello" );
  assert_eq!( x, Test14 { a : 10, b : "hello" } );

  let x = Test14::from( ( 20, "world" ) );
  assert_eq!( x, Test14 { a : 20, b : "world" } );
}