#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Implementation of automata.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Abstract layer.
pub mod abs;
/// Canonical representation.
pub mod canonical;
/// Matrix representation.
pub mod matrix;
/// Algorithms.
pub mod algo;

/// Namespace with dependencies.
pub mod dependency
{
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::abs::exposed::*;
  pub use super::prelude::*;
  pub use super::algo::exposed::*;
  pub use super::canonical::exposed::*;
  pub use super::matrix::exposed::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::abs::prelude::*;
  pub use super::algo::prelude::*;
  pub use super::canonical::prelude::*;
  pub use super::matrix::prelude::*;
}

// xxx : implement checks
//
// - graph is connected
// - graph is complete
// - graph is isomorphic with another graph
// - graph get regularity degree
// - graph is bipartite
// - graph decomposition on cycles
// - graph decomposition on connected components
//
// - node get open neighbourhood?
// - node get closed neighbourhood?
// - node get degree ( nodes )
// - node get size ( edges )
//