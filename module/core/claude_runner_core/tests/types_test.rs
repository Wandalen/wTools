//! Type definitions tests
//!
//! ## Purpose
//!
//! Verify `ActionMode` and `LogLevel` enums have correct string conversions and defaults.
//!
//! ## Evidence
//!
//! - `ActionMode::as_str()` returns correct lowercase strings
//! - `LogLevel::as_str()` returns correct lowercase strings
//! - `ActionMode::default()` returns `Ask` (security)
//! - `LogLevel::default()` returns `Info`

use claude_runner_core::{ ActionMode, LogLevel };

#[test]
fn action_mode_as_str_conversions() {
  assert_eq!( ActionMode::Ask.as_str(), "ask" );
  assert_eq!( ActionMode::Allow.as_str(), "allow" );
  assert_eq!( ActionMode::Deny.as_str(), "deny" );
}

#[test]
fn action_mode_default_is_ask() {
  // Fix(issue-action-mode-default): Default must be Ask for security
  assert_eq!( ActionMode::default(), ActionMode::Ask );
}

#[test]
fn log_level_as_str_conversions() {
  assert_eq!( LogLevel::Error.as_str(), "error" );
  assert_eq!( LogLevel::Warn.as_str(), "warn" );
  assert_eq!( LogLevel::Info.as_str(), "info" );
  assert_eq!( LogLevel::Debug.as_str(), "debug" );
  assert_eq!( LogLevel::Trace.as_str(), "trace" );
}

#[test]
fn log_level_default_is_info() {
  assert_eq!( LogLevel::default(), LogLevel::Info );
}

#[test]
fn log_level_ordering() {
  // Verify LogLevel has correct ordering (Error < Warn < Info < Debug < Trace)
  assert!( LogLevel::Error < LogLevel::Warn );
  assert!( LogLevel::Warn < LogLevel::Info );
  assert!( LogLevel::Info < LogLevel::Debug );
  assert!( LogLevel::Debug < LogLevel::Trace );
}
