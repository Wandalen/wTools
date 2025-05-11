//! Purpose: Provides a hand-written implementation of the standalone scalar constructor function
//! for a multi-field named (struct-like) variant (`MultiStructArgs { a: i32, b: bool }`) within
//! an enum, demonstrating the manual implementation corresponding to the derived behavior when the
//! enum has `#[standalone_constructors]` and all fields have `#[arg_for_constructor]`.
//!
//! Coverage:
//! - Rule 4a (#[standalone_constructors]): Manually implements the top-level constructor function (`multi_struct_args`).
//! - Rule 4b (Option 2 Logic): Manually implements the logic for a scalar standalone constructor that takes arguments for all fields in a multi-field named variant.
//! - Rule 1g (Struct + Multi-Field + `#[scalar]`): Implicitly relevant as `MultiStructArgs` is a multi-field named variant.
//! - Rule 3g (Struct + Multi-Field + Default): Implicitly relevant as `MultiStructArgs` is a multi-field named variant.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines the `TestEnumArgs` enum with the multi-field named variant `MultiStructArgs { a: i32, b: bool }`.
//! - Provides a hand-written `multi_struct_args` function that takes `i32` and `bool` as arguments and returns `TestEnumArgs::MultiStructArgs { a: i32, b: bool }`. This mimics the behavior expected when `#[standalone_constructors]` is on the enum and `#[arg_for_constructor]` is on all fields of the variant.
//! - Includes shared test logic from `standalone_constructor_args_named_only_test.rs`.
//! - The included test calls this manually implemented standalone constructor and asserts that the returned enum instance matches a manually constructed `TestEnumArgs::MultiStructArgs { a: value1, b: value2 }`. This verifies the manual implementation of the scalar standalone constructor with field arguments.

// File: module/core/former/tests/inc/enum_named_tests/standalone_constructor_args_named_multi_manual.rs

#[ allow( unused_imports ) ]
use ::former::prelude::*;
#[ allow( unused_imports ) ]
use ::former_types::
{
  Storage, StoragePreform,
  FormerDefinitionTypes, FormerMutator, FormerDefinition,
  FormingEnd, ReturnPreformed,
};
use std::marker::PhantomData;

// === Enum Definition ===

/// Enum for manual testing of standalone constructors with arguments (multi named variant).
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnumArgs // New name
{
  /// A struct variant with multiple fields (intended as constructor args).
  MultiStructArgs // <<< New Variant
  {
    a : i32,
    b : bool,
  },
}

// === Manual Former Implementation for MultiStructArgs === <<< NEW >>>

// Storage
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiStructArgsFormerStorage
{
  pub a : ::core::option::Option< i32 >,
  pub b : ::core::option::Option< bool >,
}
impl Storage for TestEnumArgsMultiStructArgsFormerStorage
{
  type Preformed = ( i32, bool );
}
impl StoragePreform for TestEnumArgsMultiStructArgsFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    ( self.a.take().unwrap_or_default(), self.b.take().unwrap_or_default() )
  }
}
// Definition Types
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiStructArgsFormerDefinitionTypes
< Context = (), Formed = TestEnumArgs >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}
impl< Context, Formed > FormerDefinitionTypes
for TestEnumArgsMultiStructArgsFormerDefinitionTypes< Context, Formed >
{
  type Storage = TestEnumArgsMultiStructArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
}
impl< Context, Formed > FormerMutator
for TestEnumArgsMultiStructArgsFormerDefinitionTypes< Context, Formed >
{
}
// Definition
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiStructArgsFormerDefinition
< Context = (), Formed = TestEnumArgs, End = TestEnumArgsMultiStructArgsEnd >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}
impl< Context, Formed, End > FormerDefinition
for TestEnumArgsMultiStructArgsFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< TestEnumArgsMultiStructArgsFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = TestEnumArgsMultiStructArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = TestEnumArgsMultiStructArgsFormerDefinitionTypes< Context, Formed >;
  type End = End;
}
// Former
#[ derive( Debug ) ]
pub struct TestEnumArgsMultiStructArgsFormer
< Definition = TestEnumArgsMultiStructArgsFormerDefinition >
where
  Definition : FormerDefinition< Storage = TestEnumArgsMultiStructArgsFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
impl< Definition > TestEnumArgsMultiStructArgsFormer< Definition >
where
  Definition : FormerDefinition< Storage = TestEnumArgsMultiStructArgsFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = TestEnumArgsMultiStructArgsFormerStorage >,
  Definition::Types : FormerMutator,
{
  #[ inline( always ) ]
  pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed
  {
    self.end()
  }
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
  #[ inline ]
  #[allow(dead_code)]
  pub fn a( mut self, src : impl Into< i32 > ) -> Self
  {
    self.storage.a = Some( src.into() );
    self
  }
  #[ inline ]
  #[allow(dead_code)]
  pub fn b( mut self, src : impl Into< bool > ) -> Self
  {
    self.storage.b = Some( src.into() );
    self
  }
}
// End Struct
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiStructArgsEnd;
impl FormingEnd< TestEnumArgsMultiStructArgsFormerDefinitionTypes< (), TestEnumArgs > >
for TestEnumArgsMultiStructArgsEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : TestEnumArgsMultiStructArgsFormerStorage,
    _context : Option< () >,
  ) -> TestEnumArgs
  {
    let ( val_a, val_b ) = storage.preform();
    TestEnumArgs::MultiStructArgs { a : val_a, b : val_b }
  }
}


// === Standalone Constructors (Manual - Argument Taking) ===

/// Manual standalone constructor for TestEnumArgs::MultiStructArgs (takes args).
/// Returns Self directly as per Option 2.
pub fn multi_struct_args( a : impl Into< i32 >, b : impl Into< bool > ) -> TestEnumArgs // Changed return type
{
  TestEnumArgs::MultiStructArgs { a : a.into(), b : b.into() } // Direct construction
}

// === Include Test Logic ===
include!( "standalone_constructor_args_named_only_test.rs" );