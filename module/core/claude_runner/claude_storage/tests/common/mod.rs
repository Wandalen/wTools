//! Shared test utilities for claude_storage integration tests.
//!
//! Provides pre-compiled binary resolution to avoid cargo compilation
//! during test execution. Each `cargo run` inside a test triggers a full
//! compilation cycle (300s+), causing timeouts under workspace-wide runs.
//!
//! Fix(issue-claude-storage-timeout)
//! Root cause: 85 `Command::new("cargo").args(["run", ...])` calls across
//! 13 test files each triggered cargo compilation during test execution.
//! Under workspace-wide nextest runs, the 300s timeout was exceeded.
//! Pitfall: Never use `cargo run` or `cargo build` inside tests — use
//! `cargo_bin!()` to get the pre-compiled binary path from nextest.

/// Return a Command pointing to the pre-compiled claude_storage binary.
///
/// Uses `cargo_bin!()` macro which resolves to the binary built by
/// nextest/cargo-test BEFORE test execution. No compilation at test time.
pub fn claude_storage_cmd() -> std::process::Command
{
  std::process::Command::new( assert_cmd::cargo::cargo_bin!( "claude_storage" ) )
}
