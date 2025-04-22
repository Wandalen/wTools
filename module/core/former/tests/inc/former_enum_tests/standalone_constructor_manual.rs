// module/core/former/tests/inc/former_enum_tests/standalone_constructor_manual.rs
//!
//! Manual implementation for testing standalone constructors for enums.
//! Uses consistent names matching the derive version for testing.
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

/// Enum for manual testing of standalone constructors.
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnum // Consistent name
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
/// Storage for TestEnumTupleVariantFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumTupleVariantFormerStorage
{
  /// Option to store the value for the tuple field.
  pub _0 : ::core::option::Option< i32 >,
}

impl Storage for TestEnumTupleVariantFormerStorage
{
  type Preformed = i32; // Preformed is the inner type
}

impl StoragePreform for TestEnumTupleVariantFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    self._0.take().unwrap_or_default()
  }
}

// Definition Types
/// Definition types for TestEnumTupleVariantFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumTupleVariantFormerDefinitionTypes< Context = (), Formed = TestEnum >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > FormerDefinitionTypes
for TestEnumTupleVariantFormerDefinitionTypes< Context, Formed >
{
  type Storage = TestEnumTupleVariantFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Mutator
impl< Context, Formed > FormerMutator
for TestEnumTupleVariantFormerDefinitionTypes< Context, Formed >
{
}

// Definition
/// Definition for TestEnumTupleVariantFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumTupleVariantFormerDefinition< Context = (), Formed = TestEnum, End = TestEnumTupleVariantEnd > // Use consistent End name
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > FormerDefinition
for TestEnumTupleVariantFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< TestEnumTupleVariantFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = TestEnumTupleVariantFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = TestEnumTupleVariantFormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// Former
/// Manual Former implementation for TestEnum::TupleVariant.
#[ derive( Debug ) ]
pub struct TestEnumTupleVariantFormer< Definition = TestEnumTupleVariantFormerDefinition > // Use consistent Def name
where
  Definition : FormerDefinition< Storage = TestEnumTupleVariantFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

impl< Definition > TestEnumTupleVariantFormer< Definition >
where
  Definition : FormerDefinition< Storage = TestEnumTupleVariantFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = TestEnumTupleVariantFormerStorage >,
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
  #[ allow( dead_code) ]
  pub fn new( on_end : Definition::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }

  /// Setter for the tuple field.
  #[ inline ]
  #[ allow( dead_code) ]
  pub fn _0( mut self, src : impl Into< i32 > ) -> Self
  {
    debug_assert!( self.storage._0.is_none(), "Field '_0' was already set" );
    self.storage._0 = Some( src.into() );
    self
  }
}

// End Struct for TupleVariant
/// End handler for TestEnumTupleVariantFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumTupleVariantEnd; // Consistent name

impl FormingEnd< TestEnumTupleVariantFormerDefinitionTypes< (), TestEnum > >
for TestEnumTupleVariantEnd // Consistent name
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : TestEnumTupleVariantFormerStorage,
    _context : Option< () >,
  ) -> TestEnum
  {
    let val = storage.preform();
    TestEnum::TupleVariant( val ) // Use consistent enum name
  }
}

// === Manual Former Implementation for StructVariant ===

// Storage
/// Storage for TestEnumStructVariantFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumStructVariantFormerStorage
{
  /// Option to store the value for the struct field.
  pub field : ::core::option::Option< String >,
}

impl Storage for TestEnumStructVariantFormerStorage
{
  type Preformed = String; // Preformed is the inner type
}

impl StoragePreform for TestEnumStructVariantFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    self.field.take().unwrap_or_default()
  }
}

// Definition Types
/// Definition types for TestEnumStructVariantFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumStructVariantFormerDefinitionTypes< Context = (), Formed = TestEnum >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > FormerDefinitionTypes
for TestEnumStructVariantFormerDefinitionTypes< Context, Formed >
{
  type Storage = TestEnumStructVariantFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Mutator
impl< Context, Formed > FormerMutator
for TestEnumStructVariantFormerDefinitionTypes< Context, Formed >
{
}

// Definition
/// Definition for TestEnumStructVariantFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumStructVariantFormerDefinition< Context = (), Formed = TestEnum, End = TestEnumStructVariantEnd > // Use consistent End name
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > FormerDefinition
for TestEnumStructVariantFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< TestEnumStructVariantFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = TestEnumStructVariantFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = TestEnumStructVariantFormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// Former
/// Manual Former implementation for TestEnum::StructVariant.
#[ derive( Debug ) ]
pub struct TestEnumStructVariantFormer< Definition = TestEnumStructVariantFormerDefinition > // Use consistent Def name
where
  Definition : FormerDefinition< Storage = TestEnumStructVariantFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

impl< Definition > TestEnumStructVariantFormer< Definition >
where
  Definition : FormerDefinition< Storage = TestEnumStructVariantFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = TestEnumStructVariantFormerStorage >,
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
  #[ allow( dead_code) ]
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
/// End handler for TestEnumStructVariantFormer.
#[ derive( Debug, Default ) ]
pub struct TestEnumStructVariantEnd; // Consistent name

impl FormingEnd< TestEnumStructVariantFormerDefinitionTypes< (), TestEnum > >
for TestEnumStructVariantEnd // Consistent name
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : TestEnumStructVariantFormerStorage,
    _context : Option< () >,
  ) -> TestEnum
  {
    let val = storage.preform();
    TestEnum::StructVariant { field : val } // Use consistent enum name
  }
}

// === Standalone Constructors (Manual) ===

/// Manual standalone constructor for TestEnum::UnitVariant.
pub fn unit_variant() -> TestEnum // Consistent name
{
  TestEnum::UnitVariant // Consistent name
}

/// Manual standalone constructor for TestEnum::TupleVariant.
/// Returns a Former instance for the variant.
// <<< Takes ZERO arguments >>>
pub fn tuple_variant() // Consistent name
-> // Arrow and type on new line
TestEnumTupleVariantFormer< TestEnumTupleVariantFormerDefinition< (), TestEnum, TestEnumTupleVariantEnd > > // Consistent names
{
  // <<< Begins with None storage >>>
  TestEnumTupleVariantFormer::begin( None, None, TestEnumTupleVariantEnd ) // Consistent names
}

/// Manual standalone constructor for TestEnum::StructVariant.
/// Returns a Former instance for the variant.
// <<< Takes ZERO arguments >>>
pub fn struct_variant() // Consistent name
-> // Arrow and type on new line
TestEnumStructVariantFormer< TestEnumStructVariantFormerDefinition< (), TestEnum, TestEnumStructVariantEnd > > // Consistent names
{
  // <<< Begins with None storage >>>
  TestEnumStructVariantFormer::begin( None, None, TestEnumStructVariantEnd ) // Consistent names
}

// === Include Test Logic ===
include!( "standalone_constructor_only_test.rs" ); // Use the consistent name