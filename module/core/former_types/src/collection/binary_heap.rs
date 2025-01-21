//! This module provides a comprehensive approach to applying the builder pattern to `BinaryHeap` collections.
//!
//! By leveraging traits such as `Collection`, `CollectionAdd`, `CollectionAssign`, and `CollectionValToEntry`,
//! this module abstracts the operations on binary heap-like data structures, making them more flexible and easier to integrate as
//! as subformer, enabling fluid and intuitive manipulation of binary heaps via builder patterns.
//!

#[ allow( clippy::wildcard_imports ) ]
use crate::*;
#[ allow( unused ) ]
use collection_tools::BinaryHeap;

impl< E > Collection for BinaryHeap< E >
{
  type Entry = E;
  type Val = E;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e
  }

}

impl< E > CollectionAdd for BinaryHeap< E >
where
  E : Ord
{

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.push( e );
    true
  }

}

impl< E > CollectionAssign for BinaryHeap< E >
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

impl< E > CollectionValToEntry< E > for BinaryHeap< E >
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
for BinaryHeap< E >
where
  E : Ord
{
  type Preformed = BinaryHeap< E >;
}

impl< E > StoragePreform
for BinaryHeap< E >
where
  E : Ord
{
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

/// Represents the formation definition for a binary heap-like collection within the former framework.
///
/// This structure defines the necessary parameters and relationships needed to form a binary heap-like collection,
/// including its storage, context, the result of the formation process, and the behavior at the end of the formation.
///
/// # Type Parameters
/// - `E`: The element type of the binary heap.
/// - `Context`: The context needed for the formation, can be provided externally.
/// - `Formed`: The type formed at the end of the formation process, typically a `BinaryHeap<E>`.
/// - `End`: A trait determining the behavior at the end of the formation process.
///

#[ derive( Debug, Default ) ]
pub struct BinaryHeapDefinition< E, Context, Formed, End >
where
  E : Ord,
  End : FormingEnd< BinaryHeapDefinitionTypes< E, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed, End ) >,
}

impl< E, Context, Formed, End > FormerDefinition
for BinaryHeapDefinition< E, Context, Formed, End >
where
  E : Ord,
  End : FormingEnd< BinaryHeapDefinitionTypes< E, Context, Formed > >,
{
  type Storage = BinaryHeap< E >;
  type Context = Context;
  type Formed = Formed;

  type Types = BinaryHeapDefinitionTypes< E, Context, Formed >;
  type End = End;
}

// = definition type

/// Holds the generic parameters for the `BinaryHeapDefinition`.
///
/// This struct acts as a companion to `BinaryHeapDefinition`, providing a concrete definition of types used
/// in the formation process.
///
/// # Type Parameters
///
/// - `E`: The element type of the binary heap.
/// - `Context`: The context in which the binary heap is formed.
/// - `Formed`: The type produced as a result of the formation process.

#[ derive( Debug, Default ) ]
pub struct BinaryHeapDefinitionTypes< E, Context = (), Formed = BinaryHeap< E > >
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed ) >,
}

impl< E, Context, Formed > FormerDefinitionTypes
for BinaryHeapDefinitionTypes< E, Context, Formed >
where
  E : Ord
{
  type Storage = BinaryHeap< E >;
  type Context = Context;
  type Formed = Formed;
}

// = mutator

impl< E, Context, Formed > FormerMutator
for BinaryHeapDefinitionTypes< E, Context, Formed >
where
  E : Ord
{
}

// = Entity To

impl< E, Definition > EntityToFormer< Definition >
for BinaryHeap< E >
where
  E : Ord,
  Definition : FormerDefinition
  <
    Storage = BinaryHeap< E >,
    Types = BinaryHeapDefinitionTypes
    <
      E,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = BinaryHeapFormer< E, Definition::Context, Definition::Formed, Definition::End >;
}

impl< E > crate::EntityToStorage
for BinaryHeap< E >
{
  type Storage = BinaryHeap< E >;
}

impl< E, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for BinaryHeap< E >
where
  E : Ord,
  End : crate::FormingEnd< BinaryHeapDefinitionTypes< E, Context, Formed > >,
{
  type Definition = BinaryHeapDefinition< E, Context, Formed, End >;
  type Types = BinaryHeapDefinitionTypes< E, Context, Formed >;
}

impl< E, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for BinaryHeap< E >
where
  E : Ord
{
  type Types = BinaryHeapDefinitionTypes< E, Context, Formed >;
}

// = subformer

/// Provides a streamlined builder interface for constructing binary heap-like collections.
///
/// `BinaryHeapFormer` is a type alias that configures the `CollectionFormer` for use specifically with binary heaps.
/// It integrates the `BinaryHeapDefinition` to facilitate the fluent and dynamic construction of binary heaps, leveraging
/// predefined settings to reduce boilerplate code. This approach enhances readability and simplifies the use of
/// binary heaps in custom data structures where builder patterns are desired.
///
/// The alias encapsulates complex generic parameters, making the construction process more accessible and maintainable.
/// It is particularly useful in scenarios where binary heaps are repeatedly used or configured in similar ways across different
/// parts of an application.
///

pub type BinaryHeapFormer< E, Context, Formed, End > =
CollectionFormer::< E, BinaryHeapDefinition< E, Context, Formed, End > >;

// = extension

/// Provides an extension method for binary heaps to facilitate the use of the builder pattern.
///
/// This trait extends the `BinaryHeap` type, enabling it to use the `BinaryHeapFormer` interface directly.
/// This allows for fluent, expressive construction and manipulation of binary heaps, integrating seamlessly
/// with the builder pattern provided by the `former` framework. It's a convenience trait that simplifies
/// creating configured binary heap builders with default settings.
///
pub trait BinaryHeapExt< E > : sealed::Sealed
where
  E : Ord
{
  /// Initializes a builder pattern for `BinaryHeap` using a default `BinaryHeapFormer`.
  fn former() -> BinaryHeapFormer< E, (), BinaryHeap< E >, ReturnStorage >;
}

impl< E > BinaryHeapExt< E > for BinaryHeap< E >
where
  E : Ord
{
  #[ allow( clippy::default_constructed_unit_structs ) ]
  fn former() -> BinaryHeapFormer< E, (), BinaryHeap< E >, ReturnStorage >
  {
    BinaryHeapFormer::< E, (), BinaryHeap< E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  pub trait Sealed {}
  impl< E > Sealed for super::BinaryHeap< E > {}
}
