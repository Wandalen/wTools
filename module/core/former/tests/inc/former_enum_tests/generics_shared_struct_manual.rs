// File: module/core/former/tests/inc/former_enum_tests/generics_shared_struct_manual.rs
use super::*; // Imports testing infrastructure and potentially other common items
use std::marker::PhantomData;
use former_types::
{
  Assign, // Needed for manual setter impls if we were doing that deeply
  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage,
  ReturnPreformed, FormerBegin, FormerMutator, // Added necessary imports
};

// --- Dummy Bounds ---
// Defined in _only_test.rs, but repeated here conceptually for clarity
// pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
// pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// --- Inner Struct Definition with Bounds ---
#[ derive( Debug, Clone, PartialEq ) ]
pub struct InnerG4< T : BoundB > // BoundB required by the inner struct
{
  pub inner_field : T,
}

impl<T: BoundB + Default> Default for InnerG4<T> {
    fn default() -> Self {
        Self { inner_field: T::default() }
    }
}

// --- Enum Definition with Bounds ---
#[ derive( Debug, PartialEq, Clone ) ]
pub enum EnumG4< T : BoundA + BoundB > // BoundA required by the enum, BoundB required by InnerG4<T>
{
  V1 // Struct-like variant
  {
    inner : InnerG4< T >,
    flag : bool,
  },
}

// --- Manual IMPLICIT Former Implementation for Variant V1 ---

// Storage for V1's fields
#[ derive( Debug, Default ) ]
pub struct EnumG4V1FormerStorage< T : BoundA + BoundB > // Needs combined bounds
{
  pub inner : Option< InnerG4< T > >,
  pub flag : Option< bool >,
  _phantom : PhantomData<T>,
}
impl< T : BoundA + BoundB > Storage for EnumG4V1FormerStorage< T >
{
  type Preformed = ( InnerG4< T >, bool );
}
impl< T : BoundA + BoundB > StoragePreform for EnumG4V1FormerStorage< T >
{
  fn preform( mut self ) -> Self::Preformed
  {
    (
      self.inner.take().unwrap_or_default(),
      self.flag.take().unwrap_or_default(),
    )
  }
}

// Definition Types for V1's implicit former
#[ derive( Default, Debug ) ]
pub struct EnumG4V1FormerDefinitionTypes< T : BoundA + BoundB, C = (), F = EnumG4< T > >
{ _p : PhantomData< ( T, C, F ) > }

impl< T : BoundA + BoundB, C, F > FormerDefinitionTypes for EnumG4V1FormerDefinitionTypes< T, C, F >
{
  type Storage = EnumG4V1FormerStorage< T >;
  type Context = C;
  type Formed = F;
}
impl< T : BoundA + BoundB, C, F > FormerMutator for EnumG4V1FormerDefinitionTypes< T, C, F > {}

// Definition for V1's implicit former
#[ derive( Default, Debug ) ]
pub struct EnumG4V1FormerDefinition< T : BoundA + BoundB, C = (), F = EnumG4< T >, E = EnumG4V1End< T > >
{ _p : PhantomData< ( T, C, F, E ) > }

impl< T : BoundA + BoundB, C, F, E > FormerDefinition for EnumG4V1FormerDefinition< T, C, F, E >
where E : FormingEnd< EnumG4V1FormerDefinitionTypes< T, C, F > >
{
  type Storage = EnumG4V1FormerStorage< T >;
  type Context = C;
  type Formed = F;
  type Types = EnumG4V1FormerDefinitionTypes< T, C, F >;
  type End = E;
}

// Implicit Former for V1
pub struct EnumG4V1Former< T : BoundA + BoundB, Definition = EnumG4V1FormerDefinition< T > >
where Definition : FormerDefinition< Storage = EnumG4V1FormerStorage< T > >
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
// Standard Former methods + Setters for V1's fields
impl< T : BoundA + BoundB, Definition > EnumG4V1Former< T, Definition >
where Definition : FormerDefinition< Storage = EnumG4V1FormerStorage< T > >
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

  // Setter for V1's 'inner' field
  #[ inline ] pub fn inner( mut self, src : impl Into< InnerG4< T > > ) -> Self
  { self.storage.inner = Some( src.into() ); self }

  // Setter for V1's 'flag' field
  #[ inline ] pub fn flag( mut self, src : impl Into< bool > ) -> Self
  { self.storage.flag = Some( src.into() ); self }
}

// --- Specialized End Struct for the V1 Variant ---
#[ derive( Default, Debug ) ]
pub struct EnumG4V1End< T : BoundA + BoundB > // Requires *both* bounds
{
  _phantom : PhantomData< T >,
}

// --- FormingEnd Implementation for the End Struct ---
// Requires *both* bounds
#[ automatically_derived ]
impl< T : BoundA + BoundB > FormingEnd
<
  EnumG4V1FormerDefinitionTypes< T, (), EnumG4< T > >
>
for EnumG4V1End< T >
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : EnumG4V1FormerStorage< T >,
    _context : Option< () >,
  ) -> EnumG4< T >
  {
    let ( inner_data, flag_data ) = former_types::StoragePreform::preform( sub_storage );
    EnumG4::V1 { inner : inner_data, flag : flag_data }
  }
}

// --- Static Method on EnumG4 ---
// Requires *both* bounds
impl< T : BoundA + BoundB > EnumG4< T >
{
  /// Manually implemented subformer starter for the V1 variant.
  // CORRECTED: Renamed v1 to v_1
  #[ inline( always ) ]
  pub fn v_1() -> EnumG4V1Former
  <
    T,
    EnumG4V1FormerDefinition
    <
      T,
      (),
      EnumG4< T >,
      EnumG4V1End< T >
    >
  >
  {
    EnumG4V1Former::begin( None, None, EnumG4V1End::< T >::default() )
  }
}

// --- Include the Test Logic ---
include!( "generics_shared_struct_only_test.rs" );