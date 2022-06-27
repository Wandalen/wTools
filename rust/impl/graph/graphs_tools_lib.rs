#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/graph_logo_v1_trans.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/graph_logo_v1_trans.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/graphs_tools/latest/graphs_tools/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( type_alias_impl_trait ) ]
// #![ feature( trace_macros ) ]

//!
//! Implementation of automata.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// // xxx : move
//
// pub struct IteratorAdapter< Item >
// {
//   iterator : Box< dyn Iterator< Item = Item > >
// }
//
// impl< Item > core::iter::Iterator
// for IteratorAdapter< Item >
// {
//   type Item = Item;
//   fn next( &mut self ) -> Option< Self::Item >
//   {
//     self.iterator.next()
//   }
// }
//
// trait IntoIteratorAdapter
// where
//   Self : Iterator + 'static,
// {
//   fn into_iter_adapter( self ) -> IteratorAdapter< < Self as Iterator >::Item >
//   where
//     Self : Sized,
//   {
//     let iterator : Box< dyn Iterator< Item = < Self as Iterator >::Item > > = Box::new( self );
//     IteratorAdapter::< < Self as Iterator >::Item > { iterator }
//   }
// }
//
// impl< T > IntoIteratorAdapter for T
// where
//   T : Iterator + 'static,
// {
// }

/// Abstract layer.
#[ cfg( feature = "use_std" ) ]
pub mod abs;
/// Canonical representation.
#[ cfg( feature = "use_std" ) ]
pub mod canonical;
/// Algorithms.
#[ cfg( feature = "use_std" ) ]
pub mod algo;
// /// Matrix representation.
// pub mod matrix;

/// Namespace with dependencies.
pub mod dependency
{
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::abs::exposed::*;
  pub use super::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::algo::exposed::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::canonical::exposed::*;
  // pub use super::matrix::exposed::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "use_std" ) ]
  pub use super::abs::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::algo::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::canonical::prelude::*;
  // pub use super::matrix::prelude::*;
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
