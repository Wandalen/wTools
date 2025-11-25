//! Configuration options for the unilang instruction parser.
//!
//! This module defines the `UnilangParserOptions` struct, which allows
//! customization of the parsing behavior, such as delimiters, whitespace
//! handling, and error policies.

use alloc :: { vec, vec ::Vec };

#[ derive( Clone, PartialEq, Eq ) ]
/// Configuration options for the Unilang parser.
#[ derive( Debug ) ]
pub struct UnilangParserOptions
{
  /// A list of main delimiters used to split the input string into initial tokens.
  pub main_delimiters: Vec< &'static str >,
  /// A list of operators recognized by the parser.
  ///
  /// **Important:** The named argument operator `::` appears in TWO variants:
  /// - `"::"` - No surrounding spaces (e.g., `cmd::value`)
  /// - `" :: "` - With surrounding spaces (e.g., `cmd :: value`)
  ///
  /// Both variants must be included in the operators list. The tokenizer produces
  /// different tokens based on whitespace in the input, and parser code must check
  /// for both variants when detecting named argument operators.
  ///
  /// See module-level docs in `parser_engine.rs` for implementation patterns.
  pub operators: Vec< &'static str >,
  /// If `true`, whitespace characters are treated as separators between tokens.
  pub whitespace_is_separator: bool,
  /// If `true`, a `ParseError` is returned if a positional argument appears after a named argument.
  pub error_on_positional_after_named: bool,
  /// If `true`, a `ParseError` is returned if a named argument is duplicated. Otherwise, the last one wins.
  pub error_on_duplicate_named_arguments: bool,
  /// A list of character pairs used for quoting (e.g., `('"', '"')` for double quotes).
  pub quote_pairs: Vec< ( char, char ) >,
  /// Verbosity level for debug output (0 = quiet, 1 = normal, 2 = debug).
  pub verbosity: u8,
}

impl Default for UnilangParserOptions
{
  fn default() -> Self
  {
  Self
  {
   main_delimiters: vec![ " ", "." ],
   operators: vec![ "::", " :: ", "?", "!" ],
   whitespace_is_separator: true,
   error_on_positional_after_named: false,
   error_on_duplicate_named_arguments: false,
   quote_pairs: vec![ ( '"', '"' ), ( '\'', '\'' ) ],
   verbosity: 1, // Default to normal verbosity
 }
 }
}
