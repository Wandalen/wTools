//!
//! `unilang_instruction_parser` is a Rust crate designed to parse `unilang` CLI-like instruction strings.
//! It leverages `strs_tools` for initial itemization and then performs syntactic analysis
//! to produce structured `GenericInstruction` objects. The parser is capable of handling
//! commands, named arguments, positional arguments, and provides location-aware error reporting.
//!

#![ cfg_attr( feature = "no_std", no_std ) ]
#![ cfg_attr( docsrs, feature( doc_auto_cfg ) ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_hr.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_hr.png" ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( rust_2018_idioms ) ]

/// Contains types related to parser configuration.
pub mod config;
/// Defines error types for the parser.
pub mod error;
/// Defines instruction and argument structures.
pub mod instruction;
/// Adapts and classifies items from the splitter.
pub mod item_adapter;
/// Contains the core parsing engine.
pub mod parser_engine;

/// Prelude for commonly used items.
pub mod prelude
{
  pub use super::config::*;
  pub use super::error::*;
  pub use super::instruction::*;
  pub use super::item_adapter::*;
  pub use super::parser_engine::*;
}

pub use prelude::*;
