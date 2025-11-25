//! Smoke testing of the package.
//!
//! Basic smoke tests to verify core functionality without external dependencies.
//! Note: `test_tools` dependency removed due to circular dependency
//! (`macro_tools` → `clone_dyn_types` → `test_tools` → `impls_index_meta` → `macro_tools`).

#[ test ]
fn smoke_test_basic_clone()
{
  // Test basic cloning of a simple type
  #[ derive( Clone, Debug, PartialEq ) ]
  struct TestStruct
  {
    value : i32,
  }

  let original = TestStruct { value : 42 };
  let cloned = clone_dyn_types :: clone( &original );

  assert_eq!( original.value, cloned.value );
}

#[ test ]
fn smoke_test_clone_into_box()
{
  // Test cloning into Box with trait object
  use clone_dyn_types :: { CloneDyn, clone_into_box };

  trait TestTrait : CloneDyn
  {
    fn get_value( &self ) -> i32;
  }

  #[ derive( Clone ) ]
  struct TestImpl
  {
    value : i32,
  }

  impl TestTrait for TestImpl
  {
    fn get_value( &self ) -> i32
    {
      self.value
    }
  }

  #[ allow( non_local_definitions ) ]
  impl Clone for Box< dyn TestTrait >
  {
    fn clone( &self ) -> Self
    {
      clone_into_box( &**self )
    }
  }

  let original : Box< dyn TestTrait > = Box :: new( TestImpl { value : 100 } );
  let cloned = original.clone();

  assert_eq!( original.get_value(), cloned.get_value() );
  assert_eq!( cloned.get_value(), 100 );
}
