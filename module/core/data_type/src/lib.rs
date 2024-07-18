#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/data_type/latest/data_type/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// zzz : proc macro for standard lib epilogue
// zzz : expose one_cell

/// Wrap dependencies under a namespace.
pub mod dt;

/// Namespace with dependencies.

#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  #[ cfg( feature = "either" ) ]
  pub use ::either;
  // #[ cfg( feature = "type_constructor" ) ]
  // pub use ::type_constructor; // xxx : rid of
  #[ cfg( feature = "dt_interval" ) ]
  pub use ::interval_adapter;
  #[ cfg( feature = "dt_collection" ) ]
  pub use ::collection_tools;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::dt::orphan::*;
}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::dt::exposed::*;

  #[ cfg( feature = "dt_interval" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate::dependency::interval_adapter::exposed::*;

  #[ cfg( feature = "dt_collection" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate::dependency::collection_tools::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::dt::prelude::*;

  // #[ cfg( not( feature = "no_std" ) ) ]
  // #[ cfg( feature = "prelude" ) ]
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use std::collections::
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

  // #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  // #[ cfg( feature = "prelude" ) ]
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use std::vec::
  // {
  //   Vec,
  //   Vec as DynArray,
  // };

  #[ cfg( feature = "dt_interval" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate::dependency::interval_adapter::prelude::*;

  #[ cfg( feature = "dt_collection" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate::dependency::collection_tools::prelude::*;

  // #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ cfg( feature = "dt_prelude" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use core::
  {
    fmt,
  };

}

// zzz : use maybe
// https://github.com/CAD97/pointer-utils/tree/master/crates/slice-dst
// zzz : add once_cell maybe
