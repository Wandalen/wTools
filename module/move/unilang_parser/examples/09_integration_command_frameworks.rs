//! Integration with Command Frameworks Example
//!
//! This example demonstrates :
//! - Converting `GenericInstruction` to application-specific structures
//! - Building command dispatch systems
//! - Integration patterns for CLI frameworks
//!
//! Run this example with: `cargo run --example 09_integration_command_frameworks`

use unilang_parser :: { GenericInstruction, Parser, UnilangParserOptions };
use std ::collections ::HashMap;

// Example application command structure
#[ derive( Debug, Clone ) ]
struct AppCommand
{
  name: String,
  args: HashMap< String, String >,
  positional_args: Vec< String >,
  help_requested: bool,
}

// Example command handler trait
trait CommandHandler
{
  fn execute( &self, cmd: &AppCommand ) -> Result< String, String >;
}

// Sample command handlers
struct EchoHandler;
impl CommandHandler for EchoHandler
{
  fn execute( &self, cmd: &AppCommand ) -> Result< String, String >
  {
  if let Some( message ) = cmd.args.get( "message" )
  {
   Ok( format!( "Echo: {message}" ) )
 }
  else if !cmd.positional_args.is_empty()
  {
   Ok( format!( "Echo: {}", cmd.positional_args[ 0 ] ) )
 }
  else
  {
   Err( "No message to echo".to_string() )
 }
 }
}

struct UserHandler;
impl CommandHandler for UserHandler
{
  fn execute( &self, cmd: &AppCommand ) -> Result< String, String >
  {
  match cmd.name.as_str()
  {
   "user.create" =>
   {
  let name = cmd.args.get( "name" ).ok_or( "Missing name" )?;
  let email = cmd.args.get( "email" ).ok_or( "Missing email" )?;
  Ok( format!( "Created user: {name} ({email})" ) )
 }
   "user.list" =>
   {
  let active_only = cmd.args.get( "active" ).unwrap_or( & "false".to_string() ) == "true";
  Ok( format!( "Listing users (active only: {active_only})" ) )
 }
   _ => Err( format!( "Unknown user command: {}", cmd.name ) )
 }
 }
}

// Simple command registry
struct CommandRegistry
{
  handlers: HashMap< String, Box< dyn CommandHandler > >,
}

impl CommandRegistry
{
  fn new() -> Self
  {
  let mut registry = Self
  {
   handlers: HashMap ::new(),
 };

  // Register command handlers
  registry.handlers.insert( "echo".to_string(), Box ::new( EchoHandler ) );
  registry.handlers.insert( "user.create".to_string(), Box ::new( UserHandler ) );
  registry.handlers.insert( "user.list".to_string(), Box ::new( UserHandler ) );

  registry
 }

  fn execute( &self, cmd: &AppCommand ) -> Result< String, String >
  {
  if cmd.help_requested
  {
   return Ok( format!( "Help for command: {}", cmd.name ) );
 }

  if let Some( handler ) = self.handlers.get( &cmd.name )
  {
   handler.execute( cmd )
 }
  else
  {
   Err( format!( "Unknown command: {}", cmd.name ) )
 }
 }
}

// Conversion function from GenericInstruction to AppCommand
fn convert_instruction( instruction: GenericInstruction ) -> AppCommand
{
  AppCommand
  {
  name: instruction.command_path_slices.join( "." ),
  args: instruction.named_arguments.into_iter().map( | ( k, v ) | ( k, v.value ) ).collect(),
  positional_args: instruction.positional_arguments.into_iter().map( | arg | arg.value ).collect(),
  help_requested: instruction.help_requested,
 }
}

fn main() -> Result< (), Box< dyn core ::error ::Error > >
{
  println!( "=== Integration with Command Frameworks ===" );

  let parser = Parser ::new( UnilangParserOptions ::default() );
  let registry = CommandRegistry ::new();

  // Test cases for integration
  let test_commands = ["echo message :: \"Hello, World!\"",
  "echo \"Direct positional message\"",
  "user.create name ::john email ::john@example.com",
  "user.list active ::true",
  "user.create ?",
  "unknown.command test ::value"];

  println!( "Processing commands through the framework: \n" );

  for ( i, cmd_str ) in test_commands.iter().enumerate()
  {
  println!( "{}. Command: '{}'", i + 1, cmd_str );

  match parser.parse_single_instruction( cmd_str )
  {
   Ok( instruction ) =>
   {
  println!( "   Parsed: {:?}", instruction.command_path_slices );

  // Convert to application command
  let app_cmd = convert_instruction( instruction );
  println!( "   App Command: {}", app_cmd.name );

  if !app_cmd.positional_args.is_empty()
  {
   println!( "   Positional: {:?}", app_cmd.positional_args );
 }
  if !app_cmd.args.is_empty()
  {
   println!( "   Named: {:?}", app_cmd.args );
 }
  if app_cmd.help_requested
  {
   println!( "   Help requested: true" );
 }

  // Execute through registry
  match registry.execute( &app_cmd )
  {
   Ok( result ) => println!( "   Result: {result}" ),
   Err( error ) => println!( "   Error: {error}" ),
 }
 }
   Err( parse_error ) =>
   {
  println!( "   Parse Error: {parse_error}" );
 }
 }
  println!();
 }

  // Demonstrate batch processing
  println!( "=== Batch Command Processing ===" );
  let batch_commands = parser.parse_multiple_instructions
  (
  "echo \"Starting batch\" ;; user.create name ::alice email ::alice@test.com ;; user.list active ::true ;; echo \"Batch complete\""
 )?;

  println!( "Processing {} commands in batch: ", batch_commands.len() );
  for ( i, instruction ) in batch_commands.into_iter().enumerate()
  {
  let app_cmd = convert_instruction( instruction );
  match registry.execute( &app_cmd )
  {
   Ok( result ) => println!( "  Step {} : {} -> {}", i + 1, app_cmd.name, result ),
   Err( error ) => println!( "  Step {} : {} -> Error: {}", i + 1, app_cmd.name, error ),
 }
 }

  // Demonstrate advanced integration patterns
  println!( "\n=== Advanced Integration Patterns ===" );

  // Pattern 1 : Command validation before execution
  let validation_cmd = parser.parse_single_instruction( "user.create name :: \"\" email ::invalid-email" )?;
  let app_cmd = convert_instruction( validation_cmd );

  println!( "Validating command before execution: " );
  if app_cmd.args.get( "name" ).is_none_or( std ::string ::String ::is_empty )
  {
  println!( "  Validation failed: Empty name" );
 }
  else if !app_cmd.args.get( "email" ).unwrap_or( &String ::new() ).contains( '@' )
  {
  println!( "  Validation failed: Invalid email format" );
 }
  else
  {
  println!( "  Validation passed" );
 }

  // Pattern 2 : Command aliasing
  println!( "\nCommand aliasing pattern: " );
  let alias_mapping = | cmd_name: &str | -> String
  {
  match cmd_name
  {
   "u.c" => "user.create".to_string(),
   "u.l" => "user.list".to_string(),
   _ => cmd_name.to_string(),
 }
 };

  let aliased_cmd = parser.parse_single_instruction( "u.c name ::bob email ::bob@test.com" )?;
  let mut app_cmd = convert_instruction( aliased_cmd );
  app_cmd.name = alias_mapping( &app_cmd.name );

  println!( "  Aliased 'u.c' to '{}'", app_cmd.name );
  match registry.execute( &app_cmd )
  {
  Ok( result ) => println!( "  Result: {result}" ),
  Err( error ) => println!( "  Error: {error}" ),
 }

  println!( "\n✓ Integration with command frameworks demonstration complete!" );
  Ok( () )
}