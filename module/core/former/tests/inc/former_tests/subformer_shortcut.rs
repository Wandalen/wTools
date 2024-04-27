#![ allow( dead_code ) ]

use super::*;

/// Child
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  is_mandatory : bool,
}

/// Parent
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Parent
{
  // xxx : is definition as argument fine?
  #[ container( former::VectorDefinition ) ]
  #[ subform ]
  children : Vec< Child >,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn _children_former_with_closure< Former2, Definition2, Types2 >( self ) ->
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
  pub fn _children_former< Former2, Definition2 >( self ) ->
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

  #[ inline( always ) ]
  pub fn descriptor( self, name : &str ) ->
  ChildSubformer< Self, impl ChildSubformerEnd< Self > >
  {
    self._children_former
    ::< ChildFormer< _ >, _, >()
    .name( name )
  }

}

//

include!( "./only_test/subformer_shortcut.rs" );
