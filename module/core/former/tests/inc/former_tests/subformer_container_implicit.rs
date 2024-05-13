#![ deny( missing_docs ) ]
#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  data : bool,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  // #[ container( definition = former::VectorDefinition ) ]
  #[ container ]
  children : Vec< Child >,
}

// == begin of generated

// == end of generated

include!( "./only_test/subformer_container.rs" );
