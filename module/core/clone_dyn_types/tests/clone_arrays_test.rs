//!
//! Tests for cloning array types via `CloneDyn` trait.
//!
//! ## Purpose
//!
//! Validates spec.md claim: "Arrays ✅ Up to 32" (line 103).
//!
//! The spec claims array support up to size 32 via blanket `impl<T: Clone> CloneDyn for T`.
//! This test file validates that claim across various array sizes and element types.
//!
//! Note: Modern Rust implements Clone for arrays of ALL sizes (not just up to 32),
//! so this tests the spec's claimed limit and beyond.
//!
//! ## Coverage
//!
//! - Empty array `[T; 0]`
//! - Small arrays (1-10 elements)
//! - Medium arrays (16, 32 elements - spec's claimed limit)
//! - Large arrays (64, 128 elements - beyond spec's claim)
//! - Arrays with different element types (primitives, String)
//! - Arrays as trait object values
//!

#[ cfg( feature = "enabled" ) ]
mod clone_arrays
{
  use clone_dyn_types::{ clone, clone_into_box, CloneDyn };

  #[ test ]
  fn clone_empty_array()
  {
    let original: [ i32; 0 ] = [];
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_array_size_1()
  {
    let original = [ 42 ];
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_array_size_3()
  {
    let original = [ 1, 2, 3 ];
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_array_size_8()
  {
    let original = [ 1, 2, 3, 4, 5, 6, 7, 8 ];
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_array_size_16()
  {
    let original = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 ];
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_array_size_32()
  {
    // Spec's claimed limit (line 103)
    let original = [
      1, 2, 3, 4, 5, 6, 7, 8,
      9, 10, 11, 12, 13, 14, 15, 16,
      17, 18, 19, 20, 21, 22, 23, 24,
      25, 26, 27, 28, 29, 30, 31, 32,
    ];
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_array_size_64()
  {
    // Beyond spec's claimed limit - tests if blanket impl actually works
    let original = [ 42; 64 ];
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_array_size_128()
  {
    // Well beyond spec's claimed limit
    let original = [ 99; 128 ];
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_array_of_strings()
  {
    let original = [
      String::from( "hello" ),
      String::from( "world" ),
      String::from( "test" ),
    ];
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_array_of_tuples()
  {
    let original = [ ( 1, 2 ), ( 3, 4 ), ( 5, 6 ) ];
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  // Test trait object containing array
  trait ArrayHolder : CloneDyn
  {
    fn get_array( &self ) -> [ i32; 4 ];
  }

  #[ derive( Clone ) ]
  struct FixedArray
  {
    data : [ i32; 4 ],
  }

  impl ArrayHolder for FixedArray
  {
    fn get_array( &self ) -> [ i32; 4 ]
    {
      self.data
    }
  }

  impl Clone for Box< dyn ArrayHolder >
  {
    fn clone( &self ) -> Self
    {
      clone_into_box( &**self )
    }
  }

  #[ test ]
  fn clone_trait_object_with_array()
  {
    let original: Box< dyn ArrayHolder > = Box::new( FixedArray { data: [ 10, 20, 30, 40 ] } );
    let cloned = original.clone();

    assert_eq!( original.get_array(), cloned.get_array() );
    assert_eq!( original.get_array(), [ 10, 20, 30, 40 ] );
  }
}
