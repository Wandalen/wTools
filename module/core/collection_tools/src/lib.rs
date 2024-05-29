#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/collection_tools/latest/collection_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
extern crate alloc;

/// Module containing all collection macros
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod collections;
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
pub use collections::*;

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

  // #[ cfg( feature = "reexports" ) ]
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate::
  {
    bmap::BTreeMap,
    bset::BTreeSet,
    heap::BinaryHeap,
    hmap::HashMap,
    hset::HashSet,
    list::LinkedList,
    vec::Vec,
    vecd::VecDeque,
  };

  // #[ cfg( feature = "reexports" ) ]
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use
  {
    HashMap as Map,
    HashSet as Set,
    Vec as DynArray,
  };

  // qqq : cover by tests presence of all containers immidiately in collection_tools::* and in collection_tools::exposed::*

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
}
