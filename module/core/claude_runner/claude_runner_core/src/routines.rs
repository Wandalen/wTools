//! AI command routines for willbe binaries (Claude Code integration).
//!
//! Provides the `.claude` and `.claude.help` command implementations
//! as a reusable library. Any willbe binary with a unilang command registry can register
//! these commands by pointing at [`super::COMMANDS_YAML`].
//!
//! Provides adapter layer between `unilang`'s validated commands and `dream_agent`.
//! All AI routines delegate to `dream_agent::execute_claude()` after converting
//! `unilang`'s `VerifiedCommand` to `Vec<String>` format.
//!
//! # AI Commands
//!
//! - `.claude` - Direct AI assistance with Claude Code
//! - `.claude.help` - Show help for .claude command
//!
//! # Architecture
//!
//! `claude_runner` (AI commands) → `dream_agent` (session management) → Claude Code
//! `wplan` (base commands) → `wplan_client` → `wplan_daemon` (NO AI)

use unilang::{ VerifiedCommand, ExecutionContext, OutputData, ErrorData, ErrorCode };

/// Common delegation helper: converts `dream_agent` result to `unilang` result.
fn delegate_to_agent( args : &[ String ] ) -> Result< OutputData, ErrorData >
{
  match dream_agent::execute_claude( args )
  {
    Ok( output ) => Ok( OutputData::new( output, "text" ) ),
    Err( err ) => Err( ErrorData::new( ErrorCode::InternalError, err.to_string() ) ),
  }
}

/// Helper: add non-empty optional string parameter to args
fn add_optional_string( args : &mut Vec< String >, name : &str, value : Option< &str > )
{
  if let Some( val ) = value
  {
    if !val.is_empty()
    {
      args.push( format!( "{name}::{val}" ) );
    }
  }
}

/// Helper: add boolean parameter to args
fn add_boolean( args : &mut Vec< String >, name : &str, value : bool )
{
  if value
  {
    args.push( format!( "{name}::1" ) );
  }
  else
  {
    args.push( format!( "{name}::0" ) );
  }
}

/// Routine: AI-assisted development with Claude Code
///
/// Main entry point for AI assistance. Delegates to `dream_agent` which manages
/// Claude Code sessions with context injection from `wplan` daemon.
///
/// # Errors
///
/// Returns `ErrorData` if `dream_agent::execute_claude` fails (e.g., daemon not running,
/// Claude Code not installed, session directory inaccessible).
#[ inline ]
#[ allow( clippy::too_many_lines ) ]
#[ allow( clippy::needless_pass_by_value ) ] // Required: unilang routine signature takes owned VerifiedCommand
pub fn claude_routine(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext,
) -> Result< OutputData, ErrorData >
{
  // FR-14: Resolve @file references in command value
  let message_from_unilang : Option< String > = match cmd.get_string( "command" )
  {
    Some( msg ) if msg.starts_with( '@' ) =>
    {
      Some( wplan_core::resolve_file_reference( msg ).map_err( | e | ErrorData::new(
        ErrorCode::ValidationRuleFailed,
        format!( "Invalid command file reference: {e}" ),
      ) )? )
    }
    Some( msg ) => Some( msg.to_string() ),
    None => None,
  };
  let session_from_unilang = cmd.get_string( "session" );
  let topic_from_unilang = cmd.get_string( "topic" );
  let receiver = cmd.get_string( "receiver" );
  let job = cmd.get_string( "job" );
  let continuation = cmd.get_string( "continuation" );
  let timeout = cmd.get_integer( "timeout" );

  let dry = cmd.get_boolean( "dry" );
  let verbosity = cmd.get_integer( "verbosity" );
  let ultrathink = cmd.get_boolean( "ultrathink" );
  let interactive_from_unilang = cmd.get_boolean( "interactive" );

  // Parse original command line to extract positional arguments (message words)
  // Users can type: dream .claude hello world
  // This enables natural syntax without requiring command:: parameter
  let original_args = std::env::args().collect::< Vec< _ > >();

  // CRITICAL: Check if user explicitly provided each parameter
  //
  // WHY: Unilang binds positional args to optional parameters with defaults,
  // causing spurious behavior. We only forward parameters explicitly provided by user.
  //
  // PITFALL: MUST check ALL parameter aliases! For example, verbosity has both
  // "v::" and "verbosity::" - if you only check "v::", then "verbosity::5" won't
  // be detected as user-provided and won't be forwarded to dream_agent, causing
  // diagnostic logging to fail silently.
  //
  // Pattern: param1:: || param2:: || alias1:: || alias2::
  let user_provided_message = original_args.iter().any( | arg | arg.starts_with( "command::" ) || arg.starts_with( "m::" ) );
  let user_provided_interactive = original_args.iter().any( | arg | arg.starts_with( "interactive::" ) );
  let user_provided_session = original_args.iter().any( | arg | arg.starts_with( "session::" ) );
  let user_provided_topic = original_args.iter().any( | arg | arg.starts_with( "topic::" ) );
  let user_provided_receiver = original_args.iter().any( | arg | arg.starts_with( "receiver::" ) );
  let user_provided_job = original_args.iter().any( | arg | arg.starts_with( "job::" ) );
  let user_provided_continuation = original_args.iter().any( | arg | arg.starts_with( "continuation::" ) );
  let user_provided_timeout = original_args.iter().any( | arg | arg.starts_with( "timeout::" ) );
  let user_provided_dry = original_args.iter().any( | arg | arg.starts_with( "dry::" ) );
  let user_provided_verbosity = original_args.iter().any( | arg | arg.starts_with( "v::" ) || arg.starts_with( "verbosity::" ) ); // Check both primary and alias!
  let user_provided_ultrathink = original_args.iter().any( | arg | arg.starts_with( "ultrathink::" ) );

  // Collect positional args (anything without ::) after the command name
  let mut positional_args = Vec::new();
  let mut found_command = false;
  for arg in &original_args
  {
    if !found_command
    {
      if arg == ".claude"
      {
        found_command = true;
      }
      continue;
    }

    // Skip parameters (contain ::)
    if arg.contains( "::" )
    {
      continue;
    }

    // This is a positional argument (part of the message)
    positional_args.push( arg.clone() );
  }

  // Fix(issue-boolean-parameter-ignored): Don't include command name in args
  //
  // Root cause: dream_agent::execute_claude() expects args WITHOUT the command name.
  // When ".claude" was included as first arg, it was treated as part of the message,
  // causing all subsequent parameters to be shifted and misinterpreted.
  //
  // Pitfall: When delegating to another layer's execute function, verify whether that
  // function expects the command name to be included or excluded from args.
  let mut args = vec![];

  // Fix(issue-dream-spurious-topic-warning): Only pass optional parameters when explicitly provided by user
  //
  // Root cause: Unilang's parameter binding incorrectly binds positional arguments to optional
  // parameters with defaults (session, topic, job). When user types `dream .claude session::a x=13`,
  // unilang binds `x=13` to topic param (since session is already bound). This caused spurious
  // "Failed to fetch topic context" warnings even though user didn't provide `topic::`.
  //
  // Pitfall: ALL optional parameters with YAML defaults need explicit user-provided checks.
  // Not just `session` - also `topic`, `job`, and any future optional parameters. Without these
  // checks, unilang will bind positional args to them, causing incorrect behavior downstream.

  // Only pass session if user explicitly provided it
  // (unilang may incorrectly bind first positional arg to session param)
  if user_provided_session
  {
    if let Some( session ) = session_from_unilang
    {
      args.push( format!( "session::{session}" ) );
    }
  }
  // Note: No session tracking - wplan architecture uses only queue (path + topic)

  // Only pass topic if user explicitly provided it
  // (unilang may incorrectly bind positional args to topic param)
  if user_provided_topic
  {
    add_optional_string( &mut args, "topic", topic_from_unilang );
  }

  // Only pass receiver if user explicitly provided it
  // (unilang may incorrectly bind positional args to receiver param)
  if user_provided_receiver
  {
    add_optional_string( &mut args, "receiver", receiver );
  }

  // Only pass job if user explicitly provided it
  // (unilang may incorrectly bind positional args to job param)
  if user_provided_job
  {
    add_optional_string( &mut args, "job", job );
  }

  // Only pass parameters if user explicitly provided them (not YAML defaults)
  if user_provided_continuation
  {
    if let Some( cont ) = continuation
    {
      args.push( format!( "continuation::{cont}" ) );
    }
  }
  if user_provided_timeout
  {
    if let Some( t ) = timeout
    {
      args.push( format!( "timeout::{t}" ) );
    }
  }
  if user_provided_dry
  {
    if let Some( d ) = dry
    {
      add_boolean( &mut args, "dry", d );
    }
  }
  if user_provided_verbosity
  {
    if let Some( v ) = verbosity
    {
      args.push( format!( "v::{v}" ) );
    }
  }
  if user_provided_ultrathink
  {
    if let Some( u ) = ultrathink
    {
      add_boolean( &mut args, "ultrathink", u );
    }
  }

  // Only pass interactive if user explicitly provided it
  // This allows dream_agent to apply smart defaults (interactive when no message)
  if user_provided_interactive
  {
    if let Some( i ) = interactive_from_unilang
    {
      add_boolean( &mut args, "interactive", i );
    }
  }

  // Fix(issue-dream-message-doubling): Prevent duplicate first positional arg
  // Fix(issue-dream-message-parameter-binding): Handle command:: parameter without breaking positional args
  //
  // Root cause (original): When session:: not provided, unilang binds first positional arg to session parameter.
  // Root cause (new): Adding command:: parameter causes unilang to bind first positional arg to command,
  // second arg to session. Must recover both when user didn't explicitly provide them.
  //
  // Pitfall: Never assume parameter binding and positional collection are mutually exclusive.
  // Always track which args have been added to prevent duplication.
  //
  // Add message as positional arguments from command line
  // Priority order for positional arg recovery:
  // 1. command:: (if explicitly provided, use it)
  // 2. command (if bound by unilang, recover it)
  // 3. session (if bound by unilang, recover it)
  // 4. remaining positional args

  // If user explicitly provided command::, use it as first positional arg
  if user_provided_message
  {
    if let Some( ref msg ) = message_from_unilang
    {
      if !msg.is_empty()
      {
        args.push( msg.clone() );
      }
    }
    // Add remaining positional args (if any)
    args.extend( positional_args );
  }
  else
  {
    // User didn't provide command::, so unilang may have bound positional args to parameters
    // Recovery order: command -> session -> positional_args

    let mut recovered_from_message = false;
    let mut recovered_from_session = false;

    // Recover first positional arg from message parameter (if bound by unilang)
    if let Some( ref msg ) = message_from_unilang
    {
      if !msg.is_empty()
      {
        args.push( msg.clone() );
        recovered_from_message = true;
      }
    }

    // Recover second positional arg from session parameter (if bound by unilang)
    if !user_provided_session
    {
      if let Some( session ) = session_from_unilang
      {
        if !session.is_empty()
        {
          args.push( session.to_string() );
          recovered_from_session = true;
        }
      }
    }

    // Add remaining positional args
    if !positional_args.is_empty()
    {
      // Calculate how many args to skip based on what was recovered
      // Each recovered parameter (message or session) consumes one positional arg slot
      let skip_count = usize::from( recovered_from_message ) + usize::from( recovered_from_session );

      args.extend( positional_args.iter().skip( skip_count ).cloned() );
    }
  }

  delegate_to_agent( &args )
}

/// Routine: Show help for .claude command
///
/// # Errors
///
/// This function never returns an error. The `Result` return type is required
/// by the unilang routine signature.
#[ inline ]
pub fn claude_help_routine(
  _cmd : VerifiedCommand,
  _ctx : ExecutionContext,
) -> Result< OutputData, ErrorData >
{
  let help_text = r#"
.claude - AI-assisted development using Claude Code with isolated conversation topics

USAGE:
  dream .claude [OPTIONS] [message...]

POSITIONAL ARGUMENTS:
  message...              Natural language message to Claude (e.g., "help me debug this")
                          If omitted, opens interactive session

NAMED PARAMETERS:
  topic::NAME             Topic name for queue isolation and context injection (default: default_topic)
                          Creates queue: /current/dir@topic
                          Different topics = separate conversation contexts
                          Provides Claude with queue context (recent job failures, config)

  job::ID                 Inject job context (logs, exit code, timing)
                          Format: queue/job_number or just job_number
                          Provides Claude with information about a specific job failure

  continuation::MODE      Session continuation: "resume" or "fresh" (default: resume)
                          - resume: Continue existing conversation
                          - fresh: Start clean conversation, discard history

  timeout::MS             Claude timeout in milliseconds (default: 7200000 = 2 hours)

  dry::BOOL               Dry run mode - show command without executing (default: false)

  v::LEVEL                Verbosity level (default: 2)
                          0=quiet, 1=normal, 2=verbose, 3=debug, 4=trace

  ultrathink::BOOL        Append ' ultrathink' to message for deeper analysis (default: true)

  interactive::BOOL       Force interactive mode (default: auto)
                          Auto mode: interactive if no message, non-interactive if message provided

EXAMPLES:
  # Quick question (non-interactive)
  dream .claude what is 2+2?
  dream .claude hello

  # Interactive session
  dream .claude
  dream .claude interactive::true

  # With context from queue or job
  dream .claude topic::build why are builds failing?
  dream .claude job::build/42 explain this failure

  # Different conversation topics
  dream .claude topic::debug investigate the issue
  dream .claude topic::testing help me write tests

  # Combine options
  dream .claude topic::bugfix job::build/42 v::3 help me fix this

NOTES:
  - Messages are positional arguments (no command:: parameter needed)
  - Use topic:: to maintain separate conversation contexts and inject queue context
  - job:: injects specific job failure context automatically
  - Interactive mode opens full Claude Code session
  - Non-interactive mode prints response and exits
"#;

  Ok( OutputData::new( help_text.trim().to_string(), "text" ) )
}
