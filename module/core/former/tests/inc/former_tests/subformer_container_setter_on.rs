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
  // Such parameters switch off generation of front-end container setter and switch on scalar setter.
  // Without explicit scalar_setter( true ) scalar setter is not generated.
  #[ subform( setter = false ) ]
  #[ scalar( setter = true ) ]
  children : Vec< Child >,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn children2( self ) -> former::ContainerSubformer::
  <
    Child,
    former::VectorDefinition< Child, Self, Self, ParentFormerAssignChildrenEnd< Definition >, >
  >
  {
    self._children_assign::< _ >()
  }

}

include!( "./only_test/subformer_scalar_children.rs" );
include!( "./only_test/subformer_container_children2.rs" );
