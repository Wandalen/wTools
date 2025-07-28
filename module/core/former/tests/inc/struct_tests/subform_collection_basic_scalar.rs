#![deny(missing_docs)]

#[allow(unused_imports)]
use super::*;

use collection_tools::HashMap;
use collection_tools::HashSet;

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, PartialEq, the_module::Former)]
#[derive(Debug, PartialEq)]
// #[ derive( Debug, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Struct1 {
  vec_1: Vec<String>,
  hashmap_1: HashMap<String, String>,
  hashset_1: HashSet<String>,
}

// = begin_coercing of generated

// == end of generated

include!("./only_test/collections_without_subformer.rs");
