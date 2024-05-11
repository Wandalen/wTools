//! This module provides a comprehensive approach to applying the builder pattern to `Vec` containers.
//!
//! By leveraging traits such as `Container`, `ContainerAdd`, `ContainerAssign`, and `ContainerValToEntry`,
//! this module abstracts the operations on vector-like data structures, making them more flexible and easier to integrate as
//! as subformer, enabling fluid and intuitive manipulation of vectors via builder patterns.
//!

use super::*;
// use axiomatic::*;

#[ allow( unused ) ]
use collection_tools::Vec;

impl< E > Container for collection_tools::Vec< E >
{
  type Entry = E;
  type Val = E;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e
  }

}

impl< E > ContainerAdd for collection_tools::Vec< E >
{

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.push( e );
    true
  }

}

impl< E > ContainerAssign for collection_tools::Vec< E >
{
  #[ inline( always ) ]
  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Entry >
  {
    let initial_len = self.len();
    self.extend( elements );
    self.len() - initial_len
  }

}

impl< E > ContainerValToEntry< E > for collection_tools::Vec< E >
where
{
  type Entry = E;
  #[ inline( always ) ]
  fn val_to_entry( val : E ) -> Self::Entry
  {
    val
  }
}

// = storage

impl< E > Storage
for Vec< E >
{
  // type Formed = Vec< E >;
  type Preformed = Vec< E >;
}

impl< E > StoragePreform
for Vec< E >
{
  // type Preformed = Vec< E >;
  // fn preform( self ) -> Self::Formed
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

#[ derive( Debug, Default ) ]
pub struct VectorDefinition< E, Context, Formed, End >
where
  End : FormingEnd< VectorDefinitionTypes< E, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed, End ) >,
}

impl< E, Context, Formed, End > FormerDefinition
for VectorDefinition< E, Context, Formed, End >
where
  End : FormingEnd< VectorDefinitionTypes< E, Context, Formed > >,
{
  type Storage = Vec< E >;
  type Context = Context;
  type Formed = Formed;

  type Types = VectorDefinitionTypes< E, Context, Formed >;
  type End = End;
}

// = definition type

#[ derive( Debug, Default ) ]
pub struct VectorDefinitionTypes< E, Context = (), Formed = Vec< E > >
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed ) >,
}

impl< E, Context, Formed > FormerDefinitionTypes
for VectorDefinitionTypes< E, Context, Formed >
{
  type Storage = Vec< E >;
  type Context = Context;
  type Formed = Formed;
}

// = mutator

impl< E, Context, Formed > FormerMutator
for VectorDefinitionTypes< E, Context, Formed >
{
}

// = Entity To

impl< E, Definition > EntityToFormer< Definition >
for Vec< E >
where
  Definition : FormerDefinition
  <
    Storage = Vec< E >,
    Types = VectorDefinitionTypes
    <
      E,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = VectorAsSubformer< E, Definition::Context, Definition::Formed, Definition::End >;
}

impl< E > crate::EntityToStorage
for Vec< E >
{
  type Storage = Vec< E >;
}

impl< E, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for Vec< E >
where
  End : crate::FormingEnd< VectorDefinitionTypes< E, Context, Formed > >,
{
  type Definition = VectorDefinition< E, Context, Formed, End >;
  type Types = VectorDefinitionTypes< E, Context, Formed >;
}

impl< E, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for Vec< E >
{
  type Types = VectorDefinitionTypes< E, Context, Formed >;
}

// = subformer

/// Provides a streamlined builder interface for constructing vector-like containers.
///
/// `VectorAsSubformer` is a type alias that configures the `ContainerSubformer` for use specifically with vectors.
/// It integrates the `VectorDefinition` to facilitate the fluent and dynamic construction of vectors, leveraging
/// predefined settings to reduce boilerplate code. This approach enhances readability and simplifies the use of
/// vectors in custom data structures where builder patterns are desired.
///
/// The alias encapsulates complex generic parameters, making the construction process more accessible and maintainable.
/// It is particularly useful in scenarios where vectors are repeatedly used or configured in similar ways across different
/// parts of an application.
///

pub type VectorAsSubformer< E, Context, Formed, End > =
ContainerSubformer::< E, VectorDefinition< E, Context, Formed, End > >;

// = extension

pub trait VecExt< E > : sealed::Sealed
{
  fn former() -> VectorAsSubformer< E, (), Vec< E >, ReturnStorage >;
}

impl< E > VecExt< E > for Vec< E >
{
  fn former() -> VectorAsSubformer< E, (), Vec< E >, ReturnStorage >
  {
    VectorAsSubformer::< E, (), Vec< E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  pub trait Sealed {}
  impl< E > Sealed for super::Vec< E > {}
}
