//! Comprehensive corner case testing for `inspect_type` macros.
//!
//! This test file systematically validates behavior across all edge cases,
//! boundary conditions, and type categories to ensure robust type inspection.

use inspect_type as the_module;

// ============================================================================
// Category 1: Primitive Types
// ============================================================================

#[ test ]
fn primitive_types()
{
  // Integers
  let result = the_module ::inspect_to_str_type_of!( 42i32 );
  assert!( result.contains( "i32" ) );
  assert!( result.contains( " = 4" ) );

  let result = the_module ::inspect_to_str_type_of!( 100u64 );
  assert!( result.contains( "u64" ) );
  assert!( result.contains( " = 8" ) );

  // Floats
  let result = the_module ::inspect_to_str_type_of!( 3.15f64 );
  assert!( result.contains( "f64" ) );
  assert!( result.contains( " = 8" ) );

  // Bool
  let result = the_module ::inspect_to_str_type_of!( true );
  assert!( result.contains( "bool" ) );
  assert!( result.contains( " = 1" ) );

  // Char
  let result = the_module ::inspect_to_str_type_of!( 'a' );
  assert!( result.contains( "char" ) );
  assert!( result.contains( " = 4" ) );
}

#[ test ]
fn primitive_references()
{
  let value = 42i32;

  // Immutable reference
  let result = the_module ::inspect_to_str_type_of!( &value );
  assert!( result.contains( "&i32" ) );
  assert!( result.contains( " = 8" ) ); // pointer size on 64-bit

  // Mutable reference
  let mut mut_value = 42i32;
  let result = the_module ::inspect_to_str_type_of!( &mut mut_value );
  assert!( result.contains( "&mut i32" ) );
  assert!( result.contains( " = 8" ) );
}

// ============================================================================
// Category 2: String Types
// ============================================================================

#[ test ]
fn string_types()
{
  // String (owned)
  let result = the_module ::inspect_to_str_type_of!( String::from( "hello" ) );
  assert!( result.contains( "String" ) );
  assert!( result.contains( " = 24" ) ); // String is 3 pointers (ptr, len, cap)

  // &str (string slice)
  let result = the_module ::inspect_to_str_type_of!( "hello" );
  assert!( result.contains( "&str" ) );
  assert!( result.contains( " = 16" ) ); // fat pointer (ptr + len)

  // &String (reference to String)
  let s = String::from( "hello" );
  let result = the_module ::inspect_to_str_type_of!( &s );
  assert!( result.contains( "String" ) );
  assert!( result.contains( " = 8" ) ); // thin pointer
}

// ============================================================================
// Category 3: Collections - Arrays
// ============================================================================

#[ test ]
fn arrays_and_slices()
{
  // Empty array - this is a zero-sized type
  let result = the_module ::inspect_to_str_type_of!( [] as [ i32; 0 ] );
  assert!( result.contains( "[i32; 0]" ) );
  assert!( result.contains( " = 0" ) );

  // Small array (owned)
  let result = the_module ::inspect_to_str_type_of!( [ 1, 2, 3 ] );
  assert!( result.contains( "[i32; 3]" ) );
  assert!( result.contains( " = 12" ) ); // 3 * 4 bytes

  // Array reference
  let result = the_module ::inspect_to_str_type_of!( &[ 1, 2, 3 ] );
  assert!( result.contains( "[i32; 3]" ) );
  assert!( result.contains( " = 8" ) ); // thin pointer

  // Slice reference (fat pointer)
  let result = the_module ::inspect_to_str_type_of!( &[ 1, 2, 3 ][ .. ] );
  assert!( result.contains( "&[i32]" ) );
  assert!( result.contains( " = 16" ) ); // fat pointer (ptr + len)

  // Empty slice
  let empty : &[ i32 ] = &[];
  let result = the_module ::inspect_to_str_type_of!( empty );
  assert!( result.contains( "&[i32]" ) );
  assert!( result.contains( " = 16" ) ); // still fat pointer even if empty
}

#[ test ]
fn large_arrays()
{
  // Array larger than 32 elements (special case in Rust trait impls)
  let large_array = [ 0u8; 100 ];
  let result = the_module ::inspect_to_str_type_of!( large_array );
  assert!( result.contains( "[u8; 100]" ) );
  assert!( result.contains( " = 100" ) );

  let result = the_module ::inspect_to_str_type_of!( &large_array );
  assert!( result.contains( "[u8; 100]" ) );
  assert!( result.contains( " = 8" ) ); // reference to array
}

// ============================================================================
// Category 4: Collections - Vec
// ============================================================================

#[ test ]
fn vec_types()
{
  // Owned Vec
  let result = the_module ::inspect_to_str_type_of!( vec![ 1, 2, 3 ] );
  assert!( result.contains( "Vec" ) );
  assert!( result.contains( "i32" ) );
  assert!( result.contains( " = 24" ) ); // Vec is 3 pointers

  // Empty Vec
  let empty_vec : Vec< i32 > = Vec::new();
  let result = the_module ::inspect_to_str_type_of!( empty_vec );
  assert!( result.contains( "Vec" ) );
  assert!( result.contains( " = 24" ) ); // same size even if empty

  // Vec reference
  let v = vec![ 1, 2, 3 ];
  let result = the_module ::inspect_to_str_type_of!( &v );
  assert!( result.contains( "Vec" ) );
  assert!( result.contains( " = 8" ) ); // thin pointer to Vec

  // Slice from Vec
  let result = the_module ::inspect_to_str_type_of!( &v[ .. ] );
  assert!( result.contains( "&[i32]" ) );
  assert!( result.contains( " = 16" ) ); // fat pointer
}

// ============================================================================
// Category 5: Tuples
// ============================================================================

#[ test ]
fn tuple_types()
{
  // Unit tuple (zero-sized)
  let result = the_module ::inspect_to_str_type_of!( () );
  assert!( result.contains( "()" ) );
  assert!( result.contains( " = 0" ) );

  // Two-tuple
  let result = the_module ::inspect_to_str_type_of!( ( 1i32, 2i32 ) );
  assert!( result.contains( "i32" ) );
  assert!( result.contains( " = 8" ) ); // 2 * 4 bytes

  // Three-tuple with different types
  let result = the_module ::inspect_to_str_type_of!( ( 1u8, 2u16, 3u32 ) );
  assert!( result.contains( "u8" ) || result.contains( "u16" ) || result.contains( "u32" ) );
  // Size may vary due to padding

  // Tuple reference
  let t = ( 1i32, 2i32 );
  let result = the_module ::inspect_to_str_type_of!( &t );
  assert!( result.contains( " = 8" ) ); // pointer to tuple
}

// ============================================================================
// Category 6: Structs
// ============================================================================

#[ test ]
fn struct_types()
{
  // Zero-sized struct
  #[ derive( Debug ) ]
  #[ allow( dead_code ) ]
  struct Empty;

  let result = the_module ::inspect_to_str_type_of!( Empty );
  assert!( result.contains( "Empty" ) );
  assert!( result.contains( " = 0" ) );

  // Small struct
  #[ derive( Debug ) ]
  #[ allow( dead_code ) ]
  struct Small
  {
    a : u8,
  }

  let result = the_module ::inspect_to_str_type_of!( Small { a : 1 } );
  assert!( result.contains( "Small" ) );
  assert!( result.contains( " = 1" ) );

  // Struct with padding
  #[ derive( Debug ) ]
  #[ allow( dead_code ) ]
  struct Padded
  {
    a : u8,
    b : u64, // Will cause padding after a
  }

  let result = the_module ::inspect_to_str_type_of!( Padded { a : 1, b : 2 } );
  assert!( result.contains( "Padded" ) );
  assert!( result.contains( " = 16" ) ); // u8 + 7 padding + u64

  // Struct reference
  let s = Small { a : 1 };
  let result = the_module ::inspect_to_str_type_of!( &s );
  assert!( result.contains( "Small" ) );
  assert!( result.contains( " = 8" ) ); // pointer
}

// ============================================================================
// Category 7: Enums
// ============================================================================

#[ test ]
fn enum_types()
{
  // Unit enum
  #[ derive( Debug ) ]
  #[ allow( dead_code ) ]
  enum Color
  {
    Red,
    Green,
    Blue,
  }

  let result = the_module ::inspect_to_str_type_of!( Color::Red );
  assert!( result.contains( "Color" ) );
  // Size is at least 1 byte for discriminant

  // Enum with data
  #[ derive( Debug ) ]
  #[ allow( dead_code ) ]
  enum Message
  {
    Quit,
    Move { x : i32, y : i32 },
    Write( String ),
  }

  let result = the_module ::inspect_to_str_type_of!( Message::Quit );
  assert!( result.contains( "Message" ) );
  // Size includes largest variant + discriminant

  let result = the_module ::inspect_to_str_type_of!( Message::Move { x : 1, y : 2 } );
  assert!( result.contains( "Message" ) );

  // Option (standard library enum)
  let result = the_module ::inspect_to_str_type_of!( Some( 42i32 ) );
  assert!( result.contains( "Option" ) );
  assert!( result.contains( "i32" ) );

  let result = the_module ::inspect_to_str_type_of!( None as Option< i32 > );
  assert!( result.contains( "Option" ) );
}

// ============================================================================
// Category 8: References and Pointers
// ============================================================================

#[ test ]
fn reference_layers()
{
  let value = 42i32;
  let ref1 = &value;
  let ref2 = &ref1;
  let ref3 = &ref2;

  // Single reference
  let result = the_module ::inspect_to_str_type_of!( ref1 );
  assert!( result.contains( "&i32" ) );
  assert!( result.contains( " = 8" ) );

  // Double reference
  let result = the_module ::inspect_to_str_type_of!( ref2 );
  assert!( result.contains( "&&i32" ) );
  assert!( result.contains( " = 8" ) );

  // Triple reference
  let result = the_module ::inspect_to_str_type_of!( ref3 );
  assert!( result.contains( "&&&i32" ) );
  assert!( result.contains( " = 8" ) );
}

#[ test ]
fn smart_pointers()
{
  use std::rc::Rc;
  use std::sync::Arc;

  // Box
  let result = the_module ::inspect_to_str_type_of!( Box::new( 42i32 ) );
  assert!( result.contains( "Box" ) );
  assert!( result.contains( "i32" ) );
  assert!( result.contains( " = 8" ) ); // Box is single pointer

  // Rc
  let result = the_module ::inspect_to_str_type_of!( Rc::new( 42i32 ) );
  assert!( result.contains( "Rc" ) );
  assert!( result.contains( "i32" ) );
  assert!( result.contains( " = 8" ) ); // Rc is single pointer

  // Arc
  let result = the_module ::inspect_to_str_type_of!( Arc::new( 42i32 ) );
  assert!( result.contains( "Arc" ) );
  assert!( result.contains( "i32" ) );
  assert!( result.contains( " = 8" ) ); // Arc is single pointer
}

// ============================================================================
// Category 9: Generic Types
// ============================================================================

#[ test ]
fn generic_types()
{
  // Option with different types
  let result = the_module ::inspect_to_str_type_of!( Some( 42i32 ) );
  assert!( result.contains( "Option" ) );
  assert!( result.contains( "i32" ) );

  let result = the_module ::inspect_to_str_type_of!( Some( "hello" ) );
  assert!( result.contains( "Option" ) );
  assert!( result.contains( "str" ) );

  // Result
  let ok_result : Result< i32, String > = Ok( 42 );
  let result = the_module ::inspect_to_str_type_of!( ok_result );
  assert!( result.contains( "Result" ) );

  let err_result : Result< i32, String > = Err( String::from( "error" ) );
  let result = the_module ::inspect_to_str_type_of!( err_result );
  assert!( result.contains( "Result" ) );
}

// ============================================================================
// Category 10: Expression Testing
// ============================================================================

#[ test ]
fn expression_inspection()
{
  // Literal expressions
  let result = the_module ::inspect_to_str_type_of!( 42 );
  assert!( result.contains( "i32" ) );

  // Arithmetic expressions
  let result = the_module ::inspect_to_str_type_of!( 1 + 2 );
  assert!( result.contains( "i32" ) );
  assert!( result.contains( "1 + 2" ) ); // Expression text preserved

  // Method call results
  let v = [ 1, 2, 3 ];
  let result = the_module ::inspect_to_str_type_of!( v.len() );
  assert!( result.contains( "usize" ) );
}

// ============================================================================
// Category 11: Macro Output Format Verification
// ============================================================================

#[ test ]
fn output_format_correctness()
{
  // Verify format: "sizeof( expression : type ) = size"
  let result = the_module ::inspect_to_str_type_of!( 42i32 );

  // Must contain all format components
  assert!( result.starts_with( "sizeof(" ) );
  assert!( result.contains( " : " ) );
  assert!( result.contains( " ) = " ) );

  // Must contain expression text
  assert!( result.contains( "42i32" ) );

  // Must contain type
  assert!( result.contains( "i32" ) );

  // Must contain size
  assert!( result.contains( '4' ) );
}

#[ test ]
fn both_macros_consistency()
{
  // inspect_type_of! should produce same string as inspect_to_str_type_of!
  // Capture stdout for inspect_type_of!
  let expected = the_module ::inspect_to_str_type_of!( &[ 1, 2, 3 ][ .. ] );
  let actual = the_module ::inspect_type_of!( &[ 1, 2, 3 ][ .. ] );

  // Both should return the same string
  assert_eq!( expected, actual );
}

// ============================================================================
// Category 12: Edge Cases
// ============================================================================

#[ test ]
fn nested_generics()
{
  // Deeply nested generic types
  let nested : Vec< Option< Result< i32, String > > > = vec!
  [
    Some( Ok( 42 ) ),
    Some( Err( String::from( "error" ) ) ),
    None,
  ];

  let result = the_module ::inspect_to_str_type_of!( nested );
  assert!( result.contains( "Vec" ) );
  assert!( result.contains( "Option" ) );
  assert!( result.contains( "Result" ) );
}

#[ test ]
fn zero_sized_types()
{
  use core::marker::PhantomData;

  // PhantomData
  let result = the_module ::inspect_to_str_type_of!( PhantomData ::< i32 > );
  assert!( result.contains( "PhantomData" ) );
  assert!( result.contains( " = 0" ) );

  // Unit type
  let result = the_module ::inspect_to_str_type_of!( () );
  assert!( result.contains( "()" ) );
  assert!( result.contains( " = 0" ) );
}

// ============================================================================
// Category 13: Function Types and Closures
// ============================================================================

#[ test ]
fn function_pointers()
{
  // Function pointer with no parameters
  fn simple_fn() -> i32 { 42 }
  let fp : fn() -> i32 = simple_fn;
  let result = the_module ::inspect_to_str_type_of!( fp );
  assert!( result.contains( "fn()" ) );
  assert!( result.contains( " = 8" ) ); // Function pointer is single pointer

  // Function pointer with parameters
  fn add( a : i32, b : i32 ) -> i32 { a + b }
  let fp2 : fn( i32, i32 ) -> i32 = add;
  let result = the_module ::inspect_to_str_type_of!( fp2 );
  assert!( result.contains( "fn(i32, i32)" ) );
  assert!( result.contains( " = 8" ) );
}

#[ test ]
fn closure_types()
{
  // Non-capturing closure
  let closure = || 42;
  let result = the_module ::inspect_to_str_type_of!( closure );
  // Closure types have compiler-generated names
  assert!( result.contains( " = " ) ); // Has size

  // Capturing closure
  let x = 10;
  let capturing_closure = || x + 1;
  let result = the_module ::inspect_to_str_type_of!( capturing_closure );
  assert!( result.contains( " = " ) );

  // Closure with parameters
  let param_closure = | a : i32 | a * 2;
  let result = the_module ::inspect_to_str_type_of!( param_closure );
  assert!( result.contains( " = " ) );
}

// ============================================================================
// Category 14: Raw Pointers (Unsafe)
// ============================================================================

#[ test ]
fn raw_pointer_types()
{
  let value = 42i32;

  // Const raw pointer
  let const_ptr : *const i32 = std::ptr::addr_of!( value );
  let result = the_module ::inspect_to_str_type_of!( const_ptr );
  assert!( result.contains( "*const i32" ) );
  assert!( result.contains( " = 8" ) ); // Pointer size on 64-bit

  // Mut raw pointer
  let mut mut_value = 42i32;
  let mut_ptr : *mut i32 = std::ptr::addr_of_mut!( mut_value );
  let result = the_module ::inspect_to_str_type_of!( mut_ptr );
  assert!( result.contains( "*mut i32" ) );
  assert!( result.contains( " = 8" ) );
}

// ============================================================================
// Category 15: Trait Objects (DST - Dynamically Sized Types)
// ============================================================================

#[ test ]
fn trait_object_types()
{
  use core::fmt::Debug;

  // Reference to trait object (fat pointer)
  let value : i32 = 42;
  let trait_obj : &dyn Debug = &value;
  let result = the_module ::inspect_to_str_type_of!( trait_obj );
  assert!( result.contains( "dyn" ) );
  assert!( result.contains( " = 16" ) ); // Fat pointer (data + vtable)

  // Boxed trait object
  let boxed_trait : Box< dyn Debug > = Box::new( 42i32 );
  let result = the_module ::inspect_to_str_type_of!( boxed_trait );
  assert!( result.contains( "Box" ) );
  assert!( result.contains( "dyn" ) );
  assert!( result.contains( " = 16" ) ); // Box with fat pointer
}

// ============================================================================
// Category 16: Type Alias Resolution
// ============================================================================

#[ test ]
fn type_alias_resolution()
{
  type MyInt = i32;
  type MyVec = Vec< i32 >;

  // Type aliases should resolve to underlying type
  let value : MyInt = 42;
  let result = the_module ::inspect_to_str_type_of!( value );
  assert!( result.contains( "i32" ) ); // Not "MyInt"

  let vec_value : MyVec = vec![ 1, 2, 3 ];
  let result = the_module ::inspect_to_str_type_of!( vec_value );
  assert!( result.contains( "Vec" ) );
}
