#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Diagnostics tools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Compile-time asserting.
pub mod diagnostics;
/// Compile-time asserting of memory layout.
pub mod layout;

/// Dependencies.
pub mod dependencies
{
  #[ cfg( feature = "runtime_assertions" ) ]
  pub use ::pretty_assertions;
}

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  #[ doc( inline ) ]
  pub use super::diagnostics::orphan::*;
  #[ doc( inline ) ]
  pub use super::layout::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  #[ doc( inline ) ]
  pub use super::diagnostics::exposed::*;
  #[ doc( inline ) ]
  pub use super::layout::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::diagnostics::prelude::*;
  #[ doc( inline ) ]
  pub use super::layout::prelude::*;
}
