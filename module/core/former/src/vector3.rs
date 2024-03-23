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
pub struct VectorDescriptor< E >
{
  _phantom : core::marker::PhantomData< E >,
}

impl< E > VectorDescriptor< E >
{
  pub fn new() -> Self
  {
    Self { _phantom : core::marker::PhantomData }
  }
}

impl< E > FormerDescriptor
for VectorDescriptor< E >
{
  type Storage = Vec< E >;
  type Formed = Vec< E >;
}

impl< E > StoragePerform
for Vec< E >
{
  type Formed = Self;
  fn preform( self ) -> Self::Formed
  {
    self
  }
}

/// A builder for constructing `VectorLike` containers, facilitating a fluent and flexible interface.
///
/// `VectorSubformer` leverages the `VectorLike` trait to enable the construction and manipulation
/// of vector-like containers in a builder pattern style, promoting readability and ease of use.
#[ derive( Debug, Default ) ]
pub struct VectorSubformer< E, Descriptor, Context, End >
where
  End : FormingEnd< Descriptor, Context >,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = E >,
{
  storage : core::option::Option< Descriptor::Storage >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
}

impl< E, Descriptor, Context, End > VectorSubformer< E, Descriptor, Context, End >
where
  End : FormingEnd< Descriptor, Context >,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = E >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn storage( mut self ) -> Descriptor::Storage
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
    storage : core::option::Option< Descriptor::Storage >,
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
  pub fn form( self ) -> Descriptor::Formed
  {
    self.end()
  }

  /// Finalizes the building process, returning the formed or a context incorporating it.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Descriptor::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let storage = self.storage();
    on_end.call( storage, context )
  }

  /// Replaces the current storage with a provided one, allowing for a reset or redirection of the building process.
  #[ inline( always ) ]
  pub fn replace( mut self, vector : Descriptor::Storage ) -> Self
  {
    self.storage = Some( vector );
    self
  }

}

impl< E, Descriptor > VectorSubformer< E, Descriptor, (), ReturnStorage >
where
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = E >,
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
      ReturnStorage,
    )
  }

}

impl< E, Descriptor, Context, End > VectorSubformer< E, Descriptor, Context, End >
where
  End : FormingEnd< Descriptor, Context >,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = E >,
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

impl< E, Descriptor, Context, End > FormerBegin< Descriptor, Context >
for VectorSubformer< E, Descriptor, Context, End >
where
  End : FormingEnd< Descriptor, Context >,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = E >,
{
  type End = End;

  #[ inline( always ) ]
  fn _begin
  (
    storage : core::option::Option< Descriptor::Storage >,
    context : core::option::Option< Context >,
    on_end : End,
  ) -> Self
  {
    Self::begin( storage, context, on_end )
  }

}
