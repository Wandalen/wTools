//!
//! Manual implementation for testing standalone constructors.
//! Uses consistent names matching the derive version for testing.
//!

#[ allow( unused_imports ) ]
use ::former::prelude::*;
#[ allow( unused_imports ) ]
use ::former_types::
{
  Storage, StoragePreform,
  FormerDefinitionTypes, FormerMutator, FormerDefinition,
  FormingEnd, ReturnPreformed,
};

// === Struct Definition: No Args ===

/// Manual struct without constructor args.
#[ derive( Debug, PartialEq, Default, Clone ) ]
pub struct TestStructNoArgs
{
  /// A simple field.
  pub field1 : i32,
}

// === Manual Former Implementation: No Args ===
// ... (No changes needed here, as all methods/fields are used by no_args_test) ...
// Storage
/// Manual storage for `TestStructNoArgsFormer`.
#[ derive( Debug, Default ) ]
pub struct TestStructNoArgsFormerStorage
{
  /// Optional storage for field1.
  pub field1 : Option< i32 >,
}

impl Storage for TestStructNoArgsFormerStorage
{
  type Preformed = TestStructNoArgs;
}

impl StoragePreform for TestStructNoArgsFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    TestStructNoArgs
    {
      field1 : self.field1.take().unwrap_or_default(),
    }
  }
}

// Definition Types
/// Manual definition types for `TestStructNoArgsFormer`.
#[ derive( Debug, Default ) ]
pub struct TestStructNoArgsFormerDefinitionTypes< Context = (), Formed = TestStructNoArgs >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > FormerDefinitionTypes
for TestStructNoArgsFormerDefinitionTypes< Context, Formed >
{
  type Storage = TestStructNoArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

impl< Context, Formed > FormerMutator
for TestStructNoArgsFormerDefinitionTypes< Context, Formed >
{
}

// Definition
/// Manual definition for `TestStructNoArgsFormer`.
#[ derive( Debug, Default ) ]
pub struct TestStructNoArgsFormerDefinition< Context = (), Formed = TestStructNoArgs, End = ReturnPreformed >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > FormerDefinition
for TestStructNoArgsFormerDefinition< Context, Formed, End >
where
  End : FormingEnd< TestStructNoArgsFormerDefinitionTypes< Context, Formed > >,
{
  type Storage = TestStructNoArgsFormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = TestStructNoArgsFormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// Former
/// Manual Former for `TestStructNoArgs`.
#[ derive( Debug ) ]
pub struct TestStructNoArgsFormer< Definition = TestStructNoArgsFormerDefinition >
where
  Definition : FormerDefinition< Storage = TestStructNoArgsFormerStorage >,
{
  /// Former storage.
  pub storage : Definition::Storage,
  /// Former context.
  pub context : Option< Definition::Context >,
  /// Former end handler.
  pub on_end : Option< Definition::End >,
}

impl< Definition > TestStructNoArgsFormer< Definition >
where
  Definition : FormerDefinition< Storage = TestStructNoArgsFormerStorage >,
  Definition::Types : FormerDefinitionTypes< Storage = TestStructNoArgsFormerStorage >,
  Definition::Types : FormerMutator,
{
  /// Finalizes the forming process.
  #[ inline( always ) ]
  pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed
  {
    self.end()
  }

  /// Finalizes the forming process.
  #[ inline( always ) ]
  pub fn end( mut self )
  ->
  < Definition::Types as FormerDefinitionTypes >::Formed
  {
    let end = self.on_end.take().unwrap();
    < Definition::Types as FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
    end.call( self.storage, self.context.take() )
  }

  /// Begins the forming process.
  #[ inline( always ) ]
  pub fn begin
  (
    s : Option< Definition::Storage >,
    c : Option< Definition::Context >,
    e : Definition::End,
  ) -> Self
  {
    Self
    {
      storage : s.unwrap_or_default(),
      context : c,
      on_end : Some( e ),
    }
  }

  /// Creates a new former instance.
  #[ inline( always ) ]
  pub fn new( e : Definition::End ) -> Self
  {
    Self::begin( None, None, e )
  }

  /// Setter for field1.
  #[ inline ]
  pub fn field1( mut self, src : impl Into< i32 > ) -> Self
  {
    debug_assert!( self.storage.field1.is_none() );
    self.storage.field1 = Some( src.into() );
    self
  }
}

// === Standalone Constructor (Manual): No Args ===
/// Manual standalone constructor for `TestStructNoArgs`.
pub fn test_struct_no_args()
->
TestStructNoArgsFormer< TestStructNoArgsFormerDefinition< (), TestStructNoArgs, ReturnPreformed > >
{
  TestStructNoArgsFormer::new( ReturnPreformed )
}

// === Struct Definition: With Args ===
/// Manual struct with constructor args.
#[ derive( Debug, PartialEq, Default, Clone ) ]
pub struct TestStructWithArgs
{
  /// Field A.
  pub a : String,
  /// Field B.
  pub b : bool,
  /// Field C (optional).
  pub c : Option< f32 >,
}

// === Manual Former Implementation: With Args ===
// ... (Storage, DefTypes, Def implementations remain the same) ...
/// Manual storage for `TestStructWithArgsFormer`.
#[ derive( Debug, Default ) ]
pub struct TestStructWithArgsFormerStorage
{
  /// Optional storage for `a`.
  pub a : Option< String >,
  /// Optional storage for `b`.
  pub b : Option< bool >,
  /// Optional storage for `c`.
  pub c : Option< f32 >,
}

impl Storage for TestStructWithArgsFormerStorage
{
  type Preformed = TestStructWithArgs;
}

impl StoragePreform for TestStructWithArgsFormerStorage
{
  #[ inline( always ) ]
  fn preform( mut self ) -> Self::Preformed
  {
    TestStructWithArgs
    {
      a : self.a.take().unwrap_or_default(),
      b : self.b.take().unwrap_or_default(),
      c : self.c.take(),
    }
  }
}

/// Manual definition types for `TestStructWithArgsFormer`.
#[ derive( Debug, Default ) ]
pub struct TestStructWithArgsFormerDefinitionTypes< C = (), F = TestStructWithArgs >
{
  _p : core::marker::PhantomData< ( C, F ) >,
}

impl< C, F > FormerDefinitionTypes
for TestStructWithArgsFormerDefinitionTypes< C, F >
{
  type Storage = TestStructWithArgsFormerStorage;
  type Formed = F;
  type Context = C;
}

impl< C, F > FormerMutator
for TestStructWithArgsFormerDefinitionTypes< C, F >
{
}

/// Manual definition for `TestStructWithArgsFormer`.
#[ derive( Debug, Default ) ]
pub struct TestStructWithArgsFormerDefinition< C = (), F = TestStructWithArgs, E = ReturnPreformed >
{
  _p : core::marker::PhantomData< ( C, F, E ) >,
}

impl< C, F, E > FormerDefinition
for TestStructWithArgsFormerDefinition< C, F, E >
where
  E : FormingEnd< TestStructWithArgsFormerDefinitionTypes< C, F > >,
{
  type Storage = TestStructWithArgsFormerStorage;
  type Formed = F;
  type Context = C;
  type Types = TestStructWithArgsFormerDefinitionTypes< C, F >;
  type End = E;
}


/// Manual Former for `TestStructWithArgs`.
#[ derive( Debug ) ]
#[ allow( dead_code ) ] // Allow dead code for the whole struct as tests might not use all fields
pub struct TestStructWithArgsFormer< D = TestStructWithArgsFormerDefinition >
where
  D : FormerDefinition< Storage = TestStructWithArgsFormerStorage >,
{
  /// Former storage.
  pub storage : D::Storage,
  /// Former context.
  pub context : Option< D::Context >, // Warning: field is never read
  /// Former end handler.
  pub on_end : Option< D::End >, // Warning: field is never read
}

impl< D > TestStructWithArgsFormer< D >
where
  D : FormerDefinition< Storage = TestStructWithArgsFormerStorage >,
  D::Types : FormerDefinitionTypes< Storage = TestStructWithArgsFormerStorage >,
  D::Types : FormerMutator,
{
  /// Finalizes the forming process.
  #[ inline( always ) ]
  #[ allow( dead_code ) ] // Warning: method is never used
  pub fn form( self ) -> < D::Types as FormerDefinitionTypes >::Formed
  {
    self.end()
  }

  /// Finalizes the forming process.
  #[ inline( always ) ]
  #[ allow( dead_code ) ] // Warning: method is never used
  pub fn end( mut self )
  ->
  < D::Types as FormerDefinitionTypes >::Formed
  {
    let end = self.on_end.take().unwrap();
    < D::Types as FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
    end.call( self.storage, self.context.take() )
  }

  /// Begins the forming process.
  #[ inline( always ) ]
  pub fn begin
  (
    s : Option< D::Storage >,
    c : Option< D::Context >,
    e : D::End,
  ) -> Self
  {
    Self
    {
      storage : s.unwrap_or_default(),
      context : c,
      on_end : Some( e ),
    }
  }

  /// Creates a new former instance.
  #[ inline( always ) ]
  #[ allow( dead_code ) ]
  pub fn new( e : D::End ) -> Self
  {
    Self::begin( None, None, e )
  }

  /// Setter for `a`.
  #[ inline ]
  #[ allow( dead_code ) ]
  pub fn a( mut self, src : impl Into< String > ) -> Self
  {
    debug_assert!( self.storage.a.is_none() );
    self.storage.a = Some( src.into() );
    self
  }

  /// Setter for `b`.
  #[ inline ]
  #[ allow( dead_code ) ]
  pub fn b( mut self, src : impl Into< bool > ) -> Self
  {
    debug_assert!( self.storage.b.is_none() );
    self.storage.b = Some( src.into() );
    self
  }

  /// Setter for `c`.
  #[ inline ]
  #[ allow( dead_code ) ] // Warning: method is never used
  pub fn c( mut self, src : impl Into< f32 > ) -> Self
  {
    debug_assert!( self.storage.c.is_none() );
    self.storage.c = Some( src.into() );
    self
  }
}

// === Standalone Constructor (Manual): With Args ===
/// Manual standalone constructor for `TestStructWithArgs`.
#[ allow( dead_code ) ] // Warning: function is never used
pub fn test_struct_with_args
(
  a : impl Into< String >,
  b : impl Into< bool >,
)
->
TestStructWithArgsFormer< TestStructWithArgsFormerDefinition< (), TestStructWithArgs, ReturnPreformed > >
{
  let initial_storage = TestStructWithArgsFormerStorage
  {
    a : Some( a.into() ),
    b : Some( b.into() ),
    c : None,
  };
  TestStructWithArgsFormer::begin( Some( initial_storage ), None, ReturnPreformed )
}

// === Include Test Logic ===
include!( "standalone_constructor_only_test.rs" ); // Include the single test file