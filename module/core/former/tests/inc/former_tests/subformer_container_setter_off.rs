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
  #[ container( setter = false ) ]
  // #[ scalar( setter = false ) ]
  children : Vec< Child >,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn children( self ) -> &'static str
  {
    r#"
    Scalar setter `children` should not be generated by default if container is used.
    It can only be generated if req
    "#
  }

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

include!( "./only_test/subformer_container_children2.rs" );