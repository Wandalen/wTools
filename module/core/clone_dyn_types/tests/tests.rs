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

#[ cfg( feature = "enabled" ) ]
mod clone_corner_cases
{
  use clone_dyn_types :: { CloneDyn, clone, clone_into_box };

  #[ test ]
  fn clone_single_element_slice()
  {
    let data = vec![ 42 ];
    let slice : &[ i32 ] = &data;

    let _cloned = clone_into_box( &slice as &dyn CloneDyn );
  }

  #[ test ]
  fn clone_large_slice()
  {
    let data : Vec< i32 > = ( 0..10000 ).collect();
    let slice : &[ i32 ] = &data;

    let _cloned = clone_into_box( &slice as &dyn CloneDyn );
  }

  #[ test ]
  fn clone_slice_with_strings()
  {
    let data = vec![ String :: from( "hello" ), String :: from( "world" ) ];
    let slice : &[ String ] = &data;

    let _cloned = clone_into_box( &slice as &dyn CloneDyn );
  }

  #[ test ]
  fn clone_zero_sized_type()
  {
    #[ derive( Clone, Debug, PartialEq ) ]
    struct ZeroSized;

    let original = ZeroSized;
    let cloned = clone( &original );

    assert_eq!( original, cloned );
  }

  #[ test ]
  fn clone_type_with_drop()
  {
    use core :: sync :: atomic :: { AtomicBool, Ordering };

    static DROPPED : AtomicBool = AtomicBool :: new( false );

    #[ derive( Clone ) ]
    struct WithDrop
    {
      value : i32,
    }

    impl Drop for WithDrop
    {
      fn drop( &mut self )
      {
        DROPPED.store( true, Ordering :: Relaxed );
      }
    }

    let original = WithDrop { value : 42 };
    let cloned = clone( &original );

    assert_eq!( original.value, cloned.value );
    drop( original );
    assert!( DROPPED.load( Ordering :: Relaxed ) );
  }

  #[ test ]
  fn clone_very_long_string()
  {
    let text : String = "a".repeat( 100_000 );
    let str_slice : &str = &text;

    let _cloned = clone_into_box( &str_slice as &dyn CloneDyn );
  }
}

#[ cfg( feature = "enabled" ) ]
mod clone_iterator_from_example
{
  use clone_dyn_types :: CloneDyn;

  /// Reproduces the iterator trait from the trivial example for testing.
  pub trait IterTrait< 'a, T >
  where
    T : 'a,
    Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
    Self : CloneDyn,
  {
  }

  impl< 'a, T, I > IterTrait< 'a, T > for I
  where
    T : 'a,
    Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
    Self : CloneDyn,
  {
  }

  #[ allow( non_local_definitions ) ]
  impl< 'c, T > Clone for Box< dyn IterTrait< 'c, T > + 'c >
  {
    fn clone( &self ) -> Self
    {
      clone_dyn_types :: clone_into_box( &**self )
    }
  }

  pub fn get_iter< 'a >( src : Option< &'a Vec< i32 > > ) -> Box< dyn IterTrait< 'a, &'a i32 > + 'a >
  {
    match &src
    {
      Some( src ) => Box :: new( src.iter() ),
      _ => Box :: new( core :: iter :: empty() ),
    }
  }

  #[ test ]
  fn clone_iterator_with_some_data()
  {
    let data = vec![ 1, 2, 3 ];
    let iter = get_iter( Some( &data ) );
    let cloned = iter.clone();

    let count1 : usize = cloned.count();
    let count2 : usize = iter.count();

    assert_eq!( count1, 3 );
    assert_eq!( count2, 3 );
  }

  #[ test ]
  fn clone_iterator_with_none()
  {
    let iter = get_iter( None );
    let cloned = iter.clone();

    let count1 : usize = cloned.count();
    let count2 : usize = iter.count();

    assert_eq!( count1, 0, "Cloned empty iterator should have 0 elements" );
    assert_eq!( count2, 0, "Original empty iterator should have 0 elements" );
  }

  #[ test ]
  fn clone_iterator_independence()
  {
    let data = vec![ 10, 20, 30 ];
    let iter = get_iter( Some( &data ) );
    let cloned = iter.clone();

    // Consume cloned iterator
    let cloned_sum : i32 = cloned.copied().sum();

    // Original iterator should still be usable
    let original_sum : i32 = iter.copied().sum();

    assert_eq!( cloned_sum, 60 );
    assert_eq!( original_sum, 60 );
  }
}
