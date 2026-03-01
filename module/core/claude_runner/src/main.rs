//! Claude Runner CLI
//!
//! Command-line interface for executing Claude Code with configurable parameters.
//! Accepts Claude Code's `--flag value` syntax; an adapter layer converts it to
//! unilang's `key::value` format for command routing and validation.
//!
//! # Architecture
//!
//! ```text
//! User argv (--flag value)
//!     ↓ argv_to_unilang_tokens()
//! Unilang tokens (.run message::... dir::... dry::1)
//!     ↓ Parser::parse_from_argv()
//! GenericInstruction
//!     ↓ SemanticAnalyzer::analyze()
//! VerifiedCommand
//!     ↓ Interpreter::run()
//! ClaudeCommand → execute
//! ```
//!
//! # Usage
//!
//! ```sh
//! claude_runner "Fix the bug" --dir /path/to/project
//! claude_runner -m "Explain this" --continue
//! claude_runner --message "test" --dry-run
//! claude_runner --help
//! ```

use claude_runner_core::ClaudeCommand;
use error_tools::{ Error, Result };
use unilang::data::{ ArgumentDefinition, CommandDefinition, ErrorCode, ErrorData, Kind, OutputData };
use unilang::interpreter::{ ExecutionContext, Interpreter };
use unilang::parser::{ Parser, UnilangParserOptions };
use unilang::registry::{ CommandRegistry, CommandRoutine };
use unilang::semantic::SemanticAnalyzer;
use unilang::types::Value;

fn print_help()
{
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

/// Adapter: convert Claude-style argv (`--flag value`) to unilang token vec.
///
/// User types:  `claude_runner "Fix bug" --dir /path --dry-run`
/// Returns:     `[".run", "message::Fix bug", "dir::/path", "dry::1"]`
///
/// Parses sequentially — `--help` sets a flag but does NOT suppress later errors.
/// Duplicate flags (except `--message`) use last-wins semantics silently.
/// The token vec is passed to `Parser::parse_from_argv`, which preserves each
/// element as a complete token so spaces within values are not re-split.
fn argv_to_unilang_tokens( argv : &[ String ] ) -> Result< Vec< String > >
{
  let mut help             = false;
  let mut message          : Option< String > = None;
  let mut dir              : Option< String > = None;
  let mut max_tokens       : Option< u32 >    = None;
  let mut session_dir      : Option< String > = None;
  let mut model            : Option< String > = None;
  let mut do_continue      = false;
  let mut skip_permissions = false;
  let mut dry              = false;

  let mut i = 0;
  while i < argv.len()
  {
    match argv[ i ].as_str()
    {
      "-h" | "--help"         => { help = true; }
      "-c" | "--continue"     => { do_continue      = true; }
      "--skip-permissions"    => { skip_permissions = true; }
      "--dry-run"             => { dry              = true; }
      "-m" | "--message"      =>
      {
        i += 1;
        let val = argv.get( i ).ok_or_else( || Error::msg( "--message requires a value" ) )?;
        // Fix(CC3/CC4): guard so -m never silently overwrites a positional or earlier -m.
        // Root cause: parse arm ran unconditionally; positional-first order silently lost value.
        // Pitfall: whenever a field can be set via multiple parse paths, ALL paths must check if set.
        if message.is_some()
        {
          return Err( Error::msg( "--message conflicts with a previously set message (positional or duplicate --message)" ) );
        }
        message = Some( val.clone() );
      }
      "-d" | "--dir"          =>
      {
        i += 1;
        let val = argv.get( i ).ok_or_else( || Error::msg( "--dir requires a value" ) )?;
        dir = Some( val.clone() ); // last wins
      }
      "--max-tokens"          =>
      {
        i += 1;
        let val = argv.get( i ).ok_or_else( || Error::msg( "--max-tokens requires a value" ) )?;
        let n = val.parse::< u32 >().map_err( |_| Error::msg( format!( "invalid --max-tokens value: {val}" ) ) )?;
        max_tokens = Some( n ); // last wins
      }
      "--session-dir"         =>
      {
        i += 1;
        let val = argv.get( i ).ok_or_else( || Error::msg( "--session-dir requires a value" ) )?;
        session_dir = Some( val.clone() ); // last wins
      }
      "--model"               =>
      {
        i += 1;
        let val = argv.get( i ).ok_or_else( || Error::msg( "--model requires a value" ) )?;
        model = Some( val.clone() ); // last wins
      }
      other                   =>
      {
        if !other.starts_with( '-' ) && message.is_none()
        {
          message = Some( other.to_string() );
        }
        else
        {
          return Err( Error::msg( format!( "unknown argument: {other}" ) ) );
        }
      }
    }
    i += 1;
  }

  // Route to .help only after full sequential parse succeeds with no errors.
  // This ensures unknown flags still error even when --help is present (D3).
  if help { return Ok( vec![ ".help".to_string() ] ); }

  let mut tokens = vec![ ".run".to_string() ];
  if let Some( msg ) = message   { tokens.insert( 1, format!( "message::{msg}" ) ); }
  if let Some( d ) = dir         { tokens.push( format!( "dir::{d}" ) ); }
  if let Some( n ) = max_tokens  { tokens.push( format!( "max_tokens::{n}" ) ); }
  if let Some( s ) = session_dir { tokens.push( format!( "session_dir::{s}" ) ); }
  if let Some( m ) = model       { tokens.push( format!( "model::{m}" ) ); }
  if do_continue                 { tokens.push( "continue::1".to_string() ); }
  if skip_permissions            { tokens.push( "skip_permissions::1".to_string() ); }
  if dry                         { tokens.push( "dry::1".to_string() ); }

  Ok( tokens )
}

/// Build command registry with the `.run` command and its handler.
///
/// Panics only on programmer error (malformed `CommandDefinition`), never on user input.
fn build_registry() -> CommandRegistry
{
  let mut registry = CommandRegistry::new();

  let run_def = CommandDefinition::former()
  .name( ".run" )
  .description( "Execute Claude Code with configurable parameters" )
  .arguments( vec!
  [
    ArgumentDefinition::new( "message",          Kind::String  ).with_optional( None::< String > ),
    ArgumentDefinition::new( "dir",              Kind::String  ).with_optional( None::< String > ),
    ArgumentDefinition::new( "continue",         Kind::Boolean ).with_optional( None::< String > ),
    ArgumentDefinition::new( "max_tokens",       Kind::Integer ).with_optional( None::< String > ),
    ArgumentDefinition::new( "skip_permissions", Kind::Boolean ).with_optional( None::< String > ),
    ArgumentDefinition::new( "dry",              Kind::Boolean ).with_optional( None::< String > ),
    ArgumentDefinition::new( "session_dir",      Kind::String  ).with_optional( None::< String > ),
    ArgumentDefinition::new( "model",            Kind::String  ).with_optional( None::< String > ),
  ])
  .end();

  let run_routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let mut builder = ClaudeCommand::new();

    if let Some( Value::String( s ) ) = cmd.arguments.get( "dir" )
    {
      builder = builder.with_working_directory( s.clone() );
    }
    if let Some( Value::Integer( n ) ) = cmd.arguments.get( "max_tokens" )
    {
      // Adapter validated max_tokens as u32; unilang stores it as i64; convert back.
      builder = builder.with_max_output_tokens( u32::try_from( *n ).unwrap_or( 0 ) );
    }
    if matches!( cmd.arguments.get( "continue" ), Some( Value::Boolean( true ) ) )
    {
      builder = builder.with_continue_conversation( true );
    }
    if matches!( cmd.arguments.get( "skip_permissions" ), Some( Value::Boolean( true ) ) )
    {
      builder = builder.with_skip_permissions( true );
    }
    if let Some( Value::String( s ) ) = cmd.arguments.get( "model" )
    {
      builder = builder.with_model( s.clone() );
    }
    if let Some( Value::String( s ) ) = cmd.arguments.get( "session_dir" )
    {
      builder = builder.with_session_dir( s.clone() );
    }
    if let Some( Value::String( s ) ) = cmd.arguments.get( "message" )
    {
      builder = builder.with_message( s.clone() );
    }

    if matches!( cmd.arguments.get( "dry" ), Some( Value::Boolean( true ) ) )
    {
      let env     = builder.describe_env();
      let command = builder.describe();
      let mut out = String::new();
      if !env.is_empty() { out.push_str( &env ); out.push( '\n' ); }
      out.push_str( &command );
      return Ok( OutputData { content : out, format : "text".to_string(), execution_time_ms : None } );
    }

    let output = builder.execute().map_err( | e |
      ErrorData::new( ErrorCode::InternalError, format!( "{e}" ) )
    )?;

    if !output.stderr.is_empty() { eprint!( "{}", output.stderr ); }

    if output.exit_code != 0
    {
      return Err( ErrorData::new(
        ErrorCode::InternalError,
        format!( "Claude exited with code {}", output.exit_code ),
      ));
    }

    Ok( OutputData { content : output.stdout, format : "text".to_string(), execution_time_ms : None } )
  });

  registry
  .command_add_runtime( &run_def, run_routine )
  .expect( "internal error: failed to register .run command" );

  registry
}

fn main()
{
  let argv : Vec< String > = std::env::args().skip( 1 ).collect();

  // Phase 1: adapter — convert --flag value argv to unilang token vec
  let tokens = match argv_to_unilang_tokens( &argv )
  {
    Ok( t )  => t,
    Err( e ) =>
    {
      eprintln!( "Error: {e}" );
      eprintln!( "Run with --help for usage." );
      std::process::exit( 1 );
    }
  };

  let registry = build_registry();

  // Phase 2: help — print Claude-style usage before entering unilang pipeline
  if tokens.first().map( String::as_str ) == Some( ".help" )
  {
    print_help();
    return;
  }

  // Phase 3: parse — convert token vec to GenericInstruction
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = match parser.parse_from_argv( &tokens )
  {
    Ok( i )  => i,
    Err( e ) =>
    {
      eprintln!( "Error: {e}" );
      eprintln!( "Run with --help for usage." );
      std::process::exit( 1 );
    }
  };

  // Phase 4: semantic analysis — validate instruction against command definitions
  let instructions = [ instruction ];
  let analyzer     = SemanticAnalyzer::new( &instructions, &registry );
  let commands = match analyzer.analyze()
  {
    Ok( cmds ) => cmds,
    Err( e )   =>
    {
      eprintln!( "Error: {e}" );
      std::process::exit( 1 );
    }
  };

  // Phase 5: execute — run command routine
  let interpreter  = Interpreter::new( &commands, &registry );
  let mut context  = ExecutionContext::default();
  match interpreter.run( &mut context )
  {
    Ok( outputs ) =>
    {
      for output in outputs { print!( "{}", output.content ); }
    }
    Err( e ) =>
    {
      eprintln!( "Error: {e}" );
      std::process::exit( 1 );
    }
  }
}
