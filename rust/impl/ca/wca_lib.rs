#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! The tool to make CLI ( commands user interface ). It is able to aggregate external binary applications, as well as functions, which are written in your language.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Commands aggregator library.
pub mod ca
{
  include!( "./lib.rs" );
}

#[ doc( inline ) ]
pub use ca::*;

/// Exposed namespace of the module.
pub mod exposed
{
}
pub use exposed::*;

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  pub use super::ca::prelude::*;
}
