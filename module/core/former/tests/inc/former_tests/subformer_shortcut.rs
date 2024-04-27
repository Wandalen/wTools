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

  // #[ inline( always ) ]
  // pub fn _children_former< Former2, Definition2 >( self ) -> Former2
  // where
  //   Definition2 : former::FormerDefinition
  //   <
  //     End = ParentFormerAddChildrenEnd< Definition >,
  //     Storage = < Child as former::EntityToStorage >::Storage,
  //     Formed = Self,
  //     Context = Self,
  //   >,
  //   Definition2::Types : former::FormerDefinitionTypes
  //   <
  //     Storage = < Child as former::EntityToStorage >::Storage,
  //     Formed = Self,
  //     Context = Self,
  //   >,
  //   Former2 : former::FormerBegin< Definition2 >,
  // {
  //   Former2::former_begin( None, Some( self ), ParentFormerAddChildrenEnd::default() )
  // }

  #[ inline( always ) ]
  pub fn child( self, name : &str ) -> ChildSubformer< Self, impl ChildSubformerEnd< Self > >
  {
    self._children_former::< ChildFormer< _ >, _, >()
    .name( name )
  }

}

//

include!( "./only_test/subformer_shortcut.rs" );
