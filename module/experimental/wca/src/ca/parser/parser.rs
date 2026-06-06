mod private
{

  use crate :: *;

  use std ::collections ::HashMap;
  use parser :: { Program, ParsedCommand };
  use error_tools ::untyped ::Result;
  use error_tools ::dependency ::thiserror;

  // use error :: { return_err };

  #[ allow( missing_docs ) ]
  #[ derive( Debug, error_tools ::typed ::Error ) ]
  pub enum ParserError
  {
  #[ error( "Internal Error: {details}" ) ]
  InternalError { details: String },
  #[ error( "Unexpected input. Expected: {expected}, found {input}" ) ]
  UnexpectedInput { expected: String, input: String },
 }

  /// `Parser` is a struct used for parsing data.
  #[ derive( Debug ) ]
  pub struct Parser;

  // fix clippy error too large return type
  type ParsedArgs = ( Vec<  String  >, HashMap<  String, String  >, usize );

  impl Parser
  {
  /// Parses a vector of command line arguments and returns a `Program` containing the parsed commands.
  ///
  /// # Arguments
  ///
  /// * `args` - A vector of strings representing the command line arguments.
  ///
  /// # Returns
  ///
  /// Returns a `Result` with a `Program` containing the parsed commands if successful, or an error if parsing fails.
  /// # Errors
  /// Returns an error if the input contains invalid command syntax or unexpected tokens.
  pub fn parse< As, A >( &self, args: As ) -> Result< Program< ParsedCommand >, ParserError >
  where
   As: IntoIterator< Item = A >,
   A: Into< String >,
  {
   let args: Vec< _ > = args.into_iter().map( Into ::into ).collect();
   let mut commands = vec![];
   let mut i = 0;
   while i < args.len()
   {
  let ( command, relative_pos ) = Self ::parse_command( &args[ i.. ] )?;
  i += relative_pos;
  commands.push( command );
 }

   Ok( Program { commands } )
 }

  // with dot at the beginning
  fn valid_command_name( input: &str ) -> bool
  {
   if let Some( name ) = input.strip_prefix( '.' )
   {
  name.is_empty() || name.starts_with( '?' ) || name.chars().next().is_some_and( char ::is_alphanumeric )
 }
   else
   {
  false
 }
 }

  // Checks if string looks like Windows path or URL (not a property)
  fn looks_like_path_or_url( input: &str ) -> bool
  {
   // URL schemes
   if input.contains( "://" ) { return true; }

   // Windows drive letters
   if input.len() >= 3
   {
  let bytes = input.as_bytes();
  if bytes[ 0 ].is_ascii_alphabetic() && bytes[ 1 ] == b':'
  {
   // C:\ or C:/ patterns
   if bytes[ 2 ] == b'\\' || bytes[ 2 ] == b'/' { return true; }
 }
 }

   // UNC paths (\\server\share)
   if input.starts_with( "\\\\" ) { return true; }

   // Time format (digits and colons only)
   if input.len() >= 2 && input.chars().all( | c | c.is_ascii_digit() || c == ':' )
   { return true; }

   false
 }
  fn parse_command( args: &[ String ] ) -> Result< ( ParsedCommand, usize ), ParserError >
  {
   if args.is_empty()
   {
  return Err( ParserError ::InternalError { details: "Try to parse command without input".into() } );
 }

   let mut i = 0;

   if !Self ::valid_command_name( &args[ i ] )
   {
  return Err( ParserError ::UnexpectedInput { expected: "command".into(), input: args[ i ].clone() } );
 }
   let name = match args[ i ].strip_prefix( '.' ).unwrap()
   {
  "" => ".",
  "?" => ".?",
  other => other,
 };
   i += 1;
   let ( subjects, properties, relative_pos ) = Self ::parse_command_args( &args[ i .. ] )?;
   i += relative_pos;

   Ok(
   (
  ParsedCommand
  {
   name: name.to_string(),
   subjects,
   properties,
 },
  i,
 ))
 }




  // returns ( subjects, properties, relative_end_pos )
  fn parse_command_args( args: &[ String ] ) -> Result< ParsedArgs, ParserError >
  {
   let mut i = 0;

   let mut subjects = vec![];
   let mut properties = HashMap ::new();

   let mut properties_turn = false;
   while i < args.len()
   {
  let item = &args[ i ];

  if Self ::valid_command_name( item ) { break; }

  if item.contains( " : " )
  {
   properties_turn = true;
   let ( name, value ) = item.split_once( " : " ).unwrap();
   // prop: value
   if !value.is_empty()
   {
  properties.insert( name.to_string(), value.to_string() );
 }
   // prop: value
   else if args.len() > i + 1
   {
  properties.insert( name.to_string(), args[ i + 1 ].clone() );
  i += 1;
 }
   // we can identify this as a subject, can't we?
   // prop :
   else
   {
  return Err( ParserError ::UnexpectedInput { expected: "property value".into(), input: "end of input".into() } );
 }
 }
  // Check for property without spaces: "key:value" or "key: value"
  else if item.contains( ':' ) && !Self ::looks_like_path_or_url( item )
  {
   properties_turn = true;
   if let Some( ( name, value ) ) = item.split_once( ':' )
   {
  let value = value.trim();
  // If value is empty, try to get from next arg
  if value.is_empty()
  {
   if args.len() > i + 1
   {
  properties.insert( name.to_string(), args[ i + 1 ].clone() );
  i += 1;
 }
   else
   {
  return Err( ParserError ::UnexpectedInput { expected: "property value".into(), input: "end of input".into() } );
 }
 }
  else
  {
   properties.insert( name.to_string(), value.to_string() );
 }
 }
 }
  // prop: value | prop: value
  else if args.len() > i + 1 && args[ i + 1 ].starts_with( " : " )
  {
   let stripped = args[ i + 1 ].strip_prefix( " : " ).unwrap();
   // : value (has content after " : ")
   if !stripped.is_empty()
   {
  properties.insert( args[ i ].clone(), stripped.to_string() );
  i += 1;
 }
   // : value (next arg is exactly " : ", value follows)
   else if args.len() > i + 2
   {
  properties.insert( args[ i ].clone(), args[ i + 2 ].clone() );
  i += 2;
 }
   // : (no value)
   else
   {
  return Err( ParserError ::UnexpectedInput { expected: "property value".into(), input: "end of input".into() } );
 }
 }

  else if !properties_turn
  {
   subjects.push( item.clone() );
 }
  else
  {
   return Err( ParserError ::UnexpectedInput { expected: "`command` or `property`".into(), input: item.into() } );
 }
  i += 1;
 }

   Ok( ( subjects, properties, i ) )
 }
 }
}

//

crate ::mod_interface!
{
  exposed use Parser;
  exposed use ParserError;
}
