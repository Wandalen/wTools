#![ allow( dead_code ) ]

use super::*;
use collection_tools::HashSet;
use std::fmt;

// == define custom containers

// Custom container that logs additions
#[ derive( Debug, PartialEq ) ]
pub struct LoggingSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  set : HashSet< K >,
}

impl< K > Default for LoggingSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{

  #[ inline( always ) ]
  fn default() -> Self
  {
    Self
    {
      set : Default::default()
    }
  }

}

impl< K > former::Container for LoggingSet< K >
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

impl< K > former::ContainerAdd for LoggingSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  // type Entry = K;
  // type Val = K;

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.set.insert( e )
  }

}

impl< K > former::ContainerAssign for LoggingSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  // type Entry = K;

  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Entry >
  {
    let initial_len = self.set.len();
    self.set.extend( elements );
    self.set.len() - initial_len
  }
}

// xxx : uncomment
// impl< K > former::ValToEntry< LoggingSet< K > > for K
// where
//   K : core::cmp::Eq + core::hash::Hash,
// {
//   type Entry = K;
//   #[ inline( always ) ]
//   fn val_to_entry( self ) -> Self::Entry
//   {
//     self
//   }
// }


// xxx : test with HashSetLike
//
// impl< K > HashSetLike< K > for LoggingSet< K >
// where
//   K : core::cmp::Eq + core::hash::Hash,
// {
//   fn insert( &mut self, element : K ) -> Option< K >
//   {
//     HashSet::replace( self, element )
//   }
// }

// = storage

impl< K > former::Storage
for LoggingSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Formed = LoggingSet< K >;
  // xxx : rid off Formed maybe?
}

impl< K > former::StoragePreform
for LoggingSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Preformed = LoggingSet< K >;
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition types

#[ derive( Debug, Default ) ]
pub struct LoggingSetDefinitionTypes< K, Context = (), Formed = LoggingSet< K > >
{
  _phantom : core::marker::PhantomData< ( K, Context, Formed ) >,
}

impl< K, Context, Formed > former::FormerDefinitionTypes
for LoggingSetDefinitionTypes< K, Context, Formed >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = LoggingSet< K >;
  type Formed = Formed;
  type Context = Context;
}

// = definition

#[ derive( Debug, Default ) ]
pub struct LoggingSetDefinition< K, Context = (), Formed = LoggingSet< K >, End = former::ReturnStorage >
{
  _phantom : core::marker::PhantomData< ( K, Context, Formed, End ) >,
}

impl< K, Context, Formed, End > former::FormerDefinition
for LoggingSetDefinition< K, Context, Formed, End >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : former::FormingEnd< LoggingSetDefinitionTypes< K, Context, Formed > >,
{
  type Storage = LoggingSet< K >;
  type Formed = Formed;
  type Context = Context;

  type Types = LoggingSetDefinitionTypes< K, Context, Formed >;
  type End = End;
}

// = mutator

impl< K, Context, Formed > former::FormerMutator
for LoggingSetDefinitionTypes< K, Context, Formed >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
}

// = Entity To

impl< K, Definition > former::EntityToFormer< Definition > for LoggingSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Definition : former::FormerDefinition
  <
    Storage = LoggingSet< K >,
    Types = LoggingSetDefinitionTypes
    <
      K,
      < Definition as former::FormerDefinition >::Context,
      < Definition as former::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : former::FormingEnd< Definition::Types >,
{
  type Former = LoggingSetAsSubformer< K, Definition::Context, Definition::Formed, Definition::End >;
}

// impl< K > former::EntityToStorage
// for LoggingSet< K >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
// {
//   type Storage = LoggingSet< K >;
// }
//
// impl< K, Context, Formed, End > former::EntityToDefinition< Context, Formed, End >
// for LoggingSet< K >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
//   End : former::FormingEnd< HashSetDefinition< K, Context, Formed, NoEnd > >,
// {
//   type Definition = HashSetDefinition< K, Context, Formed, End >;
// }

// impl< K, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
// for HashSet< K >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
// {
//   type Types = HashSetDefinitionTypes< K, Context, Formed >;
// }

// = subformer

pub type LoggingSetAsSubformer< K, Context, Formed, End > =
former::ContainerSubformer::< K, LoggingSetDefinition< K, Context, Formed, End > >;

// == use custom container

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  // #[ container ]
  children : LoggingSet< i32 >,
}

// == begin of generated

// == end of generated

// #[ test ]
// fn basic()
// {
//
//   // Using the builder pattern provided by Former to manipulate Parent
//   let mut parent = Parent::former()
//   .children()
//     .add(10)
//     .add(20)
//     .add(10)
//     .end()
//   .form();
//
//   println!("Got: {:?}", parent);
//
// }

// xxx2 : get completed

