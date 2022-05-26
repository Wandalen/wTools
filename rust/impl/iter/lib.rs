#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of general purpose tools to iterate. Currently it simply reexport itertools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Core module.
#[ cfg( feature = "itertools" ) ]
pub mod iter;

/// Namespace with dependencies.
pub mod dependency
{
  #[ cfg( feature = "itertools" ) ]
  pub use ::itertools;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  #[ cfg( feature = "itertools" ) ]
  pub use super::iter::exposed::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "itertools" ) ]
  pub use super::iter::prelude::*;
}
