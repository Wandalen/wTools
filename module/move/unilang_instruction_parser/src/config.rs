//! Defines configuration options for the unilang parser.
use strs_tools::string::split::SplitOptionsFormer;
use strs_tools::string::parse_request::OpType; // Required for SplitOptionsFormer delimeter

/// High-level options for configuring the `unilang` parser.
/// These options will be translated into settings for `strs_tools::string::split::SplitOptionsFormer`.
#[derive(Debug, Clone)]
pub struct UnilangParserOptions
{
  /// Quote pairs to be used for identifying quoted values.
  /// Each tuple is (prefix, postfix).
  pub quote_pairs : Vec<( &'static str, &'static str )>,
  /// Delimiters that separate significant parts of the command.
  /// e.g., "::" for named arguments, ";;" for command separation.
  /// The "?" help operator can also be treated as a delimiter here.
  pub delimiters : Vec<&'static str>,
  /// Whether to strip leading/trailing whitespace from delimited segments.
  pub strip_whitespace : bool,
  // Note: Escape character and comment prefix handling are now responsibilities
  // of the unilang_instruction_parser itself, post-itemization by `strs_tools::string::split`.
}

impl Default for UnilangParserOptions
{
  fn default() -> Self
  {
    Self
    {
      quote_pairs : vec![ ( "\"", "\"" ), ( "'", "'" ) ],
      // Key unilang delimiters. "?" is included to be split out.
      delimiters : vec![ "::", ";;", "?" ],
      strip_whitespace : true, // Typically, whitespace around tokens is not significant.
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

    let mut former = SplitOptionsFormer::new( OpType::Vector( self.delimiters.clone() ) );
    former.src( src );
    former.preserving_empty( false ); // Typically, empty segments are not meaningful instructions or parts.
    former.preserving_delimeters( true ); // We need to see the delimiters to parse structure.
    former.stripping( self.strip_whitespace );
    former.quoting( !self.quote_pairs.is_empty() ); // Enable quoting if pairs are defined.
    former.quoting_prefixes( prefixes );
    former.quoting_postfixes( postfixes );
    // `preserving_quoting` is false by default in SplitOptionsFormer if not set.
    // For unilang, we usually want the unescaped value without the quotes,
    // so `preserving_quoting: false` (default) is often desired.
    // If quotes themselves need to be analyzed, this could be true,
    // and unilang_parser would strip them. For now, assume false is fine.
    former
  }
}