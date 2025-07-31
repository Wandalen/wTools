use unilang::registry::CommandRegistry;
use unilang::data::{ CommandDefinition, ArgumentDefinition, Kind, OutputData, ErrorData, ArgumentAttributes };
use unilang::types::Value;
use unilang::semantic::VerifiedCommand;
use unilang::interpreter::ExecutionContext;

pub fn test_basic_usage()
{
  // Create registry
  let mut registry = CommandRegistry::new();
  
  // Define command
  let greet_cmd = CommandDefinition::former()
    .name( "greet" )
    .namespace( "".to_string() )
    .description( "Greets a person".to_string() )
    .hint( "Simple greeting" )
    .status( "stable" )
    .version( "1.0.0" )
    .aliases( vec![] )
    .tags( vec![] )
    .permissions( vec![] )
    .idempotent( true )
    .deprecation_message( "".to_string() )
    .http_method_hint( "GET".to_string() )
    .examples( vec![ "greet name::\"Alice\"".to_string() ] )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "name" )
        .kind( Kind::String )
        .hint( "Person to greet" )
        .description( "Name of person to greet".to_string() )
        .attributes( ArgumentAttributes::default() )
        .validation_rules( vec![] )
        .aliases( vec![] )
        .tags( vec![] )
        .end()
    ])
    .end();
  
  // Define routine
  let routine = Box::new( | cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, ErrorData >
  {
    let name = cmd.arguments.get( "name" )
      .and_then( | v | if let Value::String( s ) = v { Some( s.clone() ) } else { None } )
      .unwrap_or_else( || "World".to_string() );
    println!( "Hello, {}!", name );
    Ok( OutputData
    {
      content : format!( "Hello, {}!", name ),
      format : "text".to_string(),
    })
  });
  
  // Register command
  registry.command_add_runtime( &greet_cmd, routine ).expect( "Failed to register command" );
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  
  #[ test ]
  fn it_works()
  {
    test_basic_usage();
  }
}