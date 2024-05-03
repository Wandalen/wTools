#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

// xxx : make it working

/// Parameter description.
#[ allow( explicit_outlives_requirements ) ]
#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ derive( Debug, PartialEq ) ]
pub struct Child< 'child, T >
where
  T : 'child + ?Sized,
{
  name : String,
  is_mandatory : &'child T,
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

// include!( "./only_test/subformer_subform_child.rs" );
// include!( "./only_test/subformer_container.rs" );
