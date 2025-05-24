//! Defines configuration options for the unilang parser.
use strs_tools::string::split::SplitOptionsFormer;
use strs_tools::string::parse_request::OpType;

/// High-level options for configuring the `unilang` parser.
///
/// These options control various aspects of the parsing process, such as how quotes and delimiters
/// are handled, and rules for argument parsing. These options are then translated into
/// lower-level settings for the `strs_tools::string::split::SplitOptionsFormer` which performs
/// the initial tokenization of the input string.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct UnilangParserOptions
{
  /// Defines pairs of characters or strings that denote the start and end of a quoted value.
  ///
  /// For example, `vec![("\"", "\""), ("'", "'")]` would recognize both double-quoted
  /// and single-quoted strings. The parser will extract the inner content of these quotes.
  /// Escape sequences within these quoted values are handled by the parser.
  pub quote_pairs : Vec<( &'static str, &'static str )>,
  /// A list of strings that act as primary delimiters or operators in the unilang syntax.
  ///
  /// This typically includes:
  /// - `"::"` for separating named argument names from their values.
  /// - `";;"` for separating multiple instructions within a single input string.
  /// - `"?"` for requesting help on a command.
  /// These delimiters are preserved during tokenization and used by the parser to
  /// determine the structure of commands and arguments.
  #[allow(clippy::doc_lazy_continuation)]
  /// These delimiters are preserved during tokenization and used by the parser to
  /// determine the structure of commands and arguments.
  pub main_delimiters : Vec<&'static str>,
  /// If `true`, leading and trailing whitespace will be stripped from each token produced
  /// by the underlying `strs_tools` splitter before classification.
  /// Defaults to `true`.
  pub strip_whitespace : bool,
  /// If `true`, the parser will return an error if a named argument is duplicated within a single instruction.
  ///
  /// For example, `cmd name::val1 name::val2` would cause an error.
  /// If `false` (the default), the last occurrence of a duplicated named argument "wins", effectively
  /// overwriting previous values for that argument name.
  pub error_on_duplicate_named_arguments : bool,
  /// If `true` (the default), the parser will return an error if a positional argument
  /// is encountered after any named argument has already been parsed for that instruction.
  ///
  /// For example, `cmd name::val pos_arg` would cause an error.
  /// If `false`, positional arguments can be interleaved with or follow named arguments,
  /// e.g., `cmd name1::val1 pos1 name2::val2 pos2`.
  pub error_on_positional_after_named : bool,
  /// If `true` (the default), whitespace characters (space, tab, newline, carriage return)
  /// will also act as separators between tokens, in addition to `main_delimiters`.
  /// If `false`, only `main_delimiters` will separate tokens, and whitespace might become
  /// part of unquoted values.
  pub whitespace_is_separator : bool,
}

impl Default for UnilangParserOptions
{
  /// Creates a default set of parser options.
  ///
  /// Default values are:
  /// - `quote_pairs`: `vec![("\"", "\""), ("'", "'")]`
  /// - `main_delimiters`: `vec![ "::", ";;", "?" ]`
  /// - `strip_whitespace`: `true`
  /// - `error_on_duplicate_named_arguments`: `false` (last one wins)
  /// - `error_on_positional_after_named`: `true` (strict order)
  /// - `whitespace_is_separator`: `true`
  fn default() -> Self
  {
    Self
    {
      quote_pairs : vec![ ( "\"", "\"" ), ( "'", "'" ) ],
      main_delimiters : vec![ "::", ";;", "?" ], // Corrected: removed duplicate line
      strip_whitespace : true,
      error_on_duplicate_named_arguments : false,
      error_on_positional_after_named : true,
      whitespace_is_separator : true,
    }
  }
}

impl UnilangParserOptions
{
  /// Translates these high-level `UnilangParserOptions` into a `SplitOptionsFormer`
  /// instance, which is used by the `strs_tools::string::split` module for initial
  /// tokenization of the input string.
  ///
  /// This method configures the splitter based on the defined quote pairs, delimiters,
  /// and whitespace handling rules.
  #[allow(clippy::must_use_candidate)]
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
    former.preserving_quoting( true );

    former
  }
}