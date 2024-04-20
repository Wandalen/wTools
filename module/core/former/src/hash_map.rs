use super::*;

use collection_tools::HashMap;

/// A trait for types that behave like hash maps, supporting insertion and custom forming behaviors.
///
/// This trait allows for generic operations on hash map-like data structures, enabling the insertion
/// of key-value pairs and the creation of formers for more complex construction patterns.
///
/// # Type Parameters
/// - `K`: The type of keys stored in the hash map. Must implement `Eq` and `Hash`.
/// - `E`: The type of elements (values) stored in the hash map.
pub trait HashMapLike< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Self : Sized + Default,
{

  /// Inserts a key-value pair into the map.
  fn insert( &mut self, k : K, e : E ) -> Option< E >;

  // /// Return former.
  // #[ inline( always ) ]
  // fn former< Definition : FormerDefinitionTypes >( self )
  // ->
  // HashMapSubformer< K, E, Definition, (), impl FormingEnd< Self, Self > >
  // {
  //   HashMapSubformer::begin( Some( self ), None, ReturnFormed )
  // }
  // xxx : uncomment and cover by tests

}

impl< K, E > HashMapLike< K, E > for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Self : Sized + Default,
{

  #[ inline( always ) ]
  fn insert( &mut self, k : K, e : E ) -> Option< E >
  {
    HashMap::insert( self, k, e )
  }

}

// = storage

impl< K, E > Storage
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  // type Types = HashMapDefinition< K, E >;
  type Formed = HashMap< K, E >;
}

impl< K, E > StoragePreform
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Preformed = HashMap< K, E >;
  // fn preform( self ) -> < < Self as Storage >::Definition as FormerDefinitionTypes >::Formed
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

#[ derive( Debug, Default ) ]
pub struct HashMapDefinition< K, E, Context = (), Formed = HashMap< K, E >, End = ReturnStorage >
{
  _phantom : core::marker::PhantomData< ( K, E, Context, Formed, End ) >,
}

impl< K, E, Context, Formed > FormerDefinitionTypes
for HashMapDefinition< K, E, Context, Formed, NoEnd >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashMap< K, E >;
  type Formed = Formed;
  type Context = Context;
}

impl< K, E, Context, Formed, End > FormerDefinition
for HashMapDefinition< K, E, Context, Formed, End >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : FormingEnd< HashMapDefinition< K, E, Context, Formed, NoEnd > >,
{
  type Types = HashMapDefinition< K, E, Context, Formed, NoEnd >;
  type End = End;
}

// #[ derive( Debug, Default ) ]
// pub struct HashMapDefinition< K, E, Context = (), Formed = HashMap< K, E >, End = ReturnStorage >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
// {
//   _phantom : ::core::marker::PhantomData< ( K, E, Context, End ) >,
// }
//
// impl< K, E, Context, End > FormerDefinitionTypes
// for HashMapDefinition< K, E, Context, End >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
//   End : FormingEnd< Self >,
// {
//   type Storage = HashMap< K, E >;
//   type Formed = HashMap< K, E >;
//   type Context = Context;
//   type End = End;
// }
//
// impl< K, E, Context, End > FormerDefinitionTypes
// for HashMapDefinition< K, E, Context, End >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
//   End : FormingEnd< Self >,
// {
//   type Storage = HashMap< K, E >;
//   type Formed = HashMap< K, E >;
//   type Context = Context;
//   type End = End;
// }

// = subformer

/// A builder for constructing hash map-like structures with a fluent interface.
///
/// `HashMapSubformer` leverages the `HashMapLike` trait to enable a flexible and customizable
/// way to build hash map-like structures. It supports the chaining of insert operations and
/// allows for the definition of custom end actions to finalize the building process.
///
/// # Type Parameters
/// - `K`: Key type, must implement `Eq` and `Hash`.
/// - `E`: Element (value) type.
/// - `Formed`: The hash map-like formed being built.
/// - `Context`: Type of the optional context used during the building process.
/// - `End`: End-of-forming action to be executed upon completion.
///
/// # Examples
/// ```
/// # #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
/// # {
/// # use test_tools::exposed::*;
///
/// #[ derive( Debug, PartialEq, former::Former ) ]
/// pub struct StructWithMap
/// {
///   #[ subformer( former::HashMapSubformer ) ]
///   map : std::collections::HashMap< &'static str, &'static str >,
/// }
///
/// let struct1 = StructWithMap::former()
/// .map()
///   .insert( "a", "b" )
///   .insert( "c", "d" )
///   .end()
/// .form()
/// ;
/// assert_eq!( struct1, StructWithMap { map : hmap!{ "a" => "b", "c" => "d" } } );
///
/// # }
/// ```

// pub type HashMapSubformer< K, E, Context, End > = ContainerSubformer::< ( K, E ), HashMapDefinition< K, E, Context, End > >;

// xxx : update documentation
// pub type HashMapSubformer< K, E, Context, End > = ContainerSubformer::< K, HashMapDefinition< K, E, Context, End > >;
pub type HashMapSubformer< K, E, Context, Formed, End > =
ContainerSubformer::< ( K, E ), HashMapDefinition< K, E, Context, Formed, End > >;

// = extension

pub trait HashMapExt< K, E > : sealed::Sealed
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashMapSubformer< K, E, (), HashMap< K, E >, ReturnStorage >;
}

impl< K, E > HashMapExt< K, E > for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashMapSubformer< K, E, (), HashMap< K, E >, ReturnStorage >
  {
    HashMapSubformer::< K, E, (), HashMap< K, E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  use super::HashMap;
  pub trait Sealed {}
  impl< K, E > Sealed for HashMap< K, E > {}
}
