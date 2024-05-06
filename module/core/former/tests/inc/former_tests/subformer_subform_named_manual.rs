#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  data : bool,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  #[ subform ]
  // #[ scalar( setter = false ) ]
  children : Vec< Child >,
}

// == begin of custom

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn child( self, name : &str ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_add
    ::< ChildFormer< _ >, _, >()
    .name( name )
  }

  // #[ inline( always ) ]
  // pub fn _child( self ) ->
  // ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  // {
  //   self._children_add
  //   ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
  // }

  #[ inline( always ) ]
  pub fn _child( self ) ->
  < < Vec< Child > as former::Container >::Entry as former::EntityToFormer
    <
      // ChildFormerDefinition< Self, Self, ParentFormerAddChildrenEnd< Definition > >,
      <
        < Vec< Child > as former::Container >::Entry as former::EntityToDefinition< Self, Self, ParentFormerAddChildrenEnd< Definition > >
      >::Definition,
    >
  >::Former
  {
    self._children_add
    ::< < < Vec< Child > as former::Container >::Entry as former::EntityToFormer< _ > >::Former, _, >()
  }

}

// == end of custom

// == begin of generated for Parent in context of attribute subform

// == end of generated for Parent in context of attribute subform

include!( "./only_test/subformer_subform_child.rs" );
