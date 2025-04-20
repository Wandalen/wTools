//!
//! Manual implementation for testing standalone constructors for enums.
//! This file defines the enum, its manual Former implementations for variants,
//! and the manual standalone constructor functions.
//!

#[ allow( unused_imports ) ]
use ::former::prelude::*; // Use the main crate's prelude for testing
#[ allow( unused_imports ) ]
use ::former_types::
{
  Storage, StoragePreform,
  FormerDefinitionTypes, FormerMutator, FormerDefinition,
  FormingEnd, ReturnPreformed, // ReturnPreformed might not be used directly for variants
};

// === Enum Definition ===

/// Enum for manual testing of standalone constructors.
#[ derive( Debug, PartialEq, Clone ) ]
pub enum ManualEnum
{
  /// A unit variant.
  UnitVariant,
  /// A tuple variant with one field.
  TupleVariant( i32 ),
  /// A struct variant with one field.
  StructVariant
  {
    field : String,
  },
}

// === Manual Former Implementation for TupleVariant ===

// Storage
/// Storage for ManualEnumTupleVariantFormer.
#[ derive( Debug, Default ) ]
pub struct ManualEnumTupleVariantFormerStorage
{
  /// Option to store the value for the tuple field.
  pub _0 : ::core::option::Option< i32 >,
}

impl Storage for ManualEnumTupleVariantFormerStorage
{
  type Preformed = i32; // Preformed is the inner type
}

impl StoragePreform for ManualEnumTupleVariantFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    self._0.take().unwrap_or_default()
  }
}

// Definition Types
/// Definition types for ManualEnumTupleVariantFormer.
#[ derive( Debug, Default ) ]
pub struct ManualEnumTupleVariantFormerDefinitionTypes< Context = (), Formed = ManualEnum >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > FormerDefinitionTypes
for ManualEnumTupleVariantFormerDefinitionTypes< Context, Formed >
{
  type Storage = ManualEnumTupleVariantFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Mutator
impl< Context, Formed > FormerMutator
for ManualEnumTupleVariantFormerDefinitionTypes< Context, Formed >
{
}

// Definition
/// Definition for ManualEnumTupleVariantFormer.
#[ derive( Debug, Default ) ]
pub struct ManualEnumTupleVariantFormerDefinition< Context = (), Formed = ManualEnum, End = ManualEnumTupleVariantEnd >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > FormerDefinition
for ManualEnumTupleVariantFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< ManualEnumTupleVariantFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = ManualEnumTupleVariantFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = ManualEnumTupleVariantFormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// Former
/// Manual Former implementation for ManualEnum::TupleVariant.
#[ derive( Debug ) ]
pub struct ManualEnumTupleVariantFormer< Definition = ManualEnumTupleVariantFormerDefinition >
where
  Definition : FormerDefinition< Storage = ManualEnumTupleVariantFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

impl< Definition > ManualEnumTupleVariantFormer< Definition >
where
  Definition : FormerDefinition< Storage = ManualEnumTupleVariantFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = ManualEnumTupleVariantFormerStorage >,
  Definition::Types : FormerMutator,
{
  #[ inline( always ) ]
  pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed { self.end() }

  #[ inline( always ) ]
  pub fn end( mut self )
  ->
  < Definition::Types as FormerDefinitionTypes >::Formed
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
  pub fn new( on_end : Definition::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }

  /// Setter for the tuple field.
  #[ inline ]
  pub fn _0( mut self, src : impl Into< i32 > ) -> Self
  {
    debug_assert!( self.storage._0.is_none(), "Field '_0' was already set" );
    self.storage._0 = Some( src.into() );
    self
  }
}

// End Struct for TupleVariant
/// End handler for ManualEnumTupleVariantFormer.
#[ derive( Debug, Default ) ]
pub struct ManualEnumTupleVariantEnd;

impl FormingEnd< ManualEnumTupleVariantFormerDefinitionTypes< (), ManualEnum > >
for ManualEnumTupleVariantEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : ManualEnumTupleVariantFormerStorage,
    _context : Option< () >,
  ) -> ManualEnum
  {
    let val = storage.preform();
    ManualEnum::TupleVariant( val )
  }
}

// === Manual Former Implementation for StructVariant ===

// Storage
/// Storage for ManualEnumStructVariantFormer.
#[ derive( Debug, Default ) ]
pub struct ManualEnumStructVariantFormerStorage
{
  /// Option to store the value for the struct field.
  pub field : ::core::option::Option< String >,
}

impl Storage for ManualEnumStructVariantFormerStorage
{
  type Preformed = String; // Preformed is the inner type
}

impl StoragePreform for ManualEnumStructVariantFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    self.field.take().unwrap_or_default()
  }
}

// Definition Types
/// Definition types for ManualEnumStructVariantFormer.
#[ derive( Debug, Default ) ]
pub struct ManualEnumStructVariantFormerDefinitionTypes< Context = (), Formed = ManualEnum >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > FormerDefinitionTypes
for ManualEnumStructVariantFormerDefinitionTypes< Context, Formed >
{
  type Storage = ManualEnumStructVariantFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Mutator
impl< Context, Formed > FormerMutator
for ManualEnumStructVariantFormerDefinitionTypes< Context, Formed >
{
}

// Definition
/// Definition for ManualEnumStructVariantFormer.
#[ derive( Debug, Default ) ]
pub struct ManualEnumStructVariantFormerDefinition< Context = (), Formed = ManualEnum, End = ManualEnumStructVariantEnd >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > FormerDefinition
for ManualEnumStructVariantFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< ManualEnumStructVariantFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = ManualEnumStructVariantFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = ManualEnumStructVariantFormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// Former
/// Manual Former implementation for ManualEnum::StructVariant.
#[ derive( Debug ) ]
pub struct ManualEnumStructVariantFormer< Definition = ManualEnumStructVariantFormerDefinition >
where
  Definition : FormerDefinition< Storage = ManualEnumStructVariantFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

impl< Definition > ManualEnumStructVariantFormer< Definition >
where
  Definition : FormerDefinition< Storage = ManualEnumStructVariantFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = ManualEnumStructVariantFormerStorage >,
  Definition::Types : FormerMutator,
{
  #[ inline( always ) ]
  pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed { self.end() }

  #[ inline( always ) ]
  pub fn end( mut self )
  ->
  < Definition::Types as FormerDefinitionTypes >::Formed
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
  pub fn new( on_end : Definition::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }

  /// Setter for the struct field.
  #[ inline ]
  pub fn field( mut self, src : impl Into< String > ) -> Self
  {
    debug_assert!( self.storage.field.is_none(), "Field 'field' was already set" );
    self.storage.field = Some( src.into() );
    self
  }
}

// End Struct for StructVariant
/// End handler for ManualEnumStructVariantFormer.
#[ derive( Debug, Default ) ]
pub struct ManualEnumStructVariantEnd;

impl FormingEnd< ManualEnumStructVariantFormerDefinitionTypes< (), ManualEnum > >
for ManualEnumStructVariantEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : ManualEnumStructVariantFormerStorage,
    _context : Option< () >,
  ) -> ManualEnum
  {
    let val = storage.preform();
    ManualEnum::StructVariant { field : val }
  }
}

// === Standalone Constructors (Manual) ===

/// Manual standalone constructor for ManualEnum::UnitVariant.
pub fn manual_unit_variant() -> ManualEnum
{
  ManualEnum::UnitVariant
}

/// Manual standalone constructor for ManualEnum::TupleVariant.
/// Returns a Former instance for the variant.
pub fn manual_tuple_variant()
-> // Arrow and type on new line
ManualEnumTupleVariantFormer< ManualEnumTupleVariantFormerDefinition< (), ManualEnum, ManualEnumTupleVariantEnd > >
{
  ManualEnumTupleVariantFormer::new( ManualEnumTupleVariantEnd )
}

/// Manual standalone constructor for ManualEnum::StructVariant.
/// Returns a Former instance for the variant.
pub fn manual_struct_variant()
-> // Arrow and type on new line
ManualEnumStructVariantFormer< ManualEnumStructVariantFormerDefinition< (), ManualEnum, ManualEnumStructVariantEnd > >
{
  ManualEnumStructVariantFormer::new( ManualEnumStructVariantEnd )
}

// === Include Test Logic ===
include!( "standalone_constructor_only_test.rs" ); // New test file needed
