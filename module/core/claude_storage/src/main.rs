//! Claude Code Storage CLI
//!
//! Interactive REPL and command-line interface for exploring Claude Code's
//! conversation storage.
//!
//! ## Command Set
//!
//! The CLI provides 7 core commands:
//! - `.status` - Show storage statistics
//! - `.list` - List projects or sessions
//! - `.show` - Display session details
//! - `.show.project` - Display project details and all sessions
//! - `.count` - Fast counting operations
//! - `.search` - Search session content
//! - `.export` - Export sessions to file
//!
//! ## Modes
//!
//! - **REPL mode**: No arguments - starts interactive shell
//! - **One-shot mode**: Arguments provided - executes single command and exits
//!
//! ## Performance
//!
//! Uses static PHF maps for O(1) command lookup (~80ns vs ~4,000ns for `HashMap`).

// allow: binary entry point; generated code (static_commands.rs) lacks doc comments on pub static
#![ allow( missing_docs ) ]

use std::{ env, io::{ self, Write }, process };
use claude_storage::cli;
use unilang::prelude::*;
use unilang::phf;

// Include compile-time generated static commands
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main()
{
  // Build command registry with routines
  let registry = build_command_registry();

  // Get arguments
  let args : Vec< String > = env::args().collect();

  if args.len() == 1
  {
    // No arguments: enter REPL mode
    run_repl( registry );
  }
  else
  {
    // Arguments provided: run one-shot command
    execute_oneshot( registry, args );
  }
}

/// Build command registry with all `claude_storage` routines
fn build_command_registry() -> CommandRegistry
{
  type RoutineFn = fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData >;

  // Map command names to routine functions
  let routines : phf::Map< &'static str, RoutineFn > = phf::phf_map!
  {
    ".status" => cli::status_routine,
    ".list" => cli::list_routine,
    ".show" => cli::show_routine,
    ".show.project" => cli::show_project_routine,
    ".count" => cli::count_routine,
    ".search" => cli::search_routine,
    ".export" => cli::export_routine,
  };

  // Create registry
  #[ allow( deprecated ) ]
  let mut registry = CommandRegistry::new();

  // Register all routines
  for ( name, static_cmd ) in AGGREGATED_COMMANDS.entries()
  {
    if let Some( &routine ) = routines.get( *name )
    {
      let cmd : CommandDefinition = ( *static_cmd ).into();

      #[ allow( deprecated ) ]
      if let Err( e ) = registry.command_add_runtime( &cmd, Box::new( routine ) )
      {
        eprintln!( "WARNING: Failed to register routine for {name}: {e}" );
      }
    }
  }

  registry
}

/// Run REPL (Read-Eval-Print Loop) mode
fn run_repl( registry : CommandRegistry )
{
  println!( "Claude Code Storage CLI" );
  println!( "Type 'help' for available commands, 'exit' to quit.\n" );

  let pipeline = Pipeline::new( registry );
  let mut command_buffer = String::new();

  loop
  {
    // Print prompt
    print!( "> " );
    io::stdout().flush().unwrap();

    // Read input
    command_buffer.clear();
    if let Err( e ) = io::stdin().read_line( &mut command_buffer )
    {
      eprintln!( "Error reading input: {e}" );
      continue;
    }

    let input = command_buffer.trim();

    // Handle empty input
    if input.is_empty()
    {
      continue;
    }

    // Handle exit commands
    if input == "exit" || input == "quit" || input == "q"
    {
      println!( "Goodbye!" );
      break;
    }

    // Execute command
    let result = pipeline.process_command_simple( input );

    if result.success
    {
      if let Some( output ) = result.outputs.first()
      {
        println!( "{}", output.content );
      }
    }
    else if let Some( error ) = result.error
    {
      eprintln!( "Error: {error}" );
    }
  }
}

/// Run one-shot command mode
#[allow(clippy::needless_pass_by_value)]
fn execute_oneshot( registry : CommandRegistry, args : Vec< String > ) -> !
{
  let pipeline = Pipeline::new( registry );

  // Join args into command line (skip program name)
  let command_line = args[ 1.. ].join( " " );

  let result = pipeline.process_command_simple( &command_line );

  if result.success
  {
    if let Some( output ) = result.outputs.first()
    {
      println!( "{}", output.content );
    }
    process::exit( 0 );
  }
  else
  {
    if let Some( error ) = result.error
    {
      eprintln!( "Error: {error}" );
    }
    process::exit( 1 );
  }
}
