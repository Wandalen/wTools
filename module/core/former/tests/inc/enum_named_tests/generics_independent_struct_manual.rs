#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Purpose: Provides a hand-written implementation of the `Former` pattern's former builder for a
//! named (struct-like) variant (`V1`) within a generic enum (`EnumG6<T>`), where the variant
//! contains a field with an independent concrete generic type (`InnerG6<TypeForU>`). This file
//! demonstrates the manual implementation corresponding to the derived behavior, showing how to
//! manually create the implicit former infrastructure and the static method.
//!
//! Coverage:
//! - Rule 3g (Struct + Multi-Field + Default): Manually implements the static method `v_1()` which returns a former builder for the variant.
//! - Rule 4b (Option 2 Logic): Manually implements the implicit former's components (Storage, DefinitionTypes, Definition, Former, End) and the `FormingEnd` trait, demonstrating the subformer mechanism in the context of independent generics.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumG6<T: BoundA>` with a named variant `V1 { inner: InnerG6<TypeForU>, flag: bool, _phantom_t: PhantomData<T> }`.
//! - Defines the inner struct `InnerG6<U: BoundB>` which also derives `Former`.
//! - Defines dummy bounds (`BoundA`, `BoundB`) and concrete types (`TypeForT`, `TypeForU`) in the included test file.
//! - Provides hand-written implementations for the implicit former's components (`EnumG6V1FormerStorage`, `EnumG6V1FormerDefinitionTypes`, etc.) and the `FormingEnd` trait for `EnumG6V1End`.
//! - Implements the static method `EnumG6::<T>::v_1()` which returns the manual former builder.
//! - Includes shared test logic from `generics_independent_struct_only_test.rs`.
//! - The included tests call the manually implemented static method `EnumG6::<TypeForT>::v_1()`, use the returned former's setters (`.inner()`, `.flag()`), and call `.form()`.
//! - Asserts that the resulting enum instances match manually constructed expected values. This verifies that the manual implementation correctly provides a former builder that handles fields with independent concrete generic types and non-generic fields within a generic enum.

// File: module/core/former/tests/inc/former_enum_tests/generics_independent_struct_manual.rs

//! # Manual Test: Independent Generics in Struct Variants
//!
//! This file provides a manual implementation of the `Former` pattern for an enum (`EnumG6<T>`)
//! with a struct-like variant (`V1`) containing a field with an independent concrete type
//! (`InnerG6<TypeForU>`).
//!
//! ## Purpose:
//!
//! - To serve as a reference implementation demonstrating how the `Former` pattern should
//!   behave for this specific scenario involving independent generics in struct variants.
//! - To manually construct the implicit former infrastructure (Storage, Definitions, Former, End)
//!   for the `V1` variant, ensuring correct handling of the enum's generic `T` and its bounds.
//! - To validate the logic used by the `#[ derive( Former ) ]` macro by comparing its generated
//!   code's behavior against this manual implementation using the shared tests in
//!   `generics_independent_struct_only_test.rs`.

use super::*; // Imports testing infrastructure and potentially other common items
// FIX: Removed redundant import, it's imported in _only_test.rs if needed there,
// but primarily needed here for manual impls.
use former_types::
{
  Assign, // Needed for manual setter impls if we were doing that deeply
  FormingEnd, StoragePreform, FormerDefinition, FormerDefinitionTypes, Storage,
  ReturnPreformed, FormerBegin, FormerMutator, // Added necessary imports
};
use std::marker::PhantomData; // Added PhantomData

// --- Dummy Bounds and Concrete Types ---
// Are defined in the included _only_test.rs file

// --- Inner Struct Definition ---
// Also defined in the included _only_test.rs file.
// Needs its own Former implementation (manual or derived) for the subform setter test case,
// but for the direct setter test case here, we only need its definition.
#[ derive( Debug, Clone, PartialEq, Default, former::Former ) ] // Uncommented InnerG6 derive
pub struct InnerG6< U : BoundB > { pub inner_field : U }

// --- Enum Definition with Bounds ---
#[ derive( Debug, PartialEq, Clone ) ]
pub enum EnumG6< T : BoundA > // BoundA required by the enum
{
  V1 // Struct-like variant
  {
    // Field holding the inner struct instantiated with a *concrete* type
    inner : InnerG6< TypeForU >, // TypeForU satisfies BoundB implicitly via _only_test.rs
    // A non-generic field for testing
    flag : bool,
    // FIX: Added PhantomData to use the generic parameter T
    _phantom_t : PhantomData<T>,
  },
}

// --- Manual IMPLICIT Former Implementation for Variant V1 ---
// Storage for V1's fields
#[ derive( Debug, Default ) ]
pub struct EnumG6V1FormerStorage< T : BoundA > // Needs enum's bound
{
  // Storage holds Option<ActualFieldType>
  pub inner : Option< InnerG6< TypeForU > >, // Uses concrete TypeForU
  pub flag : Option< bool >,
  // FIX: Storage also needs phantom data if the final struct needs it
  _phantom : PhantomData<T>, // Use the enum's generic
}
impl< T : BoundA > Storage for EnumG6V1FormerStorage< T >
{
  // Preformed type is a tuple of the *actual* field types
  // FIX: Preformed type does not include PhantomData directly
  type Preformed = ( InnerG6< TypeForU >, bool );
}
impl< T : BoundA > StoragePreform for EnumG6V1FormerStorage< T >
{
  fn preform( mut self ) -> Self::Preformed
  {
    (
      // Use unwrap_or_default because InnerG6 derives Default
      self.inner.take().unwrap_or_default(),
      self.flag.take().unwrap_or_default(), // bool implements Default
      // FIX: PhantomData is not part of the preformed tuple
    )
  }
}

// Definition Types for V1's implicit former
#[ derive( Default, Debug ) ]
// Generics: Enum's generics + Context2 + Formed2
pub struct EnumG6V1FormerDefinitionTypes< T : BoundA, Context2 = (), Formed2 = EnumG6< T > >
{ _p : PhantomData< ( T, Context2, Formed2 ) > }

impl< T : BoundA, Context2, Formed2 > FormerDefinitionTypes for EnumG6V1FormerDefinitionTypes< T, Context2, Formed2 >
{
  type Storage = EnumG6V1FormerStorage< T >; // Storage uses enum's generic T
  type Context = Context2;
  type Formed = Formed2;
  type Types = EnumG6V1FormerDefinitionTypes< T, Context2, Formed2 >;
}
impl< T : BoundA, Context2, Formed2 > FormerMutator for EnumG6V1FormerDefinitionTypes< T, Context2, Formed2 > {}

// Definition for V1's implicit former
#[ derive( Default, Debug ) ]
// Generics: Enum's generics + Context2 + Formed2 + End2
pub struct EnumG6V1FormerDefinition< T : BoundA, Context2 = (), Formed2 = EnumG6< T >, End2 = EnumG6V1End< T > >
{ _p : PhantomData< ( T, Context2, Formed2, End2 ) > }

impl< T : BoundA, Context2, Formed2, End2 > FormerDefinition for EnumG6V1FormerDefinition< T, Context2, Formed2, End2 >
where End2 : FormingEnd< EnumG6V1FormerDefinitionTypes< T, Context2, Formed2 > >
{
  type Storage = EnumG6V1FormerStorage< T >; // Storage uses enum's generic T
  type Context = Context2;
  type Formed = Formed2;
  type Types = EnumG6V1FormerDefinitionTypes< T, Context2, Formed2 >;
  type End = End2;
}

// Implicit Former for V1
// Generics: Enum's generics + Definition (which defaults appropriately)
pub struct EnumG6V1Former< T : BoundA, Definition = EnumG6V1FormerDefinition< T > >
where Definition : FormerDefinition< Storage = EnumG6V1FormerStorage< T > >
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
// Standard Former methods + Setters for V1's fields
impl< T : BoundA, Definition > EnumG6V1Former< T, Definition >
where Definition : FormerDefinition< Storage = EnumG6V1FormerStorage< T > >
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

  // Setter for V1's 'inner' field (takes InnerG6<TypeForU>)
  #[ inline ] pub fn inner( mut self, src : impl Into< InnerG6< TypeForU > > ) -> Self
  { self.storage.inner = Some( src.into() ); self }

  // Setter for V1's 'flag' field
  #[ inline ] pub fn flag( mut self, src : impl Into< bool > ) -> Self
  { self.storage.flag = Some( src.into() ); self }
}

// --- Specialized End Struct for the V1 Variant ---
#[ derive( Default, Debug ) ]
pub struct EnumG6V1End< T : BoundA > // Only requires enum's bound
{
  _phantom : PhantomData< T >,
}

// --- FormingEnd Implementation for the End Struct ---
#[ automatically_derived ]
impl< T : BoundA > FormingEnd // Only requires enum's bound
<
  // DefinitionTypes of V1's implicit former: Context=(), Formed=EnumG6<T>
  EnumG6V1FormerDefinitionTypes< T, (), EnumG6< T > >
>
for EnumG6V1End< T >
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : EnumG6V1FormerStorage< T >, // Storage from implicit former
    _context : Option< () >,
  ) -> EnumG6< T > // Returns the EnumG6<T>
  {
    // Preform the tuple (InnerG6<TypeForU>, bool)
    let ( inner_data, flag_data ) = former_types::StoragePreform::preform( sub_storage );
    // Construct the V1 variant
    // FIX: Add phantom data field during construction
    EnumG6::V1 { inner : inner_data, flag : flag_data, _phantom_t: PhantomData }
  }
}

// --- Static Method on EnumG6 ---
impl< T : BoundA > EnumG6< T > // Only requires enum's bound
{
  /// Manually implemented subformer starter for the V1 variant.
  #[ inline( always ) ]
  pub fn v_1() -> EnumG6V1Former // Return type is V1's implicit former...
  <
    T, // ...specialized with the enum's generic T...
    // ...and configured with a definition that uses the specialized End struct.
    EnumG6V1FormerDefinition
    <
      T,                     // Enum generic T
      (),                    // Context = ()
      EnumG6< T >,           // Formed = EnumG6<T>
      EnumG6V1End< T >       // End = Specialized End struct
    >
  >
  {
    // Start the implicit former using its `begin` associated function.
    EnumG6V1Former::begin( None, None, EnumG6V1End::< T >::default() )
  }
}

// --- Include the Test Logic ---
include!( "generics_independent_struct_only_test.rs" );