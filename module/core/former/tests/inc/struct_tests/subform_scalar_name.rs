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
  #[ subform_scalar( name = child2 ) ]
  child : Child,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  pub fn child()
  {
  }

  #[ inline( always ) ]
  pub fn child3( self ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._child_subform_scalar
    ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
  }

}

// == begin of generated

// == end of generated

#[ test ]
fn subforme_scalar_2()
{

  let got = Parent::former()
  .child2().name( "a" ).data( true ).end()
  .form();

  let exp = Parent { child : Child { name : "a".to_string(), data : true } };
  a_id!( got, exp );

}

#[ test ]
fn subforme_scalar_3()
{

  let got = Parent::former()
  .child3().name( "a" ).data( true ).end()
  .form();

  let exp = Parent { child : Child { name : "a".to_string(), data : true } };
  a_id!( got, exp );

}

// qqq : write tests similar to `subform_all` which apply attributes `scalar`, `subform_entry` and `subform_scalar` on the same field and check all three attribtues don't interfere with each other
