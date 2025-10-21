//! genfile CLI - Template archive management
//!
//! Command-line interface for genfile_core library providing template archive
//! creation, management, and materialization capabilities.

use unilang::pipeline::Pipeline;
use unilang::interpreter::ExecutionContext;

mod commands;
mod handlers;
mod state;
mod error;
mod repl;

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  let argv : Vec< String > = std::env::args().collect();

  // Create command registry with all genfile commands
  let registry = commands::create_registry()?;

  // Create pipeline for command processing
  let pipeline = Pipeline::new( registry );

  // Create archive state for REPL mode
  let state = state::ArchiveState::new();

  // If no arguments provided, start REPL mode
  if argv.len() == 1
  {
    return repl::run_repl( &pipeline, state );
  }

  // Otherwise, process single command in CLI mode
  let ctx = ExecutionContext::default();
  // TODO: Pass state through ExecutionContext when API supports it
  // For now, handlers will use thread-local or global state

  let result = pipeline.process_command_from_argv( &argv[ 1.. ], ctx );

  if !result.success
  {
    eprintln!( "{}", result.error.unwrap_or_default() );
    // TODO: Map error types to exit codes when API supports it
    std::process::exit( 1 );
  }

  Ok( () )
}
