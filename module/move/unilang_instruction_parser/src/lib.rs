//!
//! `unilang_instruction_parser` is a Rust crate designed to parse `unilang` CLI-like instruction strings.
//! It leverages `strs_tools` for initial itemization (splitting the input string into lexical tokens)
//! and then performs syntactic analysis to produce structured `GenericInstruction` objects.
//!
//! ## Features
//!
//! - Parses command paths (single or multi-segment).
//! - Handles positional arguments.
//! - Handles named arguments in the format `name::value`.
//! - Supports quoted arguments (e.g., `"value with spaces"`, `'another value'`) with basic escape sequence handling
//!   (`\\`, `\"`, `\'`, `\n`, `\t`).
//! - Parses the help operator `?` (if it's the last token after a command path).
//! - Splits multiple instructions separated by `;;`.
//! - Provides detailed, location-aware error reporting using `ParseError` and `SourceLocation`
//!   to pinpoint issues in the input string or slice segments.
//! - Configurable parsing behavior via `UnilangParserOptions` (e.g., error on duplicate named arguments,
//!   error on positional arguments after named ones).
//! - `no_std` support (optional, via feature flag).
//!
//! ## Core Components
//!
//! - [`Parser`]: The main entry point for parsing instructions.
//! - [`UnilangParserOptions`]: Allows customization of parsing behavior.
//! - [`GenericInstruction`]: The primary output structure, representing a single parsed instruction with its
//!   command path, positional arguments, and named arguments.
//! - [`Argument`]: Represents a parsed argument (either positional or named).
//! - [`ParseError`]: Encapsulates parsing errors, including an `ErrorKind` and `SourceLocation`.
//! - [`SourceLocation`]: Specifies the location of a token or error within the input (either a string span or a slice segment).
//!
//! ## Basic Usage Example
//!
//! ```rust
//! use unilang_instruction_parser::{Parser, UnilangParserOptions, GenericInstruction, Argument, SourceLocation};
//!
//! fn main() -> Result<(), unilang_instruction_parser::error::ParseError> {
//!     let options = UnilangParserOptions { error_on_positional_after_named: false, ..Default::default() };
//!     let parser = Parser::new(options);
//!     let input = "command.sub_command path/arg1 name::\"value with spaces\" --verbose ;; another_cmd ?";
//!
//!     let instructions = parser.parse_single_str(input)?;
//!
//!     for instruction in instructions {
//!         println!("Command Path: {:?}", instruction.command_path_slices);
//!
//!         if instruction.help_requested {
//!             println!("Help was requested for this command.");
//!         }
//!
//!         println!("Positional Arguments:");
//!         for pos_arg in instruction.positional_arguments {
//!             println!("  - Value: '{}' (at {:?})", pos_arg.value, pos_arg.value_location);
//!         }
//!
//!         println!("Named Arguments:");
//!         for (name, named_arg) in instruction.named_arguments {
//!             println!("  - {}: '{}' (name at {:?}, value at {:?})",
//!                 name,
//!                 named_arg.value,
//!                 named_arg.name_location,
//!                 named_arg.value_location
//!             );
//!         }
//!         println!("---");
//!     }
//!
//!     // Example of error handling
//!     let error_input = "cmd name_only_no_delimiter_then_value";
//!     match parser.parse_single_str(error_input) {
//!         Ok(_) => println!("Should have failed but parsed ok."),
//!         Err(e) => {
//!             println!("Successfully caught parse error for input '{}':", error_input);
//!             println!("  Error: {}", e);
//!             if let Some(location) = e.location {
//!                 println!("  Location: {:?}", location);
//!                 // You can use location.start(), location.end() with StrSpan
//!                 // or location.segment_index(), location.start_in_segment(), location.end_in_segment() with SliceSegment
//!                 // to highlight the error in the original input.
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
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
  // pub use super::instruction::*; // Removed ambiguous re-export
  pub use super::item_adapter::*;
  pub use super::parser_engine::*;
}

pub use prelude::*;
