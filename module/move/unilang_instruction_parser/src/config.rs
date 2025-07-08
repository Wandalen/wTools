//! Configuration options for the unilang instruction parser.
//!
//! This module defines the `UnilangParserOptions` struct, which allows
//! customization of parsing behavior, including delimiters, operators,
//! and error handling.

// Removed SplitOptionsFormer import as it's no longer used here.

/// Configuration options for the unilang instruction parser.
#[ derive( Debug, Clone ) ]
pub struct UnilangParserOptions
{
  /// If true, a positional argument after a named argument will result in a parse error.
  pub error_on_positional_after_named : bool,
  /// If true, duplicate named arguments will result in a parse error.
  pub error_on_duplicate_named_arguments : bool,
  /// Pairs of quote characters (e.g., `("\"", "\"")`, `("'", "'")`).
  pub quote_pairs : Vec< ( String, String ) >,
  /// Main delimiters used for splitting the input string.
  pub main_delimiters : Vec< String >,
  /// If true, whitespace is considered a separator.
  pub whitespace_is_separator : bool,
}

impl Default for UnilangParserOptions
{
  fn default() -> Self
  {
    Self
    {
      error_on_positional_after_named : true,
      error_on_duplicate_named_arguments : true,
      quote_pairs : vec!
      [
        ( "\"".to_string(), "\"".to_string() ),
        ( "'".to_string(), "'".to_string() ),
      ],
      main_delimiters : vec!
      [
        "::".to_string(),
        ";;".to_string(),
        ".".to_string(),
        "?".to_string(),
        // Removed spaces and tabs from here, as strs_tools should handle whitespace as separator
      ],
      whitespace_is_separator : true, // Reverted to true
    }
  }
}

// Removed the to_split_options_former method.