//! Smoke tests for `component_model_types` crate.
//!
//! These tests verify basic functionality without requiring `test_tools`
//! (which creates circular dependency).
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `smoke_test_assign_trait` | `Assign` trait implementation | `TestStruct` with empty string | String assigned via `Assign` trait | ✅ |
//! | `smoke_test_assign_with_type` | Explicit type assignment | `UserProfile` with empty username | Username assigned via `AssignWithType` | ✅ |
//! | `smoke_test_option_ext` | `Option` field assignment | `None` Option | `Option` becomes `Some` via `option_assign()` | ✅ |
//! | `smoke_test_crate_loads` | Basic compilation | No input | Crate loads and compiles successfully | ✅ |
//!
//! ## Corner Cases Covered
//!
//! - ✅ Basic `Assign` trait implementation with `Into` conversion
//! - ✅ Explicit type specification via `AssignWithType`
//! - ✅ `Option` extension (`None` → `Some` transition)
//! - ✅ Crate compilation verified via `Assign` + `AssignWithType` trait access

/// Verifies `Assign` trait basic functionality with `Into` conversion.
///
/// Tests that:
/// 1. Struct implements `Assign` trait correctly
/// 2. `String` assignment works via `Into` conversion
/// 3. `assign()` method mutates struct field as expected
///
/// Critical baseline test ensuring `Assign` trait pattern works for simple types.
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

/// Verifies `AssignWithType` trait enables explicit type specification.
///
/// Tests that:
/// 1. `AssignWithType` blanket implementation works for all types
/// 2. Explicit type specification via turbofish syntax succeeds
/// 3. Type inference delegates correctly to underlying `Assign` trait
///
/// Important for cases where type inference ambiguity requires explicit specification.
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

/// Verifies `OptionExt` trait simplifies `Option` field assignment.
///
/// Tests that:
/// 1. `OptionExt` sealed trait implementation works for `Option<T>`
/// 2. `option_assign()` creates `Some` from `None` state
/// 3. Assigned value correctly stored in `Option` wrapper
///
/// Critical for builder pattern where optional fields start as `None`.
/// Ensures `None` → `Some` transition works without explicit match/if-let.
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

/// Verifies crate compiles and core traits are accessible.
///
/// Tests that:
/// 1. Crate compiles successfully
/// 2. `Assign` trait accessible from public API
/// 3. `AssignWithType` trait accessible from public API
///
/// Public API accessible when `types_component_assign` feature is active.
#[ test ]
#[ cfg( all( feature = "enabled", feature = "types_component_assign" ) ) ]
fn smoke_test_crate_loads()
{
  // Verify Assign and AssignWithType traits are accessible
  use component_model_types :: { Assign, AssignWithType };
  struct Dummy { val: i32 }
  impl Assign< i32, i32 > for Dummy
  {
    fn assign( &mut self, component : i32 ) { self.val = component; }
  }
  let mut d = Dummy { val: 0 };
  d.assign_with_type :: < i32, i32 >( 42 );
  assert_eq!( d.val, 42 );
}
