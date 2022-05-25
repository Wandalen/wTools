#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Tools for writing and running tests.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Basics.
pub mod basic;
/// Helpers.
pub mod helper;

/// Dependencies.
pub mod dependencies
{
  pub use ::paste;
  pub use ::trybuild;
  pub use ::anyhow;
  pub use ::rustversion;
  pub use ::meta_tools;
  pub use ::typing_tools;
  pub use ::num_traits;
}

pub use dependencies::*;
pub use ::meta_tools as meta;
pub use ::typing_tools as typing;

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::basic::exposed::*;
  #[ doc( inline ) ]
  pub use super::helper::exposed::*;
  #[ doc( inline ) ]
  pub use ::meta_tools::{ impls, impls1, impls2, impls3, index, tests_impls, tests_index };
  #[ doc( inline ) ]
  pub use ::typing_tools::{ implements };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::basic::prelude::*;
  #[ doc( inline ) ]
  pub use super::helper::prelude::*;
}
