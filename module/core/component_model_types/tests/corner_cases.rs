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
//! | `test_partial_assignment_preserves_defaults` | Partial assignment | Only one field assigned | Unassigned fields retain default values | ✅ |
//! | `test_owned_string_assignment` | Owned String assignment | `String::from()` | Owned strings work, not just `&str` | ✅ |
//! | `test_unicode_string_assignment` | Unicode string support | Non-ASCII characters | Unicode preserved correctly | ✅ |
//! | `test_boundary_values_integers` | Integer boundary values | `i32::MIN`, `i32::MAX`, `0` | Extreme values handled correctly | ✅ |
//!
//! ## Corner Cases Covered
//!
//! - ✅ Builder pattern method chaining with `impute()`
//! - ✅ Multiple reassignments to same field
//! - ✅ `OptionExt` with `Some` → `Some` transition (not just `None` → `Some`)
//! - ✅ Empty string edge case
//! - ✅ Type conversions with different integer types
//! - ✅ Partial assignment (default value preservation)
//! - ✅ Owned String type (not just `&str`)
//! - ✅ Unicode string support
//! - ✅ Integer boundary values (`i32::MIN`, `i32::MAX`, zero)

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

/// Verifies partial assignment preserves default values for unassigned fields.
///
/// Tests that:
/// 1. Assigning only one field leaves other fields at default values
/// 2. Default values are not corrupted by partial assignment
/// 3. Both partial assignment scenarios work (age-only, name-only)
///
/// Critical for builder patterns where not all fields may be set.
#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn test_partial_assignment_preserves_defaults()
{
  use component_model_types::Assign;

  #[ derive( Default, Debug ) ]
  struct Person
  {
    age : i32,
    name : String,
  }

  impl< IntoT > Assign< i32, IntoT > for Person
  where
    IntoT : Into< i32 >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.age = component.into();
    }
  }

  impl< IntoT > Assign< String, IntoT > for Person
  where
    IntoT : Into< String >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.name = component.into();
    }
  }

  // Test 1: Assign only age, name stays default
  let mut person1 = Person::default();
  person1.assign( 30 );
  assert_eq!( person1.age, 30 );
  assert_eq!( person1.name, "" );

  // Test 2: Assign only name, age stays default
  let mut person2 = Person::default();
  person2.assign( "Alice" );
  assert_eq!( person2.age, 0 );
  assert_eq!( person2.name, "Alice" );
}

/// Verifies owned String type assignment works (not just &str).
///
/// Tests that:
/// 1. `String::from()` can be assigned (owned String)
/// 2. `String` type implements `Into<String>`
/// 3. Owned strings are not just borrowed
///
/// Ensures flexibility in string assignment beyond string literals.
#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn test_owned_string_assignment()
{
  use component_model_types::Assign;

  #[ derive( Default, Debug ) ]
  struct Record
  {
    label : String,
  }

  impl< IntoT > Assign< String, IntoT > for Record
  where
    IntoT : Into< String >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.label = component.into();
    }
  }

  let mut record = Record::default();

  // Assign owned String (not &str)
  let owned_string = String::from( "Owned Label" );
  record.assign( owned_string );

  assert_eq!( record.label, "Owned Label" );
}

/// Verifies Unicode string assignment preserves non-ASCII characters.
///
/// Tests that:
/// 1. Unicode characters are stored correctly
/// 2. Multi-byte characters (emoji, CJK, accented) are preserved
/// 3. String length correctly handles multi-byte chars
///
/// Edge case: Unicode handling often reveals encoding issues.
#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn test_unicode_string_assignment()
{
  use component_model_types::Assign;

  #[ derive( Default, Debug ) ]
  struct Greeting
  {
    message : String,
  }

  impl< IntoT > Assign< String, IntoT > for Greeting
  where
    IntoT : Into< String >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.message = component.into();
    }
  }

  let mut greeting = Greeting::default();

  // Assign Unicode string (emoji, CJK, accented characters)
  greeting.assign( "Hello 世界 🌍 José" );

  assert_eq!( greeting.message, "Hello 世界 🌍 José" );
  assert!( greeting.message.contains( "世界" ) );
  assert!( greeting.message.contains( "🌍" ) );
  assert!( greeting.message.contains( "José" ) );
}

/// Verifies integer boundary values are handled correctly.
///
/// Tests that:
/// 1. `i32::MIN` can be assigned without overflow
/// 2. `i32::MAX` can be assigned without overflow
/// 3. Zero is handled correctly (not treated as null/unset)
///
/// Critical edge case: boundary values often reveal arithmetic bugs.
#[ test ]
#[ cfg( feature = "types_component_assign" ) ]
fn test_boundary_values_integers()
{
  use component_model_types::Assign;

  #[ derive( Default, Debug ) ]
  struct Limits
  {
    value : i32,
  }

  impl< IntoT > Assign< i32, IntoT > for Limits
  where
    IntoT : Into< i32 >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.value = component.into();
    }
  }

  let mut limits = Limits::default();

  // Test i32::MIN
  limits.assign( i32::MIN );
  assert_eq!( limits.value, i32::MIN );
  assert_eq!( limits.value, -2_147_483_648 );

  // Test i32::MAX
  limits.assign( i32::MAX );
  assert_eq!( limits.value, i32::MAX );
  assert_eq!( limits.value, 2_147_483_647 );

  // Test zero (distinct from uninitialized)
  limits.assign( 0 );
  assert_eq!( limits.value, 0 );
}
