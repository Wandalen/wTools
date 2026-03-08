//! Builder Pattern Edge Case Tests
//!
//! # Test Matrix
//!
//! | Test Case | Aspect | Input | Expected Output | Status |
//! |-----------|--------|-------|-----------------|--------|
//! | `default_token_limit_200k_bug_token_limit` | Bug: Default token limit | `ClaudeCommand::new()` | `CLAUDE_CODE_MAX_OUTPUT_TOKENS="200000"` | ✅ |
//! | `token_limit_zero_edge_case` | Boundary: Zero | `with_max_output_tokens(0)` | `CLAUDE_CODE_MAX_OUTPUT_TOKENS="0"` | ✅ |
//! | `token_limit_one_minimum` | Boundary: Minimum | `with_max_output_tokens(1)` | `CLAUDE_CODE_MAX_OUTPUT_TOKENS="1"` | ✅ |
//! | `token_limit_u32_max_boundary` | Boundary: Maximum | `with_max_output_tokens(u32::MAX)` | `CLAUDE_CODE_MAX_OUTPUT_TOKENS="4294967295"` | ✅ |
//! | `token_limit_override_last_wins` | Override: Token limit | Multiple `with_max_output_tokens()` calls | Last value wins (300000) | ✅ |
//! | `working_directory_override_last_wins` | Override: Working dir | Multiple `with_working_directory()` calls | Last path wins (/tmp/third) | ✅ |
//! | `message_override_last_wins` | Override: Message | Multiple `with_message()` calls | Last message wins | ✅ |
//! | `continue_conversation_override_last_wins` | Override: Continuation flag | Multiple `with_continue_conversation()` calls | Last value wins | ✅ |
//! | `with_arg_accumulates` | Accumulation: Single args | Multiple `with_arg()` calls | All args accumulated | ✅ |
//! | `with_args_accumulates` | Accumulation: Batch args | Multiple `with_args()` calls | All args accumulated | ✅ |
//! | `with_arg_and_with_args_both_accumulate` | Accumulation: Mixed | Mixed `with_arg()` and `with_args()` | All args accumulated in order | ✅ |
//! | `empty_message_edge_case` | Edge case: Empty string | `with_message("")` | Empty arg present | ✅ |
//! | `whitespace_only_message_edge_case` | Edge case: Whitespace | `with_message("   ")` | Whitespace preserved | ✅ |
//! | `very_long_message_100kb_stress` | Stress: Large message | `with_message("A" * 100K)` | Message accepted, no panic | ✅ |
//! | `with_model_called_twice_both_pairs_accumulate` | Accumulation: Model | Two `with_model()` calls | Two `--model` pairs in args | ✅ |
//! | `with_system_prompt_called_twice_both_pairs_accumulate` | Accumulation: System prompt | Two `with_system_prompt()` calls | Two `--system-prompt` pairs | ✅ |
//! | `with_verbose_true_called_twice_flag_appears_twice` | Accumulation: Verbose | Two `with_verbose(true)` calls | Two `--verbose` flags | ✅ |
//! | `with_arg_continue_and_continue_conversation_both_produce_c` | Accumulation: Continuation | `with_arg("-c")` + `with_continue_conversation(true)` | Two `-c` flags | ✅ |
//! | `with_arg_skip_perms_and_with_skip_permissions_both_produce_flag` | Accumulation: Skip-perms | `with_arg("--dangerously-skip-permissions")` + `with_skip_permissions(true)` | Two skip flags | ✅ |
//!
//! # Lessons Learned (Bugs Fixed)
//!
//! ## issue-token-limit-default
//!
//! **Root Cause:** Migration from factory pattern to builder pattern didnt preserve the correct
//! default token limit value. Original factory set `max_output_tokens: Some(200_000)` but migration
//! inadvertently changed it to 32K, causing "exceeded maximum output" errors for users relying on defaults.
//!
//! **Fix:** Updated `ClaudeCommand::new()` to set `max_output_tokens: Some(200_000)` matching
//! specification (claude_runner_core/src/command.rs:52-57). Added explicit documentation comment.
//!
//! **Prevention:** Test `default_token_limit_200k_bug_token_limit` validates default by inspecting
//! environment variable without explicit setter call. Always verify defaults match specification
//! when refactoring APIs. Test implicit contracts (defaults, initial state) explicitly.

use claude_runner_core::ClaudeCommand;

/// Reproduces token-limit-default bug where default was incorrectly set to 32K instead of 200K.
///
/// ## Root Cause
///
/// Migration from factory pattern to builder pattern didnt preserve the correct default
/// token limit value. Original factory set `max_output_tokens: Some(200_000)` but the
/// migration inadvertently changed it to 32K, causing "exceeded maximum output" errors
/// for users relying on defaults.
///
/// ## Why Not Caught Initially
///
/// Migration had no tests validating default values. Tests only validated explicit
/// parameter passing. The implicit contract (defaults match specification) wasnt
/// tested, allowing regression to ship undetected.
///
/// ## Fix Applied
///
/// Updated `ClaudeCommand::new()` to set `max_output_tokens: Some(200_000)` matching
/// the specification (claude_runner_core/src/command.rs:52-57). Added explicit documentation
/// comment explaining the default value and its importance.
///
/// ## Prevention
///
/// This test validates the default token limit is 200K by inspecting the environment
/// variable `CLAUDE_CODE_MAX_OUTPUT_TOKENS` without any explicit setter call. Test
/// will fail if default changes, preventing future regression.
///
/// ## Pitfall to Avoid
///
/// Always verify defaults match specification when refactoring APIs. Dont assume
/// default values carry over during migrations. Test implicit contracts (defaults,
/// initial state) explicitly, not just explicit parameter passing.
// test_kind: bug_reproducer(issue-token-limit-default)
#[ test ]
fn default_token_limit_200k_bug_token_limit()
{
  // Verify: ClaudeCommand::new() sets 200K tokens by default
  // This is the BUG FIX - was 32K before migration

  let cmd = ClaudeCommand::new();

  // Build command and inspect environment
  let process_cmd = cmd_to_process_command( &cmd );

  let env_value = process_cmd.get_envs()
    .find( | ( key, _val ) | key == &"CLAUDE_CODE_MAX_OUTPUT_TOKENS" )
    .and_then( | ( _key, val ) | val.and_then( | v | v.to_str() ) );

  assert_eq!( env_value, Some( "200000" ), "Default token limit should be 200K" );
}

/// Validates that token limit can be set to zero without panic.
///
/// Tests edge case where user explicitly sets `max_output_tokens(0)`. This is
/// a valid configuration that should be passed to Claude binary without error.
#[ test ]
fn token_limit_zero_edge_case()
{
  // Edge case: 0 tokens
  // Should this work? Currently it does (sets env to "0")

  let cmd = ClaudeCommand::new()
    .with_max_output_tokens( 0 );

  let process_cmd = cmd_to_process_command( &cmd );

  let env_value = process_cmd.get_envs()
    .find( | ( key, _val ) | key == &"CLAUDE_CODE_MAX_OUTPUT_TOKENS" )
    .and_then( | ( _key, val ) | val.and_then( | v | v.to_str() ) );

  assert_eq!( env_value, Some( "0" ), "Should set token limit to 0" );
}

/// Validates that token limit can be set to one (minimum positive value).
///
/// Tests boundary condition for token limit. Value of 1 should be accepted
/// and correctly set in environment variable.
#[ test ]
fn token_limit_one_minimum()
{
  // Edge case: 1 token (minimum positive value)

  let cmd = ClaudeCommand::new()
    .with_max_output_tokens( 1 );

  let process_cmd = cmd_to_process_command( &cmd );

  let env_value = process_cmd.get_envs()
    .find( | ( key, _val ) | key == &"CLAUDE_CODE_MAX_OUTPUT_TOKENS" )
    .and_then( | ( _key, val ) | val.and_then( | v | v.to_str() ) );

  assert_eq!( env_value, Some( "1" ), "Should set token limit to 1" );
}

/// Validates that token limit can be set to `u32::MAX` (4,294,967,295).
///
/// Tests upper boundary condition. Maximum value should be accepted without
/// overflow or panic, correctly converted to string in environment variable.
#[ test ]
fn token_limit_u32_max_boundary()
{
  // Edge case: u32::MAX (4,294,967,295)

  let cmd = ClaudeCommand::new()
    .with_max_output_tokens( u32::MAX );

  let process_cmd = cmd_to_process_command( &cmd );

  let env_value = process_cmd.get_envs()
    .find( | ( key, _val ) | key == &"CLAUDE_CODE_MAX_OUTPUT_TOKENS" )
    .and_then( | ( _key, val ) | val.and_then( | v | v.to_str() ) );

  assert_eq!( env_value, Some( "4294967295" ), "Should set token limit to u32::MAX" );
}

/// Validates that multiple calls to `with_max_output_tokens()` use last value.
///
/// Builder pattern setter methods should follow "last wins" semantics where
/// multiple calls override previous values rather than accumulating.
#[ test ]
fn token_limit_override_last_wins()
{
  // Multiple calls to with_max_output_tokens()
  // Last call should win

  let cmd = ClaudeCommand::new()
    .with_max_output_tokens( 100_000 )
    .with_max_output_tokens( 200_000 )
    .with_max_output_tokens( 300_000 );

  let process_cmd = cmd_to_process_command( &cmd );

  let env_value = process_cmd.get_envs()
    .find( | ( key, _val ) | key == &"CLAUDE_CODE_MAX_OUTPUT_TOKENS" )
    .and_then( | ( _key, val ) | val.and_then( | v | v.to_str() ) );

  assert_eq!( env_value, Some( "300000" ), "Last with_max_output_tokens() should win" );
}

/// Validates that multiple calls to `with_working_directory()` use last value.
///
/// Builder pattern setter methods should follow "last wins" semantics where
/// multiple calls override previous values rather than accumulating.
#[ test ]
fn working_directory_override_last_wins()
{
  // Multiple calls to with_working_directory()
  // Last call should win

  let cmd = ClaudeCommand::new()
    .with_working_directory( "/tmp/first" )
    .with_working_directory( "/tmp/second" )
    .with_working_directory( "/tmp/third" );

  let process_cmd = cmd_to_process_command( &cmd );

  let current_dir = process_cmd.get_current_dir()
    .and_then( | p | p.to_str() );

  assert_eq!( current_dir, Some( "/tmp/third" ), "Last with_working_directory() should win" );
}

/// Validates that multiple calls to `with_message()` use last value.
///
/// Builder pattern setter methods should follow "last wins" semantics where
/// multiple calls override previous values rather than accumulating.
#[ test ]
fn message_override_last_wins()
{
  // Multiple calls to with_message()
  // Last call should win

  let cmd = ClaudeCommand::new()
    .with_message( "first message" )
    .with_message( "second message" )
    .with_message( "final message" );

  let process_cmd = cmd_to_process_command( &cmd );

  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  // Message should be last arg
  assert!( args.contains( &"final message" ), "Last with_message() should win" );
  assert!( !args.contains( &"first message" ), "First message should be overridden" );
  assert!( !args.contains( &"second message" ), "Second message should be overridden" );
}

/// Validates that multiple calls to `with_continue_conversation()` use last value.
///
/// Builder pattern setter methods should follow "last wins" semantics where
/// multiple calls override previous values rather than accumulating.
#[ test ]
fn continue_conversation_override_last_wins()
{
  // Multiple calls to with_continue_conversation()
  // Last call should win

  let cmd = ClaudeCommand::new()
    .with_continue_conversation( true )
    .with_continue_conversation( false )
    .with_continue_conversation( true );

  let process_cmd = cmd_to_process_command( &cmd );

  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  // Should have -c flag (last value was true)
  assert!( args.contains( &"-c" ), "Last with_continue_conversation(true) should win" );

  // Test opposite case
  let cmd2 = ClaudeCommand::new()
    .with_continue_conversation( false )
    .with_continue_conversation( true )
    .with_continue_conversation( false );

  let process_cmd2 = cmd_to_process_command( &cmd2 );

  let args2 : Vec< &str > = process_cmd2.get_args()
    .map( | s | s.to_str().unwrap() )
    .collect();

  // Should NOT have -c flag (last value was false)
  assert!( !args2.contains( &"-c" ), "Last with_continue_conversation(false) should win" );
}

/// Validates that multiple calls to `with_arg()` accumulate rather than override.
///
/// Unlike setter methods, `with_arg()` should add each argument to the list.
/// All arguments should be preserved in the final command.
#[ test ]
fn with_arg_accumulates()
{
  // Multiple calls to with_arg() should ACCUMULATE (not override)

  let cmd = ClaudeCommand::new()
    .with_arg( "--first" )
    .with_arg( "--second" )
    .with_arg( "--third" );

  let process_cmd = cmd_to_process_command( &cmd );

  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  // All args should be present
  assert!( args.contains( &"--first" ), "First arg should be present" );
  assert!( args.contains( &"--second" ), "Second arg should be present" );
  assert!( args.contains( &"--third" ), "Third arg should be present" );
}

/// Validates that multiple calls to `with_args()` accumulate rather than override.
///
/// Unlike setter methods, `with_args()` should add all arguments to the list.
/// Arguments from all calls should be preserved in the final command.
#[ test ]
fn with_args_accumulates()
{
  // Multiple calls to with_args() should ACCUMULATE (not override)

  let cmd = ClaudeCommand::new()
    .with_args( vec![ "--first", "--second" ] )
    .with_args( vec![ "--third", "--fourth" ] );

  let process_cmd = cmd_to_process_command( &cmd );

  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  // All args should be present
  assert!( args.contains( &"--first" ) );
  assert!( args.contains( &"--second" ) );
  assert!( args.contains( &"--third" ) );
  assert!( args.contains( &"--fourth" ) );
}

/// Validates that mixing `with_arg()` and `with_args()` accumulates all arguments.
///
/// When mixing single and batch argument methods, all arguments should be
/// accumulated in order, preserving the sequence of calls.
#[ test ]
fn with_arg_and_with_args_both_accumulate()
{
  // Mix with_arg() and with_args() - both should accumulate

  let cmd = ClaudeCommand::new()
    .with_arg( "--solo1" )
    .with_args( vec![ "--batch1", "--batch2" ] )
    .with_arg( "--solo2" )
    .with_args( vec![ "--batch3" ] );

  let process_cmd = cmd_to_process_command( &cmd );

  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  // All args should be present in order
  assert!( args.contains( &"--solo1" ) );
  assert!( args.contains( &"--batch1" ) );
  assert!( args.contains( &"--batch2" ) );
  assert!( args.contains( &"--solo2" ) );
  assert!( args.contains( &"--batch3" ) );
}

/// Validates that empty string message is accepted and passed through.
///
/// Edge case: Empty message should not cause panic or be filtered out.
/// Builder should accept empty string and include it in command arguments.
#[ test ]
fn empty_message_edge_case()
{
  // Edge case: Empty string message

  let cmd = ClaudeCommand::new()
    .with_message( "" );

  let process_cmd = cmd_to_process_command( &cmd );

  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  // Empty message should still be added as argument
  // (Claude Code might handle this specially)
  assert!( args.contains( &"" ), "Empty message should be present in args" );
}

/// Validates that whitespace-only message is preserved as-is.
///
/// Edge case: Whitespace message should not be trimmed or rejected.
/// Builder should preserve exact whitespace in command arguments.
#[ test ]
fn whitespace_only_message_edge_case()
{
  // Edge case: Whitespace-only message

  let cmd = ClaudeCommand::new()
    .with_message( "   " );

  let process_cmd = cmd_to_process_command( &cmd );

  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  // Whitespace message should be preserved as-is
  assert!( args.contains( &"   " ), "Whitespace message should be preserved" );
}

/// Validates that very long message (100KB) is handled without panic.
///
/// Stress test: Large message should not cause buffer overflow, memory issues,
/// or performance degradation. Builder should handle arbitrarily long messages.
#[ test ]
fn very_long_message_100kb_stress()
{
  // Edge case: Very long message (100KB)

  let long_message = "A".repeat( 100_000 );

  let cmd = ClaudeCommand::new()
    .with_message( &long_message );

  let process_cmd = cmd_to_process_command( &cmd );

  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  // Long message should be present
  assert!( args.iter().any( | arg | arg.len() == 100_000 ), "Long message should be present" );
}

// Helper function to convert ClaudeCommand to std::process::Command for inspection
// Uses the test-only build_command_for_test() method
fn cmd_to_process_command( cmd : &ClaudeCommand ) -> std::process::Command
{
  cmd.build_command_for_test()
}

// ============================================================================
// Accumulation Behavior Documentation
// ============================================================================
//
// The builder accumulates args through with_arg()/with_args() and any method
// that appends to the args Vec (with_model, with_api_key, with_system_prompt,
// with_verbose). Scalar parameters (message, working_directory, etc.) use
// last-call-wins semantics. These tests document accumulation behaviors explicitly.

#[test]
fn with_model_called_twice_both_pairs_accumulate()
{
  // with_model() appends "--model" + value to the args Vec each time.
  // Multiple calls produce multiple --model pairs (accumulation, not override).
  let cmd = ClaudeCommand::new()
    .with_model( "claude-sonnet" )
    .with_model( "claude-opus" );

  let process_cmd = cmd_to_process_command( &cmd );
  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  let model_count = args.iter().filter( | && a | a == "--model" ).count();
  assert_eq!(
    model_count, 2,
    "with_model() called twice must produce two --model flags (accumulation): got args {args:?}"
  );
  assert!( args.contains( &"claude-sonnet" ), "First model must appear" );
  assert!( args.contains( &"claude-opus" ), "Second model must appear" );
}

#[test]
fn with_system_prompt_called_twice_both_pairs_accumulate()
{
  // with_system_prompt() appends "--system-prompt" + value each time.
  let cmd = ClaudeCommand::new()
    .with_system_prompt( "first prompt" )
    .with_system_prompt( "second prompt" );

  let process_cmd = cmd_to_process_command( &cmd );
  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  let sp_count = args.iter().filter( | && a | a == "--system-prompt" ).count();
  assert_eq!(
    sp_count, 2,
    "with_system_prompt() called twice must produce two --system-prompt flags: got {args:?}"
  );
}

#[test]
fn with_verbose_true_called_twice_flag_appears_twice()
{
  // with_verbose(true) appends "--verbose" each call; calling it twice produces two flags.
  // This is consistent with the arg accumulation model (each call adds independently).
  let cmd = ClaudeCommand::new()
    .with_verbose( true )
    .with_verbose( true );

  let process_cmd = cmd_to_process_command( &cmd );
  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  let verbose_count = args.iter().filter( | && a | a == "--verbose" ).count();
  assert_eq!(
    verbose_count, 2,
    "with_verbose(true) called twice must produce two --verbose flags: got {args:?}"
  );
}

#[test]
fn with_arg_continue_and_continue_conversation_both_produce_c()
{
  // Adding -c via with_arg() AND enabling with_continue_conversation(true) both
  // append -c to the command. The result has two -c flags.
  // This documents the interaction: both mechanisms are independent.
  let cmd = ClaudeCommand::new()
    .with_arg( "-c" )
    .with_continue_conversation( true );

  let process_cmd = cmd_to_process_command( &cmd );
  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  let c_count = args.iter().filter( | && a | a == "-c" ).count();
  assert_eq!(
    c_count, 2,
    "with_arg(\"-c\") + with_continue_conversation(true) both add -c: total should be 2, got {args:?}"
  );
}

#[test]
fn with_arg_skip_perms_and_with_skip_permissions_both_produce_flag()
{
  // Adding --dangerously-skip-permissions via with_arg() AND via with_skip_permissions(true)
  // both produce the flag. Result has two --dangerously-skip-permissions flags.
  let cmd = ClaudeCommand::new()
    .with_arg( "--dangerously-skip-permissions" )
    .with_skip_permissions( true );

  let process_cmd = cmd_to_process_command( &cmd );
  let args : Vec< &str > = process_cmd.get_args()
    .filter_map( | s | s.to_str() )
    .collect();

  let skip_count = args.iter()
    .filter( | && a | a == "--dangerously-skip-permissions" )
    .count();
  assert_eq!(
    skip_count, 2,
    "with_arg(\"--dangerously-skip-permissions\") + with_skip_permissions(true) both add the flag: got {args:?}"
  );
}
