// File: module/core/former/tests/inc/former_enum_tests/enum_named_fields_manual.rs
use super::*;
use former::
{
  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage,
  ReturnPreformed, FormerBegin, FormerMutator,
};

// Define the enum without the derive macro
#[ derive( Debug, PartialEq ) ]
pub enum EnumWithNamedFields // Renamed enum for clarity
{
  VariantZero {},
  VariantOne { field_a : String },
  VariantTwo { field_b : i32, field_c : bool },
  UnitVariant,
}

// --- Manual Former Implementation ---

// --- Components for VariantZero ---

// Storage (empty for zero fields)
#[ derive( Debug, Default ) ]
pub struct EnumWithNamedFieldsVariantZeroFormerStorage {}
impl Storage for EnumWithNamedFieldsVariantZeroFormerStorage
{
  // The "preformed" type here is conceptually the anonymous struct `{}`
  // but we don't have a direct type for that. We'll handle construction in the End.
  // Let's use a unit type as a placeholder for the preformed data.
  type Preformed = ();
}
impl StoragePreform for EnumWithNamedFieldsVariantZeroFormerStorage
{
  fn preform( self ) -> Self::Preformed {} // Returns unit
}

// Definition Types
#[ derive( Default, Debug ) ]
pub struct EnumWithNamedFieldsVariantZeroFormerDefinitionTypes< C = (), F = EnumWithNamedFields >
{ _p : core::marker::PhantomData< ( C, F ) > }
impl< C, F > FormerDefinitionTypes for EnumWithNamedFieldsVariantZeroFormerDefinitionTypes< C, F >
{
  type Storage = EnumWithNamedFieldsVariantZeroFormerStorage;
  type Context = C;
  type Formed = F;
}
impl< C, F > FormerMutator for EnumWithNamedFieldsVariantZeroFormerDefinitionTypes< C, F > {}

// Definition
#[ derive( Default, Debug ) ]
pub struct EnumWithNamedFieldsVariantZeroFormerDefinition< C = (), F = EnumWithNamedFields, E = EnumWithNamedFieldsVariantZeroEnd >
{ _p : core::marker::PhantomData< ( C, F, E ) > }
impl< C, F, E > FormerDefinition for EnumWithNamedFieldsVariantZeroFormerDefinition< C, F, E >
where E : FormingEnd< EnumWithNamedFieldsVariantZeroFormerDefinitionTypes< C, F > >
{
  type Storage = EnumWithNamedFieldsVariantZeroFormerStorage;
  type Context = C;
  type Formed = F;
  type Types = EnumWithNamedFieldsVariantZeroFormerDefinitionTypes< C, F >;
  type End = E;
}

// Former (no setters needed)
pub struct EnumWithNamedFieldsVariantZeroFormer< Definition = EnumWithNamedFieldsVariantZeroFormerDefinition >
where Definition : FormerDefinition< Storage = EnumWithNamedFieldsVariantZeroFormerStorage >
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
// Standard Former methods (new, form, end, begin...)
impl< Definition > EnumWithNamedFieldsVariantZeroFormer< Definition >
where Definition : FormerDefinition< Storage = EnumWithNamedFieldsVariantZeroFormerStorage >
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
}

// End Struct
#[ derive( Default, Debug ) ] pub struct EnumWithNamedFieldsVariantZeroEnd;
// FormingEnd Impl
impl FormingEnd< EnumWithNamedFieldsVariantZeroFormerDefinitionTypes< (), EnumWithNamedFields > >
for EnumWithNamedFieldsVariantZeroEnd
{
  #[ inline( always ) ]
  fn call
  ( &self, _sub_storage : EnumWithNamedFieldsVariantZeroFormerStorage, _context : Option< () > )
  -> EnumWithNamedFields
  {
    // _sub_storage.preform(); // Preform returns (), which we ignore
    EnumWithNamedFields::VariantZero {} // Construct the enum variant
  }
}

// --- Components for VariantOne ---

// Storage
#[ derive( Debug, Default ) ]
pub struct EnumWithNamedFieldsVariantOneFormerStorage { pub field_a : Option< String > }
impl Storage for EnumWithNamedFieldsVariantOneFormerStorage { type Preformed = String; } // Preformed is just the inner field type
impl StoragePreform for EnumWithNamedFieldsVariantOneFormerStorage
{
  fn preform( mut self ) -> Self::Preformed { self.field_a.take().unwrap_or_default() }
}

// Definition Types
#[ derive( Default, Debug ) ]
pub struct EnumWithNamedFieldsVariantOneFormerDefinitionTypes< C = (), F = EnumWithNamedFields >
{ _p : core::marker::PhantomData< ( C, F ) > }
impl< C, F > FormerDefinitionTypes for EnumWithNamedFieldsVariantOneFormerDefinitionTypes< C, F >
{
  type Storage = EnumWithNamedFieldsVariantOneFormerStorage;
  type Context = C;
  type Formed = F;
}
impl< C, F > FormerMutator for EnumWithNamedFieldsVariantOneFormerDefinitionTypes< C, F > {}

// Definition
#[ derive( Default, Debug ) ]
pub struct EnumWithNamedFieldsVariantOneFormerDefinition< C = (), F = EnumWithNamedFields, E = EnumWithNamedFieldsVariantOneEnd >
{ _p : core::marker::PhantomData< ( C, F, E ) > }
impl< C, F, E > FormerDefinition for EnumWithNamedFieldsVariantOneFormerDefinition< C, F, E >
where E : FormingEnd< EnumWithNamedFieldsVariantOneFormerDefinitionTypes< C, F > >
{
  type Storage = EnumWithNamedFieldsVariantOneFormerStorage;
  type Context = C;
  type Formed = F;
  type Types = EnumWithNamedFieldsVariantOneFormerDefinitionTypes< C, F >;
  type End = E;
}

// Former
pub struct EnumWithNamedFieldsVariantOneFormer< Definition = EnumWithNamedFieldsVariantOneFormerDefinition >
where Definition : FormerDefinition< Storage = EnumWithNamedFieldsVariantOneFormerStorage >
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
// Standard Former methods + Setter
impl< Definition > EnumWithNamedFieldsVariantOneFormer< Definition >
where Definition : FormerDefinition< Storage = EnumWithNamedFieldsVariantOneFormerStorage >
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

  // Setter for field_a
  #[ inline ] pub fn field_a( mut self, src : impl Into< String > ) -> Self
  { self.storage.field_a = Some( src.into() ); self }
}

// End Struct
#[ derive( Default, Debug ) ] pub struct EnumWithNamedFieldsVariantOneEnd;
// FormingEnd Impl
impl FormingEnd< EnumWithNamedFieldsVariantOneFormerDefinitionTypes< (), EnumWithNamedFields > >
for EnumWithNamedFieldsVariantOneEnd
{
  #[ inline( always ) ]
  fn call
  ( &self, sub_storage : EnumWithNamedFieldsVariantOneFormerStorage, _context : Option< () > )
  -> EnumWithNamedFields
  {
    let field_a_data = sub_storage.preform(); // Get the String
    EnumWithNamedFields::VariantOne { field_a : field_a_data } // Construct the enum variant
  }
}

// --- Components for VariantTwo ---

// Storage
#[ derive( Debug, Default ) ]
pub struct EnumWithNamedFieldsVariantTwoFormerStorage
{
  pub field_b : Option< i32 >,
  pub field_c : Option< bool >,
}
// Preformed type is a tuple of the inner field types
impl Storage for EnumWithNamedFieldsVariantTwoFormerStorage { type Preformed = ( i32, bool ); }
impl StoragePreform for EnumWithNamedFieldsVariantTwoFormerStorage
{
  fn preform( mut self ) -> Self::Preformed
  {
    (
      self.field_b.take().unwrap_or_default(),
      self.field_c.take().unwrap_or_default(),
    )
  }
}

// Definition Types
#[ derive( Default, Debug ) ]
pub struct EnumWithNamedFieldsVariantTwoFormerDefinitionTypes< C = (), F = EnumWithNamedFields >
{ _p : core::marker::PhantomData< ( C, F ) > }
impl< C, F > FormerDefinitionTypes for EnumWithNamedFieldsVariantTwoFormerDefinitionTypes< C, F >
{
  type Storage = EnumWithNamedFieldsVariantTwoFormerStorage;
  type Context = C;
  type Formed = F;
}
impl< C, F > FormerMutator for EnumWithNamedFieldsVariantTwoFormerDefinitionTypes< C, F > {}

// Definition
#[ derive( Default, Debug ) ]
pub struct EnumWithNamedFieldsVariantTwoFormerDefinition< C = (), F = EnumWithNamedFields, E = EnumWithNamedFieldsVariantTwoEnd >
{ _p : core::marker::PhantomData< ( C, F, E ) > }
impl< C, F, E > FormerDefinition for EnumWithNamedFieldsVariantTwoFormerDefinition< C, F, E >
where E : FormingEnd< EnumWithNamedFieldsVariantTwoFormerDefinitionTypes< C, F > >
{
  type Storage = EnumWithNamedFieldsVariantTwoFormerStorage;
  type Context = C;
  type Formed = F;
  type Types = EnumWithNamedFieldsVariantTwoFormerDefinitionTypes< C, F >;
  type End = E;
}

// Former
pub struct EnumWithNamedFieldsVariantTwoFormer< Definition = EnumWithNamedFieldsVariantTwoFormerDefinition >
where Definition : FormerDefinition< Storage = EnumWithNamedFieldsVariantTwoFormerStorage >
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
// Standard Former methods + Setters
impl< Definition > EnumWithNamedFieldsVariantTwoFormer< Definition >
where Definition : FormerDefinition< Storage = EnumWithNamedFieldsVariantTwoFormerStorage >
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

  // Setters
  #[ inline ] pub fn field_b( mut self, src : impl Into< i32 > ) -> Self { self.storage.field_b = Some( src.into() ); self }
  #[ inline ] pub fn field_c( mut self, src : impl Into< bool > ) -> Self { self.storage.field_c = Some( src.into() ); self }
}

// End Struct
#[ derive( Default, Debug ) ] pub struct EnumWithNamedFieldsVariantTwoEnd;
// FormingEnd Impl
impl FormingEnd< EnumWithNamedFieldsVariantTwoFormerDefinitionTypes< (), EnumWithNamedFields > >
for EnumWithNamedFieldsVariantTwoEnd
{
  #[ inline( always ) ]
  fn call
  ( &self, sub_storage : EnumWithNamedFieldsVariantTwoFormerStorage, _context : Option< () > )
  -> EnumWithNamedFields
  {
    let ( field_b_data, field_c_data ) = sub_storage.preform(); // Get the tuple (i32, bool)
    EnumWithNamedFields::VariantTwo { field_b : field_b_data, field_c : field_c_data } // Construct the enum variant
  }
}

// --- Static Methods on the Enum ---
impl EnumWithNamedFields
{
  // Constructor for UnitVariant
  #[ inline( always ) ]
  pub fn unit_variant() -> Self
  {
    Self::UnitVariant
  }

  // Starter for VariantZero subformer
  #[ inline( always ) ]
  pub fn variant_zero()
  -> EnumWithNamedFieldsVariantZeroFormer< EnumWithNamedFieldsVariantZeroFormerDefinition< (), Self, EnumWithNamedFieldsVariantZeroEnd > >
  {
    EnumWithNamedFieldsVariantZeroFormer::begin( None, None, EnumWithNamedFieldsVariantZeroEnd::default() )
  }

  // Starter for VariantOne subformer
  #[ inline( always ) ]
  pub fn variant_one()
  -> EnumWithNamedFieldsVariantOneFormer< EnumWithNamedFieldsVariantOneFormerDefinition< (), Self, EnumWithNamedFieldsVariantOneEnd > >
  {
    EnumWithNamedFieldsVariantOneFormer::begin( None, None, EnumWithNamedFieldsVariantOneEnd::default() )
  }

  // Starter for VariantTwo subformer
  #[ inline( always ) ]
  pub fn variant_two()
  -> EnumWithNamedFieldsVariantTwoFormer< EnumWithNamedFieldsVariantTwoFormerDefinition< (), Self, EnumWithNamedFieldsVariantTwoEnd > >
  {
    EnumWithNamedFieldsVariantTwoFormer::begin( None, None, EnumWithNamedFieldsVariantTwoEnd::default() )
  }
}

// Include the test logic file
include!( "enum_named_fields_only_test.rs" );