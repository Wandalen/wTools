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
  type Formed = Vec< E >;
}

impl< E > StoragePerform
for Vec< E >
{
  fn preform( self ) -> Self::Formed
  {
    self
  }
}

// = definition

#[ derive( Debug ) ]
pub struct VectorDefinition< E, Context = (), Formed = Vec< E >, End = ReturnStorage >
// where
  // Self : FormerDefinitionTypes,
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed, End ) >,
}

impl< E, Context, Formed > VectorDefinition< E, Context, Formed >
{
  pub fn new() -> Self
  {
    Self { _phantom : core::marker::PhantomData }
  }
}

impl< E, Context, Formed > FormerDefinitionTypes
for VectorDefinition< E, Context, Formed >
{
  type Storage = Vec< E >;
  type Formed = Formed;
  type Context = Context;
}

// #[ derive( Debug ) ]
// pub struct VectorDefinition< E, Context = (), Formed = Vec< E >, End = ReturnStorage >
// where
//   End : FormingEnd< VectorDefinition< E, Context, Formed > >,
// {
//   _phantom : core::marker::PhantomData< ( E, Context, Formed, End ) >,
// }

// impl< E, Context, Formed, End > VectorDefinition< E, Context, Formed, End >
// where
//   End : FormingEnd< VectorDefinition< E, Context, Formed > >,
// {
//   pub fn new() -> Self
//   {
//     Self { _phantom : core::marker::PhantomData }
//   }
// }

impl< E, Context, Formed, End > FormerDefinition
for VectorDefinition< E, Context, Formed, End >
where
  End : FormingEnd< VectorDefinition< E, Context, Formed > >,
{
  type Types = VectorDefinition< E, Context, Formed >;
  type End = End;
}

// = subformer

/// A builder for constructing `VectorLike` containers, facilitating a fluent and flexible interface.
///
/// `VectorSubformer` leverages the `VectorLike` trait to enable the construction and manipulation
/// of vector-like containers in a builder pattern style, promoting readability and ease of use.

pub type VectorSubformer< E, Context, Formed, End > = ContainerSubformer::< E, VectorDefinition< E, Context, Formed, End > >;

// = extension

pub trait VecExt< E > : sealed::Sealed
{
  fn former() -> VectorSubformer< E, (), Vec< E >, ReturnStorage >;
}

impl< E > VecExt< E > for Vec< E >
{
  fn former() -> VectorSubformer< E, (), Vec< E >, ReturnStorage >
  {
    VectorSubformer::< E, (), Vec< E >, ReturnStorage >::new()
  }
}

mod sealed
{
  pub trait Sealed {}
  impl< E > Sealed for Vec< E > {}
}
