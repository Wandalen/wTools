#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/collection_tools/latest/collection_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// qqq : make subdirectory for each container -- done

// qqq : move out of lib.rs file -- moved to `collections.rs`

/// Module containing all collection macros
#[ cfg( feature = "enabled" ) ]
pub mod collections;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  #[ cfg( feature = "use_alloc" ) ]
  pub use ::hashbrown;

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;

  extern crate alloc;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use alloc::vec::Vec;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use alloc::collections::{ BinaryHeap, BTreeMap, BTreeSet, LinkedList, VecDeque };

  // qqq : what is comnination `use_alloc` + !`no_std`
  #[ cfg( feature = "use_alloc" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::dependency::hashbrown::{ HashMap, HashSet };

  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use std::collections::{ HashMap, HashSet };

}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{

  // qqq : for Anton : uncomment, make it working and cover by tests
  // #[ cfg( feature = "prelude" ) ]
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use crate::
  // {
  //   HashMap as Map,
  //   HashSet as Set,
  //   HashMap,
  //   HashSet,
  //   VecDeque,
  //   BTreeMap,
  //   BTreeSet,
  //   BinaryHeap,
  //   LinkedList,
  // };

  // #[ cfg( feature = "prelude" ) ]
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use crate::
  // {
  //   Vec,
  //   Vec as DynArray,
  // };

}
