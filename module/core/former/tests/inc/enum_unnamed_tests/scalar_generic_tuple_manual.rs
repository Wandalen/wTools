//! Purpose: This file provides a manual implementation of the `Former` pattern's static constructors
//! for an enum (`EnumScalarGeneric<T>`) with tuple variants containing generic types and bounds. It
//! demonstrates how the static constructors should behave for tuple variants involving generics,
//! including both scalar (direct value) and subformer (builder) styles, mirroring the behavior
//! tested in `scalar_generic_tuple_only_test.rs`.
//!
//! Coverage:
//! - Rule 3d (Tuple + Single-Field + Default): Manually implements the subformer behavior for a single-field tuple variant with generics, aligning with the test logic.
//! - Rule 3f (Tuple + Multi-Field + Default): Manually implements the subformer behavior for a multi-field tuple variant with generics, aligning with the test logic. Note: This contradicts the documented Rule 3f which states default for multi-field tuple is scalar. The manual implementation here reflects the current test behavior.
//! - Rule 1d (Tuple + Single-Field + `#[scalar]`): Manually implements the scalar constructor for a single-field tuple variant with generics, reflecting the test logic's expectation for `Variant1`.
//! - Rule 1f (Tuple + Multi-Field + `#[scalar]`): Not applicable, as the manual implementation for the multi-field variant uses a subformer, aligning with the test but not the documented rule for `#[scalar]`.
//! - Rule 4b (Option 2 Logic): Demonstrated by the manual implementation of the `Variant2` subformer.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumScalarGeneric<T: Bound>` with single-field (`Variant1`) and multi-field (`Variant2`) tuple variants, both containing generic types and bounds.
//! - Provides hand-written implementations of static methods (`variant_1`, `variant_2`) that mimic the behavior expected from the `#[derive(Former)]` macro for scalar and subformer constructors on these variants, specifically matching the expectations of `scalar_generic_tuple_only_test.rs`.
//! - Includes shared test logic from `scalar_generic_tuple_only_test.rs`.
//! - The tests in the included file call these manually implemented static methods.
//! - For `variant_1()`, the test expects a direct scalar return and uses `.into()`, verifying the manual implementation of the scalar constructor for a single-field tuple variant.
//! - For `variant_2()`, the test expects a former builder return, uses setters `._0()` and `._1()`, and calls `.form()`, verifying the manual implementation of the subformer for a multi-field tuple variant.
//! - Asserts that the resulting enum instances match manually constructed expected values.
//! - This file contains a hand-written former implementation and includes shared test logic via `include!("scalar_generic_tuple_only_test.rs")`.

// File: module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_manual.rs

// Imports testing infrastructure and potentially other common items
use former::{
  FormingEnd,
  StoragePreform,
  FormerDefinition,
  FormerDefinitionTypes,
  Storage,
  ReturnPreformed,
  FormerBegin,
  FormerMutator,
};
use std::marker::PhantomData;





// --- Enum Definition with Bounds ---
// Define the enum without the derive macro
#[ derive( Debug, PartialEq, Clone ) ]
pub enum EnumScalarGeneric<T: Bound>
{
  Variant1( InnerScalar< T > ), // Tuple variant with one generic field
  Variant2( InnerScalar< T >, bool ), // Tuple variant with generic and non-generic fields
}

// --- Manual Former Setup for Variant2 ---
// Needs to be generic over T: Bound
pub struct EnumScalarGenericVariant2FormerStorage< T : Bound >
{
  field0 : Option< InnerScalar< T > >,
  field1 : Option< bool >,
  _phantom : PhantomData< T >, // To use the generic parameter
}

impl< T : Bound > Default for EnumScalarGenericVariant2FormerStorage< T >
{
  fn default() -> Self
  {
    Self { field0 : None, field1 : None, _phantom : PhantomData }
  }
}

impl< T : Bound > Storage for EnumScalarGenericVariant2FormerStorage< T >
{
  type Preformed = ( InnerScalar< T >, bool );
}

impl< T : Bound + Default > StoragePreform for EnumScalarGenericVariant2FormerStorage< T >
{
  fn preform( mut self ) -> Self::Preformed
  {
    let field0 = self.field0.take().unwrap_or_default();
    let field1 = self.field1.take().unwrap_or_default();
    ( field0, field1 )
  }
}

#[ derive( Default, Debug ) ]
pub struct EnumScalarGenericVariant2FormerDefinitionTypes< T : Bound, C = (), F = EnumScalarGeneric< T > >
{
  _p : PhantomData< ( T, C, F ) >,
}

impl< T : Bound, C, F > FormerDefinitionTypes for EnumScalarGenericVariant2FormerDefinitionTypes< T, C, F >
{
  type Storage = EnumScalarGenericVariant2FormerStorage< T >;
  type Context = C;
  type Formed = F;
}

impl< T : Bound, C, F > FormerMutator for EnumScalarGenericVariant2FormerDefinitionTypes< T, C, F > {}

#[ derive( Default, Debug ) ]
pub struct EnumScalarGenericVariant2FormerDefinition< T : Bound, C = (), F = EnumScalarGeneric< T >, E = EnumScalarGenericVariant2End< T > >
{
  _p : PhantomData< ( T, C, F, E ) >,
}

impl< T : Bound, C, F, E > FormerDefinition for EnumScalarGenericVariant2FormerDefinition< T, C, F, E >
where
  E : FormingEnd< EnumScalarGenericVariant2FormerDefinitionTypes< T, C, F > >,
{
  type Storage = EnumScalarGenericVariant2FormerStorage< T >;
  type Context = C;
  type Formed = F;
  type Types = EnumScalarGenericVariant2FormerDefinitionTypes< T, C, F >;
  type End = E;
}

pub struct EnumScalarGenericVariant2Former< T : Bound, Definition = EnumScalarGenericVariant2FormerDefinition< T > >
where
  Definition : FormerDefinition< Storage = EnumScalarGenericVariant2FormerStorage< T > >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

impl< T : Bound, Definition > EnumScalarGenericVariant2Former< T, Definition >
where
  Definition : FormerDefinition< Storage = EnumScalarGenericVariant2FormerStorage< T > >,
{
  #[ inline( always ) ] pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed { self.end() }
  #[ inline( always ) ] pub fn end( mut self ) -> < Definition::Types as FormerDefinitionTypes >::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    < Definition::Types as FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
    on_end.call( self.storage, context )
  }
  #[ inline( always ) ] pub fn begin
  ( storage : Option< Definition::Storage >, context : Option< Definition::Context >, on_end : Definition::End ) -> Self
  { Self { storage : storage.unwrap_or_default(), context, on_end : Some( on_end ) } }
  #[ allow( dead_code ) ]
  #[ inline( always ) ] pub fn new( on_end : Definition::End ) -> Self { Self::begin( None, None, on_end ) }

  // Setters for fields
  #[ inline ] pub fn _0( mut self, src : impl Into< InnerScalar< T > > ) -> Self
  { self.storage.field0 = Some( src.into() ); self }
  #[ inline ] pub fn _1( mut self, src : impl Into< bool > ) -> Self
  { self.storage.field1 = Some( src.into() ); self }
}

#[ derive( Default, Debug ) ]
pub struct EnumScalarGenericVariant2End< T : Bound >
{
  _phantom : PhantomData< T >,
}

impl< T : Bound > FormingEnd< EnumScalarGenericVariant2FormerDefinitionTypes< T, (), EnumScalarGeneric< T > > >
for EnumScalarGenericVariant2End< T >
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : EnumScalarGenericVariant2FormerStorage< T >,
    _context : Option< () >,
  )
  -> EnumScalarGeneric< T >
  {
    let ( field0, field1 ) = sub_storage.preform();
    EnumScalarGeneric::Variant2( field0, field1 )
  }
}
// --- End Manual Former Setup for Variant2 ---


// --- Manual implementation of static methods on EnumScalarGeneric ---
impl< T : Bound > EnumScalarGeneric< T > // Apply bounds from enum definition
{
  /// Manually implemented constructor for the Variant1 variant (scalar style).
  #[ inline( always ) ]
  // FIX: Renamed to snake_case
  pub fn variant_1( value : impl Into< InnerScalar< T > > ) -> Self
  {
    Self::Variant1( value.into() )
  }

  /// Manually implemented former builder for the Variant2 variant.
  #[ inline( always ) ]
  pub fn variant_2() -> EnumScalarGenericVariant2Former< T >
  {
    EnumScalarGenericVariant2Former::begin( None, None, EnumScalarGenericVariant2End::< T >::default() )
  }
}

// --- Include the Test Logic ---
// This file contains the actual #[ test ] functions.
include!( "scalar_generic_tuple_only_test.rs" );