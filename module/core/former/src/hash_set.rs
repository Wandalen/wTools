// //! # HashSetLike Trait and HashSetSubformer Struct
// //!
// //! This part of the crate provides a flexible interface (`HashSetLike`) and a builder pattern implementation (`HashSetSubformer`) for `HashSet`-like containers. It's designed to extend the builder pattern, allowing for fluent and dynamic construction of sets within custom data structures.
//
// use super::*;
// use collection_tools::HashSet;
//
// /// A trait for containers behaving like a `HashSet`, allowing insertion operations.
// ///
// /// Implementing this trait enables the associated formed to be used with `HashSetSubformer`,
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
// impl< K > HashSetLike< K > for HashSet< K >
// where
//   K : core::cmp::Eq + core::hash::Hash,
// {
//   fn insert( &mut self, element : K ) -> Option< K >
//   {
//     HashSet::replace( self, element )
//   }
// }
//
// // = storage
//
// impl< K > Storage
// for HashSet< K >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
// {
//   // type Types = HashSetDefinition< K >;
//   type Formed = HashSet< K >;
// }
//
// impl< K > StoragePerform
// for HashSet< K >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
// {
//   // fn preform( self ) -> < < Self as Storage >::Definition as FormerDefinitionTypes >::Formed
//   fn preform( self ) -> Self::Formed
//   {
//     self
//   }
// }
//
// // = definition
//
// // #[ derive( Debug, Default ) ]
// // pub struct HashSetDefinition< K, Context, End >
// // where
// //   K : ::core::cmp::Eq + ::core::hash::Hash,
// //   End : FormingEnd< Self >,
// // {
// //   _phantom : ::core::marker::PhantomData< ( K, Context, End ) >,
// // }
//
// #[ derive( Debug, Default ) ]
// pub struct HashSetDefinition< K, Context = (), Formed = HashSet< K >, End = ReturnStorage >
// {
//   _phantom : core::marker::PhantomData< ( K, Context, Formed, End ) >,
// }
//
// // impl< K, Context, End > HashSetDefinition< K, Context, End >
// // where
// //   K : ::core::cmp::Eq + ::core::hash::Hash,
// //   End : FormingEnd< Self >,
// // {
// //   pub fn new() -> Self
// //   {
// //     Self { _phantom : ::core::marker::PhantomData }
// //   }
// // }
//
// impl< K, Context, End > FormerDefinitionTypes
// for HashSetDefinition< K, Context, End >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
//   End : FormingEnd< Self >,
// {
//   type Storage = HashSet< K >;
//   type Formed = HashSet< K >;
//   type Context = Context;
// }
//
// impl< K, Context, End > FormerDefinition
// for HashSetDefinition< K, Context, End >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
//   End : FormingEnd< Self >,
// {
//   type Types = HashSetDefinition< K, Context, End >;
//   type End = End;
// }
//
// /// Facilitates building `HashSetLike` containers with a fluent API.
// ///
// /// `HashSetSubformer` leverages the `HashSetLike` trait to enable a concise and expressive way
// /// of populating `HashSet`-like containers. It exemplifies the crate's builder pattern variation for sets.
// ///
// /// # Example Usage
// ///
// /// Using `HashSetSubformer` to populate a `HashSet` within a struct:
// ///
// /// ```rust
// /// # #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
// /// # {
// /// # use test_tools::exposed::*;
// ///
// /// #[ derive( Debug, PartialEq, former::Former ) ]
// /// pub struct StructWithSet
// /// {
// ///   #[ subformer( former::HashSetSubformer ) ]
// ///   set : std::collections::HashSet< &'static str >,
// /// }
// ///
// /// let instance = StructWithSet::former()
// /// .set()
// ///   .insert( "apple" )
// ///   .insert( "banana" )
// ///   .end()
// /// .form();
// ///
// /// assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });
// /// # }
// /// ```
//
// // xxx : update documentation
// // pub type HashSetSubformer< K, Context, End > = ContainerSubformer::< K, HashSetDefinition< K, Context, End > >;
// pub type HashSetSubformer< K, Context, Formed, End > =
// ContainerSubformer::< K, HashSetDefinition< K, Context, Formed, End > >;
//
// // = extension
//
// pub trait HashSetExt< K > : sealed::Sealed
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
// {
//   fn former() -> HashSetSubformer< K, (), HashSet< K >, ReturnStorage >;
// }
//
// impl< K > HashSetExt< K > for HashSet< K >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
// {
//   fn former() -> HashSetSubformer< K, (), HashSet< K >, ReturnStorage >
//   {
//     HashSetSubformer::< K, (), HashSet< K >, ReturnStorage >::new( ReturnStorage::default() )
//   }
// }
//
// mod sealed
// {
//   use super::HashSet;
//   pub trait Sealed {}
//   impl< K > Sealed for HashSet< K > {}
// }
