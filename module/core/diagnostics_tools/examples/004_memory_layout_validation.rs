//! # Example 004: Memory Layout Validation
//!
//! This example demonstrates memory layout validation - ensuring types have
//! expected sizes, alignments, and memory characteristics at compile-time.
//!
//! ## What you'll learn:
//! - Type size validation with `cta_type_same_size!`
//! - Alignment validation with `cta_type_same_align!` 
//! - Pointer and memory size checks
//! - Low-level memory safety validation
//!
//! ## Run this example:
//! ```bash
//! cargo run --example 004_memory_layout_validation
//! ```

use diagnostics_tools::*;
use core::mem::{ size_of, align_of };

// ✅ Compile-time memory layout validation
// These checks will be performed inside functions where they're allowed

#[ repr( C ) ]
#[ derive( Debug ) ]
struct Point
{
  x : f32,
  y : f32,
}

#[ repr( C ) ]  
#[ derive( Debug ) ]
struct Vector2
{
  x : f32,
  y : f32,
}

fn main()
{
  println!( "🧠 Demonstrating memory layout validation" );
  println!( "All memory checks in this example happen at compile-time!\n" );

  // ✅ Perform compile-time layout validation
  perform_layout_validation();

  // ✅ Display actual sizes and alignments
  println!( "1. Fundamental type sizes (validated at compile-time):" );
  println!( "   u32: {} bytes (aligned to {})", size_of::< u32 >(), align_of::< u32 >() );
  println!( "   i32: {} bytes (aligned to {})", size_of::< i32 >(), align_of::< i32 >() );
  println!( "   f32: {} bytes (aligned to {})", size_of::< f32 >(), align_of::< f32 >() );
  println!( "   u64: {} bytes (aligned to {})", size_of::< u64 >(), align_of::< u64 >() );
  println!( "   ✓ All size relationships validated at compile-time" );

  // ✅ Pointer validation
  println!( "\n2. Pointer sizes:" );
  println!( "   *const u8: {} bytes", size_of::< *const u8 >() );
  println!( "   *mut u64:  {} bytes", size_of::< *mut u64 >() );
  println!( "   ✓ All pointers have same size (validated at compile-time)" );

  // ✅ Struct layout validation
  println!( "\n3. Struct layouts:" );
  println!( "   Point:    {} bytes (aligned to {})", size_of::< Point >(), align_of::< Point >() );
  println!( "   Vector2:  {} bytes (aligned to {})", size_of::< Vector2 >(), align_of::< Vector2 >() );
  println!( "   ✓ Equivalent structs have same layout (validated at compile-time)" );

  // ✅ Runtime memory validation
  demonstrate_runtime_memory_checks();

  // ✅ Advanced layout scenarios
  demonstrate_advanced_layouts();

  println!( "\n🎉 All memory layout validations passed!" );
  println!( "\n💡 Key benefits of memory layout validation:" );
  println!( "   • Catch size assumption errors at compile-time" );
  println!( "   • Ensure struct layouts match across platforms" );
  println!( "   • Validate pointer size assumptions" );
  println!( "   • Document memory requirements in code" );
  println!( "\n➡️  Next: Run example 005 to learn about debug variants!" );
}

fn demonstrate_runtime_memory_checks()
{
  println!( "\n4. Runtime memory validation:" );
  
  let point = Point { x : 1.0, y : 2.0 };
  let vector = Vector2 { x : 3.0, y : 4.0 };
  
  // Runtime validation that actual values have expected sizes
  cta_mem_same_size!( point, vector );
  println!( "   ✓ Point and Vector2 instances have same memory size" );
  
  let ptr1 : *const u8 = core::ptr::null();
  let ptr2 : *const i64 = core::ptr::null();
  
  // Validate that different pointer types have same size
  cta_ptr_same_size!( &raw const ptr1, &raw const ptr2 );
  println!( "   ✓ Pointers to different types have same size" );
}

fn demonstrate_advanced_layouts()
{
  println!( "\n5. Advanced layout scenarios:" );
  
  // Arrays vs slices
  let array : [ u32; 4 ] = [ 1, 2, 3, 4 ];
  let array_size = size_of::< [ u32; 4 ] >();
  let slice_ref_size = size_of::< &[ u32 ] >();
  
  println!( "   [u32; 4]: {array_size} bytes" );
  println!( "   &[u32]:   {slice_ref_size} bytes (fat pointer)" );
  
  // String vs &str
  let string_size = size_of::< String >();
  let str_ref_size = size_of::< &str >();
  
  println!( "   String:   {string_size} bytes (owned)" );
  println!( "   &str:     {str_ref_size} bytes (fat pointer)" );
  
  // Option optimization
  let option_ptr_size = size_of::< Option< &u8 > >();
  let ptr_size = size_of::< &u8 >();
  
  println!( "   Option<&u8>: {option_ptr_size} bytes" );
  println!( "   &u8:         {ptr_size} bytes" );
  
  if option_ptr_size == ptr_size
  {
    println!( "   ✓ Option<&T> has same size as &T (null optimization)" );
  }
  
  // Demonstrate usage with actual data
  let _data_point = point_from_array( &array );
  println!( "   ✓ Successfully converted array to point (size validation passed)" );
}

// Function to perform compile-time layout validation
fn perform_layout_validation()
{
  // Validate fundamental type sizes
  cta_type_same_size!( u32, i32 );   // Same size: 4 bytes each
  cta_type_same_size!( u64, i64 );   // Same size: 8 bytes each  
  cta_type_same_size!( f32, u32 );   // Both are 4 bytes
  cta_type_same_size!( f64, u64 );   // Both are 8 bytes

  // Validate pointer sizes
  cta_type_same_size!( *const u8, *mut u8 );     // All raw pointers same size
  cta_type_same_size!( *const u8, *const u64 );  // Pointer size independent of target type

  // Validate alignment requirements  
  cta_type_same_align!( u32, f32 );  // Both have 4-byte alignment
  cta_type_same_align!( u64, f64 );  // Both have 8-byte alignment
  
  // Validate that equivalent structs have same layout
  cta_type_same_size!( Point, Vector2 );
  cta_type_same_align!( Point, Vector2 );
}

// Example function that relies on memory layout assumptions
fn point_from_array( arr : &[ u32 ] ) -> Point
{
  // This function creates a point from array data
  // In real code, you'd want proper conversion, but this demonstrates the concept
  
  // Simple safe conversion for demonstration
  let x = arr.first().copied().unwrap_or( 0 ) as f32;
  let y = arr.get( 1 ).copied().unwrap_or( 0 ) as f32;
  Point { x, y }
}

#[ allow( dead_code ) ]
fn examples_that_would_fail_compilation()
{
  // These would cause COMPILE-TIME errors if uncommented:
  
  // Size mismatch (u32 is 4 bytes, u64 is 8 bytes):
  // cta_type_same_size!( u32, u64 );
  
  // Different alignment (u8 has 1-byte alignment, u64 has 8-byte):  
  // cta_type_same_align!( u8, u64 );
  
  // Array sizes differ:
  // cta_type_same_size!( [u32; 2], [u32; 4] );
}

#[ cfg( target_pointer_width = "64" ) ]
#[allow(dead_code)]
fn pointer_width_specific_checks()
{
  // Only compile these checks on 64-bit targets
  cta_type_same_size!( usize, u64 );  // usize is 8 bytes on 64-bit
  cta_type_same_size!( *const u8, u64 ); // Pointers are 8 bytes on 64-bit
  
  println!( "   ✓ 64-bit pointer validations passed" );
}