#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/data_type/latest/data_type/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of primal data types.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// zzz : proc macro for standard lib epilogue
// zzz : expose one_cell

/// Collection of primal data types.
pub mod dt;

/// Dependencies.
pub mod dependency
{
  #[ cfg( feature = "either" ) ]
  pub use ::either;
  #[ cfg( feature = "type_constructor" ) ]
  pub use ::type_constructor;
  #[ cfg( feature = "interval" ) ]
  pub use ::winterval;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::dt::orphan::*;
}

pub use protected::*;

/// Shared with parent namespace of the module
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::dt::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::dt::prelude::*;

  #[ cfg( feature = "use_std" ) ]
  #[ cfg( feature = "prelude" ) ]
  #[ doc( inline ) ]
  pub use std::collections::
  {
    HashMap as Map,
    HashSet as Set,
    HashMap,
    HashSet,
    VecDeque,
    BTreeMap,
    BTreeSet,
    BinaryHeap,
    LinkedList,
  };

  #[ cfg( any( feature = "use_std", feature = "use_alloc" ) ) ]
  #[ cfg( feature = "prelude" ) ]
  #[ doc( inline ) ]
  pub use std::vec::
  {
    Vec,
    Vec as DynArray,
  };

  // #[ cfg( any( feature = "use_std", feature = "use_alloc" ) ) ]
  #[ cfg( feature = "prelude" ) ]
  #[ doc( inline ) ]
  pub use core::
  {
    fmt,
  };

}

// zzz : use maybe
// https://github.com/CAD97/pointer-utils/tree/master/crates/slice-dst
// zzz : add once_cell maybe
