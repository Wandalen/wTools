//! This module provides a builder pattern implementation (`HashSetFormer`) for `HashSet`-like collections. It is designed to extend the builder pattern, allowing for fluent and dynamic construction of sets within custom data structures.
#[ allow( clippy::wildcard_imports ) ]
use crate::*;
use collection_tools::HashSet;

#[ allow( clippy::implicit_hasher ) ]
impl< K > Collection for HashSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  type Entry = K;
  type Val = K;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e
  }

}

#[ allow( clippy::implicit_hasher ) ]
impl< K > CollectionAdd for HashSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  // type Entry = K;
  // type Val = K;

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.insert( e )
  }

}

#[ allow( clippy::implicit_hasher ) ]
impl< K > CollectionAssign for HashSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  // type Entry = K;

  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Entry >
  {
    let initial_len = self.len();
    self.extend( elements );
    self.len() - initial_len
  }
}

#[ allow( clippy::implicit_hasher ) ]
impl< K > CollectionValToEntry< K > for HashSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  type Entry = K;
  #[ inline( always ) ]
  fn val_to_entry( val : K ) -> Self::Entry
  {
    val
  }
}

// /// A trait for collections behaving like a `HashSet`, allowing insertion operations.
// ///
// /// Implementing this trait enables the associated formed to be used with `HashSetFormer`,
// /// facilitating a builder pattern that is both intuitive and concise.
// ///
// /// # Example Implementation
// ///
// /// Implementing `HashSetLike` for `std::collections::HashSet`:
// ///
//
// pub trait HashSetLike< K >
// where
//   K : core::cmp::Eq + core::hash::Hash,
// {
//   /// Inserts a key-value pair into the map.
//   fn insert( &mut self, element : K ) -> Option< K >;
// }
//
// // impl< K > HashSetLike< K > for HashSet< K >
// // where
// //   K : core::cmp::Eq + core::hash::Hash,
// // {
// //   fn insert( &mut self, element : K ) -> Option< K >
// //   {
// //     HashSet::replace( self, element )
// //   }
// // }

// = storage

#[ allow( clippy::implicit_hasher ) ]
impl< K > Storage
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  // type Formed = HashSet< K >;
  type Preformed = HashSet< K >;
}

#[ allow( clippy::implicit_hasher ) ]
impl< K > StoragePreform
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  // type Preformed = HashSet< K >;
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

/// Represents the formation definition for a hash set-like collection within the former framework.
///
/// This structure defines the essential elements required to form a hash set-like collection, detailing
/// the type of elements, the contextual environment during formation, the final formed type, and the
/// behavior at the end of the formation process. It is designed to support the construction and configuration
/// of hash set collections with dynamic characteristics and behaviors.
///
/// # Type Parameters
/// - `K`: The type of elements in the hash set.
/// - `Context`: The optional context provided during the formation process.
/// - `Formed`: The type of the entity produced, typically a `HashSet<K>`.
/// - `End`: A trait defining the end behavior of the formation process, managing how the hash set is finalized.
///

#[ derive( Debug, Default ) ]
pub struct HashSetDefinition< K, Context = (), Formed = HashSet< K >, End = ReturnStorage >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : FormingEnd< HashSetDefinitionTypes< K, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( K, Context, Formed, End ) >,
}

impl< K, Context, Formed, End > FormerDefinition
for HashSetDefinition< K, Context, Formed, End >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : FormingEnd< HashSetDefinitionTypes< K, Context, Formed > >,
{
  type Storage = HashSet< K >;
  type Formed = Formed;
  type Context = Context;

  type Types = HashSetDefinitionTypes< K, Context, Formed >;
  type End = End;
}

// = definition types

/// Holds the generic parameters for the `HashSetDefinition`.
///
/// This struct encapsulates the type relationships and characteristics essential for the formation process
/// of a `HashSet`, including the storage type, the context, and the type ultimately formed. It ensures that
/// these elements are congruent and coherent throughout the lifecycle of the hash set formation.
///

#[ derive( Debug, Default ) ]
pub struct HashSetDefinitionTypes< K, Context = (), Formed = HashSet< K > >
{
  _phantom : core::marker::PhantomData< ( K, Context, Formed ) >,
}

impl< K, Context, Formed > FormerDefinitionTypes
for HashSetDefinitionTypes< K, Context, Formed >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashSet< K >;
  type Formed = Formed;
  type Context = Context;
}

// = mutator

impl< K, Context, Formed > FormerMutator
for HashSetDefinitionTypes< K, Context, Formed >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
}

// = entity to

#[ allow( clippy::implicit_hasher ) ]
impl< K, Definition > EntityToFormer< Definition > for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Definition : FormerDefinition
  <
    Storage = HashSet< K >,
    Types = HashSetDefinitionTypes
    <
      K,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = HashSetFormer< K, Definition::Context, Definition::Formed, Definition::End >;
}

#[ allow( clippy::implicit_hasher ) ]
impl< K > crate::EntityToStorage
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashSet< K >;
}

#[ allow( clippy::implicit_hasher ) ]
impl< K, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : crate::FormingEnd< HashSetDefinitionTypes< K, Context, Formed > >,
{
  type Definition = HashSetDefinition< K, Context, Formed, End >;
  type Types = HashSetDefinitionTypes< K, Context, Formed >;
}

#[ allow( clippy::implicit_hasher ) ]
impl< K, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Types = HashSetDefinitionTypes< K, Context, Formed >;
}

// = subformer

/// Provides a concise alias for `CollectionFormer` configured specifically for `HashSet`-like collections.
///
/// `HashSetFormer` simplifies the creation of `HashSet` collections within builder patterns by leveraging
/// the `CollectionFormer` with predefined settings. This approach minimizes boilerplate code and enhances
/// readability, making it ideal for fluent and expressive construction of set collections within custom data structures.
///

pub type HashSetFormer< K, Context, Formed, End > =
CollectionFormer::< K, HashSetDefinition< K, Context, Formed, End > >;

// = extension

/// Provides an extension method for `HashSet` to facilitate the use of the builder pattern.
///
/// This trait extends `HashSet`, enabling direct use of the `HashSetFormer` interface for fluent and expressive
/// set construction. It simplifies the process of building `HashSet` instances by providing a straightforward
/// way to start the builder pattern with default context and termination behavior.
///

pub trait HashSetExt< K > : sealed::Sealed
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  /// Initializes a builder pattern for `HashSet` using a default `HashSetFormer`.
  fn former() -> HashSetFormer< K, (), HashSet< K >, ReturnStorage >;
}

#[ allow( clippy::implicit_hasher ) ]
impl< K > HashSetExt< K > for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  #[ allow( clippy::default_constructed_unit_structs ) ]
  fn former() -> HashSetFormer< K, (), HashSet< K >, ReturnStorage >
  {
    HashSetFormer::< K, (), HashSet< K >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  use super::HashSet;
  pub trait Sealed {}
  impl< K > Sealed for HashSet< K > {}
}
