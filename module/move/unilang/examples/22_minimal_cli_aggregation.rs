//! Example 22: Minimal CLI Aggregation with Proper Naming
//!
//! This example demonstrates CLI aggregation with correct naming conventions:
//! - All commands start with dot prefix (`.`)
//! - Multi-word commands use dots as delimiters (`.db.migrate`, not `db migrate`)
//! - Commands are aggregated from multiple modules into a unified interface
//!
//! **Usage:**
//! ```bash
//! cargo run --example 22_minimal_cli_aggregation
//! ```

#![ allow( clippy::needless_pass_by_value ) ]
#![ allow( clippy::unnecessary_wraps ) ]

use unilang::prelude::*;
use unilang::ExecutionContext;

// =============================================================================
// Database Module Commands
// =============================================================================

fn create_db_commands() -> Vec< CommandDefinition >
{
  vec!
  [
    CommandDefinition::former()
      .name( ".migrate" )
      .namespace( ".db" )
      .description( "Run database migrations".to_string() )
      .arguments( vec!
      [
        ArgumentDefinition::former()
          .name( "direction" )
          .description( "Migration direction: up or down".to_string() )
          .kind( Kind::String )
          .attributes( ArgumentAttributes
          {
            optional: true,
            default: Some( "up".to_string() ),
            ..ArgumentAttributes::default()
          })
          .form(),
      ])
      .form(),
    CommandDefinition::former()
      .name( ".backup" )
      .namespace( ".db" )
      .description( "Create database backup".to_string() )
      .arguments( vec!
      [
        ArgumentDefinition::former()
          .name( "path" )
          .description( "Backup file path".to_string() )
          .kind( Kind::String )
          .attributes( ArgumentAttributes
          {
            optional: true,
            default: Some( "./backup.sql".to_string() ),
            ..ArgumentAttributes::default()
          })
          .form(),
      ])
      .form(),
  ]
}

// =============================================================================
// File System Module Commands
// =============================================================================

fn create_fs_commands() -> Vec< CommandDefinition >
{
  vec!
  [
    CommandDefinition::former()
      .name( ".copy" )
      .namespace( ".fs" )
      .description( "Copy files or directories".to_string() )
      .arguments( vec!
      [
        ArgumentDefinition::former()
          .name( "source" )
          .description( "Source path".to_string() )
          .kind( Kind::String )
          .attributes( ArgumentAttributes::default() )
          .form(),
        ArgumentDefinition::former()
          .name( "destination" )
          .description( "Destination path".to_string() )
          .kind( Kind::String )
          .attributes( ArgumentAttributes::default() )
          .form(),
        ArgumentDefinition::former()
          .name( "recursive" )
          .description( "Copy recursively".to_string() )
          .kind( Kind::Boolean )
          .attributes( ArgumentAttributes
          {
            optional: true,
            default: Some( "false".to_string() ),
            ..ArgumentAttributes::default()
          })
          .form(),
      ])
      .form(),
    CommandDefinition::former()
      .name( ".list" )
      .namespace( ".fs" )
      .description( "List directory contents".to_string() )
      .arguments( vec!
      [
        ArgumentDefinition::former()
          .name( "path" )
          .description( "Directory path to list".to_string() )
          .kind( Kind::String )
          .attributes( ArgumentAttributes
          {
            optional: true,
            default: Some( ".".to_string() ),
            ..ArgumentAttributes::default()
          })
          .form(),
      ])
      .form(),
  ]
}

// =============================================================================
// Command Routines
// =============================================================================

fn db_migrate_routine( cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  let direction = cmd.arguments.get( "direction" )
    .and_then( | v | if let Value::String( s ) = v { Some( s.as_str() ) } else { None } )
    .unwrap_or( "up" );

  Ok( OutputData
  {
    content: format!( "Running database migration: {direction}" ),
    format: "text".to_string(),
    execution_time_ms: None,
  })
}

fn db_backup_routine( cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  let path = cmd.arguments.get( "path" )
    .and_then( | v | if let Value::String( s ) = v { Some( s.as_str() ) } else { None } )
    .unwrap_or( "./backup.sql" );

  Ok( OutputData
  {
    content: format!( "Creating database backup at: {path}" ),
    format: "text".to_string(),
    execution_time_ms: None,
  })
}

fn fs_copy_routine( cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  let source = cmd.arguments.get( "source" )
    .and_then( | v | if let Value::String( s ) = v { Some( s.as_str() ) } else { None } )
    .unwrap_or( "" );

  let destination = cmd.arguments.get( "destination" )
    .and_then( | v | if let Value::String( s ) = v { Some( s.as_str() ) } else { None } )
    .unwrap_or( "" );

  let recursive = cmd.arguments.get( "recursive" )
    .and_then( | v | if let Value::Boolean( b ) = v { Some( *b ) } else { None } )
    .unwrap_or( false );

  Ok( OutputData
  {
    content: format!( "Copying {source} -> {destination} (recursive: {recursive})" ),
    format: "text".to_string(),
    execution_time_ms: None,
  })
}

fn fs_list_routine( cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  let path = cmd.arguments.get( "path" )
    .and_then( | v | if let Value::String( s ) = v { Some( s.as_str() ) } else { None } )
    .unwrap_or( "." );

  Ok( OutputData
  {
    content: format!( "Listing directory: {path}" ),
    format: "text".to_string(),
    execution_time_ms: None,
  })
}

// =============================================================================
// Main: CLI Aggregation
// =============================================================================

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "=== Minimal CLI Aggregation Example ===" );
  println!();

  // Create command definitions for each module
  let db_commands = create_db_commands();
  let fs_commands = create_fs_commands();

  // Build registry with module aggregation using proper dot-prefix naming
  #[ allow( deprecated ) ]
  let mut registry = CommandRegistry::new();

  // Register database commands
  for cmd in db_commands
  {
    let routine : CommandRoutine = match cmd.name.as_str()
    {
      ".migrate" => Box::new( db_migrate_routine ),
      ".backup" => Box::new( db_backup_routine ),
      _ => unreachable!(),
    };
    #[ allow( deprecated ) ]
    registry.command_add_runtime( &cmd, routine )?;
  }

  // Register file system commands
  for cmd in fs_commands
  {
    let routine : CommandRoutine = match cmd.name.as_str()
    {
      ".copy" => Box::new( fs_copy_routine ),
      ".list" => Box::new( fs_list_routine ),
      _ => unreachable!(),
    };
    #[ allow( deprecated ) ]
    registry.command_add_runtime( &cmd, routine )?;
  }

  // Create pipeline
  let pipeline = Pipeline::new( registry );

  // Demonstrate aggregated commands with proper dot-prefix naming
  println!( "Testing aggregated commands:" );
  println!();

  // Database commands
  println!( "1. Database migration (default direction):" );
  let result = pipeline.process_command_simple( ".db.migrate" );
  if result.success
  {
    println!( "   ✓ {}", result.outputs[ 0 ].content );
  }
  println!();

  println!( "2. Database migration (down):" );
  let result = pipeline.process_command_simple( ".db.migrate direction::down" );
  if result.success
  {
    println!( "   ✓ {}", result.outputs[ 0 ].content );
  }
  println!();

  println!( "3. Database backup:" );
  let result = pipeline.process_command_simple( ".db.backup path::./my-backup.sql" );
  if result.success
  {
    println!( "   ✓ {}", result.outputs[ 0 ].content );
  }
  println!();

  // File system commands
  println!( "4. File system copy:" );
  let result = pipeline.process_command_simple( ".fs.copy source::./src destination::./dest recursive::true" );
  if result.success
  {
    println!( "   ✓ {}", result.outputs[ 0 ].content );
  }
  println!();

  println!( "5. File system list:" );
  let result = pipeline.process_command_simple( ".fs.list path::/tmp" );
  if result.success
  {
    println!( "   ✓ {}", result.outputs[ 0 ].content );
  }
  println!();

  println!( "=== Key Points ===" );
  println!( "✓ All commands start with dot prefix (.db.migrate, .fs.copy)" );
  println!( "✓ Multi-word commands use dots as delimiters, not spaces" );
  println!( "✓ Commands from different modules are aggregated into single interface" );
  println!( "✓ Namespace isolation prevents naming conflicts" );

  Ok(())
}
