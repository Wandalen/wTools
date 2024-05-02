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
  // Such parameters switch off generation of front-end subform setter and switch on scalar setter.
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
  pub fn children2( self, name : &str ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_add
    ::< ChildFormer< _ >, _, >()
    .name( name )
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
fn subform()
{

  let got = Parent::former()
  .children2( "a" ).end()
  .children2( "b" ).end()
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), is_mandatory : false },
    Child { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
