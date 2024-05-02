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
  // #[ container( definition = former::VectorDefinition ) ]
  #[ scalar( setter = false ) ]
  children : Vec< Child >,
}

// == begin of generated for Parent in context of attribute container( former::VectorDefinition ) ]

#[ automatically_derived ]
impl< Definition, > ParentFormer< Definition, >
where
  Definition : former::FormerDefinition< Storage = ParentFormerStorage< > >,
{

  #[ inline( always ) ]
  pub fn _children_assign< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin< former::VectorDefinition< Child, Self, Self, ParentFormerAssignChildrenEnd< Definition >, > >,
  {
    Former2::former_begin( None, Some( self ), ParentFormerAssignChildrenEnd::< Definition >::default() )
  }

  #[ inline( always ) ]
  pub fn children( self ) -> former::ContainerSubformer::
  <
    Child,
    former::VectorDefinition< Child, Self, Self, ParentFormerAssignChildrenEnd< Definition >, >
  >
  {
    self._children_assign::< former::ContainerSubformer::< Child, former::VectorDefinition< Child, Self, Self, ParentFormerAssignChildrenEnd< Definition >, > > >()
  }

}

//

#[ doc = r"Callback to return original former after forming of container for `vec_1` is done. Callback replace content of container assigning new content from subformer's storage." ]
pub struct ParentFormerAssignChildrenEnd< Definition >
{
  _phantom : core::marker::PhantomData< ( Definition, ) >,
}

impl< Definition > Default for ParentFormerAssignChildrenEnd< Definition >
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

#[ automatically_derived ]
impl< Definition, > former::FormingEnd
<
  <
    Vec< Child > as former::EntityToDefinition< ParentFormer< Definition, >, ParentFormer< Definition, >, former::NoEnd >
  >::Definition
>
for ParentFormerAssignChildrenEnd< Definition >
where
  Definition : former::FormerDefinition< Storage = ParentFormerStorage< > >,
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : Vec< Child >,
    super_former : Option< ParentFormer< Definition, > >,
  )
  -> ParentFormer< Definition, >
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

// == end of generated for Parent in context of attribute container( former::VectorDefinition ) ]

include!( "./only_test/subformer_container.rs" );
