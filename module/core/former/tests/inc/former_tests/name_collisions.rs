#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;

pub mod core {}
pub mod std {}
pub trait CloneAny{}
pub trait Context{}
pub trait Formed{}
pub trait OnEnd{}

#[ allow( dead_code ) ]
type Option = ();
#[ allow( dead_code ) ]
type Some = ();
#[ allow( dead_code ) ]
type None = ();
#[ allow( dead_code ) ]
type Result = ();
#[ allow( dead_code ) ]
type Ok = ();
#[ allow( dead_code ) ]
type Err = ();
#[ allow( dead_code ) ]
type Box = ();
#[ allow( dead_code ) ]
type Default = ();
#[ allow( dead_code ) ]
type HashSet = ();
#[ allow( dead_code ) ]
type HashMap = ();

#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ derive( Debug, PartialEq ) ]
// #[ debug ]
pub struct Struct1
{
  vec_1 : Vec< String >,
  hashmap_1 : collection_tools::HashMap< String, String >,
  hashset_1 : collection_tools::HashSet< String >,
}

//

include!( "./only_test/collections_without_subformer.rs" );
