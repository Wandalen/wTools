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
// #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  // Such parameters switch off generation of front-end container setter and switch on scalar setter.
  // Without explicit scalar_setter( true ) scalar setter is not generated.
  #[ subform( setter = false ) ]
  #[ scalar_setter( true ) ]
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

#[ test ]
fn scalar()
{

  let children = vec!
  [
    Child { name : "a".to_string(), is_mandatory : false },
    Child { name : "b".to_string(), is_mandatory : false },
  ];
  let got = Parent::former()
  .children( children )
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), is_mandatory : false },
    Child { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}

#[ test ]
fn container()
{

  let got = Parent::former()
  .children2()
    .add( Child::former().name( "a" ).form() )
    .add( Child::former().name( "b" ).form() )
    .end()
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), is_mandatory : false },
    Child { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
