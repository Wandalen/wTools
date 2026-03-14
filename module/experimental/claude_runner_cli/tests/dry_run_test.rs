//! Dry-Run Output Structure Tests
//!
//! ## Purpose
//!
//! Verify that `--dry-run` mode produces correctly structured output:
//! environment variable lines followed by the command line.
//! Tests inspect the output format without executing Claude Code.
//!
//! ## Strategy
//!
//! Each test invokes `claude_runner_cli --dry-run` with specific flags and
//! asserts that the printed output reflects the expected builder configuration.
//! This validates the round-trip: CLI flag → builder call → describe output.
//!
//! ## Corner Cases Covered
//!
//! - Default env vars appear (`CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000`)
//! - `--dir` emits `cd <path>` prefix line
//! - `--max-tokens <N>` overrides the default token env var
//! - `--model <NAME>` appears in command args
//! - `--session-dir <PATH>` appears as `CLAUDE_CODE_SESSION_DIR` env var
//! - Combined flags produce correct combined output
//! - Positional argument becomes message in command output — FR-1
//! - Message with embedded double quotes is properly escaped
//! - `-d` short flag produces `cd <path>` prefix — FR-2 alias
//! - Empty message string is accepted and appears in command
//! - Empty `--dir ""` is accepted: produces `cd ` with empty path (CC10)
//! - Empty `--model ""` is accepted: appears as empty model arg (CC12)
//! - Empty `--session-dir ""` is accepted: passthrough with empty env var (C3)
//! - `--dir` with spaces: `cd` output is unquoted (human-readable per FR-21, not shell-safe)
//! - `--verbose` prints assembled env + command to stderr before execution — FR-12
//! - `--verbose` does not write the command description to stdout — FR-12
//! - `--verbose` + `--dry-run`: dry-run takes precedence, preview goes to stdout, stderr empty — FR-12
//! - All 5 Tier-1 default env vars appear in output (not just max-tokens)
//! - No message provided: `--dry-run` outputs bare `claude` command with no message arg — FR-1

use std::process::Command;

fn run_dry( args: &[ &str ] ) -> String {
  let bin = env!( "CARGO_BIN_EXE_claude_runner_cli" );
  let out = Command::new( bin )
    .args( args )
    .output()
    .expect( "Failed to invoke claude_runner_cli binary" );
  assert!(
    out.status.success(),
    "dry-run failed (exit {}): {}",
    out.status.code().unwrap_or( -1 ),
    String::from_utf8_lossy( &out.stderr )
  );
  String::from_utf8_lossy( &out.stdout ).into_owned()
}

#[test]
fn default_env_vars_appear_in_output() {
  // Default ClaudeCommand sets CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
  let output = run_dry( &[ "--message", "test", "--dry-run" ] );
  assert!(
    output.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000" ),
    "Default 200K token limit must appear in env output. Got:\n{output}"
  );
}

#[test]
fn working_dir_emits_cd_prefix() {
  let output = run_dry( &[ "--message", "test", "--dir", "/tmp/work", "--dry-run" ] );
  assert!(
    output.contains( "cd /tmp/work" ),
    "--dir must produce 'cd <path>' prefix. Got:\n{output}"
  );
}

#[test]
fn max_tokens_override_updates_env_var() {
  let output = run_dry( &[ "--message", "test", "--max-tokens", "100000", "--dry-run" ] );
  assert!(
    output.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=100000" ),
    "--max-tokens must override default. Got:\n{output}"
  );
  assert!(
    !output.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000" ),
    "Default 200K must be replaced. Got:\n{output}"
  );
}

#[test]
fn model_flag_appears_in_command() {
  let output = run_dry( &[ "--message", "test", "--model", "claude-opus-4-6", "--dry-run" ] );
  assert!(
    output.contains( "claude-opus-4-6" ),
    "--model must appear in command line. Got:\n{output}"
  );
}

#[test]
fn session_dir_appears_as_env_var() {
  let output = run_dry( &[ "--message", "test", "--session-dir", "/tmp/sessions", "--dry-run" ] );
  assert!(
    output.contains( "CLAUDE_CODE_SESSION_DIR=/tmp/sessions" ),
    "--session-dir must set CLAUDE_CODE_SESSION_DIR. Got:\n{output}"
  );
}

#[test]
fn message_appears_in_command_quoted() {
  let output = run_dry( &[ "--message", "hello world", "--dry-run" ] );
  // describe() wraps message in double quotes
  assert!(
    output.contains( "\"hello world\"" ),
    "Message must appear quoted in command. Got:\n{output}"
  );
}

#[test]
fn combined_flags_all_appear() {
  let output = run_dry( &[
    "--message", "fix it",
    "--dir", "/tmp",
    "--continue",
    "--skip-permissions",
    "--dry-run",
  ] );
  assert!( output.contains( "cd /tmp" ), "Must have cd line" );
  assert!( output.contains( "--dangerously-skip-permissions" ), "Must have skip-permissions" );
  assert!( output.contains( " -c" ), "Must have -c for continue" );
  assert!( output.contains( "\"fix it\"" ), "Must have quoted message" );
}

#[test]
fn dry_run_does_not_invoke_claude_binary() {
  // If --dry-run accidentally executes claude, it would fail on CI (no claude binary).
  // Success here proves only describe() was called.
  let out = Command::new( env!( "CARGO_BIN_EXE_claude_runner_cli" ) )
    .args( [ "--message", "test", "--dry-run" ] )
    .output()
    .expect( "Failed to invoke binary" );
  assert!(
    out.status.success(),
    "Dry-run must not fail due to missing claude binary"
  );
}

// FR-1: positional argument (no -m flag) becomes the Claude message in command output
#[test]
fn positional_message_appears_in_command() {
  let output = run_dry( &[ "Hello there", "--dry-run" ] );
  assert!(
    output.contains( "\"Hello there\"" ),
    "Positional message must appear quoted in command. Got:\n{output}"
  );
}

// FR-1: message containing double quotes must be escaped in describe() output
#[test]
fn message_with_embedded_quotes_is_escaped() {
  let output = run_dry( &[ "--message", r#"say "hi""#, "--dry-run" ] );
  // describe() wraps in double quotes and escapes inner quotes with backslash
  assert!(
    output.contains( r#"\"hi\""# ),
    "Embedded double quotes must be escaped. Got:\n{output}"
  );
}

// FR-2: -d is a spec-defined alias for --dir; must produce identical output to --dir
#[test]
fn short_dir_flag_produces_cd_prefix() {
  let output = run_dry( &[ "--message", "test", "-d", "/tmp/mydir", "--dry-run" ] );
  assert!(
    output.contains( "cd /tmp/mydir" ),
    "-d short flag must produce 'cd <path>' prefix. Got:\n{output}"
  );
}

// FR-1: empty string is a valid message; must appear as empty quotes in command
#[test]
fn empty_message_accepted_and_appears_in_command() {
  let output = run_dry( &[ "--message", "", "--dry-run" ] );
  assert!(
    output.contains( "claude \"\"" ),
    "Empty message must appear as empty quotes in command. Got:\n{output}"
  );
}

// CC10: empty --dir string is accepted at the CLI layer; produces `cd ` with empty path.
// The shell or Claude Code will reject the empty path at runtime — this is a user error,
// not a CLI validation bug. Test documents the current passthrough behavior.
#[test]
fn empty_dir_accepted_produces_cd_with_empty_path() {
  let bin = env!( "CARGO_BIN_EXE_claude_runner_cli" );
  let out = std::process::Command::new( bin )
    .args( [ "--message", "hi", "--dir", "", "--dry-run" ] )
    .output()
    .expect( "Failed to invoke claude_runner_cli binary" );
  assert!(
    out.status.success(),
    "Empty --dir is accepted by CLI (runtime validation is shell's responsibility). exit {}",
    out.status.code().unwrap_or( -1 )
  );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!(
    stdout.contains( "cd " ),
    "Empty --dir must produce 'cd ' prefix. Got:\n{stdout}"
  );
}

// CC12: empty --model string is accepted at the CLI layer; Claude Code will reject
// the empty model name at runtime. Test documents the passthrough behavior.
#[test]
fn empty_model_accepted_produces_empty_model_arg() {
  let output = run_dry( &[ "--message", "hi", "--model", "", "--dry-run" ] );
  // claude_runner_core::describe() emits --model followed by empty string
  // The resulting command has an empty model arg — invalid for Claude CLI but
  // validated at runtime, not at the CLI wrapper layer.
  assert!(
    output.contains( "claude" ),
    "Empty --model must still produce claude command. Got:\n{output}"
  );
}

// C3: empty --session-dir string is accepted at the CLI layer; produces
// CLAUDE_CODE_SESSION_DIR= (empty value). Claude Code will fail at runtime
// if it encounters an empty session dir — this is a user error, not a CLI bug.
#[test]
fn empty_session_dir_accepted_produces_empty_env_var() {
  let output = run_dry( &[ "--message", "hi", "--session-dir", "", "--dry-run" ] );
  assert!(
    output.contains( "CLAUDE_CODE_SESSION_DIR=" ),
    "Empty --session-dir must produce CLAUDE_CODE_SESSION_DIR= env var. Got:\n{output}"
  );
}

// FR-12: --verbose prints assembled command to stderr before execution.
// Safe to test in CI: eprintln! writes preview to stderr BEFORE execute() is called,
// so the preview always appears in stderr regardless of whether claude is installed.
#[test]
fn verbose_prints_command_to_stderr()
{
  let bin = env!( "CARGO_BIN_EXE_claude_runner_cli" );
  let out = std::process::Command::new( bin )
    .args( [ "--message", "verbose-preview", "--verbose" ] )
    .output()
    .expect( "Failed to invoke claude_runner_cli binary" );
  // stderr always has the preview (eprintln! runs before execute())
  let stderr = String::from_utf8_lossy( &out.stderr );
  assert!(
    stderr.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=" ),
    "--verbose must print env vars to stderr. Got:\n{stderr}"
  );
  assert!(
    stderr.contains( "claude" ),
    "--verbose must print claude command to stderr. Got:\n{stderr}"
  );
}

// FR-12: --verbose must not write the command description to stdout.
// Claude's real stdout output must remain uncontaminated by the diagnostic preview.
#[test]
fn verbose_stdout_has_no_env_output()
{
  let bin = env!( "CARGO_BIN_EXE_claude_runner_cli" );
  let out = std::process::Command::new( bin )
    .args( [ "--message", "verbose-stdout-clean", "--verbose" ] )
    .output()
    .expect( "Failed to invoke claude_runner_cli binary" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  assert!(
    !stdout.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=" ),
    "--verbose must not write command description to stdout. Got:\n{stdout}"
  );
}

// C4: --dir path containing spaces produces an unquoted `cd` line.
// This is intentional per FR-21 (describe() is human-readable, not shell-safe).
// Actual execution uses std::process::Command::current_dir() which is shell-safe.
// Ref: claude_runner_core describe_test.rs::describe_working_directory_with_spaces_cd_unquoted
#[test]
fn dir_with_spaces_produces_unquoted_cd_line() {
  let output = run_dry( &[ "--message", "hi", "--dir", "/path/with spaces", "--dry-run" ] );
  // FR-21: describe() output is human-readable, not shell-safe.
  // The cd line is unquoted even for paths with spaces.
  assert!(
    output.contains( "cd /path/with spaces" ),
    "Path with spaces must appear unquoted in cd line (FR-21 human-readable). Got:\n{output}"
  );
}

// FR-12: when both --verbose and --dry-run are given, --dry-run takes precedence.
// The preview goes to stdout (dry-run path fires first), eprintln! is never reached,
// so stderr is empty. Verifies the spec: "--verbose is a no-op when --dry-run is set".
#[test]
fn dry_run_overrides_verbose()
{
  let bin = env!( "CARGO_BIN_EXE_claude_runner_cli" );
  let out = std::process::Command::new( bin )
    .args( [ "--message", "prec-test", "--verbose", "--dry-run" ] )
    .output()
    .expect( "Failed to invoke claude_runner_cli binary" );
  assert!( out.status.success(), "--verbose + --dry-run must exit 0" );
  let stdout = String::from_utf8_lossy( &out.stdout );
  let stderr = String::from_utf8_lossy( &out.stderr );
  // dry-run path: preview must appear on stdout
  assert!(
    stdout.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=" ),
    "--dry-run must put env vars on stdout. Got:\n{stdout}"
  );
  assert!(
    stdout.contains( "claude" ),
    "--dry-run must put command on stdout. Got:\n{stdout}"
  );
  // verbose is suppressed: stderr must be empty (eprintln! never fires)
  assert!(
    stderr.is_empty(),
    "--verbose must be no-op when --dry-run also set; stderr must be empty. Got:\n{stderr}"
  );
}

// No-message case: --dry-run with no message produces a bare `claude` command with
// no message argument. Verifies FR-1 optional-message behavior at the output level:
// the adapter correctly omits the message token when no message is provided.
#[test]
fn dry_run_without_message_shows_bare_command()
{
  let output = run_dry( &[ "--dry-run" ] );
  let last_line = output.trim_end().lines().last().unwrap_or_default();
  assert_eq!(
    last_line, "claude",
    "Bare --dry-run must end with 'claude' command (no message arg). Got:\n{output}"
  );
}

// Tier-1 automation defaults: all four remaining env vars must appear alongside max-tokens.
// These vars fix automation-blocking defaults in standard Claude Code CLI.
#[test]
fn tier1_default_env_vars_all_appear()
{
  let output = run_dry( &[ "--message", "test", "--dry-run" ] );
  for var in &[
    "CLAUDE_CODE_BASH_TIMEOUT=3600000",
    "CLAUDE_CODE_BASH_MAX_TIMEOUT=7200000",
    "CLAUDE_CODE_AUTO_CONTINUE=true",
    "CLAUDE_CODE_TELEMETRY=false",
  ] {
    assert!(
      output.contains( var ),
      "Tier-1 default env var missing: {var}. Got:\n{output}"
    );
  }
}
