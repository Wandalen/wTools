#![ allow( dead_code ) ]

use super::*;

/// Child
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  data : bool,
}

/// Parent

#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  // Such parameters switch off generation of front-end subform setter and switch on scalar setter.
  // Without explicit scalar_setter( true ) scalar setter is not generated.
  #[ subform_entry( setter = false ) ]
  #[ scalar( setter = true ) ]
  children : Vec< Child >,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn children2( self, name : &str ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_subform_entry
    ::< ChildFormer< _ >, _, >()
    .name( name )
  }

}

include!( "./only_test/scalar_children.rs" );
include!( "./only_test/subform_entry_children2.rs" );
