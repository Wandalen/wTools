//! Claude Runner CLI
//!
//! Command-line interface for executing Claude Code with configurable parameters.
//! Parses CLI flags and delegates all process execution to `claude_runner_core::ClaudeCommand`.
//! This crate contains no execution logic — it is a pure argument-parsing front-end.
//!
//! # Usage
//!
//! ```sh
//! claude_runner --message "Fix the bug" --dir /path/to/project
//! claude_runner -m "Explain this" --continue
//! claude_runner --message "test" --dry-run
//! ```

use claude_runner_core::ClaudeCommand;
use error_tools::{ Result, Error };

// CLI argument bag — one field per supported flag
// All bool fields default to false; all Option fields default to None
#[derive( Default )]
struct Args {
  message: Option<String>,
  working_dir: Option<String>,
  continue_conversation: bool,
  max_tokens: Option<u32>,
  skip_permissions: bool,
  dry_run: bool,
  session_dir: Option<String>,
  model: Option<String>,
  help: bool,
}

fn print_help() {
  println!( "claude_runner — Execute Claude Code with configurable parameters" );
  println!();
  println!( "USAGE:" );
  println!( "  claude_runner [OPTIONS] [MESSAGE]" );
  println!();
  println!( "OPTIONS:" );
  println!( "  -m, --message <MSG>        Prompt message for Claude" );
  println!( "  -d, --dir <PATH>           Working directory (default: current dir)" );
  println!( "  -c, --continue             Continue existing conversation" );
  println!( "      --max-tokens <N>       Max output tokens (default: 200000)" );
  println!( "      --skip-permissions     Skip tool permission prompts" );
  println!( "      --dry-run              Print command without executing" );
  println!( "      --session-dir <PATH>   Session storage directory" );
  println!( "      --model <NAME>         Claude model to use" );
  println!( "  -h, --help                 Show this help" );
}

fn parse_args( argv: &[String] ) -> Result<Args> {
  let mut opts = Args::default();
  let mut i = 0;

  while i < argv.len() {
    match argv[ i ].as_str() {
      "-h" | "--help" => { opts.help = true; }
      "-c" | "--continue" => { opts.continue_conversation = true; }
      "--skip-permissions" => { opts.skip_permissions = true; }
      "--dry-run" => { opts.dry_run = true; }
      "-m" | "--message" => {
        i += 1;
        let val = argv.get( i ).ok_or_else( || Error::msg( "--message requires a value" ) )?;
        // Fix(CC3/CC4): guard added so -m never silently overwrites a positional or earlier -m
        // Root cause: `-m` arm ran unconditionally; positional-first order silently lost its value to a later -m flag.
        // Pitfall: whenever a field can be set via multiple parse paths, ALL paths must check if already set.
        if opts.message.is_some() {
          return Err( Error::msg( "--message conflicts with a previously set message (positional or duplicate --message)" ) );
        }
        opts.message = Some( val.clone() );
      }
      "-d" | "--dir" => {
        i += 1;
        let val = argv.get( i ).ok_or_else( || Error::msg( "--dir requires a value" ) )?;
        opts.working_dir = Some( val.clone() );
      }
      "--max-tokens" => {
        i += 1;
        let val = argv.get( i ).ok_or_else( || Error::msg( "--max-tokens requires a value" ) )?;
        opts.max_tokens = Some
        (
          val.parse().map_err( |_| Error::msg( format!( "invalid --max-tokens value: {val}" ) ) )?
        );
      }
      "--session-dir" => {
        i += 1;
        let val = argv.get( i ).ok_or_else( || Error::msg( "--session-dir requires a value" ) )?;
        opts.session_dir = Some( val.clone() );
      }
      "--model" => {
        i += 1;
        let val = argv.get( i ).ok_or_else( || Error::msg( "--model requires a value" ) )?;
        opts.model = Some( val.clone() );
      }
      other => {
        // First positional argument (not starting with '-') becomes the message
        if opts.message.is_none() && !other.starts_with( '-' ) {
          opts.message = Some( other.to_string() );
        } else {
          return Err( Error::msg( format!( "unknown argument: {other}" ) ) );
        }
      }
    }
    i += 1;
  }

  Ok( opts )
}

fn run( args: Args ) -> Result<()> {
  let mut cmd = ClaudeCommand::new();

  if let Some( dir ) = args.working_dir {
    cmd = cmd.with_working_directory( dir );
  }
  if let Some( tokens ) = args.max_tokens {
    cmd = cmd.with_max_output_tokens( tokens );
  }
  if args.continue_conversation {
    cmd = cmd.with_continue_conversation( true );
  }
  if args.skip_permissions {
    cmd = cmd.with_skip_permissions( true );
  }
  if let Some( model ) = args.model {
    cmd = cmd.with_model( model );
  }
  if let Some( session ) = args.session_dir {
    cmd = cmd.with_session_dir( session );
  }
  if let Some( msg ) = args.message {
    cmd = cmd.with_message( msg );
  }

  if args.dry_run {
    let env = cmd.describe_env();
    let command = cmd.describe();
    if !env.is_empty() {
      println!( "{env}" );
    }
    println!( "{command}" );
    return Ok( () );
  }

  let output = cmd.execute()?;
  print!( "{}", output.stdout );
  if !output.stderr.is_empty() {
    eprint!( "{}", output.stderr );
  }
  if output.exit_code != 0 {
    return Err( Error::msg( format!( "Claude exited with code {}", output.exit_code ) ) );
  }
  Ok( () )
}

fn main() {
  let argv: Vec<String> = std::env::args().skip( 1 ).collect();

  let parsed = match parse_args( &argv ) {
    Ok( a ) => a,
    Err( e ) => {
      eprintln!( "Error: {e}" );
      eprintln!( "Run with --help for usage." );
      std::process::exit( 1 );
    }
  };

  if parsed.help {
    print_help();
    return;
  }

  if let Err( e ) = run( parsed ) {
    eprintln!( "Error: {e}" );
    std::process::exit( 1 );
  }
}
