//! Corner case tests for `component_model_types` crate.
//!
//! Tests edge cases, boundary conditions, and advanced usage patterns
//! not covered in smoke tests.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `test_impute_builder_pattern` | Builder pattern with `impute()` | Struct with multiple fields | Method chaining works, fields assigned | ✅ |
//! | `test_multiple_assignments_same_field` | Reassign same field multiple times | Assign value 3 times | Last value retained | ✅ |
//! | `test_option_ext_some_to_some` | Update existing `Some` value | `Some(10)` → `Some(20)` | Value updated via `option_assign()` | ✅ |
//! | `test_assign_empty_string` | Assign empty string | Empty `""` | Empty string stored correctly | ✅ |
//! | `test_type_conversion_integers` | Type conversions for integers | `u8`, `i16` → `i32` | All conversions work via `Into` | ✅ |
//!
//! ## Corner Cases Covered
//!
//! - ✅ Builder pattern method chaining with `impute()`
//! - ✅ Multiple reassignments to same field
//! - ✅ `OptionExt` with `Some` → `Some` transition (not just `None` → `Some`)
//! - ✅ Empty string edge case
//! - ✅ Type conversions with different integer types

/// Verifies `impute()` method enables builder pattern with method chaining.
///
/// Tests that:
/// 1. `impute()` returns self for chaining
/// 2. Multiple `impute()` calls can be chained
/// 3. Final struct has all values assigned correctly
///
/// Critical for fluent API / builder pattern usage.
#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn test_impute_builder_pattern()
{
  use component_model_types::Assign;

  #[ derive( Default, PartialEq, Debug ) ]
  struct Config
  {
    timeout : i32,
    retry_count : i32,
    endpoint : String,
  }

  impl< IntoT > Assign< i32, IntoT > for Config
  where
    IntoT : Into< i32 >,
  {
    fn assign( &mut self, component : IntoT )
    {
      let val = component.into();
      if self.timeout == 0
      {
        self.timeout = val;
      }
      else
      {
        self.retry_count = val;
      }
    }
  }

  impl< IntoT > Assign< String, IntoT > for Config
  where
    IntoT : Into< String >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.endpoint = component.into();
    }
  }

  let config = Config::default()
    .impute( 30 )
    .impute( 3 )
    .impute( "https://api.example.com" );

  assert_eq!( config.timeout, 30 );
  assert_eq!( config.retry_count, 3 );
  assert_eq!( config.endpoint, "https://api.example.com" );
}

/// Verifies multiple assignments to same field work correctly.
///
/// Tests that:
/// 1. Field can be reassigned multiple times
/// 2. Last assignment wins
/// 3. No side effects from previous assignments
///
/// Ensures `Assign` trait doesn't accumulate state incorrectly.
#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn test_multiple_assignments_same_field()
{
  use component_model_types::Assign;

  #[ derive( Default, Debug ) ]
  struct Counter
  {
    value : i32,
  }

  impl< IntoT > Assign< i32, IntoT > for Counter
  where
    IntoT : Into< i32 >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.value = component.into();
    }
  }

  let mut counter = Counter::default();

  counter.assign( 10 );
  assert_eq!( counter.value, 10 );

  counter.assign( 20 );
  assert_eq!( counter.value, 20 );

  counter.assign( 30 );
  assert_eq!( counter.value, 30 );
}

/// Verifies `OptionExt::option_assign()` updates existing `Some` value.
///
/// Tests that:
/// 1. `option_assign()` on `Some(T)` updates value (not just `None` → `Some`)
/// 2. Updated value replaces previous value
/// 3. `Option` remains `Some` after update
///
/// Critical edge case: many implementations only test `None` → `Some` transition.
#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn test_option_ext_some_to_some()
{
  use component_model_types::{ Assign, OptionExt };

  #[ derive( Debug, PartialEq ) ]
  struct Data
  {
    value : i32,
  }

  impl< IntoT > Assign< Data, IntoT > for Data
  where
    IntoT : Into< Data >,
  {
    fn assign( &mut self, component : IntoT )
    {
      *self = component.into();
    }
  }

  impl From< i32 > for Data
  {
    fn from( value : i32 ) -> Self
    {
      Data { value }
    }
  }

  let mut opt_data : Option< Data > = Some( Data { value : 10 } );
  assert_eq!( opt_data.as_ref().unwrap().value, 10 );

  opt_data.option_assign( Data { value : 20 } );
  assert!( opt_data.is_some() );
  assert_eq!( opt_data.unwrap().value, 20 );
}

/// Verifies empty string assignment works correctly.
///
/// Tests that:
/// 1. Empty string `""` can be assigned
/// 2. Field correctly stores empty string (not null/None)
/// 3. Comparison with empty string succeeds
///
/// Edge case: empty values sometimes reveal initialization bugs.
#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn test_assign_empty_string()
{
  use component_model_types::Assign;

  #[ derive( Default, Debug ) ]
  struct Record
  {
    description : String,
  }

  impl< IntoT > Assign< String, IntoT > for Record
  where
    IntoT : Into< String >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.description = component.into();
    }
  }

  let mut record = Record::default();
  record.assign( "" );

  assert_eq!( record.description, "" );
  assert_eq!( record.description.len(), 0 );
}

/// Verifies type conversions work for different integer types.
///
/// Tests that:
/// 1. `u8` → `i32` conversion works
/// 2. `i16` → `i32` conversion works
/// 3. `Into` trait enables flexible type acceptance
///
/// Ensures generic `IntoT : Into<T>` pattern works across type boundaries.
#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn test_type_conversion_integers()
{
  use component_model_types::Assign;

  #[ derive( Default, Debug ) ]
  struct Metrics
  {
    count : i32,
  }

  impl< IntoT > Assign< i32, IntoT > for Metrics
  where
    IntoT : Into< i32 >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.count = component.into();
    }
  }

  let mut metrics = Metrics::default();

  // Test u8 → i32
  let val_u8 : u8 = 255;
  metrics.assign( val_u8 );
  assert_eq!( metrics.count, 255 );

  // Test i16 → i32
  let val_i16 : i16 = -1000;
  metrics.assign( val_i16 );
  assert_eq!( metrics.count, -1000 );

  // Test direct i32
  metrics.assign( 42 );
  assert_eq!( metrics.count, 42 );
}
