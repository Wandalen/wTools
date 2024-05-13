#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

/// Parameter description.
#[ allow( explicit_outlives_requirements ) ]
#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ derive( Debug, PartialEq ) ]
pub struct Child< 'child, T >
where
  T : 'child + ?Sized,
{
  name : String,
  data : &'child T,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent< 'child >
{
  #[ subform( name = _child ) ]
  #[ container( name = children2 ) ]
  #[ scalar( name = children3 ) ]
  children : Vec< Child< 'child, str > >,
}

impl< 'child, Definition > ParentFormer< 'child, Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent< 'child > as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn child( self, name : &str ) ->
  ChildAsSubformer< 'child, str, Self, impl ChildAsSubformerEnd< 'child, str, Self > >
  {
    self._children_add
    ::< ChildFormer< '_, _, _ >, _, >()
    .name( name )
  }

}

// == begin of generated

// == end of generated

#[ test ]
fn subform_child()
{

  let got = Parent::former()
  .child( "a" ).data( "aa" ).end()
  .child( "b" ).data( "bb" ).end()
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), data : "aa" },
    Child { name : "b".to_string(), data : "bb" },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}

#[ test ]
fn subform_child_generated()
{

  let got = Parent::former()
  ._child().name( "a" ).data( "aa" ).end()
  ._child().name( "b" ).data( "bb" ).end()
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), data : "aa" },
    Child { name : "b".to_string(), data : "bb" },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}

#[ test ]
fn container()
{

  let got = Parent::former()
  .children2()
    .add( Child::former().name( "a" ).data( "aa" ).form() )
    .add( Child::former().name( "b" ).data( "bb" ).form() )
    .end()
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), data : "aa" },
    Child { name : "b".to_string(), data : "bb" },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}


#[ test ]
fn scalar()
{

  let children = vec!
  [
    Child { name : "a".to_string(), data : "aa" },
    Child { name : "b".to_string(), data : "bb" },
  ];
  let got = Parent::former()
  .children3( children )
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), data : "aa" },
    Child { name : "b".to_string(), data : "bb" },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}

// include!( "./only_test/subformer_subform_child.rs" );
// include!( "./only_test/subformer_container_children2.rs" );
// include!( "./only_test/subformer_scalar_children3.rs" );
