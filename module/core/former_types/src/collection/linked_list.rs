//! This module provides a comprehensive approach to applying the builder pattern to `LinkedList` collections.
//!
//! By leveraging traits such as `Collection`, `CollectionAdd`, `CollectionAssign`, and `CollectionValToEntry`,
//! this module abstracts the operations on list-like data structures, making them more flexible and easier to integrate as
//! as subformer, enabling fluid and intuitive manipulation of lists via builder patterns.
//!

use crate::*;
#[ allow( unused ) ]
use collection_tools::LinkedList;

impl< E > Collection for LinkedList< E >
{
  type Entry = E;
  type Val = E;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e
  }

}

impl< E > CollectionAdd for LinkedList< E >
{

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.push_back( e );
    true
  }

}

impl< E > CollectionAssign for LinkedList< E >
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

impl< E > CollectionValToEntry< E > for LinkedList< E >
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
for LinkedList< E >
{
  type Preformed = LinkedList< E >;
}

impl< E > StoragePreform
for LinkedList< E >
{
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

/// Represents the formation definition for a list-like collection within the former framework.
///
/// This structure defines the necessary parameters and relationships needed to form a list-like collection,
/// including its storage, context, the result of the formation process, and the behavior at the end of the formation.
///
/// # Type Parameters
/// - `E`: The element type of the list.
/// - `Context`: The context needed for the formation, can be provided externally.
/// - `Formed`: The type formed at the end of the formation process, typically a `LinkedList<E>`.
/// - `End`: A trait determining the behavior at the end of the formation process.
///

#[ derive( Debug, Default ) ]
pub struct LinkedListDefinition< E, Context, Formed, End >
where
  End : FormingEnd< LinkedListDefinitionTypes< E, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed, End ) >,
}

impl< E, Context, Formed, End > FormerDefinition
for LinkedListDefinition< E, Context, Formed, End >
where
  End : FormingEnd< LinkedListDefinitionTypes< E, Context, Formed > >,
{
  type Storage = LinkedList< E >;
  type Context = Context;
  type Formed = Formed;

  type Types = LinkedListDefinitionTypes< E, Context, Formed >;
  type End = End;
}

// = definition type

/// Holds the generic parameters for the `LinkedListDefinition`.
///
/// This struct acts as a companion to `LinkedListDefinition`, providing a concrete definition of types used
/// in the formation process. It is crucial for linking the type parameters with the operational mechanics
/// of the formation and ensuring type safety and correctness throughout the formation lifecycle.
///
/// # Type Parameters
///
/// - `E`: The element type of the list.
/// - `Context`: The context in which the list is formed.
/// - `Formed`: The type produced as a result of the formation process.

#[ derive( Debug, Default ) ]
pub struct LinkedListDefinitionTypes< E, Context = (), Formed = LinkedList< E > >
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed ) >,
}

impl< E, Context, Formed > FormerDefinitionTypes
for LinkedListDefinitionTypes< E, Context, Formed >
{
  type Storage = LinkedList< E >;
  type Context = Context;
  type Formed = Formed;
}

// = mutator

impl< E, Context, Formed > FormerMutator
for LinkedListDefinitionTypes< E, Context, Formed >
{
}

// = Entity To

impl< E, Definition > EntityToFormer< Definition >
for LinkedList< E >
where
  Definition : FormerDefinition
  <
    Storage = LinkedList< E >,
    Types = LinkedListDefinitionTypes
    <
      E,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = LinkedListFormer< E, Definition::Context, Definition::Formed, Definition::End >;
}

impl< E > crate::EntityToStorage
for LinkedList< E >
{
  type Storage = LinkedList< E >;
}

impl< E, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for LinkedList< E >
where
  End : crate::FormingEnd< LinkedListDefinitionTypes< E, Context, Formed > >,
{
  type Definition = LinkedListDefinition< E, Context, Formed, End >;
  type Types = LinkedListDefinitionTypes< E, Context, Formed >;
}

impl< E, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for LinkedList< E >
{
  type Types = LinkedListDefinitionTypes< E, Context, Formed >;
}

// = subformer

/// Provides a streamlined builder interface for constructing list-like collections.
///
/// `LinkedListFormer` is a type alias that configures the `CollectionFormer` for use specifically with lists.
/// It integrates the `LinkedListDefinition` to facilitate the fluent and dynamic construction of lists, leveraging
/// predefined settings to reduce boilerplate code. This approach enhances readability and simplifies the use of
/// lists in custom data structures where builder patterns are desired.
///
/// The alias encapsulates complex generic parameters, making the construction process more accessible and maintainable.
/// It is particularly useful in scenarios where lists are repeatedly used or configured in similar ways across different
/// parts of an application.
///

pub type LinkedListFormer< E, Context, Formed, End > =
CollectionFormer::< E, LinkedListDefinition< E, Context, Formed, End > >;

// = extension

/// Provides an extension method for lists to facilitate the use of the builder pattern.
///
/// This trait extends the `LinkedList` type, enabling it to use the `LinkedListFormer` interface directly.
/// This allows for fluent, expressive construction and manipulation of lists, integrating seamlessly
/// with the builder pattern provided by the `former` framework. It's a convenience trait that simplifies
/// creating configured list builders with default settings.
///
pub trait LinkedListExt< E > : sealed::Sealed
{
  /// Initializes a builder pattern for `LinkedList` using a default `LinkedListFormer`.
  fn former() -> LinkedListFormer< E, (), LinkedList< E >, ReturnStorage >;
}

impl< E > LinkedListExt< E > for LinkedList< E >
{
  fn former() -> LinkedListFormer< E, (), LinkedList< E >, ReturnStorage >
  {
    LinkedListFormer::< E, (), LinkedList< E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  pub trait Sealed {}
  impl< E > Sealed for super::LinkedList< E > {}
}
