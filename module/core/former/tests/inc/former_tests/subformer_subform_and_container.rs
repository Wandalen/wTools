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
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  // #[ subform ]
  #[ subform( name = _child ) ]
  #[ container( former::VectorDefinition ) ]
  // #[ setter( false ) ]
  children : Vec< Child >,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
  // Definition::Types : former::FormerDefinitionTypes< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn child( self, name : &str ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_add_subformer
    ::< ChildFormer< _ >, _, >()
    .name( name )
  }

}

// == begin of generated

// == end of generated

include!( "./only_test/subformer_subform.rs" );
include!( "./only_test/subformer_container.rs" );
