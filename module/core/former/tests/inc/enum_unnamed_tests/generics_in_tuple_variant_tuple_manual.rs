//! Purpose: Provides a manual implementation of constructors and `FormingEnd` for an enum
//! with unnamed (tuple) variants that have shared generic parameters and bounds, using the
//! default subform behavior, to serve as a reference for verifying the `#[derive(Former)]`
//! macro's behavior.
//!
//! Coverage:
//! - Rule 3d (Tuple + Single-Field + Default -> Subform): Manual implementation of static method `EnumOuter::variant()`.
//! - Rule 4b (Option 2 Logic): Manual implementation of `FormingEnd` for the variant end type.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumOuter<X: Copy>` with a single-field tuple variant `Variant(InnerGeneric<X>)`.
//! - The inner struct `InnerGeneric<T: Debug + Copy + Default + PartialEq>` has its own generic `T` and bounds,
//!   and is instantiated with the enum's generic `X` in the variant.
//! - Manually implements a static method `EnumOuter::variant()` that mirrors the expected generated code for a subform variant.
//! - Manually implements `FormingEnd` for the end type associated with the variant subformer.
//! - This file is included by `generics_in_tuple_variant_only_test.rs` to provide the manual implementations
//!   that the shared tests compare against.
//! Manual implementation for testing enum variants with independent generic parameters.
//!
//! Purpose:
//! - Define an enum `EnumG5<T: BoundA>` where `T` is the enum's generic.
//! - Define an inner struct `InnerG5<U: BoundB>` where `U` is the inner struct's generic.
//! - Define a variant `V1(InnerG5<TypeForU>, PhantomData<T>)` where `U` is instantiated with a specific
//!   concrete type (`TypeForU`) that satisfies `BoundB`, while `T` remains generic for the enum.
//!   `PhantomData<T>` is added to ensure the `T` parameter is used.
//! - Manually implement the `Former` logic (static method `v1`, `End` struct, `impl FormingEnd`)
//!   to ensure the distinct generics `T` and `U` (instantiated as `TypeForU`) and their bounds
//!   are handled correctly. The static method `v1` should be generic over `T`, while the
//!   returned former and the `End` logic operate on the concrete `InnerG5<TypeForU>`.
//!
//! This setup tests the macro's ability to handle scenarios where the enum's state (`T`)
//! is independent of the specific type (`TypeForU`) being formed within one of its variants.
//!
// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/generics_in_tuple_variant_tuple_manual.rs
#[ allow( unused_imports ) ]
use super::*; // Imports testing infrastructure and potentially other common items
use std::fmt::Debug; // Import Debug trait for bounds
use std::marker::PhantomData; // Import PhantomData
use former::
{
  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage,
  ReturnPreformed, FormerBegin, FormerMutator, // Added necessary imports
};

// --- Inner Struct Definition with Bounds ---
// Needs its own Former implementation (manual or derived)
// Added PartialEq derive and Default bound to T
#[ derive( Debug, PartialEq, Clone, Copy ) ]
pub struct InnerGeneric< T >
where
  T : Debug + Copy + Default + PartialEq, // Added Default + PartialEq bounds
{
  pub inner_field: T,
}

// --- Manual Former for InnerGeneric ---
// (Simplified manual implementation for brevity in this example)

// Storage
#[ derive( Debug, Default ) ]
pub struct InnerGenericFormerStorage< T >
where
  T : Debug + Copy + Default + PartialEq, // Added Default + PartialEq bounds
{
  pub inner_field : Option< T >,
}
// Added Default + PartialEq bounds to T
impl< T > Storage for InnerGenericFormerStorage< T >
where
  T : Debug + Copy + Default + PartialEq,
{
  type Preformed = InnerGeneric< T >;
}
impl< T > StoragePreform for InnerGenericFormerStorage< T >
where
  T : Debug + Copy + Default + PartialEq, // Added Default + PartialEq bounds
{
  fn preform( mut self ) -> Self::Preformed
  {
    // Use unwrap_or_default now that T: Default
    InnerGeneric { inner_field : self.inner_field.take().unwrap_or_default() }
  }
}

// Definition Types
#[ derive( Default, Debug ) ]
pub struct InnerGenericFormerDefinitionTypes< T, C = (), F = InnerGeneric< T > >
where // Added where clause and bounds
  T : Debug + Copy + Default + PartialEq,
{ _p : PhantomData< ( T, C, F ) > }

// Added where clause and bounds
impl< T, C, F > FormerDefinitionTypes for InnerGenericFormerDefinitionTypes< T, C, F >
where
  T : Debug + Copy + Default + PartialEq,
{
  type Storage = InnerGenericFormerStorage< T >;
  type Context = C;
  type Formed = F;
  type Types = InnerGenericFormerDefinitionTypes< T, C, F >;
}
// Added where clause and bounds
impl< T, C, F > FormerMutator for InnerGenericFormerDefinitionTypes< T, C, F >
where
  T : Debug + Copy + Default + PartialEq,
{}

// Definition
#[ derive( Default, Debug ) ]
pub struct InnerGenericFormerDefinition< T, C = (), F = InnerGeneric< T >, E = ReturnPreformed >
where // Added where clause and bounds
  T : Debug + Copy + Default + PartialEq,
{ _p : PhantomData< ( T, C, F, E ) > }

// Added where clause and bounds
impl< T, C, F, E > FormerDefinition for InnerGenericFormerDefinition< T, C, F, E >
where
  T : Debug + Copy + Default + PartialEq,
  E : FormingEnd< InnerGenericFormerDefinitionTypes< T, C, F > >
{
  type Storage = InnerGenericFormerStorage< T >;
  type Context = C;
  type Formed = F;
  type Types = InnerGenericFormerDefinitionTypes< T, C, F >;
  type End = E;
}

// Former
pub struct InnerGenericFormer< T, Definition = InnerGenericFormerDefinition< T > >
where // Added where clause and bounds
  T : Debug + Copy + Default + PartialEq,
  Definition : FormerDefinition< Storage = InnerGenericFormerStorage< T > >
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
// Standard Former methods + Setter
// Added where clause and bounds
impl< T, Definition > InnerGenericFormer< T, Definition >
where
  T : Debug + Copy + Default + PartialEq,
  Definition : FormerDefinition< Storage = InnerGenericFormerStorage< T > >
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
// Added Debug + PartialEq bounds to X
#[ derive( Debug, PartialEq ) ]
pub enum EnumOuter< X >
where
  X : Copy + Debug + Default + PartialEq, // Added Debug + Default + PartialEq
{
  // --- Tuple Variant with Generics ---
  Variant( InnerGeneric< X > ), // Inner type uses X, which must satisfy InnerGeneric's bounds
}

// --- Specialized End Struct for the Variant ---
// Added Debug + Default + PartialEq bounds to X
#[ derive( Default, Debug ) ]
pub struct EnumOuterVariantEnd< X >
where
  X : Copy + Debug + Default + PartialEq, // Added Debug + Default + PartialEq
{
  _phantom: PhantomData< X >,
}

// --- FormingEnd Implementation for the End Struct ---
// This is the core part demonstrating bound merging
#[ automatically_derived ]
impl< X > FormingEnd
<
  // DefinitionTypes of InnerGenericFormer: Context=(), Formed=EnumOuter<X>
  InnerGenericFormerDefinitionTypes< X, (), EnumOuter< X > >
>
for EnumOuterVariantEnd< X >
where
  X : Copy + Debug + Default + PartialEq, // Added Debug + Default + PartialEq
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage: InnerGenericFormerStorage< X >, // Storage from InnerGenericFormer
    _context: Option<()>,                       // Context is () from static method
  ) -> EnumOuter< X >                           // Returns the EnumOuter
  {
    // Preform the inner data and wrap it in the correct enum variant.
    let data = former::StoragePreform::preform( sub_storage );
    EnumOuter::Variant( data )
  }
}


// --- Static Method on EnumOuter ---
// This is the other core part demonstrating bound merging
// Added Debug + Default + PartialEq bounds to X
impl< X > EnumOuter< X >
where
  X : Copy + Debug + Default + PartialEq, // Added Debug + Default + PartialEq
{
  /// Manually implemented subformer starter for the Variant variant.
  #[ inline( always ) ]
  pub fn variant() -> InnerGenericFormer // Return type is InnerGenericFormer...
  <
    X, // ...specialized with the enum's generic X...
    // ...and configured with a definition that uses the specialized End struct.
    InnerGenericFormerDefinition
    <
      X,                     // Generic for InnerGeneric
      (),                    // Context = ()
      EnumOuter< X >,        // Formed = EnumOuter<X>
      EnumOuterVariantEnd< X > // End = Specialized End struct
    >
  >
  {
    // Start the inner former using its `begin` associated function.
    InnerGenericFormer::begin( None, None, EnumOuterVariantEnd::< X >::default() )
  }
}


// --- Include the Test Logic ---
include!( "generics_in_tuple_variant_only_test.rs" );