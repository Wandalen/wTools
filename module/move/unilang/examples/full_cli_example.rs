//! This example demonstrates a comprehensive usage of the `unilang` framework,
//! showcasing command definitions with various features like namespaces, aliases,
//! argument kinds, and default values. It sets up a full CLI application
//! pipeline, including argument parsing, semantic analysis, and command execution.

use std::collections::HashMap;
use unilang::data::{ CommandDefinition, ArgumentDefinition, ArgumentAttributes, OutputData };
use unilang::data::Kind as ArgumentKind;
use unilang::registry::{ CommandRegistry, CommandRoutine };
use unilang::semantic::SemanticAnalyzer;
use unilang::interpreter::Interpreter;
use unilang::error::Error;
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

fn main()
->
Result< (), Error >
{
  // 1. Initialize Command Registry
  let mut registry = CommandRegistry::new();

  // 2. Define and Register Commands with Routines

  // .math.add command
  let math_add_def = CommandDefinition::former()
  .name( "add" )
  .namespace( ".math" )
  .hint( "Adds two numbers." )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec! [ "sum".to_string(), "plus".to_string() ] )
  .arguments
  (
    vec!
    [
      ArgumentDefinition::former()
      .name( "a" )
      .kind( ArgumentKind::Integer )
      .hint( "First number." )
      .end(),
      ArgumentDefinition::former()
      .name( "b" )
      .kind( ArgumentKind::Integer )
      .hint( "Second number." )
      .end(),
    ]
  )
  .end();

  let math_add_routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let a = cmd.arguments.get( "a" ).unwrap();
    let b = cmd.arguments.get( "b" ).unwrap();
    if let ( Value::Integer( val_a ), Value::Integer( val_b ) ) = ( a, b )
    {
      let result = val_a + val_b;
      println!( "Result: {}", result );
      return Ok( OutputData
      {
        content : result.to_string(),
        format : "text".to_string(),
      });
    }
    unreachable!();
  });
  registry.command_add_runtime( &math_add_def, math_add_routine )?;

  // .math.sub command
  let math_sub_def = CommandDefinition::former()
  .name( "sub" )
  .namespace( ".math" )
  .hint( "Subtracts two numbers." )
  .status( "beta" )
  .version( "0.9.0" )
  .aliases( vec! [ "minus".to_string() ] )
  .arguments
  (
    vec!
    [
      ArgumentDefinition::former()
      .name( "x" )
      .kind( ArgumentKind::Integer )
      .hint( "Minuend." )
      .end(),
      ArgumentDefinition::former()
      .name( "y" )
      .kind( ArgumentKind::Integer )
      .hint( "Subtrahend." )
      .end(),
    ]
  )
  .end();

  let math_sub_routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let x = cmd.arguments.get( "x" ).unwrap();
    let y = cmd.arguments.get( "y" ).unwrap();
    if let ( Value::Integer( val_x ), Value::Integer( val_y ) ) = ( x, y )
    {
      let result = val_x - val_y;
      println!( "Result: {}", result );
      return Ok( OutputData
      {
        content : result.to_string(),
        format : "text".to_string(),
      });
    }
    unreachable!();
  });
  registry.command_add_runtime( &math_sub_def, math_sub_routine )?;

  // .greet command
  let greet_def = CommandDefinition::former()
  .name( "greet" )
  .namespace( "" ) // Global command
  .hint( "Greets the specified person." )
  .status( "stable" )
  .version( "1.0.0" )
  .arguments
  (
    vec!
    [
      ArgumentDefinition::former()
      .name( "name" )
      .kind( ArgumentKind::String )
      .hint( "Name of the person to greet." )
      .default_value( "World".to_string() )
      .end(),
    ]
  )
  .end();

  let greet_routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let name = cmd
    .arguments
    .get( "name" )
    .map( | v | v.to_string() )
    .unwrap_or_else( || "World".to_string() );
    let result = format!( "Hello, {}!", name );
    println!( "{}", result );
    Ok( OutputData
    {
      content : result,
      format : "text".to_string(),
    })
  });
  registry.command_add_runtime( &greet_def, greet_routine )?;

  // .config.set command
  let config_set_def = CommandDefinition::former()
  .name( "set" )
  .namespace( ".config" )
  .hint( "Sets a configuration value." )
  .status( "experimental" )
  .version( "0.1.0" )
  .arguments
  (
    vec!
    [
      ArgumentDefinition::former()
      .name( "key" )
      .kind( ArgumentKind::String )
      .hint( "Configuration key." )
      .end(),
      ArgumentDefinition::former()
      .name( "value" )
      .kind( ArgumentKind::String )
      .hint( "Configuration value." )
      .attributes( ArgumentAttributes::former().interactive( true ).sensitive( true ).end() )
      .end(),
    ]
  )
  .end();

  let config_set_routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let key = cmd.arguments.get( "key" ).unwrap();
    let value = cmd.arguments.get( "value" ).unwrap();
    let result = format!( "Setting config: {} = {}", key, value );
    println!( "{}", result );
    Ok( OutputData
    {
      content : result,
      format : "text".to_string(),
    })
  });
  registry.command_add_runtime( &config_set_def, config_set_routine )?;
  let args : Vec< String > = std::env::args().skip( 1 ).collect();

  // 3. Parse Command Line Arguments
  // Handle 'help' command manually
  if args.first().map_or( false, | arg | arg == "help" )
  {
    let help_generator = unilang::help::HelpGenerator::new( &registry );
    if let Some( command_name ) = args.get( 1 )
    {
      if let Some( help_text ) = help_generator.command( command_name )
      {
        println!( "{}", help_text );
      }
      else
      {
        println!( "Command '{}' not found.", command_name );
      }
    }
    else
    {
      println!( "{}", help_generator.list_commands() );
    }
    return Ok( () );
  }

  let parser = Parser::new( UnilangParserOptions::default() );

  // Build alias map for CLI resolution
  let mut alias_map : HashMap< String, String > = HashMap::new();
  for cmd_def in registry.commands.values()
  {
    for alias in &cmd_def.aliases
    {
      alias_map.insert( alias.clone(), cmd_def.name.clone() );
    }
  }

  let mut processed_args = args.clone();
  if let Some( first_arg ) = processed_args.first_mut()
  {
    if let Some( canonical_name ) = alias_map.get( first_arg )
    {
      *first_arg = canonical_name.clone();
    }
  }

  let input_str = processed_args.join( " " );
  let instructions = vec! [ parser.parse_single_instruction( &input_str )? ];

  // 4. Semantic Analysis
  let semantic_analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let commands = semantic_analyzer.analyze()?;

  // 5. Interpret and Execute
  let interpreter = Interpreter::new( &commands, &registry );
  let mut context = unilang::interpreter::ExecutionContext::default();
  interpreter.run( &mut context )?;

  Ok( () )
}
