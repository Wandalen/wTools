//! Environment variable setting tests
//!
//! ## Purpose
//!
//! Verify `build_command()` sets correct environment variables for all parameters.
//!
//! ## Evidence
//!
//! - Each parameter sets its corresponding environment variable
//! - Environment variable names match Claude Code expectations
//! - Values are correctly formatted (strings, booleans, numbers)

use claude_runner_core::{ ClaudeCommand, ActionMode, LogLevel };

#[test]
fn bash_timeout_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_bash_timeout_ms(3_600_000);

  let cmd = cmd_builder.build_command_for_test();

  // Verify CLAUDE_CODE_BASH_TIMEOUT is set
  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_BASH_TIMEOUT" ), "Missing CLAUDE_CODE_BASH_TIMEOUT env var" );
  assert!( debug.contains( "3600000" ), "Incorrect timeout value" );
}

#[test]
fn bash_max_timeout_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_bash_max_timeout_ms(7_200_000);

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_BASH_MAX_TIMEOUT" ), "Missing CLAUDE_CODE_BASH_MAX_TIMEOUT env var" );
  assert!( debug.contains( "7200000" ), "Incorrect max timeout value" );
}

#[test]
fn auto_continue_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_auto_continue(true);

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_AUTO_CONTINUE" ), "Missing CLAUDE_CODE_AUTO_CONTINUE env var" );
  assert!( debug.contains( "true" ), "Incorrect auto_continue value" );
}

#[test]
fn telemetry_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_telemetry(false);

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_TELEMETRY" ), "Missing CLAUDE_CODE_TELEMETRY env var" );
  assert!( debug.contains( "false" ), "Incorrect telemetry value" );
}

#[test]
fn auto_approve_tools_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_auto_approve_tools(true);

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_AUTO_APPROVE_TOOLS" ), "Missing CLAUDE_CODE_AUTO_APPROVE_TOOLS env var" );
  assert!( debug.contains( "true" ), "Incorrect auto_approve_tools value" );
}

#[test]
fn action_mode_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_action_mode(ActionMode::Allow);

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_ACTION_MODE" ), "Missing CLAUDE_CODE_ACTION_MODE env var" );
  assert!( debug.contains( "allow" ), "Incorrect action_mode value" );
}

#[test]
fn log_level_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_log_level(LogLevel::Debug);

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_LOG_LEVEL" ), "Missing CLAUDE_CODE_LOG_LEVEL env var" );
  assert!( debug.contains( "debug" ), "Incorrect log_level value" );
}

#[test]
fn temperature_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_temperature(0.7);

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ), "Missing CLAUDE_CODE_TEMPERATURE env var" );
  assert!( debug.contains( "0.7" ), "Incorrect temperature value" );
}

#[test]
fn sandbox_mode_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_sandbox_mode(false);

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_SANDBOX_MODE" ), "Missing CLAUDE_CODE_SANDBOX_MODE env var" );
  assert!( debug.contains( "false" ), "Incorrect sandbox_mode value" );
}

#[test]
fn session_dir_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_session_dir("/tmp/sessions");

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_SESSION_DIR" ), "Missing CLAUDE_CODE_SESSION_DIR env var" );
  assert!( debug.contains( "/tmp/sessions" ), "Incorrect session_dir value" );
}

#[test]
fn top_p_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_top_p(0.9);

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ), "Missing CLAUDE_CODE_TOP_P env var" );
  assert!( debug.contains( "0.9" ), "Incorrect top_p value" );
}

#[test]
fn top_k_sets_env_var() {
  let cmd_builder = ClaudeCommand::new()
    .with_top_k(40);

  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );
  assert!( debug.contains( "CLAUDE_CODE_TOP_K" ), "Missing CLAUDE_CODE_TOP_K env var" );
  assert!( debug.contains( "40" ), "Incorrect top_k value" );
}

#[test]
fn defaults_set_tier1_env_vars() {
  // Verify Tier 1 defaults are set (different from standard)
  let cmd_builder = ClaudeCommand::new();
  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );

  // Tier 1 should have env vars set
  assert!( debug.contains( "CLAUDE_CODE_BASH_TIMEOUT" ), "Default bash_timeout not set" );
  assert!( debug.contains( "3600000" ), "Incorrect default bash_timeout" );

  assert!( debug.contains( "CLAUDE_CODE_BASH_MAX_TIMEOUT" ), "Default bash_max_timeout not set" );
  assert!( debug.contains( "7200000" ), "Incorrect default bash_max_timeout" );

  assert!( debug.contains( "CLAUDE_CODE_AUTO_CONTINUE" ), "Default auto_continue not set" );
  assert!( debug.contains( "true" ), "Incorrect default auto_continue" );

  assert!( debug.contains( "CLAUDE_CODE_TELEMETRY" ), "Default telemetry not set" );
  assert!( debug.contains( "false" ), "Incorrect default telemetry" );
}

#[test]
fn defaults_do_not_set_tier2_tier3_env_vars() {
  // Verify Tier 2 & 3 defaults are NOT set (inherit standard)
  let cmd_builder = ClaudeCommand::new();
  let cmd = cmd_builder.build_command_for_test();

  let debug = format!( "{cmd:?}" );

  // Tier 2 & 3 should NOT have env vars set (inherit standard defaults)
  assert!( !debug.contains( "CLAUDE_CODE_AUTO_APPROVE_TOOLS" ), "Tier 2 var incorrectly set" );
  assert!( !debug.contains( "CLAUDE_CODE_ACTION_MODE" ), "Tier 2 var incorrectly set" );
  assert!( !debug.contains( "CLAUDE_CODE_LOG_LEVEL" ), "Tier 2 var incorrectly set" );
  assert!( !debug.contains( "CLAUDE_CODE_TEMPERATURE" ), "Tier 2 var incorrectly set" );
  assert!( !debug.contains( "CLAUDE_CODE_SANDBOX_MODE" ), "Tier 3 var incorrectly set" );
  assert!( !debug.contains( "CLAUDE_CODE_SESSION_DIR" ), "Tier 3 var incorrectly set" );
  assert!( !debug.contains( "CLAUDE_CODE_TOP_P" ), "Tier 3 var incorrectly set" );
  assert!( !debug.contains( "CLAUDE_CODE_TOP_K" ), "Tier 3 var incorrectly set" );
}
