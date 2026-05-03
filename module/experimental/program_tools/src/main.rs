//! CLI entry point: `program_tools run [OPTIONS] <TARGET>`.

use clap::{ Parser, Subcommand };
use std::path::PathBuf;
use program_tools::{ run, run_project, RunOptions };
use program_tools::program::Plan;

#[ derive( Parser ) ]
#[ command( name = env!( "CARGO_BIN_NAME" ), about = "Run Rust files and projects as scripts." ) ]
struct Cli
{
  #[ command( subcommand ) ]
  command : Commands,
}

#[ derive( Subcommand ) ]
enum Commands
{
  /// Execute a Rust source file or Cargo project directory.
  Run( RunArgs ),
}

#[ derive( clap::Args ) ]
struct RunArgs
{
  /// Path to a Rust source file or a Cargo project directory.
  target : PathBuf,

  /// Cargo build profile (`debug` or `release`).
  #[ arg( long, default_value = "" ) ]
  profile : String,

  /// Persistent artifact cache directory.
  #[ arg( long ) ]
  target_dir : Option< String >,

  /// Path to the Cargo binary.
  #[ arg( long = "cargo", default_value = "" ) ]
  cargo_bin : String,

  /// Maximum execution duration in milliseconds.
  #[ arg( long ) ]
  timeout : Option< u64 >,

  /// Enable a Cargo feature (repeatable).
  #[ arg( long = "feature" ) ]
  features : Vec< String >,

  /// Set a subprocess environment variable as `KEY=VALUE` (repeatable).
  #[ arg( long = "env" ) ]
  env_vars : Vec< String >,

  /// Rust edition for generated manifests.
  #[ arg( long, default_value = "" ) ]
  edition : String,

  /// Package name for generated manifests.
  #[ arg( long, default_value = "" ) ]
  name : String,

  /// Capture stdout/stderr as a structured summary instead of forwarding to the terminal.
  #[ arg( long ) ]
  capture : bool,

  /// Retain the temporary workspace directory after the run.
  #[ arg( long ) ]
  keep : bool,
}

fn main()
{
  let cli = Cli::parse();
  let Commands::Run( args ) = cli.command;

  let opts = RunOptions
  {
    build_profile : args.profile,
    target_dir : args.target_dir,
    cargo_path : args.cargo_bin,
    timeout_ms : args.timeout,
    features : args.features,
    env_vars : args.env_vars,
    edition : args.edition,
    package_name : args.name,
    capture : args.capture,
    cleanup : !args.keep,
  };

  let result = if args.target.is_dir()
  {
    run_project( &args.target, &opts )
  }
  else
  {
    let code = match std::fs::read_to_string( &args.target )
    {
      Ok( s ) => s,
      Err( e ) =>
      {
        eprintln!( "error: failed to read '{}': {e}", args.target.display() );
        std::process::exit( 1 );
      }
    };
    let plan = Plan::former()
      .program()
        .source()
          .file_path( "src/main.rs".to_string() )
          .data( code )
          .end()
        .end()
      .run_options( opts )
      .form();
    run( plan )
  };

  match result
  {
    Ok( output ) =>
    {
      if args.capture
      {
        print!( "{}", output.stdout_str() );
        eprint!( "{}", output.stderr_str() );
      }
      std::process::exit( output.exit_status );
    }
    Err( e ) =>
    {
      eprintln!( "error: {e}" );
      std::process::exit( 1 );
    }
  }
}
