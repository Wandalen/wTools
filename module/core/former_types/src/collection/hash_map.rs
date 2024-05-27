//! This module provides a comprehensive approach to applying the builder pattern to `HashMap` collections.
//!
//! By leveraging traits such as `Collection`, `CollectionAdd`, `CollectionAssign`, and `CollectionValToEntry`,
//! this module abstracts the operations on hashmap-like data structures, making them more flexible and easier to integrate as
//! as subformer, enabling fluid and intuitive manipulation of hashmaps via builder patterns.
//!

use crate::*;
use collection_tools::HashMap;

impl< K, V > Collection for HashMap< K, V >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  type Entry = ( K, V );
  type Val = V;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e.1
  }

}

impl< K, V > CollectionAdd for HashMap< K, V >
where
  K : core::cmp::Eq + core::hash::Hash,
{

  #[ inline( always ) ]
  fn add( &mut self, ( k, v ) : Self::Entry ) -> bool
  {
    self.insert( k, v ).map_or_else( || true, | _ | false )
  }

}

impl< K, V > CollectionAssign for HashMap< K, V >
where
  K : core::cmp::Eq + core::hash::Hash,
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
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Preformed = HashMap< K, E >;
}

impl< K, E > StoragePreform
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
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
/// - `Formed`: The type of the entity produced, typically a `HashMap<K, E>`.
/// - `End`: A trait defining the end behavior of the formation process, managing how the hash map is finalized.
///

#[ derive( Debug, Default ) ]
pub struct HashMapDefinition< K, E, Context = (), Formed = HashMap< K, E >, End = ReturnStorage >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : FormingEnd< HashMapDefinitionTypes< K, E, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( K, E, Context, Formed, End ) >,
}

impl< K, E, Context, Formed, End > FormerDefinition
for HashMapDefinition< K, E, Context, Formed, End >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : FormingEnd< HashMapDefinitionTypes< K, E, Context, Formed > >,
{

  type Storage = HashMap< K, E >;
  type Formed = Formed;
  type Context = Context;

  type Types = HashMapDefinitionTypes< K, E, Context, Formed >;
  type End = End;

}

// = definition types

/// Holds the generic parameters for the `HashMapDefinition`.
///
/// This companion struct to `HashMapDefinition` defines the storage type and the context, along with the
/// type that is ultimately formed through the process. It is crucial for maintaining the integrity and
/// consistency of type relations throughout the former lifecycle.
///
/// # Type Parameters
/// - `K`: The key type of the hash map.
/// - `E`: The value type of the hash map.
/// - `Context`: The operational context in which the hash map is formed.
/// - `Formed`: The type produced, typically mirroring the structure of a `HashMap<K, E>`.

#[ derive( Debug, Default ) ]
pub struct HashMapDefinitionTypes< K, E, Context = (), Formed = HashMap< K, E > >
{
  _phantom : core::marker::PhantomData< ( K, E, Context, Formed ) >,
}

impl< K, E, Context, Formed > FormerDefinitionTypes
for HashMapDefinitionTypes< K, E, Context, Formed >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashMap< K, E >;
  type Formed = Formed;
  type Context = Context;
}

// = mutator

impl< K, E, Context, Formed > FormerMutator
for HashMapDefinitionTypes< K, E, Context, Formed >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
}

// = Entity To

impl< K, E, Definition > EntityToFormer< Definition > for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Definition : FormerDefinition
  <
    Storage = HashMap< K, E >,
    Types = HashMapDefinitionTypes
    <
      K,
      E,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = HashMapFormer< K, E, Definition::Context, Definition::Formed, Definition::End >;
}

impl< K, E > crate::EntityToStorage
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashMap< K, E >;
}

impl< K, E, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : crate::FormingEnd< HashMapDefinitionTypes< K, E, Context, Formed > >,
{
  type Definition = HashMapDefinition< K, E, Context, Formed, End >;
  type Types = HashMapDefinitionTypes< K, E, Context, Formed >;
}

impl< K, E, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Types = HashMapDefinitionTypes< K, E, Context, Formed >;
}

// = subformer

/// Provides a streamlined builder interface for constructing hash map-like collections.
///
/// `HashMapFormer` is a type alias that configures the `CollectionFormer` specifically for hash maps,
/// facilitating a more intuitive and flexible way to build and manipulate hash maps within custom data structures.
/// This type alias simplifies the usage of hash maps in builder patterns by encapsulating complex generic parameters
/// and leveraging the `HashMapDefinition` to handle the construction logic. It supports fluent chaining of key-value
/// insertions and can be customized with various end actions to finalize the hash map upon completion.
///
/// The alias helps reduce boilerplate code and enhances readability, making the construction of hash maps in
/// a builder pattern both efficient and expressive.

pub type HashMapFormer< K, E, Context, Formed, End > =
CollectionFormer::< ( K, E ), HashMapDefinition< K, E, Context, Formed, End > >;

// = extension

/// Provides an extension method for hash maps to facilitate the use of the builder pattern.
///
/// This trait extends the `HashMap` type, enabling it to use the `HashMapFormer` interface directly.
/// It allows for fluent, expressive construction and manipulation of hash maps, integrating seamlessly
/// with the builder pattern provided by the `former` framework. It's a convenience trait that simplifies
/// creating configured hash map builders with default settings.
///

pub trait HashMapExt< K, E > : sealed::Sealed
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  /// Initializes a builder pattern for `HashMap` using a default `HashMapFormer`.
  fn former() -> HashMapFormer< K, E, (), HashMap< K, E >, ReturnStorage >;
}

impl< K, E > HashMapExt< K, E > for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashMapFormer< K, E, (), HashMap< K, E >, ReturnStorage >
  {
    HashMapFormer::< K, E, (), HashMap< K, E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  use super::HashMap;
  pub trait Sealed {}
  impl< K, E > Sealed for HashMap< K, E > {}
}
