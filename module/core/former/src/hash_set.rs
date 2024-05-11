//! # HashSetLike Trait and HashSetAsSubformer Struct
//!
//! This part of the crate provides a flexible interface (`HashSetLike`) and a builder pattern implementation (`HashSetAsSubformer`) for `HashSet`-like containers. It's designed to extend the builder pattern, allowing for fluent and dynamic construction of sets within custom data structures.

use super::*;
use collection_tools::HashSet;

// impl< K, T > Container for T
// where
//   K : core::cmp::Eq + core::hash::Hash,
//   T : HashSetLike< K >,
// {
//   type Entry = K;
//   type Val = K;
//
//   #[ inline( always ) ]
//   fn entry_to_val( e : Self::Entry ) -> Self::Val
//   {
//     e
//   }
//
// }

impl< K > Container for collection_tools::HashSet< K >
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

impl< K > ContainerAdd for collection_tools::HashSet< K >
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

impl< K > ContainerAssign for collection_tools::HashSet< K >
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

impl< K > ContainerValToEntry< K > for HashSet< K >
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

// /// A trait for containers behaving like a `HashSet`, allowing insertion operations.
// ///
// /// Implementing this trait enables the associated formed to be used with `HashSetAsSubformer`,
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

impl< K > Storage
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  // type Formed = HashSet< K >;
  type Preformed = HashSet< K >;
}

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
      < Definition as definition::FormerDefinition >::Formed, // xxx : ?
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = HashSetAsSubformer< K, Definition::Context, Definition::Formed, Definition::End >;
}

impl< K > crate::EntityToStorage
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashSet< K >;
}

impl< K, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : crate::FormingEnd< HashSetDefinitionTypes< K, Context, Formed > >,
{
  type Definition = HashSetDefinition< K, Context, Formed, End >;
  type Types = HashSetDefinitionTypes< K, Context, Formed >;
}

impl< K, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Types = HashSetDefinitionTypes< K, Context, Formed >;
}

// = subformer

/// Provides a concise alias for `ContainerSubformer` configured specifically for `HashSet`-like containers.
///
/// `HashSetAsSubformer` simplifies the creation of `HashSet` containers within builder patterns by leveraging
/// the `ContainerSubformer` with predefined settings. This approach minimizes boilerplate code and enhances
/// readability, making it ideal for fluent and expressive construction of set containers within custom data structures.
///

pub type HashSetAsSubformer< K, Context, Formed, End > =
ContainerSubformer::< K, HashSetDefinition< K, Context, Formed, End > >;

// = extension

pub trait HashSetExt< K > : sealed::Sealed
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashSetAsSubformer< K, (), HashSet< K >, ReturnStorage >;
}

impl< K > HashSetExt< K > for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashSetAsSubformer< K, (), HashSet< K >, ReturnStorage >
  {
    HashSetAsSubformer::< K, (), HashSet< K >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  use super::HashSet;
  pub trait Sealed {}
  impl< K > Sealed for HashSet< K > {}
}
