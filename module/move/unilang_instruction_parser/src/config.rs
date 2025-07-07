//! Configuration options for the unilang instruction parser.
//!
//! This module defines the `UnilangParserOptions` struct, which allows
//! customization of the parsing behavior, such as delimiters, whitespace
//! handling, and error policies.

/// Configuration options for the unilang instruction parser.
///
/// This struct allows customization of various aspects of the parsing process.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct UnilangParserOptions
{
  /// A list of strings that are considered main delimiters for tokenization.
  ///
  /// These delimiters will split the input string into tokens.
  pub main_delimiters : Vec< &'static str >,
  /// A list of strings that are considered operators.
  pub operators : Vec< &'static str >,
  /// If `true`, whitespace characters (space, tab, newline, etc.) are treated as delimiters.
  ///
  /// If `false`, whitespace is treated as part of an identifier unless explicitly
  /// listed in `main_delimiters`.
  pub whitespace_is_separator : bool,
  /// If `true`, a `ParseError` will be returned if a positional argument is
  /// encountered after a named argument.
  ///
  /// If `false`, positional arguments after named arguments are allowed.
  pub error_on_positional_after_named : bool,
  /// If `true`, a `ParseError` will be returned if a named argument with the
  /// same name is encountered multiple times.
  ///
  /// If `false`, the last encountered value for a duplicate named argument
  /// will overwrite previous ones.
  pub error_on_duplicate_named_arguments : bool,
  /// A list of character pairs that denote quoted strings.
  ///
  /// The first character in the tuple is the opening quote, the second is the closing quote.
  /// E.g., `[ ( '"', '"' ), ( '\'', '\'' ) ]` for double and single quotes.
  pub quote_pairs : Vec< ( char, char ) >,
}

impl Default for UnilangParserOptions
{
  fn default() -> Self
  {
    Self
    {
      main_delimiters : vec![ " ", ";;" ],
      operators : vec![ "::", "?" ],
      whitespace_is_separator : true,
      error_on_positional_after_named : false,
      error_on_duplicate_named_arguments : true,
      quote_pairs : vec![ ( '"', '"' ), ( '\'', '\'' ) ],
    }
  }
}