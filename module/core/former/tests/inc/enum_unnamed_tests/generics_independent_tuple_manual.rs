//! Purpose: Provides a manual implementation of constructors and `FormingEnd` for an enum
//! with unnamed (tuple) variants that have independent generic parameters and bounds,
//! to serve as a reference for verifying the `#[derive(Former)]` macro's behavior.
//!
//! Coverage:
//! - Rule 1d (Tuple + Single-Field + `#[scalar]` -> Scalar): Manual implementation of static method `EnumG5::v_1()`.
//! - Rule 4a (#[standalone_constructors]): Not applicable to this manual implementation file.
//! - Rule 4b (Option 2 Logic): Manual implementation of `FormingEnd` for the variant end type.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumG5<T: BoundA>` with a single-field tuple variant `V1(InnerG5<TypeForU>, PhantomData<T>)`.
//! - Manually implements a static method `EnumG5::v_1()` that mirrors the expected generated code for a scalar variant.
//! - Manually implements `FormingEnd` for the end type associated with the variant subformer.
//! - This file is included by `generics_independent_tuple_only_test.rs` to provide the manual implementations
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
// File: module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_manual.rs
use super::*; // Imports testing infrastructure and potentially other common items
use std::marker::PhantomData;
use former_types::
{
  Assign,
  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage,
  ReturnPreformed, FormerBegin, FormerMutator,
};

// --- Dummy Bounds ---
// Defined in _only_test.rs
// pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
// pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// --- Concrete Types ---
// Defined in _only_test.rs
// pub struct TypeForT( String ); impl BoundA for TypeForT {}
// pub struct TypeForU( i32 );    impl BoundB for TypeForU {}

// --- Inner Struct Definition with Bounds ---
#[ derive( Debug, Clone, PartialEq ) ]
pub struct InnerG5< U : BoundB > // BoundB required by the inner struct
{
  pub inner_field : U,
}

impl<U: BoundB + Default> Default for InnerG5<U> {
    fn default() -> Self {
        Self { inner_field: U::default() }
    }
}

// --- Manual Former for InnerG5 ---
// Generic over U: BoundB

// Storage
#[ derive( Debug, Default ) ]
pub struct InnerG5FormerStorage< U : BoundB >
{
  pub inner_field : Option< U >,
}
impl< U : BoundB > Storage for InnerG5FormerStorage< U >
{
  type Preformed = InnerG5< U >;
}
impl< U : BoundB + Default > StoragePreform for InnerG5FormerStorage< U > // Added Default bound for unwrap_or_default
{
  fn preform( mut self ) -> Self::Preformed
  {
    InnerG5 { inner_field : self.inner_field.take().unwrap_or_default() }
  }
}

// Definition Types
#[ derive( Default, Debug ) ]
pub struct InnerG5FormerDefinitionTypes< U : BoundB, C = (), F = InnerG5< U > >
{ _p : PhantomData< ( U, C, F ) > }

impl< U : BoundB, C, F > FormerDefinitionTypes for InnerG5FormerDefinitionTypes< U, C, F >
{
  type Storage = InnerG5FormerStorage< U >;
  type Context = C;
  type Formed = F;
}
impl< U : BoundB, C, F > FormerMutator for InnerG5FormerDefinitionTypes< U, C, F > {}

// Definition
#[ derive( Default, Debug ) ]
pub struct InnerG5FormerDefinition< U : BoundB, C = (), F = InnerG5< U >, E = ReturnPreformed >
{ _p : PhantomData< ( U, C, F, E ) > }

impl< U : BoundB, C, F, E > FormerDefinition for InnerG5FormerDefinition< U, C, F, E >
where E : FormingEnd< InnerG5FormerDefinitionTypes< U, C, F > >
{
  type Storage = InnerG5FormerStorage< U >;
  type Context = C;
  type Formed = F;
  type Types = InnerG5FormerDefinitionTypes< U, C, F >;
  type End = E;
}

// Former
pub struct InnerG5Former< U : BoundB, Definition = InnerG5FormerDefinition< U > >
where Definition : FormerDefinition< Storage = InnerG5FormerStorage< U > >
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
// Standard Former methods + Setter
impl< U : BoundB, Definition > InnerG5Former< U, Definition >
where Definition : FormerDefinition< Storage = InnerG5FormerStorage< U > >
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
  #[ inline ] pub fn _0( mut self, src : impl Into< U > ) -> Self
  { self.storage.inner_field = Some( src.into() ); self }
}

// --- Enum Definition with Bounds ---
#[ derive( Debug, PartialEq, Clone ) ]
pub enum EnumG5< T : BoundA > // BoundA required by the enum
{
  // CORRECTED: Added PhantomData<T> to use the generic parameter
  V1( InnerG5< TypeForU >, PhantomData< T > ),
}

// Implement Into manually for testing the constructor signature
impl< U : BoundB > From< U > for InnerG5< U >
{
  fn from( data : U ) -> Self { Self { inner_field : data } }
}

// --- Specialized End Struct for the V1 Variant ---
#[ derive( Default, Debug ) ]
// Only needs T: BoundA because U is fixed to TypeForU which satisfies BoundB
pub struct EnumG5V1End< T : BoundA >
{
  _phantom : PhantomData< T >,
}

// --- FormingEnd Implementation for the End Struct ---
// Only needs T: BoundA
#[ automatically_derived ]
impl< T : BoundA > FormingEnd
<
  // DefinitionTypes of InnerG5Former *specialized with TypeForU*:
  // Context=(), Formed=EnumG5<T>
  InnerG5FormerDefinitionTypes< TypeForU, (), EnumG5< T > >
>
for EnumG5V1End< T >
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : InnerG5FormerStorage< TypeForU >, // Storage from InnerG5Former<TypeForU>
    _context : Option< () >,                        // Context is () from static method
  ) -> EnumG5< T >                                  // Returns the EnumG5<T>
  {
    // Preform the inner data (which is InnerG5<TypeForU>)
    let data : InnerG5<TypeForU> = former_types::StoragePreform::preform( sub_storage );
    // CORRECTED: Construct V1 with PhantomData
    EnumG5::V1( data, PhantomData ) // Construct the V1 variant
  }
}

// --- Static Method on EnumG5 ---
// Only needs T: BoundA
impl< T : BoundA > EnumG5< T >
{
  /// Manually implemented subformer starter for the V1 variant.
  #[ inline( always ) ]
  pub fn v_1() -> InnerG5Former // Return type is InnerG5Former specialized with TypeForU...
  <
    TypeForU, // <<< U is fixed to TypeForU here
    // ...and configured with a definition that uses the specialized End struct.
    InnerG5FormerDefinition
    <
      TypeForU,              // <<< U is fixed to TypeForU here
      (),                    // Context = ()
      EnumG5< T >,           // Formed = EnumG5<T> (depends on T)
      EnumG5V1End< T >       // End = Specialized End struct (depends on T)
    >
  >
  {
    // Start the inner former using its `begin` associated function.
    // The End struct passed depends on T.
    InnerG5Former::begin( None, None, EnumG5V1End::< T >::default() )
  }
}

// --- Include the Test Logic ---
include!( "generics_independent_tuple_only_test.rs" );
// xxx : qqq : uncomment and fix issues