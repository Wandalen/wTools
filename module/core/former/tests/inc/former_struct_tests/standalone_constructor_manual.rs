//!
//! Manual implementation for testing standalone constructors.
//! This file defines the struct, its manual Former implementation,
//! and the manual standalone constructor function.
//!

// Use necessary items from the former_types crate explicitly
#[ allow( unused_imports ) ]
use ::former::prelude::*; // Keep prelude for convenience if needed elsewhere
#[ allow( unused_imports ) ]
use ::former_types::
{
  Storage, StoragePreform,
  FormerDefinitionTypes, FormerMutator, FormerDefinition,
  FormingEnd, ReturnPreformed,
};

// === Struct Definition: No Args ===

/// Structure for manual testing of standalone constructors without arguments.
#[ derive( Debug, PartialEq, Default, Clone ) ]
pub struct ManualNoArgsStruct
{
  field1 : i32,
}

// === Manual Former Implementation: No Args ===

// Storage
/// Storage for ManualNoArgsStructFormer.
#[ derive( Debug, Default ) ]
pub struct ManualNoArgsStructFormerStorage
{
  /// Option to store the value for field1.
  pub field1 : ::core::option::Option< i32 >,
}

impl Storage for ManualNoArgsStructFormerStorage
{
  type Preformed = ManualNoArgsStruct;
}

impl StoragePreform for ManualNoArgsStructFormerStorage
{
  #[ inline( always ) ] // Attribute on its own line
  fn preform( mut self ) -> Self::Preformed
  {
    ManualNoArgsStruct
    {
      field1 : self.field1.take().unwrap_or_default(),
    }
  }
}

// Definition Types
/// Definition types for ManualNoArgsStructFormer.
#[ derive( Debug, Default ) ]
pub struct ManualNoArgsStructFormerDefinitionTypes< Context = (), Formed = ManualNoArgsStruct >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > FormerDefinitionTypes
for ManualNoArgsStructFormerDefinitionTypes< Context, Formed >
{
  type Storage = ManualNoArgsStructFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Mutator (empty default)
impl< Context, Formed > FormerMutator
for ManualNoArgsStructFormerDefinitionTypes< Context, Formed >
{
}

// Definition
/// Definition for ManualNoArgsStructFormer.
#[ derive( Debug, Default ) ]
pub struct ManualNoArgsStructFormerDefinition< Context = (), Formed = ManualNoArgsStruct, End = ReturnPreformed >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > FormerDefinition
for ManualNoArgsStructFormerDefinition< Context, Formed, End >
where
  // CORRECTED: Added bound for End
  End : FormingEnd< ManualNoArgsStructFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = ManualNoArgsStructFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = ManualNoArgsStructFormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// Former
/// Manual Former implementation for ManualNoArgsStruct.
#[ derive( Debug ) ]
pub struct ManualNoArgsStructFormer< Definition = ManualNoArgsStructFormerDefinition >
where
  // CORRECTED: Added bound for Definition
  Definition : FormerDefinition< Storage = ManualNoArgsStructFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

// Former methods (new, form, end, begin, setters)
impl< Definition > ManualNoArgsStructFormer< Definition >
where
  // CORRECTED: Added bound for Definition and its associated Types
  Definition : FormerDefinition< Storage = ManualNoArgsStructFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = ManualNoArgsStructFormerStorage >, // Needed for end/form
  Definition::Types : FormerMutator, // Needed for end/form
{
  /// Finalizes the forming process and returns the formed object.
  #[ inline( always ) ] // Attribute on its own line
  pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed
  {
    self.end()
  }

  /// Finalizes the forming process and returns the formed object.
  #[ inline( always ) ] // Attribute on its own line
  pub fn end( mut self )
  -> // Arrow and type on new line
  < Definition::Types as FormerDefinitionTypes >::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    // Apply mutations before finalizing
    < Definition::Types as FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
    // Call the end handler
    on_end.call( self.storage, context )
  }

  /// Begins the forming process with optional initial storage and context.
  #[ inline( always ) ] // Attribute on its own line
  pub fn begin
  (
    storage : Option< Definition::Storage >,
    context : Option< Definition::Context >,
    on_end : Definition::End,
  ) -> Self
  {
    Self
    {
      storage : storage.unwrap_or_default(), // Use default storage if none provided
      context,
      on_end : Some( on_end ),
    }
  }

  /// Creates a new former instance with a specific end condition.
  #[ inline( always ) ] // Attribute on its own line
  pub fn new( on_end : Definition::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }

  // Setter for field1
  /// Sets the value for field1.
  #[ inline ] // Attribute on its own line
  pub fn field1( mut self, src : impl Into< i32 > ) -> Self
  {
    debug_assert!( self.storage.field1.is_none(), "Field 'field1' was already set" );
    self.storage.field1 = Some( src.into() );
    self
  }
}

// === Standalone Constructor (Manual) ===

/// Manual standalone constructor for ManualNoArgsStruct.
/// Returns a Former instance ready to build a ManualNoArgsStruct.
pub fn manual_no_args_struct()
-> // Arrow and type on new line
ManualNoArgsStructFormer< ManualNoArgsStructFormerDefinition< (), ManualNoArgsStruct, ReturnPreformed > >
{
  // Instantiate the Former with the default ReturnPreformed end condition.
  ManualNoArgsStructFormer::new( ReturnPreformed ) // Use the explicitly imported type
}

// === Include Test Logic ===
// This assumes the test logic will be in a file named `standalone_constructor_only_test.rs`
// in the same directory. We will create this file next.
include!( "standalone_constructor_only_test.rs" );
