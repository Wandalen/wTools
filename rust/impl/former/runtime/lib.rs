#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Former - variation of builder pattern. Implementation of its runtime.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Namespace with dependencies.
pub mod dependency
{
}

/// Former of a fector.
mod vector;
/// Former of a hash map.
mod hash_map;
/// Former of a hash set.
mod hash_set;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::exposed::*;
  pub use super::vector::*;
  pub use super::hash_map::*;
  pub use super::hash_set::*;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
