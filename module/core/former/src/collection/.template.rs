//! This module provides a comprehensive approach to applying the builder pattern to `___` collections.
//!
//! By leveraging traits such as `Collection`, `CollectionAdd`, `CollectionAssign`, and `CollectionValToEntry`,
//! this module abstracts the operations on ___-like data structures, making them more flexible and easier to integrate as
//! as subformer, enabling fluid and intuitive manipulation of ___s via builder patterns.
//!

use crate::*;
#[ allow( unused ) ]
use collection_tools::___::___;

impl< E > Collection for ___< E >
{
  type Entry = E;
  type Val = E;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e
  }

}

impl< E > CollectionAdd for ___< E >
{

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.push( e );
    true
  }

}

impl< E > CollectionAssign for ___< E >
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

impl< E > CollectionValToEntry< E > for ___< E >
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
for ___< E >
{
  type Preformed = ___< E >;
}

impl< E > StoragePreform
for ___< E >
{
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

/// Represents the formation definition for a ___-like collection within the former framework.
///
/// This structure defines the necessary parameters and relationships needed to form a ___-like collection,
/// including its storage, context, the result of the formation process, and the behavior at the end of the formation.
///
/// # Type Parameters
/// - `E`: The element type of the ___.
/// - `Context`: The context needed for the formation, can be provided externally.
/// - `Formed`: The type formed at the end of the formation process, typically a `___<E>`.
/// - `End`: A trait determining the behavior at the end of the formation process.
///

#[ derive( Debug, Default ) ]
pub struct ___Definition< E, Context, Formed, End >
where
  End : FormingEnd< ___DefinitionTypes< E, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed, End ) >,
}

impl< E, Context, Formed, End > FormerDefinition
for ___Definition< E, Context, Formed, End >
where
  End : FormingEnd< ___DefinitionTypes< E, Context, Formed > >,
{
  type Storage = ___< E >;
  type Context = Context;
  type Formed = Formed;

  type Types = ___DefinitionTypes< E, Context, Formed >;
  type End = End;
}

// = definition type

/// Holds the generic parameters for the `___Definition`.
///
/// This struct acts as a companion to `___Definition`, providing a concrete definition of types used
/// in the formation process. It is crucial for linking the type parameters with the operational mechanics
/// of the formation and ensuring type safety and correctness throughout the formation lifecycle.
///
/// # Type Parameters
///
/// - `E`: The element type of the ___.
/// - `Context`: The context in which the ___ is formed.
/// - `Formed`: The type produced as a result of the formation process.

#[ derive( Debug, Default ) ]
pub struct ___DefinitionTypes< E, Context = (), Formed = ___< E > >
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed ) >,
}

impl< E, Context, Formed > FormerDefinitionTypes
for ___DefinitionTypes< E, Context, Formed >
{
  type Storage = ___< E >;
  type Context = Context;
  type Formed = Formed;
}

// = mutator

impl< E, Context, Formed > FormerMutator
for ___DefinitionTypes< E, Context, Formed >
{
}

// = Entity To

impl< E, Definition > EntityToFormer< Definition >
for ___< E >
where
  Definition : FormerDefinition
  <
    Storage = ___< E >,
    Types = ___DefinitionTypes
    <
      E,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = ___Former< E, Definition::Context, Definition::Formed, Definition::End >;
}

impl< E > crate::EntityToStorage
for ___< E >
{
  type Storage = ___< E >;
}

impl< E, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for ___< E >
where
  End : crate::FormingEnd< ___DefinitionTypes< E, Context, Formed > >,
{
  type Definition = ___Definition< E, Context, Formed, End >;
  type Types = ___DefinitionTypes< E, Context, Formed >;
}

impl< E, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for ___< E >
{
  type Types = ___DefinitionTypes< E, Context, Formed >;
}

// = subformer

/// Provides a streamlined builder interface for constructing ___-like collections.
///
/// `___Former` is a type alias that configures the `CollectionFormer` for use specifically with ___s.
/// It integrates the `___Definition` to facilitate the fluent and dynamic construction of ___s, leveraging
/// predefined settings to reduce boilerplate code. This approach enhances readability and simplifies the use of
/// ___s in custom data structures where builder patterns are desired.
///
/// The alias encapsulates complex generic parameters, making the construction process more accessible and maintainable.
/// It is particularly useful in scenarios where ___s are repeatedly used or configured in similar ways across different
/// parts of an application.
///

pub type ___Former< E, Context, Formed, End > =
CollectionFormer::< E, ___Definition< E, Context, Formed, End > >;

// = extension

/// Provides an extension method for ___s to facilitate the use of the builder pattern.
///
/// This trait extends the `___` type, enabling it to use the `___Former` interface directly.
/// This allows for fluent, expressive construction and manipulation of ___s, integrating seamlessly
/// with the builder pattern provided by the `former` framework. It's a convenience trait that simplifies
/// creating configured ___ builders with default settings.
///
pub trait ___Ext< E > : sealed::Sealed
{
  /// Initializes a builder pattern for `___` using a default `___Former`.
  fn former() -> ___Former< E, (), ___< E >, ReturnStorage >;
}

impl< E > ___Ext< E > for ___< E >
{
  fn former() -> ___Former< E, (), ___< E >, ReturnStorage >
  {
    ___Former::< E, (), ___< E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  pub trait Sealed {}
  impl< E > Sealed for super::___< E > {}
}
