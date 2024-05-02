#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

// xxx : make it working

/// Parameter description.
// #[ derive( Debug, PartialEq, the_module::Former ) ]
#[ derive( Debug, PartialEq ) ]
pub struct Child< 'child, T >
{
  name : String,
  is_mandatory : &'child T,
}

// /// Parent required for the template.
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// // #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// // #[ derive( Debug, Default, PartialEq ) ]
// pub struct Parent
// {
//   // #[ subform ]
//   #[ subform( name = _child ) ]
//   #[ container( definition = former::VectorDefinition ) ]
//   // #[ scalar_setter( false ) ]
//   children : Vec< Child >,
// }
//
// impl< Definition > ParentFormer< Definition >
// where
//   Definition : former::FormerDefinition,
//   Definition::Types : former::FormerDefinitionTypes< Storage = < Parent as former::EntityToStorage >::Storage >,
// {
//
//   #[ inline( always ) ]
//   pub fn child( self, name : &str ) ->
//   ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
//   {
//     self._children_add
//     ::< ChildFormer< _ >, _, >()
//     .name( name )
//   }
//
// }

// == begin of generated

// == end of generated

// include!( "./only_test/subformer_subform_child.rs" );
// include!( "./only_test/subformer_container.rs" );
