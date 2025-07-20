//! Configuration options for the unilang instruction parser.
//!
//! This module defines the `UnilangParserOptions` struct, which allows
//! customization of the parsing behavior, such as delimiters, whitespace
//! handling, and error policies.

#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct UnilangParserOptions
{
  pub main_delimiters : Vec< &'static str >,
  pub operators : Vec< &'static str >,
  pub whitespace_is_separator : bool,
  pub error_on_positional_after_named : bool,
  pub error_on_duplicate_named_arguments : bool,
  pub quote_pairs : Vec< ( char, char ) >,
}

impl Default for UnilangParserOptions
{
  fn default() -> Self
  {
    Self
    {
      main_delimiters : vec![ " ", "." ],
      operators : vec![ "::", "?" ],
      whitespace_is_separator : true,
      error_on_positional_after_named : false,
      error_on_duplicate_named_arguments : true,
      quote_pairs : vec![ ( '"', '"' ), ( '\'', '\'' ) ],
    }
  }
}