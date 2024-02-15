#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/graph_logo_v1_trans.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/graph_logo_v1_trans.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/graphs_tools/latest/graphs_tools/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]
#![ deny( unused_imports ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( type_alias_impl_trait ) ]
// #![ feature( trace_macros ) ]

//!
//! Implementation of automata.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#![ allow( unused_imports ) ]
use iter_tools::iter;
use data_type::dt;
use meta_tools::meta;
use strs_tools::string;

use meta_tools::mod_interface;
mod_interface!
{
  /// Abstract layer.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer abs;
  /// Canonical representation.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer canonical;
  /// Algorithms.
  #[ cfg( not( feature = "no_std" ) ) ]
  layer algo;

  protected( crate ) use ::meta_tools::prelude::*;
}

// zzz : implement checks
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
