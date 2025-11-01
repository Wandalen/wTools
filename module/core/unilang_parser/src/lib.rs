//! This is a parser for Unilang instructions.
//!
//! It provides functionality to parse single or multiple instructions from a string,
//! handling command paths, arguments, and various syntax rules.
//!
//! The parser is designed to be robust against various input formats and provides
//! detailed error reporting for invalid instructions.
#![ cfg_attr( feature = "no_std", no_std ) ]
#![ cfg_attr( docsrs, feature( doc_auto_cfg ) ) ]
#![ doc( html_logo_url = "https: //raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_hr.png" ) ]
#![ doc( html_favicon_url = "https: //raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_hr.png" ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( rust_2018_idioms ) ]
extern crate alloc;
/// `unilang_parser` is a Rust crate designed to parse `unilang` CLI-like instruction strings.
/// It leverages `strs_tools` for initial itemization (splitting the input string into lexical tokens)
/// and then performs syntactic analysis to produce structured `GenericInstruction` objects.
///
/// ## Features
///
/// - Parses command paths (single or multi-segment).
/// - Handles positional arguments.
/// - Handles named arguments in the format `name ::value`.
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
/// - [`Parser`] : The main entry point for parsing instructions.
/// - [`UnilangParserOptions`] : Allows customization of parsing behavior.
/// - [`GenericInstruction`] : The primary output structure, representing a single parsed instruction with its
///   command path, positional arguments, and named arguments.
/// - [`Argument`] : Represents a parsed argument (either positional or named).
/// - [`ParseError`] : Encapsulates parsing errors, including an `ErrorKind` and `SourceLocation`.
/// - [`SourceLocation`] : Specifies the location of a token or error within the input (either a string span or a slice segment).
/// ## Basic Usage Example
///
/// ```rust
/// use unilang_parser :: { Parser, UnilangParserOptions };
///
/// fn main() -> Result< (), Box<dyn std ::error ::Error >> {
///     let options = UnilangParserOptions ::default();
///     let parser = Parser ::new(options);
///     let input = "my.command arg1 name ::value";
///
///     let instruction = parser.parse_single_instruction(input)?;
///
///     println!("Command Path: {:?}", instruction.command_path_slices);
///     Ok(())
/// }
/// ```
///
/// ## ⚠️ CLI Integration: Using Shell Arguments Correctly
///
/// When integrating `unilang_parser` into a CLI application that receives arguments from a shell,
/// **you must use the `parse_from_argv()` method**, NOT `split_whitespace()`.
///
/// ### ✅ Correct Usage (CLI Application)
///
/// ```rust
/// use unilang_parser :: { Parser, UnilangParserOptions };
///
/// fn main() -> Result< (), Box<dyn std ::error ::Error >> {
///     let options = UnilangParserOptions ::default();
///     let parser = Parser ::new(options);
///
///     // Collect shell arguments (already tokenized by the shell)
///     let argv : Vec<String> = std ::env ::args().collect();
///
///     // ✅ CORRECT: Use parse_from_argv for shell arguments
///     let instruction = parser.parse_from_argv(&argv)?;
///
///     println!("Command: {:?}", instruction.command_path_slices);
///     Ok(())
/// }
/// ```
///
/// ### ❌ Common Pitfall (WRONG)
///
/// ```rust,ignore
/// use unilang_parser :: { Parser, UnilangParserOptions };
///
/// fn main() -> Result< (), Box<dyn std ::error ::Error >> {
///     let options = UnilangParserOptions ::default();
///     let parser = Parser ::new(options);
///
///     let argv : Vec<String> = std ::env ::args().collect();
///     let joined = argv.join(" ");
///
///     // ❌ WRONG: Don't use split_whitespace() on shell argv!
///     // This breaks quote handling that the shell already performed
///     let instruction = parser.parse_single_instruction(&joined)?;
///
///     Ok(())
/// }
/// ```
///
/// ### Why This Matters
///
/// The shell has **already tokenized** the arguments, handling quotes, escapes, and whitespace.
/// When you receive `argv` from the shell:
///
/// - `my-app "foo bar"` → shell produces `argv = ["my-app", "foo bar"]` (2 tokens)
/// - If you join and re-split: `"my-app foo bar".split_whitespace()` → produces `["my-app", "foo", "bar"]` (3 tokens) ❌
///
/// **Result:** Arguments containing spaces are incorrectly split, breaking user expectations.
///
/// ### Rule of Thumb
///
/// - **From shell (CLI app):** Use `parse_from_argv(&argv)` - shell already tokenized
/// - **From string (embedded/scripting):** Use `parse_single_instruction(input)` - string needs parsing
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
  pub use super ::config :: *;
  pub use super ::error :: *;
  pub use super ::instruction :: { GenericInstruction, Argument };
  pub use super ::item_adapter :: *;
  pub use super ::parser_engine :: *;
}

pub use prelude :: *;
