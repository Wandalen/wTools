// File: module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_only_test.rs

/// # Test Logic: #[scalar] Attribute on Generic Tuple Variants
///
/// This file contains the core test logic for verifying the `Former` derive macro's
/// handling of enums where a tuple variant containing generic types is explicitly marked
/// with the `#[scalar]` attribute.
///
/// ## Purpose:
///
/// - **Verify Direct Constructor Generation:** Ensure that `#[derive(Former)]` generates a direct
///   static constructor method (e.g., `Enum::variant_name(InnerType<T>) -> Enum<T>`) for tuple
///   variants marked with `#[scalar]`, instead of a subformer starter.
/// - **Verify Generic Handling in Constructor:** Confirm that the generated constructor correctly
///   handles the enum's generic parameters (`T`) and any generics within the tuple variant's
///   data types (`InnerType<T>`), including applying necessary bounds from the enum definition.
/// - **Verify Multi-Field Tuple Handling:** Test the constructor generation for `#[scalar]` variants
///   with multiple fields, some or all of which might be generic.
///
/// This file is included via `include!` by both the `_manual.rs` and `_derive.rs`
/// test files for this scenario.

use super::*; // Imports items from the parent file (either manual or derive)
// use std::marker::PhantomData; // Keep PhantomData import needed for manual test case construction

// Define a simple bound for testing generics
pub trait Bound : core::fmt::Debug + Default + Clone + PartialEq {}

// Define a concrete type satisfying the bound
#[ derive( Debug, Default, Clone, PartialEq ) ]
pub struct MyType( String );
impl Bound for MyType {}

// Define an inner generic struct to be used within the enum variants
#[ derive( Debug, Clone, PartialEq, Default ) ]
pub struct InnerScalar< T : Bound >
{
  pub data : T,
}
// Implement Into manually for testing the constructor signature
impl< T : Bound > From< T > for InnerScalar< T >
{
  fn from( data : T ) -> Self { Self { data } }
}


#[ test ]
fn scalar_on_single_generic_tuple_variant()
{
  // Tests the direct constructor generated for a single-field tuple variant
  // `Variant1(InnerScalar<T>)` marked with `#[scalar]`.
  let inner_data = InnerScalar { data: MyType( "value1".to_string() ) };
  // Expect a direct static constructor `variant_1` taking `impl Into<InnerScalar<MyType>>`
  // FIX: Changed call to snake_case
  let got = EnumScalarGeneric::< MyType >::variant_1( inner_data.clone() );

  let expected = EnumScalarGeneric::< MyType >::Variant1( inner_data );
  assert_eq!( got, expected );

  // Test with Into
  // FIX: Changed call to snake_case
  let got_into = EnumScalarGeneric::< MyType >::variant_1( MyType( "value1_into".to_string() ) );
   let expected_into = EnumScalarGeneric::< MyType >::Variant1( InnerScalar { data: MyType( "value1_into".to_string() ) } );
  assert_eq!( got_into, expected_into );
}

#[ test ]
fn scalar_on_multi_generic_tuple_variant()
{
  // Tests the former builder generated for a multi-field tuple variant
  // `Variant2(InnerScalar<T>, bool)` marked with `#[scalar]`.
  let inner_data = InnerScalar { data: MyType( "value2".to_string() ) };
  // Expect a former builder `variant_2` with setters `_0` and `_1`
  let got = EnumScalarGeneric::< MyType >::variant_2()
    ._0( inner_data.clone() )
    ._1( true )
    .form();

  let expected = EnumScalarGeneric::< MyType >::Variant2( inner_data, true );
  assert_eq!( got, expected );

  // Test with Into
  let got_into = EnumScalarGeneric::< MyType >::variant_2()
    ._0( MyType( "value2_into".to_string() ) )
    ._1( false )
    .form();
  let expected_into = EnumScalarGeneric::< MyType >::Variant2( InnerScalar { data: MyType( "value2_into".to_string() ) }, false );
  assert_eq!( got_into, expected_into );
}