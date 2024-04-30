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
  #[ container( former::VectorDefinition ) ]
  #[ subform( name : child ) ]
  // #[ setter( false ) ]
  children : Vec< Child >,
}

//

// xxx
// include!( "./only_test/subformer_subform.rs" );
include!( "./only_test/subformer_container.rs" );
