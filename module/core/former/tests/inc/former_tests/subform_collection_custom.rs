#![ deny( missing_docs ) ]
#![ allow( dead_code ) ]

use super::*;
use collection_tools::HashSet;

// == define custom collections

// Custom collection that logs additions
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

impl< K > IntoIterator for LoggingSet< K >
where
  K : std::cmp::Eq + std::hash::Hash,
{
  type Item = K;
  type IntoIter = collection_tools::hset::IntoIter< K >;

  fn into_iter( self ) -> Self::IntoIter
  {
    self.set.into_iter()
  }
}

impl<'a, K> IntoIterator for &'a LoggingSet< K >
where
  K : std::cmp::Eq + std::hash::Hash,
{
  type Item = &'a K;
  type IntoIter = collection_tools::hset::Iter< 'a, K >;

  fn into_iter( self ) -> Self::IntoIter
  {
    self.set.iter()
  }
}

impl< K > former::Collection for LoggingSet< K >
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

impl< K > former::CollectionAdd for LoggingSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.set.insert( e )
  }

}

impl< K > former::CollectionAssign for LoggingSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Entry >
  {
    let initial_len = self.set.len();
    self.set.extend( elements );
    self.set.len() - initial_len
  }
}

impl< K > former::CollectionValToEntry< K > for LoggingSet< K >
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

// = storage

impl< K > former::Storage
for LoggingSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Preformed = LoggingSet< K >;
}

impl< K > former::StoragePreform
for LoggingSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
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

impl< K > former::EntityToStorage
for LoggingSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = LoggingSet< K >;
}

impl< K, Context, Formed, End > former::EntityToDefinition< Context, Formed, End >
for LoggingSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : former::FormingEnd< LoggingSetDefinitionTypes< K, Context, Formed > >,
{
  type Definition = LoggingSetDefinition< K, Context, Formed, End >;
  type Types = LoggingSetDefinitionTypes< K, Context, Formed >;
}

impl< K, Context, Formed > former::EntityToDefinitionTypes< Context, Formed >
for LoggingSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Types = LoggingSetDefinitionTypes< K, Context, Formed >;
}

// = subformer

pub type LoggingSetAsSubformer< K, Context, Formed, End > =
former::CollectionFormer::< K, LoggingSetDefinition< K, Context, Formed, End > >;

// == use custom collection

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Parent
{
  #[ subform_collection ]
  children : LoggingSet< i32 >,
}

// == begin of generated

// == end of generated

#[ test ]
fn basic()
{

  // Using the builder pattern provided by Former to manipulate Parent
  let parent = Parent::former()
  .children()
    .add(10)
    .add(20)
    .add(10)
    .end()
  .form();

  println!("Got: {:?}", parent);

}
