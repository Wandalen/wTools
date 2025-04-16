// File: module/core/former/tests/inc/former_enum_tests/multi_field_manual.rs
use super::*; // Assuming it's in a module within `former_enum_tests`
use former::
{
  FormingEnd,
  StoragePreform,
  FormerDefinition,
  FormerDefinitionTypes,
  Storage,
  ReturnPreformed,
  FormerBegin,
  FormerMutator,
}; // Added FormerMutator

// --- Inner Struct Definitions ---

// InnerData needs a manual Former setup for the Struct variant test
#[ derive( Debug, Default, PartialEq, former::Former ) ] // Keep derive here for simplicity in manual test setup
pub struct InnerData
{
  data1 : i32,
  data2 : bool,
}

// OtherInnerData needs a manual Former setup for the ImplicitSubform variant test
#[ derive( Debug, Default, PartialEq ) ]
pub struct OtherInnerData
{
  info : String,
}

// --- Manual Former Setup for OtherInnerData ---
pub struct OtherInnerDataFormerStorage
{
  info : Option< String >,
}
impl Default for OtherInnerDataFormerStorage
{
  fn default() -> Self
  {
    Self { info : None }
  }
}
impl Storage for OtherInnerDataFormerStorage
{
  type Preformed = OtherInnerData;
}
impl StoragePreform for OtherInnerDataFormerStorage
{
  fn preform( mut self ) -> Self::Preformed
  {
    OtherInnerData
    {
      info : self.info.take().unwrap_or_default(),
    }
  }
}
#[ derive( Default, Debug ) ]
pub struct OtherInnerDataFormerDefinitionTypes< C = (), F = OtherInnerData >
{
  _p : core::marker::PhantomData< ( C, F ) >,
}
impl< C, F > FormerDefinitionTypes for OtherInnerDataFormerDefinitionTypes< C, F >
{
  type Storage = OtherInnerDataFormerStorage;
  type Context = C;
  type Formed = F;
}
// --- Added FormerMutator impl ---
impl< C, F > FormerMutator for OtherInnerDataFormerDefinitionTypes< C, F > {}
// --- End Added FormerMutator impl ---
#[ derive( Default, Debug ) ]
pub struct OtherInnerDataFormerDefinition< C = (), F = OtherInnerData, E = ReturnPreformed >
{
  _p : core::marker::PhantomData< ( C, F, E ) >,
}
impl< C, F, E > FormerDefinition for OtherInnerDataFormerDefinition< C, F, E >
where
  E : FormingEnd< OtherInnerDataFormerDefinitionTypes< C, F > >,
{
  type Storage = OtherInnerDataFormerStorage;
  type Context = C;
  type Formed = F;
  type Types = OtherInnerDataFormerDefinitionTypes< C, F >;
  type End = E;
}
pub struct OtherInnerDataFormer< Definition = OtherInnerDataFormerDefinition >
where
  Definition : FormerDefinition< Storage = OtherInnerDataFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
impl< Definition > OtherInnerDataFormer< Definition >
where
  Definition : FormerDefinition< Storage = OtherInnerDataFormerStorage >,
{
  pub fn info( mut self, value : impl Into< String > ) -> Self
  {
    self.storage.info = Some( value.into() );
    self
  }
  pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed
  {
    let end = self.on_end.unwrap();
    end.call( self.storage, self.context )
  }
  pub fn begin
  (
    storage : Option< Definition::Storage >,
    context : Option< Definition::Context >,
    on_end : Definition::End,
  ) -> Self
  {
    Self
    {
      storage : storage.unwrap_or_default(),
      context,
      on_end : Some( on_end ),
    }
  }
  #[ allow( dead_code ) ]
  pub fn new( on_end : Definition::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }
}
// --- End Manual Former Setup for OtherInnerData ---

/// Enum with different variant types for testing.
#[ derive( Debug, PartialEq ) ]
enum EnumWithMultiField
{
  /// A simple variant with one field.
  Simple( String ),
  /// A variant with multiple unnamed fields.
  MultiTuple( i32, String, bool ),
  /// A variant with no fields.
  Empty,
  /// Explicit Subform: Expects Enum::struct_() -> InnerDataFormer<...>
  Struct( InnerData ), // No attribute needed for manual impl
  /// Implicit Subform (default for single field with Former type): Expects Enum::implicit_subform() -> OtherInnerDataFormer<...>
  ImplicitSubform( OtherInnerData ),
}

// --- Specialized End Structs ---
#[ derive( Default, Debug ) ]
struct EnumWithMultiFieldStructEnd; // End struct for the Struct variant
#[ derive( Default, Debug ) ]
struct EnumWithMultiFieldImplicitSubformEnd; // End struct for the ImplicitSubform variant

// --- Manual implementation of static methods ---
impl EnumWithMultiField
{
  /// Manually implemented "scalar setter" style constructor for the Simple variant.
  #[ inline( always ) ]
  pub fn simple( value : impl Into< String > ) -> Self
  {
    Self::Simple( value.into() )
  }

  /// Manually implemented constructor for the MultiTuple variant.
  #[ inline( always ) ]
  pub fn multi_tuple( field0 : i32, field1 : impl Into< String >, field2 : bool ) -> Self
  {
    Self::MultiTuple( field0, field1.into(), field2 )
  }

  /// Manually implemented constructor for the Empty variant.
  #[ inline( always ) ]
  pub fn empty() -> Self
  {
    Self::Empty
  }

  /// Manually implemented subformer starter for the Struct variant.
  #[ inline( always ) ]
  pub fn r#struct() // Use raw identifier if needed, though 'struct' is not reserved here
  ->
  InnerDataFormer< InnerDataFormerDefinition< (), Self, EnumWithMultiFieldStructEnd > >
  {
    InnerDataFormer::begin( None, None, EnumWithMultiFieldStructEnd::default() )
  }

  /// Manually implemented subformer starter for the ImplicitSubform variant.
  #[ inline( always ) ]
  pub fn implicit_subform()
  ->
  OtherInnerDataFormer< OtherInnerDataFormerDefinition< (), Self, EnumWithMultiFieldImplicitSubformEnd > >
  {
    OtherInnerDataFormer::begin( None, None, EnumWithMultiFieldImplicitSubformEnd::default() )
  }
}

// --- FormingEnd Implementations ---

// End for Struct variant
impl FormingEnd< InnerDataFormerDefinitionTypes< (), EnumWithMultiField > >
for EnumWithMultiFieldStructEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : InnerDataFormerStorage,
    _context : Option< () >,
  )
  -> EnumWithMultiField
  {
    let data = sub_storage.preform();
    EnumWithMultiField::Struct( data )
  }
}

// End for ImplicitSubform variant
impl FormingEnd< OtherInnerDataFormerDefinitionTypes< (), EnumWithMultiField > >
for EnumWithMultiFieldImplicitSubformEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : OtherInnerDataFormerStorage,
    _context : Option< () >,
  )
  -> EnumWithMultiField
  {
    let data = sub_storage.preform();
    EnumWithMultiField::ImplicitSubform( data )
  }
}

// Include the actual test logic from the adjacent file
include!( "multi_field_only_test.rs" );