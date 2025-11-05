//! Test suite for the `clone_dyn_types` crate.
//!
//! Comprehensive tests for `CloneDyn` trait implementations.
//! Tests cover generic types, slices, and string slices.

#[ allow( unused_imports ) ]
use clone_dyn_types as the_module;

#[ cfg( feature = "enabled" ) ]
mod clone_generic_types
{
  use clone_dyn_types :: clone;

  #[ test ]
  fn clone_simple_struct()
  {
    #[ derive( Clone, Debug, PartialEq ) ]
    struct Point
    {
      x : i32,
      y : i32,
    }

    let original = Point { x : 10, y : 20 };
    let cloned = clone( &original );

    assert_eq!( original, cloned );
    assert_eq!( cloned.x, 10 );
    assert_eq!( cloned.y, 20 );
  }

  #[ test ]
  fn clone_primitive_types()
  {
    let int_original = 42i32;
    let int_cloned = clone( &int_original );
    assert_eq!( int_original, int_cloned );

    let bool_original = true;
    let bool_cloned = clone( &bool_original );
    assert_eq!( bool_original, bool_cloned );

    let char_original = 'A';
    let char_cloned = clone( &char_original );
    assert_eq!( char_original, char_cloned );
  }

  #[ test ]
  fn clone_string()
  {
    let original = String :: from( "Hello, world!" );
    let cloned = clone( &original );

    assert_eq!( original, cloned );
    assert_eq!( cloned, "Hello, world!" );
  }

  #[ test ]
  fn clone_vec()
  {
    let original = vec![ 1, 2, 3, 4, 5 ];
    let cloned = clone( &original );

    assert_eq!( original, cloned );
    assert_eq!( cloned.len(), 5 );
    assert_eq!( cloned[ 0 ], 1 );
    assert_eq!( cloned[ 4 ], 5 );
  }
}

#[ cfg( feature = "enabled" ) ]
mod clone_slices
{
  use clone_dyn_types :: { CloneDyn, clone_into_box };

  #[ test ]
  fn clone_slice_basic()
  {
    let data = vec![ 1, 2, 3, 4, 5 ];
    let slice : &[ i32 ] = &data;

    // Create boxed slice trait object
    // Note: Slices are unsized types, requiring double reference for trait object coercion
    let _cloned = clone_into_box( &slice as &dyn CloneDyn );
  }

  #[ test ]
  fn clone_empty_slice()
  {
    let data : Vec< i32 > = vec![];
    let slice : &[ i32 ] = &data;

    // Unsized type requires double reference (&slice) for trait object cast
    let _cloned = clone_into_box( &slice as &dyn CloneDyn );
  }
}

#[ cfg( feature = "enabled" ) ]
mod clone_str_slices
{
  use clone_dyn_types :: { CloneDyn, clone_into_box };

  #[ test ]
  fn clone_str_basic()
  {
    let text = "Hello, Rust!";
    let str_slice : &str = text;

    // str is unsized type, requires double reference for trait object cast
    let _cloned = clone_into_box( &str_slice as &dyn CloneDyn );
  }

  #[ test ]
  fn clone_empty_str()
  {
    let text = "";
    let str_slice : &str = text;

    // Unsized type requires double reference (&str_slice) for trait object cast
    let _cloned = clone_into_box( &str_slice as &dyn CloneDyn );
  }

  #[ test ]
  fn clone_unicode_str()
  {
    let text = "Hello, 世界! 🦀";
    let str_slice : &str = text;

    // Unsized type requires double reference for trait object coercion
    let _cloned = clone_into_box( &str_slice as &dyn CloneDyn );
  }
}

#[ cfg( feature = "enabled" ) ]
mod clone_trait_objects
{
  use clone_dyn_types :: { CloneDyn, clone_into_box };

  trait Animal : CloneDyn
  {
    fn make_sound( &self ) -> &'static str;
  }

  #[ derive( Clone ) ]
  struct Dog;

  impl Animal for Dog
  {
    fn make_sound( &self ) -> &'static str
    {
      "Woof!"
    }
  }

  #[ allow( non_local_definitions ) ]
  impl Clone for Box< dyn Animal >
  {
    fn clone( &self ) -> Self
    {
      clone_into_box( &**self )
    }
  }

  #[ test ]
  fn clone_boxed_trait_object()
  {
    let dog : Box< dyn Animal > = Box :: new( Dog );
    let cloned_dog = dog.clone();

    assert_eq!( dog.make_sound(), cloned_dog.make_sound() );
    assert_eq!( cloned_dog.make_sound(), "Woof!" );
  }
}
