#![ allow( dead_code ) ]

use super::*;

// xxx : rename
/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  is_mandatory : bool,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  // #[ container( former::VectorDefinition ) ]
  // #[ subform ]
  #[ setter( false ) ]
  children : Vec< Child >,
}

// = begin of generated for Parent in context of attribute subform

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn _children_element_subformer_with_closure< Former2, Definition2, Types2 >( self ) ->
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
  pub fn _children_element_subformer< Former2, Definition2 >( self ) ->
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
  pub fn child( self, name : &str ) ->
  ChildSubformer< Self, impl ChildSubformerEnd< Self > >
  {
    self._children_element_subformer
    ::< ChildFormer< _ >, _, >()
    .name( name )
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

// = end of generated for Parent in context of attribute subform

// = begin of generated for Parent in context of attribute container( former::VectorDefinition ) ]

#[ automatically_derived ]
impl< Definition, > ParentFormer< Definition, >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = ParentFormerStorage< > >,
{

  #[ inline( always ) ]
  pub fn _children_assign< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin< former::VectorDefinition< Child, Self, Self, ParentFormerAssignChildrenEnd, > >,
  {
    Former2::former_begin( None, Some( self ), ParentFormerAssignChildrenEnd )
  }
  #[ doc =
  "Subformer setter for the 'children' field. Method _children_assign unlike method children accept custom container subformer." ]
  #[ inline( always ) ]
  pub fn children( self ) -> former::ContainerSubformer::< Child, former::VectorDefinition< Child, Self, Self, ParentFormerAssignChildrenEnd, > >
  {
    self._children_assign::< former::ContainerSubformer::< Child, former::VectorDefinition< Child, Self, Self, ParentFormerAssignChildrenEnd, > > >()
  }

}

//

#[ doc = r" Return original former after container for `vec_1` is done." ]
#[ allow( non_camel_case_types ) ]
pub struct ParentFormerAssignChildrenEnd;

#[ automatically_derived ]
impl< Definition, > former::FormingEnd
< former::VectorDefinition< Child, ParentFormer< Definition, >, ParentFormer< Definition, >, former::NoEnd >, >
for ParentFormerAssignChildrenEnd
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = ParentFormerStorage< > >,
{
  #[ inline( always ) ]
  fn call(
    &self,
    storage : Vec< Child >,
    super_former : Option< ParentFormer< Definition, > >,
  ) -> ParentFormer< Definition, >
  {
    let mut super_former = super_former.unwrap();
    if let Some( ref mut field ) = super_former.storage.children
    {
      former::ContainerAssign::assign( field, storage );
    }
    else
    {
      super_former.storage.children = Some( storage );
    }
    super_former
  }
}

// = end of generated for Parent in context of attribute container( former::VectorDefinition ) ]

include!( "./only_test/subformer_subform.rs" );
include!( "./only_test/subformer_container.rs" );
