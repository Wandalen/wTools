//! Purpose: Provides a manual implementation of constructors and `FormingEnd` for an enum
//! with unnamed (tuple) variants that have shared generic parameters and bounds, using the
//! default subform behavior, to serve as a reference for verifying the `#[ derive( Former ) ]`
//! macro's behavior.
//!
//! Coverage:
//! - Rule 3d (Tuple + Single-Field + Default -> Subform): Manual implementation of static method `EnumG3::v_1()`.
//! - Rule 4b (Option 2 Logic): Manual implementation of `FormingEnd` for the variant end type.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumG3<T: BoundA + BoundB>` with a single-field tuple variant `V1(InnerG3<T>)`.
//! - The inner struct `InnerG3<T: BoundB>` has its own generic `T` and bound `BoundB`, and is instantiated with the enum's generic `T` in the variant.
//! - Manually implements a static method `EnumG3::v_1()` that mirrors the expected generated code for a subform variant.
//! - Manually implements `FormingEnd` for the end type associated with the variant subformer.
//! - This file is included by `generics_shared_tuple_only_test.rs` to provide the manual implementations
//!   that the shared tests compare against.
#[ allow( unused_imports ) ]
use super::*; // Imports testing infrastructure and potentially other common items
use core::marker::PhantomData;
use former_types::
{

  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage,
  ReturnPreformed, FormerBegin, FormerMutator, // Added necessary imports
};

// --- Dummy Bounds ---
// Defined in _only_test.rs, but repeated here conceptually for clarity
// pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
// pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// --- Inner Struct Definition with Bounds ---
#[ derive( Debug, Clone, PartialEq ) ]
pub struct InnerG3< T : BoundB > // BoundB required by the inner struct
{
  pub inner_field : T,
}

// --- Manual Former for InnerG3 ---
// (Simplified manual implementation for brevity)

// Storage
#[ derive( Debug, Default ) ]
pub struct InnerG3FormerStorage< T : BoundB > // BoundB needed here
{
  pub inner_field : Option< T >,
}
impl< T : BoundB > Storage for InnerG3FormerStorage< T >
{
  type Preformed = InnerG3< T >;
}
impl< T : BoundB > StoragePreform for InnerG3FormerStorage< T >
{
  fn preform( mut self ) -> Self::Preformed
  {
    InnerG3 { inner_field : self.inner_field.take().unwrap_or_default() } // Assumes T: Default
  }
}

// Definition Types
#[ derive( Default, Debug ) ]
pub struct InnerG3FormerDefinitionTypes< T : BoundB, C = (), F = InnerG3< T > >
{ _p : PhantomData< ( T, C, F ) > }

impl< T : BoundB, C, F > FormerDefinitionTypes for InnerG3FormerDefinitionTypes< T, C, F >
{
  type Storage = InnerG3FormerStorage< T >;
  type Context = C;
  type Formed = F;
}
impl< T : BoundB, C, F > FormerMutator for InnerG3FormerDefinitionTypes< T, C, F > {}

// Definition
#[ derive( Default, Debug ) ]
pub struct InnerG3FormerDefinition< T : BoundB, C = (), F = InnerG3< T >, E = ReturnPreformed >
{ _p : PhantomData< ( T, C, F, E ) > }

impl< T : BoundB, C, F, E > FormerDefinition for InnerG3FormerDefinition< T, C, F, E >
where E : FormingEnd< InnerG3FormerDefinitionTypes< T, C, F > >
{
  type Storage = InnerG3FormerStorage< T >;
  type Context = C;
  type Formed = F;
  type Types = InnerG3FormerDefinitionTypes< T, C, F >;
  type End = E;
}

// Former
pub struct InnerG3Former< T : BoundB, Definition = InnerG3FormerDefinition< T > >
where Definition : FormerDefinition< Storage = InnerG3FormerStorage< T > >
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
// Standard Former methods + Setter
impl< T : BoundB, Definition > InnerG3Former< T, Definition >
where Definition : FormerDefinition< Storage = InnerG3FormerStorage< T > >
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

  // Setter for inner_field
  #[ inline ] pub fn inner_field( mut self, src : impl Into< T > ) -> Self
  { self.storage.inner_field = Some( src.into() ); self }
}

// --- Enum Definition with Bounds ---
#[ derive( Debug, PartialEq, Clone ) ]
// CORRECTED: Added BoundB to the enum's generic constraint for T
pub enum EnumG3< T : BoundA + BoundB > // BoundA required by enum, BoundB required by InnerG3<T>
{
  V1( InnerG3< T > ), // Inner type uses T, so T must satisfy InnerG3's bounds (BoundB) *in addition* to EnumG3's bounds (BoundA)
}

// --- Specialized End Struct for the V1 Variant ---
#[ derive( Default, Debug ) ]
pub struct EnumG3V1End< T : BoundA + BoundB > // Requires *both* bounds
{
  _phantom : PhantomData< T >,
}

// --- FormingEnd Implementation for the End Struct ---
// Requires *both* bounds
#[ automatically_derived ]
impl< T : BoundA + BoundB > FormingEnd
<
  // DefinitionTypes of InnerG3Former: Context=(), Formed=EnumG3<T>
  InnerG3FormerDefinitionTypes< T, (), EnumG3< T > >
>
for EnumG3V1End< T >
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : InnerG3FormerStorage< T >, // Storage from InnerG3Former
    _context : Option< () >,                 // Context is () from static method
  ) -> EnumG3< T >                           // Returns the EnumG3
  {
    // Preform the inner data and wrap it in the correct enum variant.
    let data = former_types::StoragePreform::preform( sub_storage );
    EnumG3::V1( data )
  }
}

// --- Static Method on EnumG3 ---
// Requires *both* bounds
impl< T : BoundA + BoundB > EnumG3< T >
{
  /// Manually implemented subformer starter for the V1 variant.
  #[ inline( always ) ]
  pub fn v_1() -> InnerG3Former // Return type is InnerG3Former...
  <
    T, // ...specialized with the enum's generic T...
    // ...and configured with a definition that uses the specialized End struct.
    InnerG3FormerDefinition
    <
      T,                     // Generic for InnerG3
      (),                    // Context = ()
      EnumG3< T >,           // Formed = EnumG3<T>
      EnumG3V1End< T >       // End = Specialized End struct
    >
  >
  {
    // Start the inner former using its `begin` associated function.
    InnerG3Former::begin( None, None, EnumG3V1End::< T >::default() )
  }
}

// --- Include the Test Logic ---
include!( "generics_shared_tuple_only_test.rs" );