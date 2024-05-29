//! This module provides a comprehensive approach to applying the builder pattern to `Vec` collections.
//!
//! By leveraging traits such as `Collection`, `CollectionAdd`, `CollectionAssign`, and `CollectionValToEntry`,
//! this module abstracts the operations on vector-like data structures, making them more flexible and easier to integrate as
//! as subformer, enabling fluid and intuitive manipulation of vectors via builder patterns.
//!

use crate::*;
#[ allow( unused ) ]
use collection_tools::Vec;

impl< E > Collection for Vec< E >
{
  type Entry = E;
  type Val = E;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e
  }

}

impl< E > CollectionAdd for Vec< E >
{

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.push( e );
    true
  }

}

impl< E > CollectionAssign for Vec< E >
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

impl< E > CollectionValToEntry< E > for Vec< E >
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
  type Preformed = Vec< E >;
}

impl< E > StoragePreform
for Vec< E >
{
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

/// Represents the formation definition for a vector-like collection within the former framework.
///
/// This structure defines the necessary parameters and relationships needed to form a vector-like collection,
/// including its storage, context, the result of the formation process, and the behavior at the end of the formation.
///
/// # Type Parameters
/// - `E`: The element type of the vector.
/// - `Context`: The context needed for the formation, can be provided externally.
/// - `Formed`: The type formed at the end of the formation process, typically a `Vec<E>`.
/// - `End`: A trait determining the behavior at the end of the formation process.
///

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

/// Holds the generic parameters for the `VectorDefinition`.
///
/// This struct acts as a companion to `VectorDefinition`, providing a concrete definition of types used
/// in the formation process. It is crucial for linking the type parameters with the operational mechanics
/// of the formation and ensuring type safety and correctness throughout the formation lifecycle.
///
/// # Type Parameters
///
/// - `E`: The element type of the vector.
/// - `Context`: The context in which the vector is formed.
/// - `Formed`: The type produced as a result of the formation process.

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
  type Former = VectorFormer< E, Definition::Context, Definition::Formed, Definition::End >;
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

/// Provides a streamlined builder interface for constructing vector-like collections.
///
/// `VectorFormer` is a type alias that configures the `CollectionFormer` for use specifically with vectors.
/// It integrates the `VectorDefinition` to facilitate the fluent and dynamic construction of vectors, leveraging
/// predefined settings to reduce boilerplate code. This approach enhances readability and simplifies the use of
/// vectors in custom data structures where builder patterns are desired.
///
/// The alias encapsulates complex generic parameters, making the construction process more accessible and maintainable.
/// It is particularly useful in scenarios where vectors are repeatedly used or configured in similar ways across different
/// parts of an application.
///

pub type VectorFormer< E, Context, Formed, End > =
CollectionFormer::< E, VectorDefinition< E, Context, Formed, End > >;

// = extension

/// Provides an extension method for vectors to facilitate the use of the builder pattern.
///
/// This trait extends the `Vec` type, enabling it to use the `VectorFormer` interface directly.
/// This allows for fluent, expressive construction and manipulation of vectors, integrating seamlessly
/// with the builder pattern provided by the `former` framework. It's a convenience trait that simplifies
/// creating configured vector builders with default settings.
///
pub trait VecExt< E > : sealed::Sealed
{
  /// Initializes a builder pattern for `Vec` using a default `VectorFormer`.
  fn former() -> VectorFormer< E, (), Vec< E >, ReturnStorage >;
}

impl< E > VecExt< E > for Vec< E >
{
  fn former() -> VectorFormer< E, (), Vec< E >, ReturnStorage >
  {
    VectorFormer::< E, (), Vec< E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  pub trait Sealed {}
  impl< E > Sealed for super::Vec< E > {}
}
