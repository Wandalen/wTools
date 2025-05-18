//! Defines configuration options for the unilang parser.
use strs_tools::string::split::SplitOptionsFormer;
use strs_tools::string::parse_request::OpType;

/// High-level options for configuring the `unilang` parser.
/// These options will be translated into settings for `strs_tools::string::split::SplitOptionsFormer`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnilangParserOptions
{
  /// Quote pairs to be used for identifying quoted values.
  /// Each tuple is (prefix, postfix).
  pub quote_pairs : Vec<( &'static str, &'static str )>,
  /// Delimiters that separate significant parts of the command, e.g., "::", ";;", "?".
  pub main_delimiters : Vec<&'static str>,
  /// Whether to strip leading/trailing whitespace from delimited segments.
  pub strip_whitespace : bool,
  /// If true, the parser will return an error if a named argument is duplicated.
  /// If false (default), the last occurrence of a duplicated named argument wins.
  pub error_on_duplicate_named_arguments : bool,
  /// If true (default), the parser will return an error if a positional argument
  /// is encountered after any named argument has already been parsed for that instruction.
  /// If false, positional arguments can be interleaved with or follow named arguments.
  pub error_on_positional_after_named : bool,
  /// Whether whitespace should also act as a separator between tokens.
  pub whitespace_is_separator : bool,
  // /// Whether to preserve quoting characters in the output of `SplitIterator`.
  // pub preserve_quotes_in_split : bool, // New option, might not be needed if classify_split handles it
}

impl Default for UnilangParserOptions
{
  fn default() -> Self
  {
    Self
    {
      quote_pairs : vec![ ( "\"", "\"" ), ( "'", "'" ) ],
      main_delimiters : vec![ "::", ";;", "?" ],
      strip_whitespace : true,
      error_on_duplicate_named_arguments : false,
      error_on_positional_after_named : true,
      whitespace_is_separator : true,
      // preserve_quotes_in_split : false, // Default to false, let classify_split manage
    }
  }
}

impl UnilangParserOptions
{
  /// Translates these high-level options into `SplitOptionsFormer` for the `strs_tools::string::split` module.
  pub fn to_split_options_former<'s>( &'s self, src : &'s str ) -> SplitOptionsFormer<'s>
  {
    let mut prefixes = Vec::with_capacity( self.quote_pairs.len() );
    let mut postfixes = Vec::with_capacity( self.quote_pairs.len() );
    for (prefix, postfix) in &self.quote_pairs
    {
      prefixes.push( *prefix );
      postfixes.push( *postfix );
    }

    let mut effective_delimiters = self.main_delimiters.clone();
    if self.whitespace_is_separator
    {
      effective_delimiters.extend( vec![ " ", "\t", "\n", "\r" ] );
    }

    let mut former = SplitOptionsFormer::new( OpType::Vector( Vec::new() ) );
    former.src( src );
    former.delimeter( OpType::Vector( effective_delimiters ) );
    former.preserving_empty( false );
    former.preserving_delimeters( true );
    former.stripping( self.strip_whitespace );
    former.quoting( !self.quote_pairs.is_empty() );
    former.quoting_prefixes( prefixes );
    former.quoting_postfixes( postfixes );
    former.preserving_quoting( true ); // Preserve outer quotes from SplitIterator

    former
  }
}