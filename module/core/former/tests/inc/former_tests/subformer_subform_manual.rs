#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  is_mandatory : bool,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  // #[ container( former::VectorDefinition ) ]
  // #[ subform ]
  #[ setter( false ) ]
  children : Vec< Child >,
}

// = custom

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn _children_add_subformer_with_closure< Former2, Definition2, Types2 >( self ) ->
  Former2
  where
    Types2 : former::FormerDefinitionTypes
    <
      Storage = ChildFormerStorage,
      Formed = Self,
      Context = Self,
    >,
    Definition2 : former::FormerDefinition
    <
      Types = Types2,
      End = former::FormingEndClosure< Types2 >,
      Storage = ChildFormerStorage,
      Formed = Self,
      Context = Self,
    >,
    Definition2::End : former::FormingEnd< Definition2::Types >,
    Former2 : former::FormerBegin
    <
      Definition2,
    >,
  {
    let on_end = | substorage : ChildFormerStorage, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      if super_former.storage.children.is_none()
      {
        super_former.storage.children = Some( Default::default() );
      }
      if let Some( ref mut children ) = super_former.storage.children
      {
        former::ContainerAdd::add( children, former::StoragePreform::preform( substorage ) );
      }
      super_former
    };
    Former2::former_begin( None, Some( self ), former::FormingEndClosure::new( on_end ) )
  }

  #[ inline( always ) ]
  pub fn child( self, name : &str ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_add_subformer
    ::< ChildFormer< _ >, _, >()
    .name( name )
  }

  // #[ inline( always ) ]
  // pub fn _child( self ) ->
  // ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  // {
  //   self._children_add_subformer
  //   ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
  // }

  #[ inline( always ) ]
  pub fn _child( self ) ->
  < < Vec< Child > as former::ContainerAdd >::Element as former::EntityToFormer
    <
      // ChildFormerDefinition< Self, Self, ParentFormerAddChildrenEnd< Definition > >,
      <
        < Vec< Child > as former::ContainerAdd >::Element as former::EntityToDefinition< Self, Self, ParentFormerAddChildrenEnd< Definition > >
      >::Definition,
    >
  >::Former
  {
    self._children_add_subformer
    ::< < < Vec< Child > as former::ContainerAdd >::Element as former::EntityToFormer< _ > >::Former, _, >()
  }

}

// == begin of generated for Parent in context of attribute subform

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn _children_add_subformer< Former2, Definition2 >( self ) ->
  Former2
  where
    Definition2 : former::FormerDefinition
    <
      End = ParentFormerAddChildrenEnd< Definition >,
      Storage = < Child as former::EntityToStorage >::Storage,
      Formed = Self,
      Context = Self,
    >,
    Definition2::Types : former::FormerDefinitionTypes
    <
      Storage = < Child as former::EntityToStorage >::Storage,
      Formed = Self,
      Context = Self,
    >,
    Former2 : former::FormerBegin< Definition2 >,
  {
    Former2::former_begin( None, Some( self ), ParentFormerAddChildrenEnd::default() )
  }

}

/// Handles the completion of and element of subformer's container.
pub struct ParentFormerAddChildrenEnd< Definition >
{
  _phantom : core::marker::PhantomData< fn( Definition ) >,
}

impl< Definition > Default
for ParentFormerAddChildrenEnd< Definition >
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

impl< Types2, Definition > former::FormingEnd< Types2, >
for ParentFormerAddChildrenEnd< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes
  <
    Storage = < Parent as former::EntityToStorage >::Storage,
  >,
  Types2 : former::FormerDefinitionTypes
  <
    Storage = < < Vec< Child > as former::ContainerAdd >::Element as former::EntityToStorage >::Storage,
    Formed = ParentFormer< Definition >,
    Context = ParentFormer< Definition >,
  >,
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    substorage : Types2::Storage,
    super_former : core::option::Option< Types2::Context >,
  )
  -> Types2::Formed
  {
    let mut super_former = super_former.unwrap();
    if super_former.storage.children.is_none()
    {
      super_former.storage.children = Some( Default::default() );
    }
    if let Some( ref mut fields ) = super_former.storage.children
    {
      former::ContainerAdd::add( fields, former::StoragePreform::preform( substorage ) );
    }
    super_former
  }
}

// == end of generated for Parent in context of attribute subform

include!( "./only_test/subformer_subform.rs" );