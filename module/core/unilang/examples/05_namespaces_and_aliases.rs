#![allow(clippy::all)]
//! # Namespaces and Aliases Demo
//!
//! **⚠️ NOTE:** This example uses runtime registration for demonstration purposes.
//! For production use, define namespaces and aliases in YAML and use compile-time generation.
//!
//! This example demonstrates how to organize commands using namespaces
//! and provide aliases for easier command invocation.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::help::HelpGenerator;
use unilang::types::Value;

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Namespaces and Aliases Demo ===\n" );

  let mut registry = CommandRegistry::new();

  // Step 1: Commands in the 'math' namespace

  // .math.add command with aliases
  let add_command = CommandDefinition::former()
  .name( ".add" )
  .namespace( ".math".to_string() )
  .description( "Adds two or more numbers".to_string() )
  .hint( "Mathematical addition" )
  .status( "stable" )
  .version( "1.2.0" )
  .aliases( vec![ ".sum".to_string(), ".plus".to_string(), ".+".to_string() ] )
  .tags( vec![ "arithmetic".to_string(), "basic".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec!
  [
    "math.add numbers::1,2,3".to_string(),
    "sum numbers::10,20".to_string(),
    "+ numbers::5,7".to_string()
  ])
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "numbers".to_string(),
      description: "Numbers to add together".to_string(),
      kind: Kind::List( Box::new( Kind::Integer ), None ),
      hint: "Space-separated integers".to_string(),
      attributes: ArgumentAttributes {
        optional: false,
        multiple: true,
        ..Default::default()
      },
      validation_rules: vec![ ValidationRule::MinItems(2) ],
      aliases: vec![ "nums".to_string(), "values".to_string() ],
      tags: vec![ "required".to_string() ],
    }
  ])
  .end();

  let add_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    if let Some( Value::List( numbers ) ) = cmd.arguments.get( "numbers" )
    {
      let mut sum = 0i64;
      let mut num_strs = Vec::new();

      for num in numbers
      {
        if let Value::Integer( n ) = num
        {
          sum += n;
          num_strs.push( n.to_string() );
        }
      }

      let calculation = format!( "{} = {}", num_strs.join( " + " ), sum );
      println!( "Result: {calculation}" );

      Ok( OutputData
      {
        content : sum.to_string(),
        format : "text".to_string(),
      execution_time_ms : None,
      })
    }
    else
    {
      Ok( OutputData
      {
        content : "0".to_string(),
        format : "text".to_string(),
      execution_time_ms : None,
      })
    }
  });

  registry.command_add_runtime( &add_command, add_routine )?;

  // .math.multiply command
  let multiply_command = CommandDefinition::former()
  .name( ".multiply" )
  .namespace( ".math".to_string() )
  .description( "Multiplies two or more numbers".to_string() )
  .hint( "Mathematical multiplication" )
  .status( "stable" )
  .version( "1.1.0" )
  .aliases( vec![ ".mul".to_string(), ".times".to_string(), ".*".to_string() ] )
  .tags( vec![ "arithmetic".to_string(), "basic".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec!
  [
    "math.multiply 2 3 4".to_string(),
    "mul 5 6".to_string(),
    "* 7 8".to_string()
  ])
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "factors".to_string(),
      description: "Numbers to multiply together".to_string(),
      kind: Kind::List( Box::new( Kind::Integer ), None ),
      hint: "Space-separated integers".to_string(),
      attributes: ArgumentAttributes {
        optional: false,
        multiple: true,
        ..Default::default()
      },
      validation_rules: vec![ ValidationRule::MinItems(2) ],
      aliases: vec![ "nums".to_string() ],
      tags: vec![ "required".to_string() ],
    }
  ])
  .end();

  let multiply_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    if let Some( Value::List( factors ) ) = cmd.arguments.get( "factors" )
    {
      let mut product = 1i64;
      let mut num_strs = Vec::new();

      for num in factors
      {
        if let Value::Integer( n ) = num
        {
          product *= n;
          num_strs.push( n.to_string() );
        }
      }

      let calculation = format!( "{} = {}", num_strs.join( " × " ), product );
      println!( "Result: {calculation}" );

      Ok( OutputData
      {
        content : product.to_string(),
        format : "text".to_string(),
      execution_time_ms : None,
      })
    }
    else
    {
      Ok( OutputData
      {
        content : "1".to_string(),
        format : "text".to_string(),
      execution_time_ms : None,
      })
    }
  });

  registry.command_add_runtime( &multiply_command, multiply_routine )?;

  // Step 2: Commands in the 'text' namespace

  let uppercase_command = CommandDefinition::former()
  .name( ".upper" )
  .namespace( ".text".to_string() )
  .description( "Converts text to uppercase".to_string() )
  .hint( "Text case conversion" )
  .status( "stable" )
  .version( "2.0.0" )
  .aliases( vec![ ".uppercase".to_string(), ".caps".to_string(), ".UP".to_string() ] )
  .tags( vec![ "text-processing".to_string(), "formatting".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec!
  [
    "text.upper 'hello world'".to_string(),
    "uppercase 'convert me'".to_string(),
    "caps test".to_string()
  ])
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "text".to_string(),
      description: "Text to convert to uppercase".to_string(),
      kind: Kind::String,
      hint: "Any text string".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![ ValidationRule::MinLength(1) ],
      aliases: vec![ "input".to_string(), "str".to_string() ],
      tags: vec![ "required".to_string() ],
    }
  ])
  .end();

  let uppercase_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    if let Some( Value::String( text ) ) = cmd.arguments.get( "text" )
    {
      let upper_text = text.to_uppercase();
      println!( "Original: {text}" );
      println!( "Uppercase: {upper_text}" );

      Ok( OutputData
      {
        content : upper_text,
        format : "text".to_string(),
      execution_time_ms : None,
      })
    }
    else
    {
      Ok( OutputData
      {
        content : String::new(),
        format : "text".to_string(),
      execution_time_ms : None,
      })
    }
  });

  registry.command_add_runtime( &uppercase_command, uppercase_routine )?;

  // Step 3: Commands in the 'file' namespace

  let list_command = CommandDefinition::former()
  .name( ".list" )
  .namespace( ".file".to_string() )
  .description( "Lists files in a directory".to_string() )
  .hint( "Directory listing" )
  .status( "beta" )
  .version( "0.8.0" )
  .aliases( vec![ ".ls".to_string(), ".dir".to_string(), ".show".to_string() ] )
  .tags( vec![ "filesystem".to_string(), "utility".to_string() ] )
  .permissions( vec![ "read_directory".to_string() ] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec!
  [
    "file.list /home/user".to_string(),
    "ls .".to_string(),
    "dir /tmp".to_string()
  ])
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "path".to_string(),
      description: "Directory path to list".to_string(),
      kind: Kind::Directory,
      hint: "Valid directory path".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some(".".to_string()),
        ..Default::default()
      },
      validation_rules: vec![],
      aliases: vec![ "dir".to_string(), "directory".to_string() ],
      tags: vec![ "filesystem".to_string() ],
    }
  ])
  .end();

  let list_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let path = match cmd.arguments.get( "path" )
    {
      Some( Value::String( p ) ) => p.clone(),
      _ => ".".to_string(),
    };

    println!( "Listing directory: {path}" );

    match std::fs::read_dir( &path )
    {
      Ok( entries ) =>
      {
        let mut files = Vec::new();
        for entry in entries.flatten()
        {
          if let Some( name ) = entry.file_name().to_str()
          {
            files.push( name.to_string() );
            println!( "  {name}" );
          }
        }

        Ok( OutputData
        {
          content : files.join( "\n" ),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      },
      Err( e ) =>
      {
        use unilang::data::ErrorCode;
        let error_msg = format!( "Failed to list directory '{path}': {e}" );
        Err( unilang::data::ErrorData::new(
          ErrorCode::InternalError,
          error_msg,
        ))
      }
    }
  });

  registry.command_add_runtime( &list_command, list_routine )?;

  println!( "✓ Registered commands in multiple namespaces with aliases" );

  // Step 4: Demonstrate help generation with namespaces
  let help_generator = HelpGenerator::new( &registry );

  println!( "\n=== Registered Commands by Namespace ===" );
  println!( "{}", help_generator.list_commands() );

  println!( "\n=== Namespace Organization ===" );
  println!( "Commands are organized into logical namespaces:" );
  println!( "  • .math.*     - Mathematical operations" );
  println!( "  • .text.*     - Text processing utilities" );
  println!( "  • .file.*     - File system operations" );
  println!( "  • (global)    - Commands without namespace prefix" );

  println!( "\n=== Alias System ===" );
  println!( "Commands can have multiple aliases for convenience:" );
  println!( "  • .math.add     → sum, plus, +" );
  println!( "  • .math.multiply → mul, times, *" );
  println!( "  • .text.upper   → uppercase, caps, UP" );
  println!( "  • .file.list    → ls, dir, show" );

  println!( "\n=== Usage Examples ===" );
  println!( "# Using full namespace:" );
  println!( "cargo run --bin unilang_cli math.add numbers::1,2,3,4" );
  println!( "cargo run --bin unilang_cli text.upper text::'hello world'" );
  println!( "cargo run --bin unilang_cli file.list path::/tmp" );

  println!( "\n# Using aliases:" );
  println!( "cargo run --bin unilang_cli sum numbers::10,20,30" );
  println!( "cargo run --bin unilang_cli + numbers::5,7" );
  println!( "cargo run --bin unilang_cli caps text::'make me big'" );
  println!( "cargo run --bin unilang_cli ls path::." );

  println!( "\n# Getting help for specific commands:" );
  println!( "cargo run --bin unilang_cli help math.add" );
  println!( "cargo run --bin unilang_cli help sum" );
  println!( "cargo run --bin unilang_cli help text.upper" );

  Ok(())
}