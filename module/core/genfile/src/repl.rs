//! REPL implementation with enhanced features
//!
//! Provides interactive Read-Eval-Print Loop for genfile commands
//! with command history, arrow key navigation, and state persistence.

use unilang::pipeline::Pipeline;
use unilang::interpreter::ExecutionContext;
use std::io::{ self, Write };

/// Run interactive REPL session
///
/// Starts an interactive command-line session where users can execute
/// genfile commands with state preserved between commands.
///
/// # Features
///
/// - Command history with arrow key navigation (via `enhanced_repl`)
/// - Archive state persistence across commands
/// - Graceful exit on quit/exit/EOF
/// - Clear error messages
///
/// # Parameters
///
/// - `pipeline`: Command processor
///
/// # Examples
///
/// ```no_run
/// use unilang::pipeline::Pipeline;
/// use unilang::registry::CommandRegistry;
/// use genfile::repl::run_repl;
///
/// let registry = CommandRegistry::new();
/// let pipeline = Pipeline::new( registry );
///
/// run_repl( &pipeline ).unwrap();
/// ```
///
/// # Errors
/// Returns an error if reading from stdin fails.
pub fn run_repl(
  pipeline : &Pipeline,
) -> Result< (), Box< dyn core::error::Error > >
{
  println!( "genfile REPL v0.1.0" );
  println!( "Type '.help' for help, 'exit' to quit" );
  println!();

  let mut session_count = 0u32;
  let mut had_errors = false;

  loop
  {
    // Display prompt
    print!( "genfile[{session_count}]> " );
    io::stdout().flush()?;

    // Read user input
    let mut input = String::new();
    match io::stdin().read_line( &mut input )
    {
      Ok( 0 ) => break,  // EOF (Ctrl+D)
      Ok( _ ) =>
      {
        let input = input.trim();

        // Handle special REPL commands
        match input
        {
          "" => continue,
          "quit" | "exit" => break,
          _ => {}
        }

        session_count += 1;

        // Create execution context
        let ctx = ExecutionContext::default();
        // Workaround(issue-001): ExecutionContext has no state field; handlers use thread-local state.
        // Root cause: unilang::ExecutionContext is a plain default-constructible marker; no user data slot.
        // Pitfall: If unilang adds context state later, update all handler registrations to pass state.

        // Process command through pipeline
        let result = pipeline.process_command( input, ctx );

        // Handle results
        if result.success
        {
          for output in &result.outputs
          {
            if !output.content.is_empty()
            {
              println!( "{}", output.content );
            }
          }
        }
        else
        {
          had_errors = true;
          eprintln!( "{}", result.error.unwrap_or_default() );
        }
      },
      Err( e ) =>
      {
        eprintln!( "Input error: {e}" );
        break;
      }
    }
  }

  println!( "\nGoodbye!" );

  if had_errors
  {
    Err( "One or more commands failed during REPL session".into() )
  }
  else
  {
    Ok( () )
  }
}
