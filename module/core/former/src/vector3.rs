use super::*;
use axiomatic::*;

#[ allow( unused ) ]
use collection_tools::Vec;

/// Trait for containers that behave like a vector, providing an interface for element addition.
///
/// This trait enables the use of custom or standard vector-like containers within the builder pattern,
/// allowing for a unified and flexible approach to constructing collections.
///
pub trait VectorLike< E >
{
  /// Appends an element to the back of a storage.
  fn push( &mut self, element : E );
}

impl< E > VectorLike< E > for Vec< E >
{
  fn push( &mut self, element : E )
  {
    Vec::push( self, element );
  }
}

#[ derive( Debug ) ]
pub struct VectorDefinition< E >
{
  _phantom : core::marker::PhantomData< E >,
}

impl< E > VectorDefinition< E >
{
  pub fn new() -> Self
  {
    Self { _phantom : core::marker::PhantomData }
  }
}

impl< E > FormerDefinition
for VectorDefinition< E >
{
  type Storage = Vec< E >;
  type Formed = Vec< E >;
}

impl< E > Storage
for Vec< E >
{
  type Definition = VectorDefinition< E >;
}

impl< E > StoragePerform
for Vec< E >
{
  // type Definition = VectorDefinition< E >;
  fn preform( self ) -> < < Self as Storage >::Definition as FormerDefinition >::Formed
  {
    self
  }
}

/// A builder for constructing `VectorLike` containers, facilitating a fluent and flexible interface.
///
/// `VectorSubformer` leverages the `VectorLike` trait to enable the construction and manipulation
/// of vector-like containers in a builder pattern style, promoting readability and ease of use.
#[ derive( Debug, Default ) ]
pub struct VectorSubformer< E, Definition, Context, End >
where
  End : FormingEnd< Definition, Context >,
  Definition : FormerDefinition,
  Definition::Storage : ContainerAdd< Element = E >,
{
  storage : core::option::Option< Definition::Storage >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
}

impl< E, Definition, Context, End > VectorSubformer< E, Definition, Context, End >
where
  End : FormingEnd< Definition, Context >,
  Definition : FormerDefinition,
  Definition::Storage : ContainerAdd< Element = E >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn storage( mut self ) -> Definition::Storage
  {
    let storage = if self.storage.is_some()
    {
      self.storage.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };
    storage
  }

  /// Begins the building process, optionally initializing with a context and storage.
  #[ inline( always ) ]
  pub fn begin
  (
    storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Context >,
    on_end : End
  ) -> Self
  {
    Self
    {
      storage,
      context,
      on_end : Some( on_end ),
    }
  }

  /// Finalizes the building process, returning the formed or a context incorporating it.
  #[ inline( always ) ]
  pub fn form( self ) -> Definition::Formed
  {
    self.end()
  }

  /// Finalizes the building process, returning the formed or a context incorporating it.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Definition::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let storage = self.storage();
    on_end.call( storage, context )
  }

  /// Replaces the current storage with a provided one, allowing for a reset or redirection of the building process.
  #[ inline( always ) ]
  pub fn replace( mut self, vector : Definition::Storage ) -> Self
  {
    self.storage = Some( vector );
    self
  }

}

impl< E, Definition > VectorSubformer< E, Definition, (), ReturnFormed >
where
  Definition : FormerDefinition,
  Definition::Storage : ContainerAdd< Element = E >,
  Definition::Storage : StoragePerform< Definition = Definition >,
{

  /// Initializes a new `VectorSubformer` instance, starting with an empty formed.
  /// This function serves as the entry point for the builder pattern.
  ///
  /// # Returns
  /// A new instance of `VectorSubformer` with an empty internal formed.
  ///
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    Self::begin
    (
      None,
      None,
      ReturnFormed,
    )
  }

}

impl< E, Definition, Context, End > VectorSubformer< E, Definition, Context, End >
where
  End : FormingEnd< Definition, Context >,
  Definition : FormerDefinition,
  Definition::Storage : ContainerAdd< Element = E >,
{

  /// Appends an element to the end of the storage, expanding the internal collection.
  #[ inline( always ) ]
  pub fn push< IntoElement >( mut self, element : IntoElement ) -> Self
  where IntoElement : core::convert::Into< E >,
  {
    if self.storage.is_none()
    {
      self.storage = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut storage ) = self.storage
    {
      ContainerAdd::add( storage, element.into() );
      // storage.push( element.into() );
    }
    self
  }

}

//

impl< E, Definition, Context, End > FormerBegin< Definition, Context >
for VectorSubformer< E, Definition, Context, End >
where
  End : FormingEnd< Definition, Context >,
  Definition : FormerDefinition,
  Definition::Storage : ContainerAdd< Element = E >,
{
  type End = End;

  #[ inline( always ) ]
  fn _begin
  (
    storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Context >,
    on_end : End,
  )
  -> Self
  {
    Self::begin( storage, context, on_end )
  }

}
