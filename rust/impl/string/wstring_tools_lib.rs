#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! String tools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// qqq : for Dmia : features should be used here

/// String tools.
pub mod string;

#[ doc( inline ) ]
pub use string::*;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::string::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  pub use super::string::prelude::*;
}
