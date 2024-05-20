//! This module provides a comprehensive approach to applying the builder pattern to `BTreeSet` collections.
//!
//! By leveraging traits such as `Collection`, `CollectionAdd`, `CollectionAssign`, and `CollectionValToEntry`,
//! this module abstracts the operations on binary tree set-like data structures, making them more flexible and easier to integrate as
//! as subformer, enabling fluid and intuitive manipulation of binary tree sets via builder patterns.
//!

use crate::*;
#[ allow( unused ) ]
use collection_tools::BTreeSet;

impl< E > Collection for BTreeSet< E >
{
  type Entry = E;
  type Val = E;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e
  }

}

impl< E > CollectionAdd for BTreeSet< E >
where
  E : Ord
{

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.insert( e );
    true
  }

}

impl< E > CollectionAssign for BTreeSet< E >
where
  E : Ord
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

impl< E > CollectionValToEntry< E > for BTreeSet< E >
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
for BTreeSet< E >
{
  type Preformed = BTreeSet< E >;
}

impl< E > StoragePreform
for BTreeSet< E >
{
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

/// Represents the formation definition for a binary tree set-like collection within the former framework.
///
/// This structure defines the necessary parameters and relationships needed to form a binary tree set-like collection,
/// including its storage, context, the result of the formation process, and the behavior at the end of the formation.
///
/// # Type Parameters
/// - `E`: The element type of the binary tree set.
/// - `Context`: The context needed for the formation, can be provided externally.
/// - `Formed`: The type formed at the end of the formation process, typically a `BTreeSet<E>`.
/// - `End`: A trait determining the behavior at the end of the formation process.
///

#[ derive( Debug, Default ) ]
pub struct BTreeSetDefinition< E, Context, Formed, End >
where
  End : FormingEnd< BTreeSetDefinitionTypes< E, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed, End ) >,
}

impl< E, Context, Formed, End > FormerDefinition
for BTreeSetDefinition< E, Context, Formed, End >
where
  End : FormingEnd< BTreeSetDefinitionTypes< E, Context, Formed > >,
{
  type Storage = BTreeSet< E >;
  type Context = Context;
  type Formed = Formed;

  type Types = BTreeSetDefinitionTypes< E, Context, Formed >;
  type End = End;
}

// = definition type

/// Holds the generic parameters for the `BTreeSetDefinition`.
///
/// This struct acts as a companion to `BTreeSetDefinition`, providing a concrete definition of types used
/// in the formation process. It is crucial for linking the type parameters with the operational mechanics
/// of the formation and ensuring type safety and correctness throughout the formation lifecycle.
///
/// # Type Parameters
///
/// - `E`: The element type of the binary tree set.
/// - `Context`: The context in which the binary tree set is formed.
/// - `Formed`: The type produced as a result of the formation process.

#[ derive( Debug, Default ) ]
pub struct BTreeSetDefinitionTypes< E, Context = (), Formed = BTreeSet< E > >
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed ) >,
}

impl< E, Context, Formed > FormerDefinitionTypes
for BTreeSetDefinitionTypes< E, Context, Formed >
{
  type Storage = BTreeSet< E >;
  type Context = Context;
  type Formed = Formed;
}

// = mutator

impl< E, Context, Formed > FormerMutator
for BTreeSetDefinitionTypes< E, Context, Formed >
{
}

// = Entity To

impl< E, Definition > EntityToFormer< Definition >
for BTreeSet< E >
where
  E : Ord,
  Definition : FormerDefinition
  <
    Storage = BTreeSet< E >,
    Types = BTreeSetDefinitionTypes
    <
      E,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = BTreeSetFormer< E, Definition::Context, Definition::Formed, Definition::End >;
}

impl< E > crate::EntityToStorage
for BTreeSet< E >
{
  type Storage = BTreeSet< E >;
}

impl< E, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for BTreeSet< E >
where
  End : crate::FormingEnd< BTreeSetDefinitionTypes< E, Context, Formed > >,
{
  type Definition = BTreeSetDefinition< E, Context, Formed, End >;
  type Types = BTreeSetDefinitionTypes< E, Context, Formed >;
}

impl< E, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for BTreeSet< E >
{
  type Types = BTreeSetDefinitionTypes< E, Context, Formed >;
}

// = subformer

/// Provides a streamlined builder interface for constructing binary tree set-like collections.
///
/// `BTreeSetFormer` is a type alias that configures the `CollectionFormer` for use specifically with binary tree sets.
/// It integrates the `BTreeSetDefinition` to facilitate the fluent and dynamic construction of binary tree sets, leveraging
/// predefined settings to reduce boilerplate code. This approach enhances readability and simplifies the use of
/// binary tree sets in custom data structures where builder patterns are desired.
///
/// The alias encapsulates complex generic parameters, making the construction process more accessible and maintainable.
/// It is particularly useful in scenarios where binary tree sets are repeatedly used or configured in similar ways across different
/// parts of an application.
///

pub type BTreeSetFormer< E, Context, Formed, End > =
CollectionFormer::< E, BTreeSetDefinition< E, Context, Formed, End > >;

// = extension

/// Provides an extension method for binary tree sets to facilitate the use of the builder pattern.
///
/// This trait extends the `BTreeSet` type, enabling it to use the `BTreeSetFormer` interface directly.
/// This allows for fluent, expressive construction and manipulation of binary tree sets, integrating seamlessly
/// with the builder pattern provided by the `former` framework. It's a convenience trait that simplifies
/// creating configured binary tree set builders with default settings.
///
pub trait BTreeSetExt< E > : sealed::Sealed
where
  E : Ord
{
  /// Initializes a builder pattern for `BTreeSet` using a default `BTreeSetFormer`.
  fn former() -> BTreeSetFormer< E, (), BTreeSet< E >, ReturnStorage >;
}

impl< E > BTreeSetExt< E > for BTreeSet< E >
where
  E : Ord
{
  fn former() -> BTreeSetFormer< E, (), BTreeSet< E >, ReturnStorage >
  {
    BTreeSetFormer::< E, (), BTreeSet< E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  pub trait Sealed {}
  impl< E > Sealed for super::BTreeSet< E > {}
}
