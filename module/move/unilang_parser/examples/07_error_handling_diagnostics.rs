//! Error Handling and Diagnostics Example
//!
//! This example demonstrates :
//! - Different types of parsing errors
//! - Error location information
//! - Comprehensive error handling patterns

use unilang_parser :: { ErrorKind, Parser, UnilangParserOptions };

#[ allow(clippy ::too_many_lines) ]
fn main()
{
  let parser = Parser ::new( UnilangParserOptions ::default() );

  // Test various error scenarios
  println!( "=== Error Handling Examples ===" );

  // Invalid command path (double dots)
  println!( "\n1. Invalid Command Path: " );
  match parser.parse_single_instruction( "invalid..command" )
  {
  Ok( _ ) => println!( "Unexpected success!" ),
  Err( error ) =>
  {
   println!( "Error type: {:?}", error.kind );
   println!
   (
  "Error location: {} to {}",
  error.location.as_ref().map_or( 0, unilang_parser ::SourceLocation ::start ),
  error.location.as_ref().map_or( 0, unilang_parser ::SourceLocation ::end )
 );
   println!( "Error message: {error}" );

   // The specific ErrorKind variants might have changed, so we check for Syntax error with specific message
   if matches!( error.kind, ErrorKind ::Syntax( _ ) )
   {
  println!( "✓ Correctly identified syntax error for invalid command path" );
 }
 }
 }

  // Unterminated quoted string
  println!( "\n2. Unterminated Quoted String: " );
  match parser.parse_single_instruction( r#"cmd arg :: "unterminated string"# )
  {
  Ok( _ ) => println!( "Unexpected success!" ),
  Err( error ) =>
  {
   println!( "Error type: {:?}", error.kind );
   println!
   (
  "Error location: {} to {}",
  error.location.as_ref().map_or( 0, unilang_parser ::SourceLocation ::start ),
  error.location.as_ref().map_or( 0, unilang_parser ::SourceLocation ::end )
 );
   println!( "Error message: {error}" );
 }
 }

  // Invalid escape sequence
  println!( "\n3. Invalid Escape Sequence: " );
  match parser.parse_single_instruction( r#"cmd text :: "invalid \x escape""# )
  {
  Ok( _ ) => println!( "Unexpected success!" ),
  Err( error ) =>
  {
   println!( "Error type: {:?}", error.kind );
   println!
   (
  "Error location: {} to {}",
  error.location.as_ref().map_or( 0, unilang_parser ::SourceLocation ::start ),
  error.location.as_ref().map_or( 0, unilang_parser ::SourceLocation ::end )
 );
   println!( "Error message: {error}" );
 }
 }

  // Empty command path
  println!( "\n4. Empty Command Path: " );
  match parser.parse_single_instruction( "" )
  {
  Ok( _ ) => println!( "Unexpected success!" ),
  Err( error ) =>
  {
   println!( "Error type: {:?}", error.kind );
   println!( "Error message: {error}" );
 }
 }

  // Invalid argument format
  println!( "\n5. Invalid Argument Format: " );
  match parser.parse_single_instruction( "cmd arg :::invalid" )
  {
  Ok( _ ) => println!( "Unexpected success!" ),
  Err( error ) =>
  {
   println!( "Error type: {:?}", error.kind );
   println!
   (
  "Error location: {} to {}",
  error.location.as_ref().map_or( 0, unilang_parser ::SourceLocation ::start ),
  error.location.as_ref().map_or( 0, unilang_parser ::SourceLocation ::end )
 );
   println!( "Error message: {error}" );
 }
 }

  // Helper function to demonstrate error categorization
  fn categorize_error( error: &unilang_parser ::ParseError ) -> &'static str
  {
  match &error.kind
  {
   ErrorKind ::Syntax( _ ) => "General syntax error",
   ErrorKind ::InvalidEscapeSequence( _ ) => "Invalid escape sequence",
   ErrorKind ::EmptyInstructionSegment => "Empty instruction segment",
   ErrorKind ::TrailingDelimiter => "Trailing delimiter",
   ErrorKind ::Unknown => "Unknown error",
 }
 }

  println!( "\n=== Error Categorization Demo ===" );
  let test_cases = ["invalid..path",
  r#"cmd "unterminated"#,
  "cmd arg :::bad",
  ""];

  for ( i, test_case ) in test_cases.iter().enumerate()
  {
  match parser.parse_single_instruction( test_case )
  {
   Ok( _ ) => println!( "Test {} : Unexpected success for '{}'", i + 1, test_case ),
   Err( error ) =>
   {
  println!( "Test {} : {} - {}", i + 1, categorize_error( &error ), error );
 }
 }
 }

  println!( "\n✓ Error handling and diagnostics demonstration complete!" );
}