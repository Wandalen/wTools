//! genfile CLI - Template archive management
//!
//! Command-line interface for `genfile_core` library providing template archive
//! creation, management, and materialization capabilities.

use unilang::pipeline::Pipeline;
use unilang::interpreter::ExecutionContext;

mod commands;
mod handlers;
mod error;
mod repl;

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  let argv : Vec< String > = std::env::args().collect();

  // Create command registry with YAML-based command definitions
  // Command definitions come from commands/*.yaml files
  // Handlers are registered in handlers module
  let registry = commands::create_registry()?;

  // Create pipeline for command processing
  let pipeline = Pipeline::new( registry );

  // If no arguments provided, start REPL mode
  if argv.len() == 1
  {
    return repl::run_repl( &pipeline );
  }

  // Otherwise, process single command in CLI mode
  let ctx = ExecutionContext::default();
  // Workaround(issue-001): ExecutionContext has no state field; handlers use thread-local state.
  // Root cause: unilang::ExecutionContext is a plain default-constructible marker; no user data slot.
  // Pitfall: If unilang adds context state later, update all handler registrations to pass state.

  let result = pipeline.process_command_from_argv( &argv[ 1.. ], ctx );

  if !result.success
  {
    eprintln!( "{}", result.error.unwrap_or_default() );
    // Workaround(issue-004): All errors exit with code 1; error category not exposed by unilang.
    // Root cause: unilang::ErrorData has no error-category field; can't distinguish user vs system errors.
    // Pitfall: When unilang exposes error category, map to: user-input=2, system=3, internal=4.
    std::process::exit( 1 );
  }

  // Print command outputs
  for output in result.outputs
  {
    if !output.content.is_empty()
    {
      println!( "{}", output.content );
    }
  }

  Ok( () )
}
