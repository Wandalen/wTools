// File: module/core/former/tests/inc/enum_named_tests/standalone_constructor_args_named_single_manual.rs

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

/// Enum for manual testing of standalone constructors with arguments (single named variant).
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnumArgs // New name
{
  /// A struct variant with one field (intended as constructor arg).
  StructVariantArgs // New name
  {
    field : String,
  },
}

// === Manual Former Implementation for StructVariantArgs ===

// Storage
/// Storage for TestEnumArgsStructVariantArgsFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsStructVariantArgsFormerStorage
{
  /// Option to store the value for the struct field.
  pub field : ::core::option::Option< String >,
}

impl Storage for TestEnumArgsStructVariantArgsFormerStorage
{
  type Preformed = String;
}

impl StoragePreform for TestEnumArgsStructVariantArgsFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    // Should ideally panic if None and not defaulted by constructor arg,
    // but for manual test, assume it's set.
    self.field.take().unwrap_or_default()
  }
}

// Definition Types
/// Definition types for TestEnumArgsStructVariantArgsFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsStructVariantArgsFormerDefinitionTypes< Context = (), Formed = TestEnumArgs >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > FormerDefinitionTypes
for TestEnumArgsStructVariantArgsFormerDefinitionTypes< Context, Formed >
{
  type Storage = TestEnumArgsStructVariantArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Mutator
impl< Context, Formed > FormerMutator
for TestEnumArgsStructVariantArgsFormerDefinitionTypes< Context, Formed >
{
}

// Definition
/// Definition for TestEnumArgsStructVariantArgsFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsStructVariantArgsFormerDefinition
< Context = (), Formed = TestEnumArgs, End = TestEnumArgsStructVariantArgsEnd >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > FormerDefinition
for TestEnumArgsStructVariantArgsFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< TestEnumArgsStructVariantArgsFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = TestEnumArgsStructVariantArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = TestEnumArgsStructVariantArgsFormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// Former
/// Manual Former implementation for TestEnumArgs::StructVariantArgs.
#[ derive( Debug ) ]
pub struct TestEnumArgsStructVariantArgsFormer
< Definition = TestEnumArgsStructVariantArgsFormerDefinition >
where
  Definition : FormerDefinition< Storage = TestEnumArgsStructVariantArgsFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

impl< Definition > TestEnumArgsStructVariantArgsFormer< Definition >
where
  Definition : FormerDefinition< Storage = TestEnumArgsStructVariantArgsFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = TestEnumArgsStructVariantArgsFormerStorage >,
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

  /// Setter for the struct field.
  #[ inline ]
  #[allow(dead_code)]
  pub fn field( mut self, src : impl Into< String > ) -> Self
  {
    // debug_assert!( self.storage.field.is_none(), "Field 'field' was already set" );
    self.storage.field = Some( src.into() );
    self
  }
}

// End Struct for StructVariantArgs
/// End handler for TestEnumArgsStructVariantArgsFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsStructVariantArgsEnd;

impl FormingEnd< TestEnumArgsStructVariantArgsFormerDefinitionTypes< (), TestEnumArgs > >
for TestEnumArgsStructVariantArgsEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : TestEnumArgsStructVariantArgsFormerStorage,
    _context : Option< () >,
  ) -> TestEnumArgs
  {
    let val = storage.preform();
    TestEnumArgs::StructVariantArgs { field : val }
  }
}


// === Standalone Constructors (Manual - Argument Taking) ===

/// Manual standalone constructor for TestEnumArgs::StructVariantArgs (takes arg).
/// Returns Self directly as per Option 2.
pub fn struct_variant_args( field : impl Into< String > ) -> TestEnumArgs // Changed return type
{
  TestEnumArgs::StructVariantArgs { field : field.into() } // Direct construction
}

// === Include Test Logic ===
include!( "standalone_constructor_args_named_only_test.rs" );