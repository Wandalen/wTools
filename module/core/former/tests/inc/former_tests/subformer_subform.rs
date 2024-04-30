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
  // xxx : add another test to make sure attributes container and subform are compatible
  // #[ container( former::VectorDefinition ) ]
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
    self._children_element_subformer::< ChildFormer< _ >, _, >()
    .name( name )
  }

}

//

include!( "./only_test/subformer_subform.rs" );
