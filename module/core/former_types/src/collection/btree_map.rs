//! This module provides a comprehensive approach to applying the builder pattern to `BTreeMap` collections.
//!
//! By leveraging traits such as `Collection`, `CollectionAdd`, `CollectionAssign`, and `CollectionValToEntry`,
//! this module abstracts the operations on binary tree map-like data structures, making them more flexible and easier to integrate as
//! as subformer, enabling fluid and intuitive manipulation of binary tree maps via builder patterns.
//!
#[ allow( clippy::wildcard_imports ) ]
use crate::*;
use collection_tools::BTreeMap;

impl< K, V > Collection for BTreeMap< K, V >
where
  K : Ord,
{
  type Entry = ( K, V );
  type Val = V;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e.1
  }

}

impl< K, V > CollectionAdd for BTreeMap< K, V >
where
  K : Ord,
{

  #[ inline( always ) ]
  fn add( &mut self, ( k, v ) : Self::Entry ) -> bool
  {
    self.insert( k, v ).map_or_else( || true, | _ | false )
  }

}

impl< K, V > CollectionAssign for BTreeMap< K, V >
where
  K : Ord,
{

  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Entry >
  {
    let initial_len = self.len();
    self.extend( elements );
    self.len() - initial_len
  }
}

// = storage

impl< K, E > Storage
for BTreeMap< K, E >
where
  K : Ord,
{
  type Preformed = BTreeMap< K, E >;
}

impl< K, E > StoragePreform
for BTreeMap< K, E >
where
  K : Ord,
{
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

/// Represents the formation definition for a hash map-like collection within the former framework.
///
/// This structure defines the essential elements required to form a hash map-like collection, detailing
/// the key and value types, the contextual environment during formation, the final formed type, and the
/// behavior at the end of the formation process. It facilitates customization and extension of hash map
/// formation within any system that implements complex data management operations.
///
/// # Type Parameters
/// - `K`: The key type of the hash map.
/// - `E`: The value type of the hash map.
/// - `Context`: The optional context provided during the formation process.
/// - `Formed`: The type of the entity produced, typically a `BTreeMap<K, E>`.
/// - `End`: A trait defining the end behavior of the formation process, managing how the hash map is finalized.
///

#[ derive( Debug, Default ) ]
pub struct BTreeMapDefinition< K, E, Context = (), Formed = BTreeMap< K, E >, End = ReturnStorage >
where
  K : Ord,
  End : FormingEnd< BTreeMapDefinitionTypes< K, E, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( K, E, Context, Formed, End ) >,
}

impl< K, E, Context, Formed, End > FormerDefinition
for BTreeMapDefinition< K, E, Context, Formed, End >
where
  K : Ord,
  End : FormingEnd< BTreeMapDefinitionTypes< K, E, Context, Formed > >,
{

  type Storage = BTreeMap< K, E >;
  type Formed = Formed;
  type Context = Context;

  type Types = BTreeMapDefinitionTypes< K, E, Context, Formed >;
  type End = End;

}

// = definition types

/// Holds the generic parameters for the `BTreeMapDefinition`.
///
/// This companion struct to `BTreeMapDefinition` defines the storage type and the context, along with the
/// type that is ultimately formed through the process. It is crucial for maintaining the integrity and
/// consistency of type relations throughout the former lifecycle.
///
/// # Type Parameters
/// - `K`: The key type of the hash map.
/// - `E`: The value type of the hash map.
/// - `Context`: The operational context in which the hash map is formed.
/// - `Formed`: The type produced, typically mirroring the structure of a `BTreeMap<K, E>`.

#[ derive( Debug, Default ) ]
pub struct BTreeMapDefinitionTypes< K, E, Context = (), Formed = BTreeMap< K, E > >
{
  _phantom : core::marker::PhantomData< ( K, E, Context, Formed ) >,
}

impl< K, E, Context, Formed > FormerDefinitionTypes
for BTreeMapDefinitionTypes< K, E, Context, Formed >
where
  K : Ord,
{
  type Storage = BTreeMap< K, E >;
  type Formed = Formed;
  type Context = Context;
}

// = mutator

impl< K, E, Context, Formed > FormerMutator
for BTreeMapDefinitionTypes< K, E, Context, Formed >
where
  K : Ord,
{
}

// = Entity To

impl< K, E, Definition > EntityToFormer< Definition > for BTreeMap< K, E >
where
  K : Ord,
  Definition : FormerDefinition
  <
    Storage = BTreeMap< K, E >,
    Types = BTreeMapDefinitionTypes
    <
      K,
      E,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = BTreeMapFormer< K, E, Definition::Context, Definition::Formed, Definition::End >;
}

impl< K, E > crate::EntityToStorage
for BTreeMap< K, E >
where
  K : Ord,
{
  type Storage = BTreeMap< K, E >;
}

impl< K, E, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for BTreeMap< K, E >
where
  K : Ord,
  End : crate::FormingEnd< BTreeMapDefinitionTypes< K, E, Context, Formed > >,
{
  type Definition = BTreeMapDefinition< K, E, Context, Formed, End >;
  type Types = BTreeMapDefinitionTypes< K, E, Context, Formed >;
}

impl< K, E, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for BTreeMap< K, E >
where
  K : Ord,
{
  type Types = BTreeMapDefinitionTypes< K, E, Context, Formed >;
}

// = subformer

/// Provides a streamlined builder interface for constructing hash map-like collections.
///
/// `BTreeMapFormer` is a type alias that configures the `CollectionFormer` specifically for hash maps,
/// facilitating a more intuitive and flexible way to build and manipulate hash maps within custom data structures.
/// This type alias simplifies the usage of hash maps in builder patterns by encapsulating complex generic parameters
/// and leveraging the `BTreeMapDefinition` to handle the construction logic. It supports fluent chaining of key-value
/// insertions and can be customized with various end actions to finalize the hash map upon completion.
///
/// The alias helps reduce boilerplate code and enhances readability, making the construction of hash maps in
/// a builder pattern both efficient and expressive.

pub type BTreeMapFormer< K, E, Context, Formed, End > =
CollectionFormer::< ( K, E ), BTreeMapDefinition< K, E, Context, Formed, End > >;

// = extension

/// Provides an extension method for hash maps to facilitate the use of the builder pattern.
///
/// This trait extends the `BTreeMap` type, enabling it to use the `BTreeMapFormer` interface directly.
/// It allows for fluent, expressive construction and manipulation of hash maps, integrating seamlessly
/// with the builder pattern provided by the `former` framework. It's a convenience trait that simplifies
/// creating configured hash map builders with default settings.
///

pub trait BTreeMapExt< K, E > : sealed::Sealed
where
  K : Ord,
{
  /// Initializes a builder pattern for `BTreeMap` using a default `BTreeMapFormer`.
  fn former() -> BTreeMapFormer< K, E, (), BTreeMap< K, E >, ReturnStorage >;
}

impl< K, E > BTreeMapExt< K, E > for BTreeMap< K, E >
where
  K : Ord,
{
  #[ allow( clippy::default_constructed_unit_structs ) ]
  fn former() -> BTreeMapFormer< K, E, (), BTreeMap< K, E >, ReturnStorage >
  {
    BTreeMapFormer::< K, E, (), BTreeMap< K, E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  use super::BTreeMap;
  pub trait Sealed {}
  impl< K, E > Sealed for BTreeMap< K, E > {}
}
