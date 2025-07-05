//! Contains types related to parser configuration.
use std::collections::HashSet;

/// Options for configuring the behavior of the `Parser`.
#[derive(Debug, Clone)]
pub struct UnilangParserOptions
{
  /// If `true`, a positional argument encountered after a named argument will result in a `ParseError`.
  /// If `false`, positional arguments after named arguments are allowed.
  pub error_on_positional_after_named : bool,
  /// If `true`, duplicate named arguments (e.g., `name::val1 name::val2`) will result in a `ParseError`.
  /// If `false`, the last value for a duplicate named argument will overwrite previous ones.
  pub error_on_duplicate_named_arguments : bool,
  /// A set of string pairs representing opening and closing quotes (e.g., `("\"", "\"")`, `("'", "'")`).
  /// The parser will treat content within these as quoted values.
  pub quote_pairs : Vec<( &'static str, &'static str )>,
  /// A set of main delimiters that `strs_tools` will split the input string by.
  /// This includes `::`, `;;`, `?`, etc.
  pub main_delimiters : HashSet< &'static str >,
  /// If `true`, whitespace is treated as a separator, meaning multiple spaces or tabs
  /// between tokens will result in separate `Split` items for the whitespace.
  /// If `false`, consecutive whitespace is treated as a single separator.
  pub whitespace_is_separator : bool,
}

impl Default for UnilangParserOptions
{
  fn default() -> Self
  {
    let mut main_delimiters = HashSet::new();
    main_delimiters.insert( "::" );
    main_delimiters.insert( ";;" );
    main_delimiters.insert( "?" );
    main_delimiters.insert( ":" );
    main_delimiters.insert( "." ); // Add dot as a delimiter
    main_delimiters.insert( " " ); // Add space as a delimiter
    main_delimiters.insert( "\t" ); // Add tab as a delimiter

    Self
    {
      error_on_positional_after_named : true,
      error_on_duplicate_named_arguments : true,
      quote_pairs : vec![ ( "\"", "\"" ), ( "'", "'" ) ],
      main_delimiters,
      whitespace_is_separator : true,
    }
  }
}

impl UnilangParserOptions
{
  /// Converts the parser options into `strs_tools::string::split::SplitOptionsFormer`.
  #[allow(clippy::must_use_candidate)]
  pub fn to_split_options_former<'input>( &'input self, src : &'input str ) -> strs_tools::string::split::SplitOptionsFormer<'input>
  {
    let mut former = strs_tools::string::split::split();
    former.src( src );
    former.delimeter( self.main_delimiters.iter().copied().collect::<Vec<&str>>() );
    former.preserving_delimeters( true );
    former.preserving_empty( false );
    former.stripping( true );
    former.quoting( false );
    former
  }
}