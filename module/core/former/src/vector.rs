use super::*;
// use axiomatic::*;

#[ allow( unused ) ]
use collection_tools::Vec;

impl< E > Container for collection_tools::Vec< E >
{
  type Entry = E;
  type Val = E;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e
  }

}

impl< E > ContainerAdd for collection_tools::Vec< E >
{
  // type Entry = E;
  // type Val = E;

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Entry ) -> bool
  {
    self.push( e );
    true
  }

}

impl< E > ContainerAssign for collection_tools::Vec< E >
{
  // type Entry = E;

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

impl< E > ContainerValToEntry< E > for collection_tools::Vec< E >
where
{
  type Entry = E;
  #[ inline( always ) ]
  fn val_to_entry( val : E ) -> Self::Entry
  {
    val
  }
}

/// Trait for containers that behave like a vector, providing an interface for element addition.
///
/// This trait enables the use of custom or standard vector-like containers within the builder pattern,
/// allowing for a unified and flexible approach to constructing collections.
///
pub trait VectorLike< E >
{
  /// Appends an element to the back of a storage.
  fn push( &mut self, element : E );
}

impl< E > VectorLike< E > for Vec< E >
{
  fn push( &mut self, element : E )
  {
    Vec::push( self, element );
  }
}

// = storage

impl< E > Storage
for Vec< E >
{
  type Formed = Vec< E >;
}

impl< E > StoragePreform
for Vec< E >
{
  type Preformed = Vec< E >;
  // fn preform( self ) -> Self::Formed
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

// xxx : split definition and definition types
// xxx : imlement custom ContainerDefinition

#[ derive( Debug, Default ) ]
// pub struct VectorDefinition< E, Context = (), Formed = Vec< E >, End = ReturnStorage >
pub struct VectorDefinition< E, Context, Formed, End >
where
  End : FormingEnd< VectorDefinitionTypes< E, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed, End ) >,
}

impl< E, Context, Formed, End > FormerDefinition
for VectorDefinition< E, Context, Formed, End >
where
  End : FormingEnd< VectorDefinitionTypes< E, Context, Formed > >,
{
  type Storage = Vec< E >;
  type Context = Context;
  type Formed = Formed;

  type Types = VectorDefinitionTypes< E, Context, Formed >;
  type End = End;
}

// = definition type

#[ derive( Debug, Default ) ]
pub struct VectorDefinitionTypes< E, Context = (), Formed = Vec< E > >
{
  _phantom : core::marker::PhantomData< ( E, Context, Formed ) >,
}

impl< E, Context, Formed > FormerDefinitionTypes
for VectorDefinitionTypes< E, Context, Formed >
{
  type Storage = Vec< E >;
  type Context = Context;
  type Formed = Formed;
}

// = mutator

impl< E, Context, Formed > FormerMutator
for VectorDefinitionTypes< E, Context, Formed >
{
}

// = Entity To

// zzz : qqq : implement for hashset / hashmap
// zzz : qqq : cover by tests
// zzz : qqq : rid off bound `Fn( Vec< E >, Option< Definition::Context > ) -> Definition::Formed` for all containers

impl< E, Definition > EntityToFormer< Definition >
for Vec< E >
// where
//   Definition : FormerDefinition< Storage = Vec< E > >,
//   Definition::Types : FormerDefinitionTypes< Storage = Vec< E >, Formed = Definition::Formed, Context = Definition::Context >,
//   Definition::End : crate::FormingEnd< Definition::Types >,
//   < Definition as definition::FormerDefinition >::End : Fn( Vec< E >, Option< Definition::Context > ) -> Definition::Formed, // xxx
where
  Definition : FormerDefinition
  <
    Storage = Vec< E >,
    Types = VectorDefinitionTypes
    <
      E,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = VectorAsSubformer< E, Definition::Context, Definition::Formed, Definition::End >;
}

impl< E > crate::EntityToStorage
for Vec< E >
{
  type Storage = Vec< E >;
}

impl< E, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for Vec< E >
where
  End : crate::FormingEnd< VectorDefinitionTypes< E, Context, Formed > >,
{
  type Definition = VectorDefinition< E, Context, Formed, End >;
  type Types = VectorDefinitionTypes< E, Context, Formed >;
}

impl< E, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for Vec< E >
{
  type Types = VectorDefinitionTypes< E, Context, Formed >;
}

// = subformer

/// A builder for constructing `VectorLike` containers, facilitating a fluent and flexible interface.
///
/// `VectorAsSubformer` leverages the `VectorLike` trait to enable the construction and manipulation
/// of vector-like containers in a builder pattern style, promoting readability and ease of use.

// zzz : update documentation

pub type VectorAsSubformer< E, Context, Formed, End > =
ContainerSubformer::< E, VectorDefinition< E, Context, Formed, End > >;

// = extension

pub trait VecExt< E > : sealed::Sealed
{
  fn former() -> VectorAsSubformer< E, (), Vec< E >, ReturnStorage >;
}

impl< E > VecExt< E > for Vec< E >
{
  fn former() -> VectorAsSubformer< E, (), Vec< E >, ReturnStorage >
  {
    VectorAsSubformer::< E, (), Vec< E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  pub trait Sealed {}
  impl< E > Sealed for super::Vec< E > {}
}
