//! Lightweight benchmarking toolkit focused on practical performance analysis and report generation.
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/benchkit/latest/benchkit/" ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::format_push_string ) ]
#![ allow( clippy::must_use_candidate ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::doc_markdown ) ]
#![ allow( clippy::missing_errors_doc ) ]
#![ allow( clippy::implicit_hasher ) ]
#![ allow( clippy::cast_possible_truncation ) ]
#![ allow( clippy::needless_pass_by_value ) ]
#![ allow( clippy::redundant_closure_for_method_calls ) ]
#![ allow( clippy::cast_sign_loss ) ]
#![ allow( clippy::used_underscore_binding ) ]
#![ allow( clippy::missing_panics_doc ) ]
#![ allow( clippy::return_self_not_must_use ) ]
#![ allow( clippy::useless_format ) ]
#![ allow( clippy::if_not_else ) ]
#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::cloned_instead_of_copied ) ]
#![ allow( clippy::unnecessary_debug_formatting ) ]
#![ allow( clippy::needless_borrows_for_generic_args ) ]
#![ allow( clippy::inherent_to_string ) ]
#![ allow( clippy::unnecessary_map_or ) ]
#![ allow( clippy::unused_self ) ]
#![ allow( clippy::too_many_lines ) ]
#![ allow( clippy::needless_borrow ) ]
#![ allow( clippy::single_char_add_str ) ]
#![ allow( clippy::match_same_arms ) ]
#![ allow( clippy::empty_line_after_outer_attr ) ]
#![ allow( clippy::similar_names ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::format_push_string ) ]
#![ allow( unused_imports ) ]

#[ cfg( feature = "enabled" ) ]
fn check_directory_recommendations()
{
  #[ cfg( debug_assertions ) ]
  if let Ok( current_dir ) = std::env::current_dir()
  {
    if current_dir.file_name()
      .and_then( | n | n.to_str() )
      .is_some_and( | s | s == "benches" )
    {
      eprintln!( "💡 benchkit: Running in standard benches/ directory ✅" );
      eprintln!( "   Remember to update benches/readme.md with your benchmark results" );
      eprintln!( "   Use MarkdownUpdater to automatically maintain comprehensive reports" );
      eprintln!( "   See: https://docs.rs/benchkit#standard-benches-directory-integration" );
    }
  }
}

#[ cfg( feature = "enabled" ) ]
pub mod measurement;

#[ cfg( feature = "enabled" ) ]
pub mod analysis;

#[ cfg( feature = "enabled" ) ]
pub mod suite;

#[ cfg( feature = "markdown_reports" ) ]
pub mod reporting;

#[ cfg( feature = "markdown_reports" ) ]
pub mod update_chain;

#[ cfg( feature = "markdown_reports" ) ]
pub mod templates;

#[ cfg( feature = "enabled" ) ]
pub mod validation;

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

  #[ cfg( feature = "markdown_reports" ) ]
  pub use crate::update_chain::*;

  #[ cfg( feature = "markdown_reports" ) ]
  pub use crate::templates::*;

  pub use crate::validation::*;

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