//! Smoke tests for `component_model_types` crate.
//!
//! These tests verify basic functionality without requiring `test_tools`
//! (which creates circular dependency).

#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn smoke_test_assign_trait()
{
  use component_model_types :: Assign;

  struct TestStruct
  {
    value : String,
  }

  impl< IntoT : Into< String > > Assign< String, IntoT > for TestStruct
  {
    fn assign( &mut self, component : IntoT )
    {
      self.value = component.into();
    }
  }

  let mut obj = TestStruct { value : String :: new() };
  obj.assign( "test_value" );
  assert_eq!( obj.value, "test_value" );
}

#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn smoke_test_assign_with_type()
{
  use component_model_types :: { Assign, AssignWithType };

  struct UserProfile
  {
    username : String,
  }

  impl< IntoT : Into< String > > Assign< String, IntoT > for UserProfile
  {
    fn assign( &mut self, component : IntoT )
    {
      self.username = component.into();
    }
  }

  let mut user = UserProfile { username : String :: new() };
  user.assign_with_type ::< String, _ >( "alice" );
  assert_eq!( user.username, "alice" );
}

#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn smoke_test_option_ext()
{
  use component_model_types :: { Assign, OptionExt };

  struct MyStruct
  {
    name : String,
  }

  impl< IntoT : Into< MyStruct > > Assign< MyStruct, IntoT > for MyStruct
  {
    fn assign( &mut self, component : IntoT )
    {
      self.name = component.into().name;
    }
  }

  let mut opt_struct : Option< MyStruct > = None;
  opt_struct.option_assign( MyStruct { name : "test_name".to_string() } );
  assert!( opt_struct.is_some() );
  assert_eq!( opt_struct.unwrap().name, "test_name" );
}

#[ test ]
fn smoke_test_crate_loads()
{
  // Minimal test that crate compiles and loads
  // This test always runs regardless of features
  let _ = component_model_types :: dependency :: collection_tools :: Vec ::< i32 > :: new();
}
