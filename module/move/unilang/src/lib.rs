#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/unilang/latest/unilang/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ]
#![ allow( clippy::mod_module_files ) ]

/// Internal namespace.
mod private
{
}

// Declare the actual modules
pub mod data;
pub mod error;
pub mod loader;
pub mod types;
pub mod help;
pub mod interpreter;
pub mod registry;
pub mod semantic;
pub mod pipeline;

mod_interface::mod_interface!
{
  /// Core data structures and types.
  orphan use super::data;
  
  /// Error handling utilities.
  orphan use super::error;
  
  /// Configuration loading from YAML/JSON.
  orphan use super::loader;
  
  /// Value types and type system.
  orphan use super::types;
  
  /// Help generation system.
  orphan use super::help;
  
  /// Command execution interpreter.
  orphan use super::interpreter;
  
  /// Command registry management.
  orphan use super::registry;
  
  /// Semantic analysis and validation.
  orphan use super::semantic;
  
  /// High-level pipeline API.
  orphan use super::pipeline;
  
  // Re-export the most commonly used types to exposed level
  exposed use super::data::CommandDefinition;
  exposed use super::data::ArgumentDefinition;
  exposed use super::data::Kind;
  exposed use super::data::OutputData;
  exposed use super::data::ErrorData;
  exposed use super::data::ArgumentAttributes;
  
  exposed use super::types::Value;
  
  exposed use super::error::Error;
  
  exposed use super::registry::CommandRegistry;
  exposed use super::registry::CommandRoutine;
  exposed use super::interpreter::ExecutionContext;
  
  exposed use super::semantic::SemanticAnalyzer;
  exposed use super::semantic::VerifiedCommand;
  
  exposed use super::interpreter::Interpreter;
  
  exposed use super::pipeline::Pipeline;
  exposed use super::pipeline::CommandResult;
  exposed use super::pipeline::BatchResult;
  exposed use super::pipeline::process_single_command;
  exposed use super::pipeline::validate_single_command;
  
  exposed use super::help::HelpGenerator;
  
  // Re-export the most essential types to prelude
  prelude use super::data::CommandDefinition;
  prelude use super::data::ArgumentDefinition;
  prelude use super::data::Kind;
  prelude use super::data::OutputData;
  prelude use super::data::ErrorData;
  prelude use super::types::Value;
  prelude use super::registry::CommandRegistry;
  prelude use super::pipeline::Pipeline;
}