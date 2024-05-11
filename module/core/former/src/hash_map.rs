//! This module provides a comprehensive approach to applying the builder pattern to `HashMap` containers.
//!
//! By leveraging traits such as `Container`, `ContainerAdd`, `ContainerAssign`, and `ContainerValToEntry`,
//! this module abstracts the operations on hashmap-like data structures, making them more flexible and easier to integrate as
//! as subformer, enabling fluid and intuitive manipulation of hashmaps via builder patterns.
//!

use super::*;

use collection_tools::HashMap;

impl< K, V > Container for collection_tools::HashMap< K, V >
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

impl< K, V > ContainerAdd for collection_tools::HashMap< K, V >
where
  K : core::cmp::Eq + core::hash::Hash,
{

  #[ inline( always ) ]
  fn add( &mut self, ( k, v ) : Self::Entry ) -> bool
  {
    self.insert( k, v ).map_or_else( || true, | _ | false )
  }

}

impl< K, V > ContainerAssign for collection_tools::HashMap< K, V >
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
  type Former = HashMapAsSubformer< K, E, Definition::Context, Definition::Formed, Definition::End >;
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

/// Provides a streamlined builder interface for constructing hash map-like containers.
///
/// `HashMapAsSubformer` is a type alias that configures the `ContainerSubformer` specifically for hash maps,
/// facilitating a more intuitive and flexible way to build and manipulate hash maps within custom data structures.
/// This type alias simplifies the usage of hash maps in builder patterns by encapsulating complex generic parameters
/// and leveraging the `HashMapDefinition` to handle the construction logic. It supports fluent chaining of key-value
/// insertions and can be customized with various end actions to finalize the hash map upon completion.
///
/// The alias helps reduce boilerplate code and enhances readability, making the construction of hash maps in
/// a builder pattern both efficient and expressive.

pub type HashMapAsSubformer< K, E, Context, Formed, End > =
ContainerSubformer::< ( K, E ), HashMapDefinition< K, E, Context, Formed, End > >;

// = extension

pub trait HashMapExt< K, E > : sealed::Sealed
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashMapAsSubformer< K, E, (), HashMap< K, E >, ReturnStorage >;
}

impl< K, E > HashMapExt< K, E > for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashMapAsSubformer< K, E, (), HashMap< K, E >, ReturnStorage >
  {
    HashMapAsSubformer::< K, E, (), HashMap< K, E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  use super::HashMap;
  pub trait Sealed {}
  impl< K, E > Sealed for HashMap< K, E > {}
}
