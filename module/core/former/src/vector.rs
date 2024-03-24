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

//

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

//

#[ derive( Debug ) ]
// pub struct VectorDefinition1< E, Context = (), End = ReturnStorage >
pub struct VectorDefinition1< E, Context >
where
  Self : FormerDefinition,
{
  _phantom : core::marker::PhantomData< ( E, Context ) >,
}

impl< E, Context > VectorDefinition1< E, Context >
{
  pub fn new() -> Self
  {
    Self { _phantom : core::marker::PhantomData }
  }
}

impl< E, Context > FormerDefinition
for VectorDefinition1< E, Context >
{
  type Storage = Vec< E >;
  type Formed = Vec< E >;
  type Context = Context;
}

#[ derive( Debug ) ]
// pub struct VectorDefinition2< E, Context = (), End = ReturnStorage >
pub struct VectorDefinition2< E, Context, End >
where
  End : FormingEnd< VectorDefinition1< E, Context > >,
{
  _phantom : core::marker::PhantomData< ( E, Context, End ) >,
}

impl< E, Context, End > VectorDefinition2< E, Context, End >
where
  End : FormingEnd< VectorDefinition1< E, Context > >,
{
  pub fn new() -> Self
  {
    Self { _phantom : core::marker::PhantomData }
  }
}

impl< E, Context, End > FormerDefinition2
for VectorDefinition2< E, Context, End >
where
  End : FormingEnd< VectorDefinition1< E, Context > >,
{
  type Definition = VectorDefinition1< E, Context >;
  type End = End;
}

/// A builder for constructing `VectorLike` containers, facilitating a fluent and flexible interface.
///
/// `VectorSubformer` leverages the `VectorLike` trait to enable the construction and manipulation
/// of vector-like containers in a builder pattern style, promoting readability and ease of use.

pub type VectorSubformer< E, Context, End > = ContainerSubformer::< E, VectorDefinition2< E, Context, End > >;
