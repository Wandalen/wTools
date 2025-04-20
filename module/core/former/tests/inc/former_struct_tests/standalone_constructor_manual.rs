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
// ... (Implementation remains the same as previous correct version) ...
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
  Definition : FormerDefinition< Storage = ManualNoArgsStructFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

// Former methods (new, form, end, begin, setters)
impl< Definition > ManualNoArgsStructFormer< Definition >
where
  Definition : FormerDefinition< Storage = ManualNoArgsStructFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = ManualNoArgsStructFormerStorage >,
  Definition::Types : FormerMutator,
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

// === Standalone Constructor (Manual): No Args ===

/// Manual standalone constructor for ManualNoArgsStruct.
/// Returns a Former instance ready to build a ManualNoArgsStruct.
pub fn manual_no_args_struct()
-> // Arrow and type on new line
ManualNoArgsStructFormer< ManualNoArgsStructFormerDefinition< (), ManualNoArgsStruct, ReturnPreformed > >
{
  // Instantiate the Former with the default ReturnPreformed end condition.
  ManualNoArgsStructFormer::new( ReturnPreformed )
}

// === Struct Definition: With Args ===

/// Structure for manual testing of standalone constructors with arguments.
#[ derive( Debug, PartialEq, Default, Clone ) ]
pub struct ManualWithArgsStruct
{
  field_a : String,
  field_b : bool,
  field_c : Option< f32 >, // A field *not* expected to be a constructor arg
}

// === Manual Former Implementation: With Args ===

// Storage
/// Storage for ManualWithArgsStructFormer.
#[ derive( Debug, Default ) ]
pub struct ManualWithArgsStructFormerStorage
{
  /// Option to store the value for field_a.
  pub field_a : ::core::option::Option< String >,
  /// Option to store the value for field_b.
  pub field_b : ::core::option::Option< bool >,
  /// Option to store the value for field_c.
  pub field_c : ::core::option::Option< f32 >,
}

impl Storage for ManualWithArgsStructFormerStorage
{
  type Preformed = ManualWithArgsStruct;
}

impl StoragePreform for ManualWithArgsStructFormerStorage
{
  #[ inline( always ) ] // Attribute on its own line
  fn preform( mut self ) -> Self::Preformed
  {
    ManualWithArgsStruct
    {
      field_a : self.field_a.take().unwrap_or_default(),
      field_b : self.field_b.take().unwrap_or_default(),
      // Optional fields default to None if not set
      field_c : self.field_c.take(),
    }
  }
}

// Definition Types
/// Definition types for ManualWithArgsStructFormer.
#[ derive( Debug, Default ) ]
pub struct ManualWithArgsStructFormerDefinitionTypes< Context = (), Formed = ManualWithArgsStruct >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > FormerDefinitionTypes
for ManualWithArgsStructFormerDefinitionTypes< Context, Formed >
{
  type Storage = ManualWithArgsStructFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Mutator (empty default)
impl< Context, Formed > FormerMutator
for ManualWithArgsStructFormerDefinitionTypes< Context, Formed >
{
}

// Definition
/// Definition for ManualWithArgsStructFormer.
#[ derive( Debug, Default ) ]
pub struct ManualWithArgsStructFormerDefinition< Context = (), Formed = ManualWithArgsStruct, End = ReturnPreformed >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > FormerDefinition
for ManualWithArgsStructFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< ManualWithArgsStructFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = ManualWithArgsStructFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = ManualWithArgsStructFormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// Former
/// Manual Former implementation for ManualWithArgsStruct.
#[ derive( Debug ) ]
pub struct ManualWithArgsStructFormer< Definition = ManualWithArgsStructFormerDefinition >
where
  Definition : FormerDefinition< Storage = ManualWithArgsStructFormerStorage >,
{
  storage : Definition::Storage,
  context : Option< Definition::Context >,
  on_end : Option< Definition::End >,
}

// Former methods (new, form, end, begin, setters)
impl< Definition > ManualWithArgsStructFormer< Definition >
where
  Definition : FormerDefinition< Storage = ManualWithArgsStructFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = ManualWithArgsStructFormerStorage >,
  Definition::Types : FormerMutator,
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
  #[ allow( dead_code ) ] // This method is unused when using the standalone constructor
  pub fn new( on_end : Definition::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }

  // Setters
  /// Sets the value for field_a.
  #[ inline ] // Attribute on its own line
  #[ allow( dead_code ) ] // This setter is unused when using the standalone constructor for this field
  pub fn field_a( mut self, src : impl Into< String > ) -> Self
  {
    debug_assert!( self.storage.field_a.is_none(), "Field 'field_a' was already set" );
    self.storage.field_a = Some( src.into() );
    self
  }

  /// Sets the value for field_b.
  #[ inline ] // Attribute on its own line
  #[ allow( dead_code ) ] // This setter is unused when using the standalone constructor for this field
  pub fn field_b( mut self, src : impl Into< bool > ) -> Self
  {
    debug_assert!( self.storage.field_b.is_none(), "Field 'field_b' was already set" );
    self.storage.field_b = Some( src.into() );
    self
  }

  /// Sets the value for field_c.
  #[ inline ] // Attribute on its own line
  pub fn field_c( mut self, src : impl Into< f32 > ) -> Self
  {
    debug_assert!( self.storage.field_c.is_none(), "Field 'field_c' was already set" );
    self.storage.field_c = Some( src.into() );
    self
  }
}

// === Standalone Constructor (Manual): With Args ===

/// Manual standalone constructor for ManualWithArgsStruct.
/// Assumes field_a and field_b are constructor arguments.
pub fn manual_with_args_struct
(
  field_a : impl Into< String >,
  field_b : impl Into< bool >,
)
-> // Arrow and type on new line
ManualWithArgsStructFormer< ManualWithArgsStructFormerDefinition< (), ManualWithArgsStruct, ReturnPreformed > >
{
  // Create initial storage with arguments pre-filled
  let initial_storage = ManualWithArgsStructFormerStorage
  {
    field_a : Some( field_a.into() ),
    field_b : Some( field_b.into() ),
    field_c : None, // Non-argument field starts as None
  };

  // Begin the former with the pre-filled storage
  ManualWithArgsStructFormer::begin
  (
    Some( initial_storage ),
    None, // No context needed for top-level constructor
    ReturnPreformed, // Default end condition
  )
}


// === Include Test Logic ===
// This assumes the test logic will be in a file named `standalone_constructor_only_test.rs`
// in the same directory. We will create this file next.
include!( "standalone_constructor_only_test.rs" );