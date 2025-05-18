//! Defines configuration options for the unilang parser.

/// Options for configuring the `unilang` parser.
///
/// This structure wraps `strs_tools::string::parse_request::ItemizerOptions` to allow
/// customization of the underlying itemization process.
#[derive(Debug, Clone)]
pub struct UnilangParserOptions
{
  /// Options for the `strs_tools::string::parse_request::Itemizer`.
  pub itemizer_options : strs_tools::string::parse_request::ItemizerOptions<'static>,
}

impl Default for UnilangParserOptions
{
  fn default() -> Self
  {
    // Configure itemizer options for unilang syntax by default.
    // These settings are based on the typical unilang specification.
    Self
    {
      itemizer_options : strs_tools::string::parse_request::ItemizerOptions
      {
        quote_pairs : vec![ ( "\"", "\"" ), ( "'", "'" ) ],
        escape_char : Some( '\\' ),
        delimiters : vec![ "::", ";;" ], // "::" for named args, ";;" for command separation
        operators : vec![ "?" ],       // "?" for help
        comment_prefix : Some( "#" ),  // Standard comment prefix
        keep_whitespace_items : false, // Whitespace is generally not significant for commands
        keep_comment_items : false,    // Comments are discarded
        implicit_whitespace_delimit : true, // Items are separated by whitespace if no other delimiter
      },
    }
  }
}