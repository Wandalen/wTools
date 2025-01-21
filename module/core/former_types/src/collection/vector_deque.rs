//! This module provides a comprehensive approach to applying the builder pattern to `VecDeque` collections.
//!
//! By leveraging traits such as `Collection`, `CollectionAdd`, `CollectionAssign`, and `CollectionValToEntry`,
//! this module abstracts the operations on vector deque-like data structures, making them more flexible and easier to integrate as
//! as subformer, enabling fluid and intuitive manipulation of vector deques via builder patterns.
//!
#[ allow( clippy::wildcard_imports ) ]
use crate::*;
#[ allow( unused ) ]
use collection_tools::VecDeque;

impl< E > Collection for VecDeque< E >
{
  type Entry = E;
  type Val = E;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e
  }

}

impl< E > CollectionAdd for VecDeque< E >
{

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.push_back( e );
    true
  }

}

impl< E > CollectionAssign for VecDeque< E >
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

impl< E > CollectionValToEntry< E > for VecDeque< E >
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
for VecDeque< E >
{
  type Preformed = VecDeque< E >;
}

impl< E > StoragePreform
for VecDeque< E >
{
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

/// Represents the formation definition for a vector deque-like collection within the former framework.
///
/// This structure defines the necessary parameters and relationships needed to form a vector deque-like collection,
/// including its storage, context, the result of the formation process, and the behavior at the end of the formation.
///
/// # Type Parameters
/// - `E`: The element type of the vector deque.
/// - `Context`: The context needed for the formation, can be provided externally.
/// - `Formed`: The type formed at the end of the formation process, typically a `VecDeque<E>`.
/// - `End`: A trait determining the behavior at the end of the formation process.
///

#[ derive( Debug, Default ) ]
pub struct VecDequeDefinition< E, Context, Formed, End >
where
  End : FormingEnd< VecDequeDefinitionTypes< E, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed, End ) >,
}

impl< E, Context, Formed, End > FormerDefinition
for VecDequeDefinition< E, Context, Formed, End >
where
  End : FormingEnd< VecDequeDefinitionTypes< E, Context, Formed > >,
{
  type Storage = VecDeque< E >;
  type Context = Context;
  type Formed = Formed;

  type Types = VecDequeDefinitionTypes< E, Context, Formed >;
  type End = End;
}

// = definition type

/// Holds the generic parameters for the `VecDequeDefinition`.
///
/// This struct acts as a companion to `VecDequeDefinition`, providing a concrete definition of types used
/// in the formation process. It is crucial for linking the type parameters with the operational mechanics
/// of the formation and ensuring type safety and correctness throughout the formation lifecycle.
///
/// # Type Parameters
///
/// - `E`: The element type of the vector deque.
/// - `Context`: The context in which the vector deque is formed.
/// - `Formed`: The type produced as a result of the formation process.

#[ derive( Debug, Default ) ]
pub struct VecDequeDefinitionTypes< E, Context = (), Formed = VecDeque< E > >
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed ) >,
}

impl< E, Context, Formed > FormerDefinitionTypes
for VecDequeDefinitionTypes< E, Context, Formed >
{
  type Storage = VecDeque< E >;
  type Context = Context;
  type Formed = Formed;
}

// = mutator

impl< E, Context, Formed > FormerMutator
for VecDequeDefinitionTypes< E, Context, Formed >
{
}

// = Entity To

impl< E, Definition > EntityToFormer< Definition >
for VecDeque< E >
where
  Definition : FormerDefinition
  <
    Storage = VecDeque< E >,
    Types = VecDequeDefinitionTypes
    <
      E,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = VecDequeFormer< E, Definition::Context, Definition::Formed, Definition::End >;
}

impl< E > crate::EntityToStorage
for VecDeque< E >
{
  type Storage = VecDeque< E >;
}

impl< E, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for VecDeque< E >
where
  End : crate::FormingEnd< VecDequeDefinitionTypes< E, Context, Formed > >,
{
  type Definition = VecDequeDefinition< E, Context, Formed, End >;
  type Types = VecDequeDefinitionTypes< E, Context, Formed >;
}

impl< E, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for VecDeque< E >
{
  type Types = VecDequeDefinitionTypes< E, Context, Formed >;
}

// = subformer

/// Provides a streamlined builder interface for constructing vector deque-like collections.
///
/// `VecDequeFormer` is a type alias that configures the `CollectionFormer` for use specifically with vector deques.
/// It integrates the `VecDequeDefinition` to facilitate the fluent and dynamic construction of vector deques, leveraging
/// predefined settings to reduce boilerplate code. This approach enhances readability and simplifies the use of
/// vector deques in custom data structures where builder patterns are desired.
///
/// The alias encapsulates complex generic parameters, making the construction process more accessible and maintainable.
/// It is particularly useful in scenarios where vector deques are repeatedly used or configured in similar ways across different
/// parts of an application.
///

pub type VecDequeFormer< E, Context, Formed, End > =
CollectionFormer::< E, VecDequeDefinition< E, Context, Formed, End > >;

// = extension

/// Provides an extension method for vector deques to facilitate the use of the builder pattern.
///
/// This trait extends the `VecDeque` type, enabling it to use the `VecDequeFormer` interface directly.
/// This allows for fluent, expressive construction and manipulation of vector deques, integrating seamlessly
/// with the builder pattern provided by the `former` framework. It's a convenience trait that simplifies
/// creating configured vector deque builders with default settings.
///
pub trait VecDequeExt< E > : sealed::Sealed
{
  /// Initializes a builder pattern for `VecDeque` using a default `VecDequeFormer`.
  fn former() -> VecDequeFormer< E, (), VecDeque< E >, ReturnStorage >;
}

impl< E > VecDequeExt< E > for VecDeque< E >
{
  #[ allow( clippy::default_constructed_unit_structs ) ]
  fn former() -> VecDequeFormer< E, (), VecDeque< E >, ReturnStorage >
  {
    VecDequeFormer::< E, (), VecDeque< E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  pub trait Sealed {}
  impl< E > Sealed for super::VecDeque< E > {}
}
