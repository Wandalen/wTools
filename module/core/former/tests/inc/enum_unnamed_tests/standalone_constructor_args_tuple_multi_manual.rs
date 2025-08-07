// File: module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_args_tuple_multi_manual.rs

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

/// Enum for manual testing of standalone constructors with arguments (multi tuple variant).
#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnumArgs // New name
{
  /// A tuple variant with multiple fields (intended as constructor args).
  MultiTupleArgs( i32, bool ), // <<< New Variant
}

// === Manual Former Implementation for MultiTupleArgs === <<< NEW >>>

// Storage
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiTupleArgsFormerStorage
{
  pub _0 : ::core::option::Option< i32 >,
  pub _1 : ::core::option::Option< bool >,
}
impl Storage for TestEnumArgsMultiTupleArgsFormerStorage
{
  type Preformed = ( i32, bool );
}
impl StoragePreform for TestEnumArgsMultiTupleArgsFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    ( self._0.take().unwrap_or_default(), self._1.take().unwrap_or_default() )
  }
}
// Definition Types
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiTupleArgsFormerDefinitionTypes
< Context = (), Formed = TestEnumArgs >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}
impl< Context, Formed > FormerDefinitionTypes
for TestEnumArgsMultiTupleArgsFormerDefinitionTypes< Context, Formed >
{
  type Storage = TestEnumArgsMultiTupleArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
}
impl< Context, Formed > FormerMutator
for TestEnumArgsMultiTupleArgsFormerDefinitionTypes< Context, Formed >
{
}
// Definition
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiTupleArgsFormerDefinition
< Context = (), Formed = TestEnumArgs, End = TestEnumArgsMultiTupleArgsEnd >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}
impl< Context, Formed, End > FormerDefinition
for TestEnumArgsMultiTupleArgsFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< TestEnumArgsMultiTupleArgsFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = TestEnumArgsMultiTupleArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = TestEnumArgsMultiTupleArgsFormerDefinitionTypes< Context, Formed >;
  type End = End;
}
// Former
#[ derive( Debug ) ]
pub struct TestEnumArgsMultiTupleArgsFormer
< Definition = TestEnumArgsMultiTupleArgsFormerDefinition >
where
  Definition : FormerDefinition< Storage = TestEnumArgsMultiTupleArgsFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}
impl< Definition > TestEnumArgsMultiTupleArgsFormer< Definition >
where
  Definition : FormerDefinition< Storage = TestEnumArgsMultiTupleArgsFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = TestEnumArgsMultiTupleArgsFormerStorage >,
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
  pub fn _0( mut self, src : impl Into< i32 > ) -> Self
  {
    self.storage._0 = Some( src.into() );
    self
  }
  #[ inline ]
  pub fn _1( mut self, src : impl Into< bool > ) -> Self
  {
    self.storage._1 = Some( src.into() );
    self
  }
}
// End Struct
#[ derive( Debug, Default ) ]
pub struct TestEnumArgsMultiTupleArgsEnd;
impl FormingEnd< TestEnumArgsMultiTupleArgsFormerDefinitionTypes< (), TestEnumArgs > >
for TestEnumArgsMultiTupleArgsEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : TestEnumArgsMultiTupleArgsFormerStorage,
    _context : Option< () >,
  ) -> TestEnumArgs
  {
    let ( val0, val1 ) = storage.preform();
    TestEnumArgs::MultiTupleArgs( val0, val1 )
  }
}


// === Standalone Constructors (Manual - Argument Taking) ===

/// Manual standalone constructor for TestEnumArgs::MultiTupleArgs.
/// Takes 0 args and returns Former as per Option 2 (derive def has no args).
pub fn multi_tuple_args() // No arguments
-> // Return Former type
TestEnumArgsMultiTupleArgsFormer
<
  TestEnumArgsMultiTupleArgsFormerDefinition< (), TestEnumArgs, TestEnumArgsMultiTupleArgsEnd >
>
{
  // Begin former with no initial storage
  TestEnumArgsMultiTupleArgsFormer::begin( None, None, TestEnumArgsMultiTupleArgsEnd )
}

// === Include Test Logic ===
include!( "standalone_constructor_args_tuple_only_test.rs" );