// File: module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_manual.rs

//! # Manual Test: #[standalone_constructors] on Multi-Field Tuple Variants (Returns Former)
//!
//! This file provides a manual implementation of the standalone constructor that returns a Former
//! for an enum (`TestEnumMultiStandalone`) with a multi-field tuple variant (`VariantMultiStandalone(i32, bool)`),
//! demonstrating the expected behavior under `#[standalone_constructors]` without `#[arg_for_constructor]`.
//!
//! ## Purpose:
//!
//! - To serve as a reference implementation demonstrating how the standalone constructor should
//!   behave for multi-field tuple variants when it returns a Former instance.
//! - To manually implement the necessary Former infrastructure and the standalone constructor
//!   function (`variant_multi_standalone`).
//! - To validate the logic used by the `#[derive(Former)]` macro by comparing its generated
//!   code's behavior against this manual implementation using the shared tests in
//!   `tuple_multi_standalone_only_test.rs`.

#[ allow( unused_imports ) ]
use ::former::prelude::*;
#[ allow( unused_imports ) ]
use ::former_types::
{
  Storage, StoragePreform,
  FormerDefinitionTypes, FormerMutator, FormerDefinition,
  FormingEnd, ReturnPreformed,
};
use std::marker::PhantomData;

// === Enum Definition ===

/// Enum for manual testing of #[standalone_constructors] on multi-field tuple variants.
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnumMultiStandalone // Consistent name
{
  /// A multi-field tuple variant.
  VariantMultiStandalone( i32, bool ), // Multi-field tuple variant
}

// === Manual Former Implementation for VariantMultiStandalone ===

// Storage
#[ derive( Debug, Default ) ]
pub struct TestEnumMultiStandaloneVariantFormerStorage
{
  pub _0 : ::core::option::Option< i32 >,
  pub _1 : ::core::option::Option< bool >,
}

impl Storage for TestEnumMultiStandaloneVariantFormerStorage
{
  type Preformed = ( i32, bool );
}

impl StoragePreform for TestEnumMultiStandaloneVariantFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    ( self._0.take().unwrap_or_default(), self._1.take().unwrap_or_default() )
  }
}

// Definition Types
#[ derive( Debug, Default ) ]
pub struct TestEnumMultiStandaloneVariantFormerDefinitionTypes< Context = (), Formed = TestEnumMultiStandalone >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > FormerDefinitionTypes
for TestEnumMultiStandaloneVariantFormerDefinitionTypes< Context, Formed >
{
  type Storage = TestEnumMultiStandaloneVariantFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Mutator
impl< Context, Formed > FormerMutator
for TestEnumMultiStandaloneVariantFormerDefinitionTypes< Context, Formed >
{
}

// Definition
#[ derive( Debug, Default ) ]
pub struct TestEnumMultiStandaloneVariantFormerDefinition
< Context = (), Formed = TestEnumMultiStandalone, End = TestEnumMultiStandaloneVariantEnd >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > FormerDefinition
for TestEnumMultiStandaloneVariantFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< TestEnumMultiStandaloneVariantFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = TestEnumMultiStandaloneVariantFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = TestEnumMultiStandaloneVariantFormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// Former
#[ derive( Debug ) ]
pub struct TestEnumMultiStandaloneVariantFormer< Definition = TestEnumMultiStandaloneVariantFormerDefinition >
where
  Definition : FormerDefinition< Storage = TestEnumMultiStandaloneVariantFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

impl< Definition > TestEnumMultiStandaloneVariantFormer< Definition >
where
  Definition : FormerDefinition< Storage = TestEnumMultiStandaloneVariantFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = TestEnumMultiStandaloneVariantFormerStorage >,
  Definition::Types : FormerMutator,
{
  #[ inline( always ) ]
  pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed
  {
    self.end()
  }

  #[ inline( always ) ]
  pub fn end( mut self ) -> < Definition::Types as FormerDefinitionTypes >::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    < Definition::Types as FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
    on_end.call( self.storage, context )
  }

  #[ inline( always ) ]
  pub fn begin
  (
    storage : Option< Definition::Storage >,
    context : Option< Definition::Context >,
    on_end : Definition::End,
  ) -> Self
  {
    Self { storage : storage.unwrap_or_default(), context, on_end : Some( on_end ) }
  }

  #[ inline( always ) ]
  #[allow(dead_code)]
  pub fn new( on_end : Definition::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }

  /// Setter for the first tuple field.
  #[ inline ]
  pub fn _0( mut self, src : impl Into< i32 > ) -> Self
  {
    debug_assert!( self.storage._0.is_none(), "Field '_0' was already set" );
    self.storage._0 = Some( src.into() );
    self
  }

  /// Setter for the second tuple field.
  #[ inline ]
  pub fn _1( mut self, src : impl Into< bool > ) -> Self
  {
    debug_assert!( self.storage._1.is_none(), "Field '_1' was already set" );
    self.storage._1 = Some( src.into() );
    self
  }
}

// End Struct for VariantMultiStandalone
#[ derive( Debug, Default ) ]
pub struct TestEnumMultiStandaloneVariantEnd;

impl FormingEnd< TestEnumMultiStandaloneVariantFormerDefinitionTypes< (), TestEnumMultiStandalone > >
for TestEnumMultiStandaloneVariantEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : TestEnumMultiStandaloneVariantFormerStorage,
    _context : Option< () >,
  ) -> TestEnumMultiStandalone
  {
    let ( val0, val1 ) = storage.preform();
    TestEnumMultiStandalone::VariantMultiStandalone( val0, val1 )
  }
}

// === Standalone Constructor (Manual) ===

/// Manual standalone constructor for TestEnumMultiStandalone::VariantMultiStandalone.
/// Returns a Former instance for the variant.
pub fn variant_multi_standalone()
->
TestEnumMultiStandaloneVariantFormer< TestEnumMultiStandaloneVariantFormerDefinition< (), TestEnumMultiStandalone, TestEnumMultiStandaloneVariantEnd > >
{
  TestEnumMultiStandaloneVariantFormer::begin( None, None, TestEnumMultiStandaloneVariantEnd )
}


// === Include Test Logic ===
include!( "tuple_multi_standalone_only_test.rs" );