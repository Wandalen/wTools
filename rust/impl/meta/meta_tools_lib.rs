#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of general purpose meta tools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Dependencies.
pub mod dependencies
{
  #[ cfg( any( feature = "container_aliases", feature = "literally" ) ) ]
  pub use ::literally;
  #[ cfg( feature = "for_each" ) ]
  pub use ::for_each;
  #[ cfg( feature = "impls_index" ) ]
  pub use ::impls_index;
}

/// Collection of general purpose meta tools.
pub mod meta;

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::meta::orphan::*;
}

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
  pub use super::meta::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::meta::prelude::*;
}
