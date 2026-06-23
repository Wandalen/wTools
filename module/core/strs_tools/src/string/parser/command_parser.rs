//! Structured command-line parser with context-aware token classification.

use super::{ ParseError, ParsedToken };

/// Parser context for state-aware parsing
#[ derive( Debug, Clone, Copy ) ]
enum ParsingContext
{
  /// Expecting command name
  Command,
  /// Expecting arguments or flags
  Arguments,
  /// Expecting value after key (reserved for future use)
  #[ allow( dead_code ) ]
  Value,
}

/// Structured command-line parser with context awareness
#[ derive( Debug, Clone ) ]
pub struct CommandParser< 'a >
{
  input: &'a str,
  token_delimiters: Vec< &'a str >,
  kv_separator: &'a str,
  flag_prefix: &'a str,
}

impl< 'a > CommandParser< 'a >
{
  /// Create new command parser with default settings
  pub fn new( input: &'a str ) -> Self
  {
  Self
  {
   input,
   token_delimiters: vec![ " ", "\t" ],
   kv_separator: ":",
   flag_prefix: "--",
 }
 }

  /// Set custom token delimiters
  pub fn with_token_delimiters( mut self, delimiters: Vec< &'a str > ) -> Self
  {
  self.token_delimiters = delimiters;
  self
 }

  /// Set custom key-value separator
  pub fn with_kv_separator( mut self, separator: &'a str ) -> Self
  {
  self.kv_separator = separator;
  self
 }

  /// Set custom flag prefix
  pub fn with_flag_prefix( mut self, prefix: &'a str ) -> Self
  {
  self.flag_prefix = prefix;
  self
 }

  /// Parse command line in single pass with context awareness
  pub fn parse_structured( self ) -> impl Iterator< Item = Result< ParsedToken< 'a >, ParseError > > + 'a
  {
  StructuredParsingIterator
  {
   parser: self,
   position: 0,
   current_context: ParsingContext ::Command,
   pending_key: None,
 }
 }
}

/// Internal iterator for structured parsing
struct StructuredParsingIterator< 'a >
{
  parser: CommandParser< 'a >,
  position: usize,
  current_context: ParsingContext,
  pending_key: Option< &'a str >,
}

impl< 'a > StructuredParsingIterator< 'a >
{
  /// Find next token boundary using position-based slicing
  fn find_next_token( &mut self ) -> Option< &'a str >
  {
  loop
  {
   if self.position >= self.parser.input.len()
   {
  return None;
 }

   let remaining = &self.parser.input[ self.position.. ];
   
   // Find the earliest delimiter match
   let mut earliest_delim_pos = None;
   let mut earliest_delim_len = 0;
   
   for delim in &self.parser.token_delimiters
   {
  if let Some( pos ) = remaining.find( delim )
  {
   match earliest_delim_pos
   {
  None => 
  {
   earliest_delim_pos = Some( pos );
   earliest_delim_len = delim.len();
 },
  Some( current_pos ) if pos < current_pos =>
  {
   earliest_delim_pos = Some( pos );
   earliest_delim_len = delim.len();
 },
  _ => {} // Keep current earliest
 }
 }
 }
   
   let (token_start, token_end) = if let Some( delim_pos ) = earliest_delim_pos
   {
  // Token is everything before the delimiter
  let token_start = self.position;
  let token_end = self.position + delim_pos;
  self.position += delim_pos + earliest_delim_len;
  (token_start, token_end)
 }
   else
   {
  // No delimiter found, rest of input is the token
  let token_start = self.position;
  let token_end = self.parser.input.len();
  self.position = self.parser.input.len();
  (token_start, token_end)
 };
   
   if token_start < token_end
   {
  let token = &self.parser.input[ token_start..token_end ];
  if !token.is_empty()
  {
   return Some( token );
 }
 }
   
   // If token is empty, continue loop to find next non-empty token
 }
 }

  /// Parse argument token based on context and characteristics
  fn parse_argument_token( &mut self, token: &'a str ) -> Result< ParsedToken< 'a >, ParseError >
  {
  // Check for key-value pairs first (can start with flag prefix)
  if token.contains( self.parser.kv_separator )
  {
   let separator_pos = token.find( self.parser.kv_separator ).unwrap();
   let key_part = &token[ ..separator_pos ];
   let value = &token[ separator_pos + self.parser.kv_separator.len().. ];

   // Extract key from potential flag prefix
   let key = if key_part.starts_with( self.parser.flag_prefix )
   {
  &key_part[ self.parser.flag_prefix.len().. ]
 }
   else
   {
  key_part
 };

   if key.is_empty()
   {
  Err( ParseError ::InvalidKeyValuePair( token.to_string() ) )
 }
   else if value.is_empty()
   {
  // Key with separator but no value - expect next token to be the value
  self.current_context = ParsingContext ::Value;
  self.pending_key = Some( key );
  Ok( ParsedToken ::Positional( token ) ) // Temporary - will be replaced when value is found
 }
   else
   {
  Ok( ParsedToken ::KeyValue { key, value } )
 }
 }
  else if token.starts_with( self.parser.flag_prefix )
  {
   // Flag argument
   let flag_name = &token[ self.parser.flag_prefix.len().. ];
   Ok( ParsedToken ::Flag( flag_name ) )
 }
  else
  {
   // Positional argument
   Ok( ParsedToken ::Positional( token ) )
 }
 }
}

impl< 'a > Iterator for StructuredParsingIterator< 'a >
{
  type Item = Result< ParsedToken< 'a >, ParseError >;

  fn next( &mut self ) -> Option< Self ::Item >
  {
  loop
  {
   let token = self.find_next_token()?;

   // Parse based on current context and token characteristics
   match self.current_context
   {
    ParsingContext ::Command =>
    {
     self.current_context = ParsingContext ::Arguments;
     return Some( Ok( ParsedToken ::Command( token ) ) );
    },
    ParsingContext ::Arguments =>
    {
     match self.parse_argument_token( token )
     {
      Ok( ParsedToken ::Positional( _ ) ) if self.pending_key.is_some() =>
      {
       // This was a key token that set pending_key, don't emit it, continue to get value
      },
      other => return Some( other ),
     }
    },
    ParsingContext ::Value =>
    {
     self.current_context = ParsingContext ::Arguments;
     if let Some( key ) = self.pending_key.take()
     {
      return Some( Ok( ParsedToken ::KeyValue { key, value: token } ) );
     }
     return Some( Ok( ParsedToken ::Positional( token ) ) );
    },
   }
  }
 }
}
