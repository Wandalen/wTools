#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Purpose: Provides a hand-written implementation of the `Former` pattern's implicit variant former
//! for a multi-field tuple variant (`Variant(u32, String)`) within an enum, demonstrating the manual
//! implementation corresponding to the default behavior when no specific variant attribute is applied.
//!
//! Coverage:
//! - Rule 3f (Tuple + Multi-Field + Default): Manually implements the implicit variant former for a multi-field tuple variant, returning a former with setters like ._`0()` and ._`1()`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `TestEnum` with a multi-field tuple variant `Variant(u32, String)`.
//! - Provides a hand-written static method `TestEnum::variant()` that returns a former with setters ._`0()` and ._`1()` and a .`form()` method.
//! - Includes shared test logic from `tuple_multi_default_only_test.rs`.
//! - The included test calls this manually implemented static method, uses the setters, and calls .`form()`. This verifies the manual implementation of the default implicit variant former for a multi-field tuple variant.

// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_default_manual.rs

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
use core::marker::PhantomData;

// Define the enum without the derive macro
#[ derive( Debug, PartialEq ) ]
pub enum TestEnum
{
  Variant( u32, String ),
}

// --- Manual Former Setup for Variant ---
#[ derive( Default ) ]
pub struct TestEnumVariantFormerStorage
{
  field0 : Option< u32 >,
  field1 : Option< String >,
}


impl Storage for TestEnumVariantFormerStorage
{
  type Preformed = ( u32, String );
}

impl StoragePreform for TestEnumVariantFormerStorage
{
  fn preform( mut self ) -> Self::Preformed
  {
    let field0 = self.field0.take().unwrap_or_default();
    let field1 = self.field1.take().unwrap_or_default();
    ( field0, field1 )
  }
}

#[ derive( Default, Debug ) ]
pub struct TestEnumVariantFormerDefinitionTypes< C = (), F = TestEnum >
{
  _p : PhantomData< ( C, F ) >,
}

impl< C, F > FormerDefinitionTypes for TestEnumVariantFormerDefinitionTypes< C, F >
{
  type Storage = TestEnumVariantFormerStorage;
  type Context = C;
  type Formed = F;
}

impl< C, F > FormerMutator for TestEnumVariantFormerDefinitionTypes< C, F > {}

#[ derive( Default, Debug ) ]
pub struct TestEnumVariantFormerDefinition< C = (), F = TestEnum, E = TestEnumVariantEnd >
{
  _p : PhantomData< ( C, F, E ) >,
}

impl< C, F, E > FormerDefinition for TestEnumVariantFormerDefinition< C, F, E >
where
  E : FormingEnd< TestEnumVariantFormerDefinitionTypes< C, F > >,
{
  type Storage = TestEnumVariantFormerStorage;
  type Context = C;
  type Formed = F;
  type Types = TestEnumVariantFormerDefinitionTypes< C, F >;
  type End = E;
}

pub struct TestEnumVariantFormer< Definition = TestEnumVariantFormerDefinition >
where
  Definition : FormerDefinition< Storage = TestEnumVariantFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

impl< Definition > TestEnumVariantFormer< Definition >
where
  Definition : FormerDefinition< Storage = TestEnumVariantFormerStorage >,
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
  #[ inline ] pub fn _0( mut self, src : impl Into< u32 > ) -> Self
  { self.storage.field0 = Some( src.into() ); self }
  #[ inline ] pub fn _1( mut self, src : impl Into< String > ) -> Self
  { self.storage.field1 = Some( src.into() ); self }
}

#[ derive( Default, Debug ) ]
pub struct TestEnumVariantEnd
{
}

impl FormingEnd< TestEnumVariantFormerDefinitionTypes< (), TestEnum > >
for TestEnumVariantEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : TestEnumVariantFormerStorage,
    _context : Option< () >,
  )
  -> TestEnum
  {
    let ( field0, field1 ) = sub_storage.preform();
    TestEnum::Variant( field0, field1 )
  }
}
// --- End Manual Former Setup for Variant ---

// Manually implement the static method for the variant
impl TestEnum
{
  /// Manually implemented constructor for the Variant variant (implicit variant former style).
  #[ inline( always ) ]
  pub fn variant() -> TestEnumVariantFormer
  {
    TestEnumVariantFormer::begin( None, None, TestEnumVariantEnd::default() )
  }
}

include!( "tuple_multi_default_only_test.rs" );