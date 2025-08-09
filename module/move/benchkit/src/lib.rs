//! Lightweight benchmarking toolkit focused on practical performance analysis and report generation.
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/benchkit/latest/benchkit/" ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::format_push_string ) ]

#[ cfg( feature = "enabled" ) ]
pub mod measurement;

#[ cfg( feature = "enabled" ) ]
pub mod analysis;

#[ cfg( feature = "enabled" ) ]
pub mod suite;

#[ cfg( feature = "markdown_reports" ) ]
pub mod reporting;

#[ cfg( feature = "data_generators" ) ]
pub mod generators;

#[ cfg( feature = "enabled" ) ]
pub mod scaling;

#[ cfg( feature = "enabled" ) ]
pub mod profiling;

#[ cfg( feature = "markdown_reports" ) ]
pub mod documentation;

#[ cfg( feature = "enabled" ) ]
pub mod comparison;

#[ cfg( feature = "diff_analysis" ) ]
pub mod diff;

#[ cfg( feature = "visualization" ) ]
pub mod plotting;

#[ cfg( feature = "statistical_analysis" ) ]
pub mod statistical;

#[ cfg( feature = "enabled" ) ]
pub mod data_generation;

#[ cfg( feature = "enabled" ) ]
pub mod throughput;

#[ cfg( feature = "enabled" ) ]
pub mod memory_tracking;

#[ cfg( feature = "enabled" ) ]
pub mod parser_analysis;

#[ cfg( feature = "enabled" ) ]
pub mod parser_data_generation;

/// Prelude module for convenient imports
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  pub use crate::measurement::*;
  pub use crate::analysis::*;
  pub use crate::suite::*;
  pub use std::time::{Duration, Instant};

  #[ cfg( feature = "markdown_reports" ) ]
  pub use crate::reporting::*;

  #[ cfg( feature = "data_generators" ) ]
  pub use crate::generators::*;
  
  pub use crate::scaling::*;
  pub use crate::profiling::*;
  pub use crate::comparison::*;
  
  #[ cfg( feature = "markdown_reports" ) ]
  pub use crate::documentation::*;
  
  #[ cfg( feature = "diff_analysis" ) ]
  pub use crate::diff::*;
  
  #[ cfg( feature = "visualization" ) ]
  pub use crate::plotting::*;
  
  #[ cfg( feature = "statistical_analysis" ) ]
  pub use crate::statistical::*;
  
  pub use crate::data_generation::*;
  pub use crate::throughput::*;
  pub use crate::memory_tracking::*;
  pub use crate::parser_analysis::*;
  pub use crate::parser_data_generation::*;
}