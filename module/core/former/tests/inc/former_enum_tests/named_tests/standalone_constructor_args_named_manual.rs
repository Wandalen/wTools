// File: module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_args_named_manual.rs

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

/// Enum for manual testing of standalone constructors with arguments.
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnumArgs // New name
{
  /// A struct variant with one field (intended as constructor arg).
  StructVariantArgs // New name
  {
    field : String,
  },
  /// A struct variant with multiple fields (intended as constructor args).
  MultiStructArgs // <<< New Variant
  {
    a : i32,
    b : bool,
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


// === Manual Former Implementation for MultiStructArgs === <<< NEW >>>

// Storage
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiStructArgsFormerStorage
{
  pub a : ::core::option::Option< i32 >,
  pub b : ::core::option::Option< bool >,
}
impl Storage for TestEnumArgsMultiStructArgsFormerStorage
{
  type Preformed = ( i32, bool );
}
impl StoragePreform for TestEnumArgsMultiStructArgsFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    ( self.a.take().unwrap_or_default(), self.b.take().unwrap_or_default() )
  }
}
// Definition Types
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiStructArgsFormerDefinitionTypes
< Context = (), Formed = TestEnumArgs >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}
impl< Context, Formed > FormerDefinitionTypes
for TestEnumArgsMultiStructArgsFormerDefinitionTypes< Context, Formed >
{
  type Storage = TestEnumArgsMultiStructArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
}
impl< Context, Formed > FormerMutator
for TestEnumArgsMultiStructArgsFormerDefinitionTypes< Context, Formed >
{
}
// Definition
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiStructArgsFormerDefinition
< Context = (), Formed = TestEnumArgs, End = TestEnumArgsMultiStructArgsEnd >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}
impl< Context, Formed, End > FormerDefinition
for TestEnumArgsMultiStructArgsFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< TestEnumArgsMultiStructArgsFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = TestEnumArgsMultiStructArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = TestEnumArgsMultiStructArgsFormerDefinitionTypes< Context, Formed >;
  type End = End;
}
// Former
#[ derive( Debug ) ]
pub struct TestEnumArgsMultiStructArgsFormer
< Definition = TestEnumArgsMultiStructArgsFormerDefinition >
where
  Definition : FormerDefinition< Storage = TestEnumArgsMultiStructArgsFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
impl< Definition > TestEnumArgsMultiStructArgsFormer< Definition >
where
  Definition : FormerDefinition< Storage = TestEnumArgsMultiStructArgsFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = TestEnumArgsMultiStructArgsFormerStorage >,
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
  #[ inline ]
  #[allow(dead_code)]
  pub fn a( mut self, src : impl Into< i32 > ) -> Self
  {
    self.storage.a = Some( src.into() );
    self
  }
  #[ inline ]
  #[allow(dead_code)]
  pub fn b( mut self, src : impl Into< bool > ) -> Self
  {
    self.storage.b = Some( src.into() );
    self
  }
}
// End Struct
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiStructArgsEnd;
impl FormingEnd< TestEnumArgsMultiStructArgsFormerDefinitionTypes< (), TestEnumArgs > >
for TestEnumArgsMultiStructArgsEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : TestEnumArgsMultiStructArgsFormerStorage,
    _context : Option< () >,
  ) -> TestEnumArgs
  {
    let ( val_a, val_b ) = storage.preform();
    TestEnumArgs::MultiStructArgs { a : val_a, b : val_b }
  }
}


// === Standalone Constructors (Manual - Argument Taking) ===

/// Manual standalone constructor for TestEnumArgs::UnitVariantArgs.
pub fn unit_variant_args() -> TestEnumArgs
{
  TestEnumArgs::UnitVariantArgs
}

/// Manual standalone constructor for TestEnumArgs::TupleVariantArgs (takes arg).
/// Returns Self directly as per Option 2.
pub fn tuple_variant_args( _0 : impl Into< i32 > ) -> TestEnumArgs // Changed return type
{
  TestEnumArgs::TupleVariantArgs( _0.into() ) // Direct construction
}

/// Manual standalone constructor for TestEnumArgs::StructVariantArgs (takes arg).
/// Returns Self directly as per Option 2.
pub fn struct_variant_args( field : impl Into< String > ) -> TestEnumArgs // Changed return type
{
  TestEnumArgs::StructVariantArgs { field : field.into() } // Direct construction
}

/// Manual standalone constructor for TestEnumArgs::MultiTupleArgs. <<< NEW >>>
/// Takes 0 args and returns Former as per Option 2 (derive def has no args).
pub fn multi_tuple_args() // No arguments
-> // Return Former type
TestEnumArgsMultiTupleArgsFormer
<
  TestEnumArgsMultiTupleArgsFormerDefinition< (), TestEnumArgs, TestEnumArgsMultiTupleArgsEnd >
>
{
  // Begin former with no initial storage
  TestEnumArgsMultiTupleArgsFormer::begin( None, None, TestEnumArgsMultiTupleArgsEnd )
}

/// Manual standalone constructor for TestEnumArgs::MultiStructArgs (takes args). <<< NEW >>>
/// Returns Self directly as per Option 2.
pub fn multi_struct_args( a : impl Into< i32 >, b : impl Into< bool > ) -> TestEnumArgs // Changed return type
{
  TestEnumArgs::MultiStructArgs { a : a.into(), b : b.into() } // Direct construction
}

// === Include Test Logic ===
include!( "standalone_constructor_args_named_only_test.rs" );