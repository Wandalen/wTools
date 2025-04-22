// module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs
//!
//! Manual implementation for testing standalone constructors for enums with arguments.
//! Uses distinct names to avoid conflicts with zero-arg tests.
//!

#[ allow( unused_imports ) ]
use ::former::prelude::*;
#[ allow( unused_imports ) ]
use ::former_types::
{
  Storage, StoragePreform,
  FormerDefinitionTypes, FormerMutator, FormerDefinition,
  FormingEnd, ReturnPreformed,
};

// === Enum Definition ===

/// Enum for manual testing of standalone constructors with arguments.
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnumArgs // New name
{
  /// A unit variant.
  UnitVariantArgs, // New name
  /// A tuple variant with one field (intended as constructor arg).
  TupleVariantArgs( i32 ), // New name
  /// A struct variant with one field (intended as constructor arg).
  StructVariantArgs // New name
  {
    field : String,
  },
}

// === Manual Former Implementation for TupleVariantArgs ===

// Storage
/// Storage for TestEnumArgsTupleVariantArgsFormer.
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
/// Definition types for TestEnumArgsTupleVariantArgsFormer.
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
/// Definition for TestEnumArgsTupleVariantArgsFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsTupleVariantArgsFormerDefinition< Context = (), Formed = TestEnumArgs, End = TestEnumArgsTupleVariantArgsEnd >
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
/// Manual Former implementation for TestEnumArgs::TupleVariantArgs.
#[ derive( Debug ) ]
pub struct TestEnumArgsTupleVariantArgsFormer< Definition = TestEnumArgsTupleVariantArgsFormerDefinition >
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
  pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed { self.end() }

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
/// End handler for TestEnumArgsTupleVariantArgsFormer.
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
pub struct TestEnumArgsStructVariantArgsFormerDefinition< Context = (), Formed = TestEnumArgs, End = TestEnumArgsStructVariantArgsEnd >
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
pub struct TestEnumArgsStructVariantArgsFormer< Definition = TestEnumArgsStructVariantArgsFormerDefinition >
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
  pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed { self.end() }

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
  #[allow(dead_code)] // <<< Added allow(dead_code)
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

/// Manual standalone constructor for TestEnumArgs::UnitVariantArgs.
pub fn unit_variant_args() -> TestEnumArgs
{
  TestEnumArgs::UnitVariantArgs
}

/// Manual standalone constructor for TestEnumArgs::TupleVariantArgs (takes arg).
pub fn tuple_variant_args( _0 : impl Into< i32 > )
->
TestEnumArgsTupleVariantArgsFormer< TestEnumArgsTupleVariantArgsFormerDefinition< (), TestEnumArgs, TestEnumArgsTupleVariantArgsEnd > >
{
  let initial_storage = TestEnumArgsTupleVariantArgsFormerStorage
  {
    _0 : Some( _0.into() ),
  };
  TestEnumArgsTupleVariantArgsFormer::begin( Some( initial_storage ), None, TestEnumArgsTupleVariantArgsEnd )
}

/// Manual standalone constructor for TestEnumArgs::StructVariantArgs (takes arg).
pub fn struct_variant_args( field : impl Into< String > )
->
TestEnumArgsStructVariantArgsFormer< TestEnumArgsStructVariantArgsFormerDefinition< (), TestEnumArgs, TestEnumArgsStructVariantArgsEnd > >
{
  let initial_storage = TestEnumArgsStructVariantArgsFormerStorage
  {
    field : Some( field.into() ),
  };
  TestEnumArgsStructVariantArgsFormer::begin( Some( initial_storage ), None, TestEnumArgsStructVariantArgsEnd )
}

// === Include Test Logic ===
include!( "standalone_constructor_args_only_test.rs" );