//! This is a parser for Unilang instructions.
//!
//! It provides functionality to parse single or multiple instructions from a string,
//! handling command paths, arguments, and various syntax rules.
//!
//! The parser is designed to be robust against various input formats and provides
//! detailed error reporting for invalid instructions.
#![ cfg_attr( feature = "no_std", no_std ) ]
#![ cfg_attr( docsrs, feature( doc_auto_cfg ) ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_hr.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_hr.png" ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( rust_2018_idioms ) ]
extern crate alloc;
/// `unilang_instruction_parser` is a Rust crate designed to parse `unilang` CLI-like instruction strings.
/// It leverages `strs_tools` for initial itemization (splitting the input string into lexical tokens)
/// and then performs syntactic analysis to produce structured `GenericInstruction` objects.
///
/// ## Features
///
/// - Parses command paths (single or multi-segment).
/// - Handles positional arguments.
/// - Handles named arguments in the format `name::value`.
/// - Supports quoted arguments (e.g., `"value with spaces"`, `'another value'`) with basic escape sequence handling
///   (`\\`, `\"`, `\'`, `\n`, `\t`).
/// - Parses the help operator `?` (if it's the last token after a command path).
/// - Splits multiple instructions separated by `;;`.
/// - Provides detailed, location-aware error reporting using `ParseError` and `SourceLocation`
///   to pinpoint issues in the input string or slice segments.
/// - Configurable parsing behavior via `UnilangParserOptions` (e.g., error on duplicate named arguments,
///   error on positional arguments after named ones).
/// - `no_std` support (optional, via feature flag).
///
/// ## Core Components
///
/// - [`Parser`]: The main entry point for parsing instructions.
/// - [`UnilangParserOptions`]: Allows customization of parsing behavior.
/// - [`GenericInstruction`]: The primary output structure, representing a single parsed instruction with its
///   command path, positional arguments, and named arguments.
/// - [`Argument`]: Represents a parsed argument (either positional or named).
/// - [`ParseError`]: Encapsulates parsing errors, including an `ErrorKind` and `SourceLocation`.
/// - \[`SourceLocation`\]: Specifies the location of a token or error within the input \(either a string span or a slice segment\).\n/// ## Basic Usage Example
///
/// ```rust
/// use unilang_instruction_parser::{Parser, UnilangParserOptions};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let options = UnilangParserOptions::default();
///     let parser = Parser::new(options);
///     let input = "my.command arg1 name::value";
///
///     let instruction = parser.parse_single_instruction(input)?;
///
///     println!("Command Path: {:?}", instruction.command_path_slices);
///     Ok(())
/// }
/// ```
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
