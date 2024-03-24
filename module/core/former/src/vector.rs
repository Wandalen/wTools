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

// = storage

impl< E > Storage
for Vec< E >
{
  // type Definition = VectorDefinition< E >;
  type Formed = Vec< E >;
}

impl< E > StoragePerform
for Vec< E >
{
  // fn preform( self ) -> < < Self as Storage >::Definition as FormerDefinition >::Formed
  fn preform( self ) -> Self::Formed
  {
    self
  }
}

// = definition

#[ derive( Debug ) ]
pub struct VectorDefinitionTypes< E, Context = () >
where
  Self : FormerDefinition,
{
  _phantom : core::marker::PhantomData< ( E, Context ) >,
}

impl< E, Context > VectorDefinitionTypes< E, Context >
{
  pub fn new() -> Self
  {
    Self { _phantom : core::marker::PhantomData }
  }
}

impl< E, Context > FormerDefinition
for VectorDefinitionTypes< E, Context >
{
  type Storage = Vec< E >;
  type Formed = Vec< E >;
  type Context = Context;
}

#[ derive( Debug ) ]
pub struct VectorDefinition< E, Context = (), End = ReturnStorage >
// pub struct VectorDefinition< E, Context, End >
where
  End : FormingEnd< VectorDefinitionTypes< E, Context > >,
{
  _phantom : core::marker::PhantomData< ( E, Context, End ) >,
}

impl< E, Context, End > VectorDefinition< E, Context, End >
where
  End : FormingEnd< VectorDefinitionTypes< E, Context > >,
{
  pub fn new() -> Self
  {
    Self { _phantom : core::marker::PhantomData }
  }
}

impl< E, Context, End > FormerDefinition2
for VectorDefinition< E, Context, End >
where
  End : FormingEnd< VectorDefinitionTypes< E, Context > >,
{
  type Definition = VectorDefinitionTypes< E, Context >;
  type End = End;
}

// = subformer

/// A builder for constructing `VectorLike` containers, facilitating a fluent and flexible interface.
///
/// `VectorSubformer` leverages the `VectorLike` trait to enable the construction and manipulation
/// of vector-like containers in a builder pattern style, promoting readability and ease of use.

pub type VectorSubformer< E, Context, End > = ContainerSubformer::< E, VectorDefinition< E, Context, End > >;

// = extension

pub trait VecExt< E > : sealed::Sealed
{
  fn former() -> VectorSubformer< E, (), ReturnStorage >;
}

impl< E > VecExt< E > for Vec< E >
{
  fn former() -> VectorSubformer< E, (), ReturnStorage >
  {
    VectorSubformer::< E, (), ReturnStorage >::new()
  }
}

mod sealed
{
  pub trait Sealed {}
  impl< E > Sealed for Vec< E > {}
}
