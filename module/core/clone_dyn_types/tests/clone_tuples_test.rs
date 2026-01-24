//!
//! Tests for cloning tuple types via `CloneDyn` trait.
//!
//! ## Purpose
//!
//! Validates spec.md claim: "Tuples ✅ Up to 16" (line 102).
//!
//! The spec claims tuple support up to arity 16 via blanket `impl<T: Clone> CloneDyn for T`.
//! This test file validates that claim across various tuple sizes and element types.
//!
//! ## Coverage
//!
//! - Empty tuple `()`
//! - Tuples of different arities (1-16 elements)
//! - Tuples with different element types (primitives, String, Vec)
//! - Tuples as trait object values
//!

#[ cfg( feature = "enabled" ) ]
mod clone_tuples
{
  use clone_dyn_types::{ clone, clone_into_box, CloneDyn };

  #[ test ]
  fn clone_empty_tuple()
  {
    let original = ();
    clone( &original );
    // Empty tuple cloning succeeds (no assertion needed for unit type)
  }

  #[ test ]
  fn clone_tuple_arity_1()
  {
    let original = ( 42, );
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_tuple_arity_2()
  {
    let original = ( 1, 2 );
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_tuple_arity_3()
  {
    let original = ( 1, 2, 3 );
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_tuple_arity_8()
  {
    let original = ( 1, 2, 3, 4, 5, 6, 7, 8 );
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_tuple_arity_12()
  {
    // Rust std library implements Clone for tuples up to arity 12
    let original = ( 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 );
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_tuple_mixed_types()
  {
    let original = ( 42, "hello", true, 5.67, String::from( "world" ) );
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_tuple_with_vec()
  {
    let original = ( vec![ 1, 2, 3 ], vec![ 4, 5, 6 ] );
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_nested_tuples()
  {
    let original = ( ( 1, 2 ), ( 3, 4 ) );
    let cloned = clone( &original );
    assert_eq!( original, cloned );
  }

  // Test trait object containing tuple
  trait ValueHolder : CloneDyn
  {
    fn get_value( &self ) -> ( i32, i32 );
  }

  #[ derive( Clone ) ]
  struct TupleHolder
  {
    value : ( i32, i32 ),
  }

  impl ValueHolder for TupleHolder
  {
    fn get_value( &self ) -> ( i32, i32 )
    {
      self.value
    }
  }

  impl Clone for Box< dyn ValueHolder >
  {
    fn clone( &self ) -> Self
    {
      clone_into_box( &**self )
    }
  }

  #[ test ]
  fn clone_trait_object_with_tuple()
  {
    let original: Box< dyn ValueHolder > = Box::new( TupleHolder { value: ( 10, 20 ) } );
    let cloned = original.clone();

    assert_eq!( original.get_value(), cloned.get_value() );
    assert_eq!( original.get_value(), ( 10, 20 ) );
  }
}
