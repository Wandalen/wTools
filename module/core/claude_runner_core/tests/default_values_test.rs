//! Default value tests
//!
//! ## Purpose
//!
//! Verify `ClaudeCommand::new()` sets correct defaults for all parameters.
//!
//! ## Evidence
//!
//! - Tier 1 parameters have different defaults (3.6M, 7.2M, true, false)
//! - Tier 2 & 3 parameters have `None` defaults (inherit standard)
//! - `max_output_tokens` has 200K default (not 32K)
//!
//! ## Measurement
//!
//! Environment variables set on command match expected defaults.
//!
//! ## Null Hypothesis
//!
//! Without correct defaults, bash timeouts would be 2min/10min (too short),
//! `auto_continue` would be false (blocks automation), telemetry would be true
//! (privacy issue), token limit would be 32K (causes errors).

use claude_runner_core::ClaudeCommand;

#[test]
fn default_max_output_tokens_is_200k() {
  // Fix(issue-token-limit-default): Must be 200K not 32K
  let cmd_builder = ClaudeCommand::new();
  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS" ), "max_output_tokens not set" );
  assert!( debug.contains( "200000" ), "Incorrect default: expected 200000" );
}

#[test]
fn default_bash_timeout_is_1_hour() {
  // Fix(issue-bash-timeout-default): Must be 1 hour not 2 minutes
  let cmd_builder = ClaudeCommand::new();
  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_BASH_TIMEOUT" ), "bash_timeout not set" );
  assert!( debug.contains( "3600000" ), "Incorrect default: expected 3600000 (1 hour)" );
}

#[test]
fn default_bash_max_timeout_is_2_hours() {
  // Fix(issue-bash-timeout-default): Must be 2 hours not 10 minutes
  let cmd_builder = ClaudeCommand::new();
  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_BASH_MAX_TIMEOUT" ), "bash_max_timeout not set" );
  assert!( debug.contains( "7200000" ), "Incorrect default: expected 7200000 (2 hours)" );
}

#[test]
fn default_auto_continue_is_true() {
  // Fix(issue-auto-continue-default): Must be true not false
  let cmd_builder = ClaudeCommand::new();
  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_AUTO_CONTINUE" ), "auto_continue not set" );
  assert!( debug.contains( "true" ), "Incorrect default: expected true" );
}

#[test]
fn default_telemetry_is_false() {
  // Fix(issue-telemetry-default): Must be false not true
  let cmd_builder = ClaudeCommand::new();
  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_TELEMETRY" ), "telemetry not set" );
  assert!( debug.contains( "false" ), "Incorrect default: expected false" );
}

#[test]
fn tier2_tier3_defaults_are_none() {
  // Tier 2 & 3 should be None (inherit standard defaults)
  let cmd_builder = ClaudeCommand::new();
  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );

  // None of these should be set
  assert!( !debug.contains( "CLAUDE_CODE_AUTO_APPROVE_TOOLS" ), "auto_approve_tools should be None" );
  assert!( !debug.contains( "CLAUDE_CODE_ACTION_MODE" ), "action_mode should be None" );
  assert!( !debug.contains( "CLAUDE_CODE_LOG_LEVEL" ), "log_level should be None" );
  assert!( !debug.contains( "CLAUDE_CODE_TEMPERATURE" ), "temperature should be None" );
  assert!( !debug.contains( "CLAUDE_CODE_SANDBOX_MODE" ), "sandbox_mode should be None" );
  assert!( !debug.contains( "CLAUDE_CODE_SESSION_DIR" ), "session_dir should be None" );
  assert!( !debug.contains( "CLAUDE_CODE_TOP_P" ), "top_p should be None" );
  assert!( !debug.contains( "CLAUDE_CODE_TOP_K" ), "top_k should be None" );
}

#[test]
fn all_tier1_defaults_set_together() {
  // Verify all 4 Tier 1 defaults are set simultaneously
  let cmd_builder = ClaudeCommand::new();
  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );

  let has_bash_timeout = debug.contains( "CLAUDE_CODE_BASH_TIMEOUT" );
  let has_bash_max_timeout = debug.contains( "CLAUDE_CODE_BASH_MAX_TIMEOUT" );
  let has_auto_continue = debug.contains( "CLAUDE_CODE_AUTO_CONTINUE" );
  let has_telemetry = debug.contains( "CLAUDE_CODE_TELEMETRY" );

  assert!( has_bash_timeout && has_bash_max_timeout && has_auto_continue && has_telemetry,
    "All Tier 1 defaults must be set: bash_timeout={has_bash_timeout}, bash_max_timeout={has_bash_max_timeout}, auto_continue={has_auto_continue}, telemetry={has_telemetry}"
  );
}
