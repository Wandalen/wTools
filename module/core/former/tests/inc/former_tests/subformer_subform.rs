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
  #[ subform ]
  children : Vec< Child >,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn child( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_add_subformer::< ChildFormer< _ >, _, >()
    .name( name )
  }

  #[ inline( always ) ]
  pub fn _child( self ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_add_subformer
    ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
  }

}

//

include!( "./only_test/subformer_subform.rs" );
