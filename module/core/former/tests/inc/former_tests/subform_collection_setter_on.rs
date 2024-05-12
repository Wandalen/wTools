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
  // Such parameters switch off generation of front-end collection setter and switch on scalar setter.
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
  pub fn children2( self ) -> former::CollectionFormer::
  <
    Child,
    former::VectorDefinition< Child, Self, Self, ParentSubformCollectionChildrenEnd< Definition >, >
  >
  {
    self._children_subform_collection::< _ >()
  }

}

include!( "./only_test/subform_scalar_children.rs" );
include!( "./only_test/subform_collection_children2.rs" );
