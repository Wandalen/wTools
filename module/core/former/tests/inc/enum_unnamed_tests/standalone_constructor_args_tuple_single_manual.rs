#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
// File: module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_args_tuple_single_manual.rs

#[ allow( unused_imports ) ]
use ::former::prelude::*;
#[ allow( unused_imports ) ]
use ::former_types::
{
  Storage, StoragePreform,
  FormerDefinitionTypes, FormerMutator, FormerDefinition,
  FormingEnd, ReturnPreformed,
};
use core::marker::PhantomData;

// === Enum Definition ===

/// Enum for manual testing of standalone constructors with arguments (single tuple variant).
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnumArgs // New name
{
  /// A tuple variant with one field (intended as constructor arg).
  TupleVariantArgs( i32 ), // New name
}

// === Manual Former Implementation for TupleVariantArgs ===

// Storage
/// Storage for `TestEnumArgsTupleVariantArgsFormer`.
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsTupleVariantArgsFormerStorage
{
  /// Option to store the value for the tuple field.
  pub _0 : ::core::option::Option< i32 >,
}

impl Storage for TestEnumArgsTupleVariantArgsFormerStorage
{
  type Preformed = i32;
}

impl StoragePreform for TestEnumArgsTupleVariantArgsFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    // Should ideally panic if None and not defaulted by constructor arg,
    // but for manual test, assume it's set.
    self._0.take().unwrap_or_default()
  }
}

// Definition Types
/// Definition types for `TestEnumArgsTupleVariantArgsFormer`.
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsTupleVariantArgsFormerDefinitionTypes< Context = (), Formed = TestEnumArgs >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > FormerDefinitionTypes
for TestEnumArgsTupleVariantArgsFormerDefinitionTypes< Context, Formed >
{
  type Storage = TestEnumArgsTupleVariantArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Mutator
impl< Context, Formed > FormerMutator
for TestEnumArgsTupleVariantArgsFormerDefinitionTypes< Context, Formed >
{
}

// Definition
/// Definition for `TestEnumArgsTupleVariantArgsFormer`.
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsTupleVariantArgsFormerDefinition
< Context = (), Formed = TestEnumArgs, End = TestEnumArgsTupleVariantArgsEnd >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > FormerDefinition
for TestEnumArgsTupleVariantArgsFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< TestEnumArgsTupleVariantArgsFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = TestEnumArgsTupleVariantArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = TestEnumArgsTupleVariantArgsFormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// Former
/// Manual Former implementation for `TestEnumArgs::TupleVariantArgs`.
#[ derive( Debug ) ]
pub struct TestEnumArgsTupleVariantArgsFormer
< Definition = TestEnumArgsTupleVariantArgsFormerDefinition >
where
  Definition : FormerDefinition< Storage = TestEnumArgsTupleVariantArgsFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

impl< Definition > TestEnumArgsTupleVariantArgsFormer< Definition >
where
  Definition : FormerDefinition< Storage = TestEnumArgsTupleVariantArgsFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = TestEnumArgsTupleVariantArgsFormerStorage >,
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
  #[ allow( dead_code ) ]
  pub fn new( on_end : Definition::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }

  /// Setter for the tuple field.
  #[ inline ]
  pub fn _0( mut self, src : impl Into< i32 > ) -> Self
  {
    // debug_assert!( self.storage._0.is_none(), "Field '_0' was already set" );
    self.storage._0 = Some( src.into() );
    self
  }
}

// End Struct for TupleVariantArgs
/// End handler for `TestEnumArgsTupleVariantArgsFormer`.
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsTupleVariantArgsEnd;

impl FormingEnd< TestEnumArgsTupleVariantArgsFormerDefinitionTypes< (), TestEnumArgs > >
for TestEnumArgsTupleVariantArgsEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : TestEnumArgsTupleVariantArgsFormerStorage,
    _context : Option< () >,
  ) -> TestEnumArgs
  {
    let val = storage.preform();
    TestEnumArgs::TupleVariantArgs( val )
  }
}


// === Standalone Constructors (Manual - Argument Taking) ===

/// Manual standalone constructor for `TestEnumArgs::TupleVariantArgs` (takes arg).
/// Returns Self directly as per Option 2.
#[ allow( clippy::just_underscores_and_digits ) ] // _0 is conventional for tuple field access
pub fn tuple_variant_args( _0 : impl Into< i32 > ) -> TestEnumArgs // Changed return type
{
  TestEnumArgs::TupleVariantArgs( _0.into() ) // Direct construction
}

// === Include Test Logic ===
// Note: Only including the single-field test since this manual implementation only has TupleVariantArgs

#[ test ]
fn tuple_variant_args_test()
{
  // Test the single field with standalone constructor
  let instance = tuple_variant_args( 202 );
  let expected = TestEnumArgs::TupleVariantArgs( 202 );
  assert_eq!( instance, expected );
}