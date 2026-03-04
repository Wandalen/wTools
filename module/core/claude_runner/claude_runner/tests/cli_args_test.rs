//! CLI Argument Parsing Tests
//!
//! ## Purpose
//!
//! Verify that `claude_runner` correctly parses CLI flags and translates them
//! into the underlying `ClaudeCommand` builder calls. Uses `--dry-run` mode
//! to inspect command construction without requiring Claude binary in PATH.
//!
//! ## Strategy
//!
//! All tests invoke the compiled binary via `env!("CARGO_BIN_EXE_claude_runner")`.
//! `--dry-run` outputs the command line that would be executed, allowing
//! assertions against the translation of flags → builder calls.
//!
//! ## Corner Cases Covered
//!
//! - `--continue` adds `-c` to command
//! - `--skip-permissions` adds `--dangerously-skip-permissions`
//! - Positional argument becomes the message
//! - Missing required value → non-zero exit (all 5 value-requiring flags)
//! - Unknown flag → non-zero exit
//! - `--help` → zero exit with USAGE in output
//! - `-d` short alias for `--dir`
//! - Two positional args → non-zero exit (ambiguous input)
//! - `-m` flag + positional → non-zero exit (conflicting message sources)
//! - Positional + `-m` flag → non-zero exit (reverse conflict; CC3 bug-reproducer)
//! - Duplicate `--message` flags → non-zero exit (CC4 bug-reproducer)
//! - Error output goes to stderr (not stdout) — FR-10
//! - `--help` contains all 10 option lines — FR-9
//! - `--max-tokens 0` and `--max-tokens 4294967295` are valid u32 boundaries
//! - Duplicate `--dir`/`--model`/`--session-dir` flags: last value wins (B4/B5/B6)
//! - `--help` with an unknown flag still errors (D3): strict parsing, no pre-scan for --help

use std::process::Command;

fn run_cli( args: &[ &str ] ) -> std::process::Output {
  let bin = env!( "CARGO_BIN_EXE_claude_runner" );
  Command::new( bin )
    .args( args )
    .output()
    .expect( "Failed to invoke claude_runner binary" )
}

#[test]
fn dry_run_prints_claude_command() {
  let out = run_cli( &[ "--message", "hello", "--dry-run" ] );
  assert!( out.status.success(), "dry-run should exit 0" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( "claude" ), "dry-run output must contain 'claude'" );
}

#[test]
fn continue_flag_adds_dash_c() {
  let out = run_cli( &[ "--message", "hi", "--continue", "--dry-run" ] );
  assert!( out.status.success() );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( " -c" ), "continue flag must add -c to command line" );
}

#[test]
fn short_continue_flag_works() {
  let out = run_cli( &[ "--message", "hi", "-c", "--dry-run" ] );
  assert!( out.status.success() );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( " -c" ), "-c short flag must work same as --continue" );
}

#[test]
fn skip_permissions_adds_dangerously_flag() {
  let out = run_cli( &[ "--message", "hi", "--skip-permissions", "--dry-run" ] );
  assert!( out.status.success() );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!(
    stdout.contains( "--dangerously-skip-permissions" ),
    "--skip-permissions must produce --dangerously-skip-permissions in command"
  );
}

#[test]
fn short_message_flag_works() {
  let out = run_cli( &[ "-m", "test message", "--dry-run" ] );
  assert!( out.status.success(), "-m short flag should work" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( "claude" ) );
}

#[test]
fn help_flag_exits_zero_with_usage() {
  let out = run_cli( &[ "--help" ] );
  assert!( out.status.success(), "--help must exit 0" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( "USAGE:" ), "--help must print USAGE" );
}

#[test]
fn short_help_flag_works() {
  let out = run_cli( &[ "-h" ] );
  assert!( out.status.success(), "-h must exit 0" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( "USAGE:" ) );
}

#[test]
fn unknown_arg_exits_nonzero() {
  let out = run_cli( &[ "--unknown-flag-xyz" ] );
  assert!( !out.status.success(), "unknown flag must exit non-zero" );
}

#[test]
fn missing_message_value_exits_nonzero() {
  // --message with no following value
  let out = run_cli( &[ "--message" ] );
  assert!( !out.status.success(), "missing --message value must exit non-zero" );
}

#[test]
fn missing_dir_value_exits_nonzero() {
  let out = run_cli( &[ "--dir" ] );
  assert!( !out.status.success(), "missing --dir value must exit non-zero" );
}

#[test]
fn missing_max_tokens_value_exits_nonzero() {
  let out = run_cli( &[ "--max-tokens" ] );
  assert!( !out.status.success(), "missing --max-tokens value must exit non-zero" );
}

#[test]
fn invalid_max_tokens_exits_nonzero() {
  let out = run_cli( &[ "--message", "hi", "--max-tokens", "not-a-number", "--dry-run" ] );
  assert!( !out.status.success(), "non-numeric --max-tokens must exit non-zero" );
}

#[test]
fn no_args_exits_zero() {
  // No args with no message: dry-run or real would show help or run with no message
  // Just verify binary runs without crashing
  let out = run_cli( &[ "--dry-run" ] );
  assert!( out.status.success(), "bare --dry-run without message should exit 0" );
}

#[test]
fn model_flag_accepted() {
  let out = run_cli( &[ "--message", "hi", "--model", "claude-opus-4-6", "--dry-run" ] );
  assert!( out.status.success(), "--model flag should be accepted" );
}

#[test]
fn session_dir_flag_accepted() {
  let out = run_cli( &[ "--message", "hi", "--session-dir", "/tmp/sess", "--dry-run" ] );
  assert!( out.status.success(), "--session-dir flag should be accepted" );
}

// FR-2: spec lists -d as an alias for --dir; verify the short form works
#[test]
fn short_dir_flag_works() {
  let out = run_cli( &[ "--message", "hi", "-d", "/tmp/test-dir", "--dry-run" ] );
  assert!( out.status.success(), "-d short flag must work same as --dir" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( "cd /tmp/test-dir" ), "-d must produce 'cd <path>' prefix. Got:\n{stdout}" );
}

// FR-10: missing --session-dir value must print error and exit 1
#[test]
fn missing_session_dir_value_exits_nonzero() {
  let out = run_cli( &[ "--session-dir" ] );
  assert!( !out.status.success(), "missing --session-dir value must exit non-zero" );
}

// FR-10: missing --model value must print error and exit 1
#[test]
fn missing_model_value_exits_nonzero() {
  let out = run_cli( &[ "--model" ] );
  assert!( !out.status.success(), "missing --model value must exit non-zero" );
}

// FR-1: second positional argument is ambiguous; must exit non-zero
#[test]
fn two_positional_args_exits_nonzero() {
  let out = run_cli( &[ "first-message", "second-message", "--dry-run" ] );
  assert!( !out.status.success(), "two positional args must exit non-zero" );
}

// FR-1: -m flag then positional arg is a conflict; must exit non-zero
#[test]
fn message_flag_then_positional_exits_nonzero() {
  let out = run_cli( &[ "-m", "flag-message", "positional-message" ] );
  assert!( !out.status.success(), "-m flag followed by positional must exit non-zero" );
}

// FR-10: errors must go to stderr, nothing to stdout
#[test]
fn error_output_goes_to_stderr_not_stdout() {
  let out = run_cli( &[ "--unknown-flag-xyz" ] );
  assert!( !out.status.success(), "unknown flag must exit non-zero" );
  assert!( out.stdout.is_empty(), "stdout must be empty on error; got: {}", String::from_utf8_lossy( &out.stdout ) );
  let stderr = String::from_utf8_lossy( &out.stderr );
  assert!( stderr.contains( "Error:" ), "stderr must contain 'Error:'; got: {stderr}" );
}

// FR-9: --help must list all documented options
#[test]
fn help_lists_all_options() {
  let out = run_cli( &[ "--help" ] );
  assert!( out.status.success() );
  let stdout = String::from_utf8_lossy( &out.stdout );
  for flag in &[
    "--message", "--dir", "--continue", "--max-tokens",
    "--skip-permissions", "--dry-run", "--session-dir", "--model", "--help", "--verbose",
  ] {
    assert!( stdout.contains( flag ), "--help missing option {flag}. Got:\n{stdout}" );
  }
}

// FR-4: --max-tokens must accept u32 boundary values: 0 (u32::MIN)
#[test]
fn max_tokens_zero_is_accepted() {
  let out = run_cli( &[ "--message", "hi", "--max-tokens", "0", "--dry-run" ] );
  assert!( out.status.success(), "--max-tokens 0 must be accepted as valid u32" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=0" ), "must set token env to 0. Got:\n{stdout}" );
}

// FR-4: --max-tokens must accept u32 boundary values: 4294967295 (u32::MAX)
#[test]
fn max_tokens_u32_max_is_accepted() {
  let out = run_cli( &[ "--message", "hi", "--max-tokens", "4294967295", "--dry-run" ] );
  assert!( out.status.success(), "--max-tokens 4294967295 (u32::MAX) must be accepted" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=4294967295" ), "must set correct token env. Got:\n{stdout}" );
}

// Bug-reproducer: CC3 — positional arg set first, then --message/-m provided
//
// ## Root Cause
// `parse_args` stored positional arg in `opts.message`, then `-m` silently
// overwrote it. The first (positional) message was lost with no error.
//
// ## Why Not Caught Initially
// The reverse case (flag-first then positional) was tested and correctly errored.
// The positional-first case was not tested, so the silent override went undetected.
//
// ## Fix Applied
// Added `opts.message.is_some()` check before overriding in the `-m/--message` arm.
// Returns `Err` with clear conflict message instead of silently overwriting.
//
// ## Prevention
// Both orderings of conflicting message sources must be tested symmetrically.
// Silent data loss is always a bug — parser must reject ambiguous input.
//
// ## Pitfall
// Flag-after-positional silently wins because flag parsing runs unconditionally.
// Always guard flag assignment when the same field can be set by multiple sources.
#[test]
fn positional_then_message_flag_exits_nonzero() {
  // CC3: positional first, then -m flag — was silently dropping positional
  let out = run_cli( &[ "positional-message", "-m", "flag-message" ] );
  assert!( !out.status.success(), "positional followed by -m must exit non-zero (conflicting message sources)" );
  let stderr = String::from_utf8_lossy( &out.stderr );
  assert!( stderr.contains( "Error:" ), "error must go to stderr. Got: {stderr}" );
}

// Bug-reproducer: CC4 — duplicate --message flags, last wins silently
//
// ## Root Cause
// Same guard fix as CC3 — `opts.message.is_some()` now catches duplicate `-m` too.
//
// ## Why Not Caught Initially
// Duplicate flag case was not in the original test matrix.
//
// ## Fix Applied
// Same fix as CC3: `opts.message.is_some()` check before overriding.
//
// ## Prevention
// Test that every flag which maps to an `Option` field rejects duplicate specification.
//
// ## Pitfall
// "Last wins" for duplicate flags is surprising to users. Reject instead.
#[test]
fn duplicate_message_flag_exits_nonzero() {
  // CC4: two --message flags — was silently using the last value
  let out = run_cli( &[ "--message", "first", "--message", "second" ] );
  assert!( !out.status.success(), "duplicate --message flags must exit non-zero" );
  let stderr = String::from_utf8_lossy( &out.stderr );
  assert!( stderr.contains( "Error:" ), "error must go to stderr. Got: {stderr}" );
}

// B4: duplicate --dir flags; last value wins, no error.
// Unlike --message (which errors on duplicates), --dir has no positional alternative,
// so "last wins" is less surprising and the spec does not require an error.
#[test]
fn duplicate_dir_flag_uses_last_value() {
  let out = run_cli( &[ "--message", "hi", "--dir", "/first", "--dir", "/last", "--dry-run" ] );
  assert!( out.status.success(), "duplicate --dir must exit 0 (last wins)" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( "cd /last" ), "last --dir value must win. Got:\n{stdout}" );
  assert!( !stdout.contains( "cd /first" ), "first --dir must be overridden. Got:\n{stdout}" );
}

// B5: duplicate --model flags; last value wins, no error.
#[test]
fn duplicate_model_flag_uses_last_value() {
  let out = run_cli( &[ "--message", "hi", "--model", "first-model", "--model", "last-model", "--dry-run" ] );
  assert!( out.status.success(), "duplicate --model must exit 0 (last wins)" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( "last-model" ), "last --model value must win. Got:\n{stdout}" );
  assert!( !stdout.contains( "first-model" ), "first --model must be overridden. Got:\n{stdout}" );
}

// B6: duplicate --session-dir flags; last value wins, no error.
#[test]
fn duplicate_session_dir_flag_uses_last_value() {
  let out = run_cli( &[ "--message", "hi", "--session-dir", "/first", "--session-dir", "/last", "--dry-run" ] );
  assert!( out.status.success(), "duplicate --session-dir must exit 0 (last wins)" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!( stdout.contains( "CLAUDE_CODE_SESSION_DIR=/last" ), "last --session-dir must win. Got:\n{stdout}" );
  assert!( !stdout.contains( "CLAUDE_CODE_SESSION_DIR=/first" ), "first --session-dir must be overridden. Got:\n{stdout}" );
}

// FR-12: --verbose flag must be accepted; --dry-run prevents actual execution so test is CI-safe.
#[test]
fn verbose_flag_accepted()
{
  let out = run_cli( &[ "--message", "hi", "--verbose", "--dry-run" ] );
  assert!( out.status.success(), "--verbose flag must be accepted" );
}

// FR-12: -v is a spec-defined alias for --verbose; must produce identical behavior.
#[test]
fn short_verbose_flag_works()
{
  let out = run_cli( &[ "--message", "hi", "-v", "--dry-run" ] );
  assert!( out.status.success(), "-v short flag must work same as --verbose" );
}

// D3: --help does not suppress parse errors for unknown flags.
// parse_args() validates all arguments sequentially; unknown flags error immediately
// without pre-scanning for --help. This is strict, consistent parsing behavior.
#[test]
fn help_flag_does_not_suppress_parse_error() {
  // --help appears before the invalid flag but parse still errors (no --help pre-scan)
  let out = run_cli( &[ "--help", "--invalid-flag-xyz" ] );
  assert!( !out.status.success(), "--help before unknown flag must still exit non-zero" );
  let stderr = String::from_utf8_lossy( &out.stderr );
  assert!( stderr.contains( "Error:" ), "error must go to stderr. Got: {stderr}" );
  // No help text should appear on stdout when there's a parse error
  assert!( out.stdout.is_empty(), "stdout must be empty on parse error. Got: {}", String::from_utf8_lossy( &out.stdout ) );
}
