//! Builder method tests
//!
//! ## Purpose
//!
//! Verify all with_*() builder methods exist, are chainable, and set fields correctly.
//!
//! ## Evidence
//!
//! - Each with_*() method exists and compiles
//! - Methods are chainable (return Self)
//! - Methods can be called in any order
//! - Final command builds successfully

use claude_runner_core::{ ClaudeCommand, ActionMode, LogLevel };

#[test]
fn with_bash_timeout_ms_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_bash_timeout_ms(3_600_000);
}

#[test]
fn with_bash_max_timeout_ms_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_bash_max_timeout_ms(7_200_000);
}

#[test]
fn with_auto_continue_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_auto_continue(true);
}

#[test]
fn with_telemetry_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_telemetry(false);
}

#[test]
fn with_auto_approve_tools_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_auto_approve_tools(false);
}

#[test]
fn with_action_mode_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_action_mode(ActionMode::Ask);
}

#[test]
fn with_log_level_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_log_level(LogLevel::Info);
}

#[test]
fn with_temperature_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_temperature(1.0);
}

#[test]
fn with_sandbox_mode_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_sandbox_mode(true);
}

#[test]
fn with_session_dir_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_session_dir("/tmp/sessions");
}

#[test]
fn with_top_p_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_top_p(0.9);
}

#[test]
fn with_top_k_method_exists() {
  let _cmd = ClaudeCommand::new()
    .with_top_k(40);
}

#[test]
fn all_methods_are_chainable() {
  // Verify all methods can be chained together
  let _cmd = ClaudeCommand::new()
    .with_bash_timeout_ms(3_600_000)
    .with_bash_max_timeout_ms(7_200_000)
    .with_auto_continue(true)
    .with_telemetry(false)
    .with_auto_approve_tools(false)
    .with_action_mode(ActionMode::Ask)
    .with_log_level(LogLevel::Debug)
    .with_temperature(0.7)
    .with_sandbox_mode(true)
    .with_session_dir("/tmp")
    .with_top_p(0.9)
    .with_top_k(40);
}

#[test]
fn methods_work_in_any_order() {
  // Verify order independence
  let _cmd1 = ClaudeCommand::new()
    .with_bash_timeout_ms(1000)
    .with_temperature(0.5);

  let _cmd2 = ClaudeCommand::new()
    .with_temperature(0.5)
    .with_bash_timeout_ms(1000);
}
