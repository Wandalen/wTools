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
mod collection;
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
pub use collection::*;

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
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;

}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  #[ cfg( feature = "collection_constructors" ) ]
  pub use crate::
  {
    vec as dlist,
    deque,
    llist,
    hset,
    hmap,
    bmap,
    bset,
  };

  #[ doc( inline ) ]
  #[ cfg( feature = "collection_into_constructors" ) ]
  pub use crate::
  {
    into_vec,
    into_vec as into_dlist,
    into_vecd,
    into_llist,
    into_hset,
    into_hmap,
    into_bmap,
    into_bset,
  };

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
    llist::LinkedList,
    vec::Vec,
    deque::VecDeque,
  };

  // #[ cfg( feature = "reexports" ) ]
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use
  {
    LinkedList as Llist,
    Vec as Dlist,
    VecDeque as Deque,
    HashMap as Map,
    HashMap as Hmap,
    HashSet as Set,
    HashSet as Hset,
    BTreeMap as Bmap,
    BTreeSet as Bset,
  };

  // qqq : cover by tests presence of all containers immidiately in collection_tools::* and in collection_tools::exposed::*

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
