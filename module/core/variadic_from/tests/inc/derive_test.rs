//! # Test Matrix for `VariadicFrom` Derive
//!
//! This file contains comprehensive tests for the `VariadicFrom` derive macro,
//! covering various scenarios as defined in `spec.md`.
//!
//! | ID   | Struct Type | Fields | Field Types | Generics | Expected Behavior |
//! |------|-------------|--------|-------------|----------|-------------------|
//! | T1.1 | Named       | 1      | `i32`       | None     | Implements `From<i32>` and `From1<i32>` |
//! | T1.2 | Tuple       | 1      | `String`    | None     | Implements `From<String>` and `From1<String>` |
//! | T2.1 | Named       | 2      | `i32, i32`  | None     | Implements `From<(i32, i32)>`, `From2<i32, i32>`, and `From1<i32>` |
//! | T2.2 | Tuple       | 2      | `u8, u8`    | None     | Implements `From<(u8, u8)>`, `From2<u8, u8>`, and `From1<u8>` |
//! | T2.3 | Named       | 2      | `i32, String` | None   | Implements `From<(i32, String)>`, `From2<i32, String>`. No `From1`. |
//! | T2.4 | Tuple       | 2      | `bool, f32` | None     | Implements `From<(bool, f32)>`, `From2<bool, f32>`. No `From1`. |
//! | T3.1 | Named       | 3      | `i32, i32, i32` | None | Implements `From<(i32,i32,i32)>`, `From3`, `From2`, `From1` |
//! | T3.2 | Tuple       | 3      | `u8, u8, u8` | None    | Implements `From<(u8,u8,u8)>`, `From3`, `From2`, `From1` |
//! | T3.3 | Named       | 3      | `i32, i32, String` | None | Implements `From<(i32,i32,String)>`, `From3`. No `From2`, `From1`. |
//! | T3.4 | Tuple       | 3      | `bool, f32, f32` | None | Implements `From<(bool,f32,f32)>`, `From3`, `From2`. No `From1`. |
//! | T4.1 | Named       | 1      | `T`         | `T: Debug` | Implements `From<T>`, `From1<T>` with generics. |
//! | T4.2 | Tuple       | 2      | `T, U`      | `T: Copy, U: Clone` | Implements `From<(T,U)>`, `From2<T,U>` with generics. |
//!
//!
use variadic_from::VariadicFrom;
use variadic_from::exposed::*; // Import FromN traits
use variadic_from::from; // Import from! macro

// Test Combination: T1.1
/// Tests a named struct with 1 field.
#[ test ]
fn test_named_struct_1_field()
{
  #[ derive( VariadicFrom ) ]
  struct MyStruct
  {
    a : i32,
  }

  let x = MyStruct::from( 10 );
  assert_eq!( x.a, 10 );

  let x = from!( 20 );
  assert_eq!( x.a, 20 );

  let x = MyStruct::from1( 30 );
  assert_eq!( x.a, 30 );
}

// Test Combination: T1.2
/// Tests a tuple struct with 1 field.
#[ test ]
fn test_tuple_struct_1_field()
{
  #[ derive( VariadicFrom ) ]
  struct MyTuple( String );

  let x = MyTuple::from( "hello".to_string() );
  assert_eq!( x.0, "hello" );

  let x = from!( "world".to_string() );
  assert_eq!( x.0, "world" );

  let x = MyTuple::from1( "rust".to_string() );
  assert_eq!( x.0, "rust" );
}

// Test Combination: T2.1
/// Tests a named struct with 2 identical fields.
#[ test ]
fn test_named_struct_2_identical_fields()
{
  #[ derive( VariadicFrom ) ]
  struct MyStruct
  {
    a : i32,
    b : i32,
  }

  let x = MyStruct::from( ( 10, 20 ) );
  assert_eq!( x.a, 10 );
  assert_eq!( x.b, 20 );

  let x = from!( 30, 40 );
  assert_eq!( x.a, 30 );
  assert_eq!( x.b, 40 );

  let x = MyStruct::from2( 50, 60 );
  assert_eq!( x.a, 50 );
  assert_eq!( x.b, 60 );

  // Convenience From1
  let x = MyStruct::from1( 70 );
  assert_eq!( x.a, 70 );
  assert_eq!( x.b, 70 );
}

// Test Combination: T2.2
/// Tests a tuple struct with 2 identical fields.
#[ test ]
fn test_tuple_struct_2_identical_fields()
{
  #[ derive( VariadicFrom ) ]
  struct MyTuple( u8, u8 );

  let x = MyTuple::from( ( 10, 20 ) );
  assert_eq!( x.0, 10 );
  assert_eq!( x.1, 20 );

  let x = from!( 30, 40 );
  assert_eq!( x.0, 30 );
  assert_eq!( x.1, 40 );

  let x = MyTuple::from2( 50, 60 );
  assert_eq!( x.0, 50 );
  assert_eq!( x.1, 60 );

  // Convenience From1
  let x = MyTuple::from1( 70 );
  assert_eq!( x.0, 70 );
  assert_eq!( x.1, 70 );
}

// Test Combination: T2.3
/// Tests a named struct with 2 different fields.
#[ test ]
fn test_named_struct_2_different_fields()
{
  #[ derive( VariadicFrom ) ]
  struct MyStruct
  {
    a : i32,
    b : String,
  }

  let x = MyStruct::from( ( 10, "hello".to_string() ) );
  assert_eq!( x.a, 10 );
  assert_eq!( x.b, "hello" );

  let x = from!( 20, "world".to_string() );
  assert_eq!( x.a, 20 );
  assert_eq!( x.b, "world" );

  let x = MyStruct::from2( 30, "rust".to_string() );
  assert_eq!( x.a, 30 );
  assert_eq!( x.b, "rust" );

  // No From1 convenience expected
  // let x = MyStruct::from1( 70 ); // Should not compile
}

// Test Combination: T2.4
/// Tests a tuple struct with 2 different fields.
#[ test ]
fn test_tuple_struct_2_different_fields()
{
  #[ derive( VariadicFrom ) ]
  struct MyTuple( bool, f32 );

  let x = MyTuple::from( ( true, 1.0 ) );
  assert_eq!( x.0, true );
  assert_eq!( x.1, 1.0 );

  let x = from!( false, 2.0 );
  assert_eq!( x.0, false );
  assert_eq!( x.1, 2.0 );

  let x = MyTuple::from2( true, 3.0 );
  assert_eq!( x.0, true );
  assert_eq!( x.1, 3.0 );

  // No From1 convenience expected
  // let x = MyTuple::from1( true ); // Should not compile
}

// Test Combination: T3.1
/// Tests a named struct with 3 identical fields.
#[ test ]
fn test_named_struct_3_identical_fields()
{
  #[ derive( VariadicFrom ) ]
  struct MyStruct
  {
    a : i32,
    b : i32,
    c : i32,
  }

  let x = MyStruct::from( ( 10, 20, 30 ) );
  assert_eq!( x.a, 10 );
  assert_eq!( x.b, 20 );
  assert_eq!( x.c, 30 );

  let x = from!( 40, 50, 60 );
  assert_eq!( x.a, 40 );
  assert_eq!( x.b, 50 );
  assert_eq!( x.c, 60 );

  let x = MyStruct::from3( 70, 80, 90 );
  assert_eq!( x.a, 70 );
  assert_eq!( x.b, 80 );
  assert_eq!( x.c, 90 );

  // Convenience From2
  let x = MyStruct::from2( 100, 110 );
  assert_eq!( x.a, 100 );
  assert_eq!( x.b, 110 );
  assert_eq!( x.c, 110 );

  // Convenience From1
  let x = MyStruct::from1( 120 );
  assert_eq!( x.a, 120 );
  assert_eq!( x.b, 120 );
  assert_eq!( x.c, 120 );
}

// Test Combination: T3.2
/// Tests a tuple struct with 3 identical fields.
#[ test ]
fn test_tuple_struct_3_identical_fields()
{
  #[ derive( VariadicFrom ) ]
  struct MyTuple( u8, u8, u8 );

  let x = MyTuple::from( ( 10, 20, 30 ) );
  assert_eq!( x.0, 10 );
  assert_eq!( x.1, 20 );
  assert_eq!( x.2, 30 );

  let x = from!( 40, 50, 60 );
  assert_eq!( x.0, 40 );
  assert_eq!( x.1, 50 );
  assert_eq!( x.2, 60 );

  let x = MyTuple::from3( 70, 80, 90 );
  assert_eq!( x.0, 70 );
  assert_eq!( x.1, 80 );
  assert_eq!( x.2, 90 );

  // Convenience From2
  let x = MyTuple::from2( 100, 110 );
  assert_eq!( x.0, 100 );
  assert_eq!( x.1, 110 );
  assert_eq!( x.2, 110 );

  // Convenience From1
  let x = MyTuple::from1( 120 );
  assert_eq!( x.0, 120 );
  assert_eq!( x.1, 120 );
  assert_eq!( x.2, 120 );
}

// Test Combination: T3.3
/// Tests a named struct with 3 fields, last one different.
#[ test ]
fn test_named_struct_3_fields_last_different()
{
  #[ derive( VariadicFrom ) ]
  struct MyStruct
  {
    a : i32,
    b : i32,
    c : String,
  }

  let x = MyStruct::from( ( 10, 20, "hello".to_string() ) );
  assert_eq!( x.a, 10 );
  assert_eq!( x.b, 20 );
  assert_eq!( x.c, "hello" );

  let x = from!( 30, 40, "world".to_string() );
  assert_eq!( x.a, 30 );
  assert_eq!( x.b, 40 );
  assert_eq!( x.c, "world" );

  let x = MyStruct::from3( 50, 60, "rust".to_string() );
  assert_eq!( x.a, 50 );
  assert_eq!( x.b, 60 );
  assert_eq!( x.c, "rust" );

  // No From2 or From1 convenience expected
  // let x = MyStruct::from2( 70, 80 ); // Should not compile
  // let x = MyStruct::from1( 90 ); // Should not compile
}

// Test Combination: T3.4
/// Tests a tuple struct with 3 fields, last two identical.
#[ test ]
fn test_tuple_struct_3_fields_last_two_identical()
{
  #[ derive( VariadicFrom ) ]
  struct MyTuple( bool, f32, f32 );

  let x = MyTuple::from( ( true, 1.0, 2.0 ) );
  assert_eq!( x.0, true );
  assert_eq!( x.1, 1.0 );
  assert_eq!( x.2, 2.0 );

  let x = from!( false, 3.0, 4.0 );
  assert_eq!( x.0, false );
  assert_eq!( x.1, 3.0 );
  assert_eq!( x.2, 4.0 );

  let x = MyTuple::from3( true, 5.0, 6.0 );
  assert_eq!( x.0, true );
  assert_eq!( x.1, 5.0 );
  assert_eq!( x.2, 6.0 );

  // Convenience From2
  let x = MyTuple::from2( false, 7.0 );
  assert_eq!( x.0, false );
  assert_eq!( x.1, 7.0 );
  assert_eq!( x.2, 7.0 );

  // No From1 convenience expected
  // let x = MyTuple::from1( true ); // Should not compile
}

// Test Combination: T4.1
/// Tests a named struct with 1 generic field.
#[ test ]
fn test_named_struct_1_generic_field()
{
  #[ derive( VariadicFrom ) ]
  struct MyStruct< T >
  where
    T : core::fmt::Debug,
  {
    a : T,
  }

  let x = MyStruct::from( 10 );
  assert_eq!( x.a, 10 );

  let x = from!( 20 );
  assert_eq!( x.a, 20 );

  let x = MyStruct::from1( 30 );
  assert_eq!( x.a, 30 );

  let x = MyStruct::from( "hello".to_string() );
  assert_eq!( x.a, "hello" );
}

// Test Combination: T4.2
/// Tests a tuple struct with 2 generic fields.
#[ test ]
fn test_tuple_struct_2_generic_fields()
{
  #[ derive( VariadicFrom ) ]
  struct MyTuple< T, U >
  (
    T,
    U,
  )
  where
    T : Copy,
    U : Clone;

  let x = MyTuple::from( ( 10, "hello".to_string() ) );
  assert_eq!( x.0, 10 );
  assert_eq!( x.1, "hello" );

  let x = from!( 20, "world".to_string() );
  assert_eq!( x.0, 20 );
  assert_eq!( x.1, "world" );

  let x = MyTuple::from2( 30, "rust".to_string() );
  assert_eq!( x.0, 30 );
  assert_eq!( x.1, "rust" );
}